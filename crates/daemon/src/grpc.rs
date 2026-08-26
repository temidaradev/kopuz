//! The tonic shell over a running session: the gRPC wire.
//!
//! Reads and mutations are unary RPCs. Event delivery is a server-streaming
//! subscription with a replay cursor, so the wire follows gRPC's native
//! request/response and streaming semantics instead of simulating an HTTP
//! request/response protocol inside a bidirectional stream.
//! Bearer auth rides metadata and is checked constant-time; the reflection
//! services are served unauthenticated so `grpcurl` can list the schema,
//! which is public in the repository anyway.

use std::collections::VecDeque;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use api::{ApiError, ApiEvent, KopuzApi};
use futures_util::{Stream, StreamExt};
use proto::convert;
use proto::kopuz_server::{Kopuz, KopuzServer};
use tokio::sync::broadcast;
use tonic::{Request, Response, Status};

use crate::session::SessionHandle;

pub struct GrpcState {
    pub api: Arc<dyn KopuzApi>,
    /// Event source with sequence numbers and the replay ring; the trait's
    /// `events()` strips ids, and Subscribe cursors need them.
    pub session: SessionHandle,
    pub token: String,
    pub started: Instant,
    /// Notified by `Shutdown` so a remote frontend can quit the daemon
    /// completely. `None` (the embedded GUI, tests) answers unsupported.
    pub shutdown: Option<Arc<tokio::sync::Notify>>,
}

pub struct KopuzGrpc(Arc<GrpcState>);

struct SubscribeLogGuard {
    peer: Option<SocketAddr>,
}

impl Drop for SubscribeLogGuard {
    fn drop(&mut self) {
        tracing::info!(peer = ?self.peer, "daemon frontend detached");
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let mut difference = a.len() ^ b.len();
    for index in 0..a.len().max(b.len()) {
        difference |= usize::from(
            a.get(index).copied().unwrap_or_default() ^ b.get(index).copied().unwrap_or_default(),
        );
    }
    difference == 0
}

fn failed(error: ApiError) -> Status {
    proto::status::to_status(error)
}

macro_rules! api_call {
    ($server:expr, $method:ident $(, $argument:expr)* $(,)?) => {
        $server
            .0
            .api
            .$method($($argument),*)
            .await
            .map_err(failed)?
    };
}

const ARTWORK_CHUNK: usize = 256 * 1024;

type ServerStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + 'static>>;

fn event_message(sequence: u64, event: &ApiEvent) -> proto::EventEnvelope {
    proto::EventEnvelope {
        sequence,
        event: Some(convert::event_to_proto(event)),
    }
}

fn resync_message() -> proto::EventEnvelope {
    event_message(0, &ApiEvent::Resync)
}

impl KopuzGrpc {
    async fn player_mutation(
        &self,
        command: api::PlayerCommand,
    ) -> Result<Response<proto::MutationResult>, Status> {
        let ack = api_call!(self, player_command, command);
        Ok(Response::new(proto::MutationResult { rev: ack.rev }))
    }
}

/// Replay backlog first, then the live broadcast. `floor` drops the live
/// events the replay snapshot already covered.
struct EventSubscription {
    pending: VecDeque<proto::EventEnvelope>,
    live: broadcast::Receiver<(u64, ApiEvent)>,
    floor: u64,
    _log_guard: SubscribeLogGuard,
}

impl EventSubscription {
    async fn next(mut self) -> Option<(Result<proto::EventEnvelope, Status>, Self)> {
        if let Some(message) = self.pending.pop_front() {
            return Some((Ok(message), self));
        }

        loop {
            match self.live.recv().await {
                Ok((sequence, event)) => {
                    if sequence <= self.floor {
                        continue;
                    }
                    self.floor = sequence;
                    return Some((Ok(event_message(sequence, &event)), self));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    return Some((Ok(resync_message()), self));
                }
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }
}

#[tonic::async_trait]
impl Kopuz for KopuzGrpc {
    type SubscribeStream = ServerStream<proto::EventEnvelope>;
    type GetArtworkStream = ServerStream<proto::ArtworkChunk>;
    type StreamLyricsStream = ServerStream<proto::Lyrics>;

    async fn subscribe(
        &self,
        request: Request<proto::SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let peer = request.remote_addr();
        let after_sequence = request.into_inner().after_sequence;
        // Subscribe before taking the replay snapshot so events emitted while
        // this RPC is being set up remain buffered for the live phase.
        let live = self.0.session.subscribe();
        let (needs_resync, replayed) = if after_sequence > 0 {
            self.0.session.replay_since(after_sequence)
        } else {
            (false, Vec::new())
        };
        let floor = replayed
            .last()
            .map(|(sequence, _)| *sequence)
            .or_else(|| (!needs_resync && after_sequence > 0).then_some(after_sequence))
            .unwrap_or(0);
        let mut pending = VecDeque::new();
        if needs_resync {
            pending.push_back(resync_message());
        }
        pending.extend(
            replayed
                .iter()
                .map(|(sequence, event)| event_message(*sequence, event)),
        );
        tracing::info!(
            peer = ?peer,
            after_sequence,
            replayed = replayed.len(),
            resync = needs_resync,
            "daemon frontend attached"
        );
        let subscription = EventSubscription {
            pending,
            live,
            floor,
            _log_guard: SubscribeLogGuard { peer },
        };
        let stream = futures_util::stream::unfold(subscription, |subscription| subscription.next());
        Ok(Response::new(Box::pin(stream)))
    }

    async fn play(
        &self,
        _request: Request<proto::PlayRequest>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::Play).await
    }

    async fn pause(
        &self,
        _request: Request<proto::PauseRequest>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::Pause).await
    }

    async fn toggle(
        &self,
        _request: Request<proto::ToggleRequest>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::Toggle).await
    }

    async fn next(
        &self,
        _request: Request<proto::NextRequest>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::Next).await
    }

    async fn previous(
        &self,
        _request: Request<proto::PreviousRequest>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::Previous).await
    }

    async fn stop(
        &self,
        _request: Request<proto::StopRequest>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::Stop).await
    }

    async fn seek(
        &self,
        request: Request<proto::Seek>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::Seek {
            position_ms: request.get_ref().position_ms,
        })
        .await
    }

    async fn set_volume(
        &self,
        request: Request<proto::SetVolume>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::SetVolume {
            volume: request.get_ref().volume,
        })
        .await
    }

    async fn set_mode(
        &self,
        request: Request<proto::SetMode>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        self.player_mutation(api::PlayerCommand::SetMode {
            shuffle: request.get_ref().shuffle,
            loop_mode: request.get_ref().r#loop.map(convert::loop_from_proto),
        })
        .await
    }

    async fn get_status(
        &self,
        _request: Request<proto::GetStatusRequest>,
    ) -> Result<Response<proto::DaemonStatus>, Status> {
        Ok(Response::new(proto::DaemonStatus {
            version: env!("CARGO_PKG_VERSION").to_string(),
            api_version: api::API_VERSION,
            uptime_secs: self.0.started.elapsed().as_secs(),
        }))
    }

    async fn get_player_state(
        &self,
        _request: Request<proto::GetPlayerStateRequest>,
    ) -> Result<Response<proto::PlayerState>, Status> {
        let state = api_call!(self, player_state);
        Ok(Response::new(convert::player_state_to_proto(&state)))
    }

    async fn get_queue(
        &self,
        request: Request<proto::Page>,
    ) -> Result<Response<proto::QueueWindow>, Status> {
        let page = convert::page_from_proto(Some(request.get_ref()));
        let window = api_call!(self, queue_window, page);
        Ok(Response::new(convert::queue_window_to_proto(&window)))
    }

    async fn get_queue_snapshot(
        &self,
        _request: Request<proto::GetQueueSnapshotRequest>,
    ) -> Result<Response<proto::QueuePersistenceSnapshot>, Status> {
        let snapshot = api_call!(self, queue_snapshot);
        Ok(Response::new(convert::queue_persistence_snapshot_to_proto(
            &snapshot,
        )))
    }

    async fn get_tracks(
        &self,
        request: Request<proto::TracksRequest>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let request = request.get_ref();
        let filter = request
            .filter
            .as_ref()
            .map(convert::track_filter_from_proto)
            .unwrap_or_default();
        let page = convert::page_from_proto(request.page.as_ref());
        let tracks = api_call!(self, tracks, filter, page);
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn get_folder_tracks(
        &self,
        request: Request<proto::FolderRequest>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let request = request.get_ref();
        let page = convert::page_from_proto(request.page.as_ref());
        let tracks = api_call!(self, folder_tracks, request.prefix.clone(), page);
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn get_stats(
        &self,
        _request: Request<proto::GetStatsRequest>,
    ) -> Result<Response<proto::Stats>, Status> {
        let stats = api_call!(self, stats);
        Ok(Response::new(convert::stats_to_proto(&stats)))
    }

    async fn get_lyrics(
        &self,
        request: Request<proto::TrackRef>,
    ) -> Result<Response<proto::Lyrics>, Status> {
        let lyrics = api_call!(self, lyrics, request.get_ref().key.clone());
        Ok(Response::new(convert::lyrics_to_proto(&lyrics)))
    }

    #[allow(clippy::result_large_err)]
    async fn stream_lyrics(
        &self,
        request: Request<proto::TrackRef>,
    ) -> Result<Response<Self::StreamLyricsStream>, Status> {
        let stream = self
            .0
            .api
            .lyrics_stream(request.get_ref().key.clone())
            .map(|result| {
                result
                    .map(|lyrics| convert::lyrics_to_proto(&lyrics))
                    .map_err(failed)
            });
        Ok(Response::new(Box::pin(stream)))
    }

    async fn get_favorites(
        &self,
        _request: Request<proto::GetFavoritesRequest>,
    ) -> Result<Response<proto::Favorites>, Status> {
        let favorites = api_call!(self, favorites);
        Ok(Response::new(convert::favorites_to_proto(&favorites)))
    }

    async fn get_jobs(
        &self,
        _request: Request<proto::GetJobsRequest>,
    ) -> Result<Response<proto::JobList>, Status> {
        let jobs = api_call!(self, jobs);
        Ok(Response::new(proto::JobList {
            jobs: jobs.iter().map(convert::job_status_to_proto).collect(),
        }))
    }

    async fn get_downloads(
        &self,
        _request: Request<proto::GetDownloadsRequest>,
    ) -> Result<Response<proto::DownloadList>, Status> {
        let keys = api_call!(self, downloads);
        Ok(Response::new(proto::DownloadList { keys }))
    }

    async fn get_download_statuses(
        &self,
        _request: Request<proto::GetDownloadStatusesRequest>,
    ) -> Result<Response<proto::DownloadStatusList>, Status> {
        let statuses = api_call!(self, download_statuses);
        Ok(Response::new(proto::DownloadStatusList {
            statuses: statuses
                .iter()
                .map(convert::download_status_to_proto)
                .collect(),
        }))
    }

    async fn get_config(
        &self,
        _request: Request<proto::GetConfigRequest>,
    ) -> Result<Response<proto::ConfigView>, Status> {
        let view = api_call!(self, config);
        Ok(Response::new(convert::config_view_to_proto(&view)))
    }

    async fn get_albums(
        &self,
        request: Request<proto::AlbumsRequest>,
    ) -> Result<Response<proto::AlbumPage>, Status> {
        let request = request.get_ref();
        let filter = request
            .filter
            .as_ref()
            .map(convert::album_filter_from_proto)
            .unwrap_or_default();
        let page = convert::page_from_proto(request.page.as_ref());
        let albums = api_call!(self, albums, filter, page);
        Ok(Response::new(convert::album_page_to_proto(&albums)))
    }

    async fn get_album(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::AlbumInfo>, Status> {
        let album = api_call!(self, album, request.get_ref().id.clone());
        Ok(Response::new(convert::album_info_to_proto(&album)))
    }

    async fn get_artists(
        &self,
        request: Request<proto::Page>,
    ) -> Result<Response<proto::ArtistPage>, Status> {
        let artists = api_call!(
            self,
            artists,
            convert::page_from_proto(Some(request.get_ref()))
        );
        Ok(Response::new(convert::artist_page_to_proto(&artists)))
    }

    async fn refresh_artist_artwork(
        &self,
        request: Request<proto::StringList>,
    ) -> Result<Response<proto::StringList>, Status> {
        let values = api_call!(self, refresh_artist_artwork, request.into_inner().values);
        Ok(Response::new(proto::StringList { values }))
    }

    async fn get_genres(
        &self,
        _request: Request<proto::GetGenresRequest>,
    ) -> Result<Response<proto::StringList>, Status> {
        let values = api_call!(self, genres);
        Ok(Response::new(proto::StringList { values }))
    }

    async fn get_recent_tracks(
        &self,
        request: Request<proto::Page>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let tracks = api_call!(
            self,
            recent_tracks,
            convert::page_from_proto(Some(request.get_ref()))
        );
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn get_album_tracks(
        &self,
        request: Request<proto::EntityPage>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let request = request.get_ref();
        let tracks = api_call!(
            self,
            album_tracks,
            request.value.clone(),
            convert::page_from_proto(request.page.as_ref())
        );
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn get_artist_tracks(
        &self,
        request: Request<proto::EntityPage>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let request = request.get_ref();
        let tracks = api_call!(
            self,
            artist_tracks,
            request.value.clone(),
            convert::page_from_proto(request.page.as_ref())
        );
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn get_genre_tracks(
        &self,
        request: Request<proto::EntityPage>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let request = request.get_ref();
        let tracks = api_call!(
            self,
            genre_tracks,
            request.value.clone(),
            convert::page_from_proto(request.page.as_ref())
        );
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn get_artist_sample_tracks(
        &self,
        request: Request<proto::Page>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let tracks = api_call!(
            self,
            artist_sample_tracks,
            convert::page_from_proto(Some(request.get_ref()))
        );
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn get_tracks_by_keys(
        &self,
        request: Request<proto::TrackKeysRequest>,
    ) -> Result<Response<proto::TrackList>, Status> {
        let tracks = api_call!(self, tracks_by_keys, request.get_ref().keys.clone());
        Ok(Response::new(proto::TrackList {
            tracks: tracks.iter().map(convert::track_info_to_proto).collect(),
        }))
    }

    async fn get_track_web_url(
        &self,
        request: Request<proto::TrackRef>,
    ) -> Result<Response<proto::OptionalString>, Status> {
        let value = api_call!(self, track_web_url, request.get_ref().key.clone());
        Ok(Response::new(proto::OptionalString { value }))
    }

    async fn get_album_web_url(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::OptionalString>, Status> {
        let value = api_call!(self, album_web_url, request.get_ref().id.clone());
        Ok(Response::new(proto::OptionalString { value }))
    }

    async fn get_top_genre(
        &self,
        _request: Request<proto::GetTopGenreRequest>,
    ) -> Result<Response<proto::OptionalString>, Status> {
        let value = api_call!(self, top_genre);
        Ok(Response::new(proto::OptionalString { value }))
    }

    async fn search(
        &self,
        request: Request<proto::SearchRequest>,
    ) -> Result<Response<proto::SearchResults>, Status> {
        let results = api_call!(self, search, request.get_ref().query.clone());
        Ok(Response::new(convert::search_results_to_proto(&results)))
    }

    async fn get_playlists(
        &self,
        _request: Request<proto::GetPlaylistsRequest>,
    ) -> Result<Response<proto::PlaylistCatalog>, Status> {
        let catalog = api_call!(self, playlists);
        Ok(Response::new(convert::playlist_catalog_to_proto(&catalog)))
    }

    async fn get_playlist_tracks(
        &self,
        request: Request<proto::PlaylistTracksRequest>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let request = request.get_ref();
        let tracks = api_call!(
            self,
            playlist_tracks,
            api::PlaylistTracksRequest {
                id: request.id.clone(),
                page: convert::page_from_proto(request.page.as_ref()),
            }
        );
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn refresh_playlist(
        &self,
        request: Request<proto::PlaylistTracksRequest>,
    ) -> Result<Response<proto::TrackPage>, Status> {
        let request = request.get_ref();
        let tracks = api_call!(
            self,
            refresh_playlist,
            api::PlaylistTracksRequest {
                id: request.id.clone(),
                page: convert::page_from_proto(request.page.as_ref()),
            }
        );
        Ok(Response::new(convert::track_page_to_proto(&tracks)))
    }

    async fn get_sources(
        &self,
        _request: Request<proto::GetSourcesRequest>,
    ) -> Result<Response<proto::SourceList>, Status> {
        let sources = api_call!(self, sources);
        Ok(Response::new(proto::SourceList {
            sources: sources.iter().map(convert::source_info_to_proto).collect(),
        }))
    }

    async fn validate_source(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::SourceValidation>, Status> {
        let state = api_call!(self, validate_source, request.get_ref().id.clone());
        Ok(Response::new(proto::SourceValidation {
            state: convert::source_state_to_proto(state) as i32,
        }))
    }

    async fn get_external_access(
        &self,
        request: Request<proto::ExternalAccessRequest>,
    ) -> Result<Response<proto::ExternalAccess>, Status> {
        let access = api_call!(self, external_access, request.get_ref().kind.clone());
        Ok(Response::new(convert::external_access_to_proto(&access)))
    }

    async fn browse_source(
        &self,
        request: Request<proto::SourceFolderRequest>,
    ) -> Result<Response<proto::SourceFolderList>, Status> {
        let request = request.get_ref();
        let entries = api_call!(
            self,
            browse_source,
            request.server_id.clone(),
            request.path.clone()
        );
        Ok(Response::new(proto::SourceFolderList {
            entries: entries
                .iter()
                .map(convert::source_folder_to_proto)
                .collect(),
        }))
    }

    async fn get_integration_credentials(
        &self,
        _request: Request<proto::GetIntegrationCredentialsRequest>,
    ) -> Result<Response<proto::IntegrationCredentialStatusList>, Status> {
        let statuses = api_call!(self, integration_credentials);
        Ok(Response::new(proto::IntegrationCredentialStatusList {
            statuses: statuses
                .iter()
                .map(convert::integration_status_to_proto)
                .collect(),
        }))
    }

    async fn get_catalog(
        &self,
        request: Request<proto::CatalogRequest>,
    ) -> Result<Response<proto::CatalogPage>, Status> {
        let page = api_call!(self, catalog, request.get_ref().continuation.clone());
        Ok(Response::new(convert::catalog_page_to_proto(&page)))
    }

    async fn get_catalog_detail(
        &self,
        request: Request<proto::CatalogDetailRequest>,
    ) -> Result<Response<proto::CatalogDetail>, Status> {
        let detail = api_call!(
            self,
            catalog_detail,
            convert::catalog_detail_request_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::catalog_detail_to_proto(&detail)))
    }

    async fn get_radio_stations(
        &self,
        _request: Request<proto::GetRadioStationsRequest>,
    ) -> Result<Response<proto::RadioStationList>, Status> {
        let stations = api_call!(self, radio_stations);
        Ok(Response::new(proto::RadioStationList {
            stations: stations
                .iter()
                .map(convert::radio_station_to_proto)
                .collect(),
        }))
    }

    async fn start_track_radio(
        &self,
        request: Request<proto::TrackRef>,
    ) -> Result<Response<proto::TrackList>, Status> {
        let tracks = api_call!(self, track_radio, request.into_inner().key);
        Ok(Response::new(proto::TrackList {
            tracks: tracks.iter().map(convert::track_info_to_proto).collect(),
        }))
    }

    async fn start_playlist_radio(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::TrackList>, Status> {
        let tracks = api_call!(self, playlist_radio, request.into_inner().id);
        Ok(Response::new(proto::TrackList {
            tracks: tracks.iter().map(convert::track_info_to_proto).collect(),
        }))
    }

    async fn search_radio(
        &self,
        request: Request<proto::RadioSearchRequest>,
    ) -> Result<Response<proto::RadioStationList>, Status> {
        let request = request.get_ref();
        let stations = api_call!(self, search_radio, request.query.clone(), request.limit);
        Ok(Response::new(proto::RadioStationList {
            stations: stations
                .iter()
                .map(convert::radio_station_to_proto)
                .collect(),
        }))
    }

    async fn get_radio_registries(
        &self,
        _request: Request<proto::GetRadioRegistriesRequest>,
    ) -> Result<Response<proto::RadioRegistryList>, Status> {
        let registries = api_call!(self, radio_registries);
        Ok(Response::new(proto::RadioRegistryList {
            registries: registries
                .iter()
                .map(convert::radio_registry_to_proto)
                .collect(),
        }))
    }

    async fn set_queue(
        &self,
        request: Request<proto::SetQueueRequest>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        let request = convert::set_queue_from_proto(request.get_ref())
            .ok_or_else(|| Status::invalid_argument("missing queue context"))?;
        let ack = api_call!(self, set_queue, request);
        Ok(Response::new(proto::MutationResult { rev: ack.rev }))
    }

    async fn edit_queue(
        &self,
        request: Request<proto::QueueEditRequest>,
    ) -> Result<Response<proto::MutationResult>, Status> {
        let edit = convert::queue_edit_from_proto(request.get_ref())
            .ok_or_else(|| Status::invalid_argument("missing queue edit op"))?;
        let ack = api_call!(self, queue_edit, edit);
        Ok(Response::new(proto::MutationResult { rev: ack.rev }))
    }

    async fn save_queue_snapshot(
        &self,
        request: Request<proto::QueuePersistenceSnapshot>,
    ) -> Result<Response<proto::SaveQueueSnapshotResponse>, Status> {
        api_call!(
            self,
            save_queue_snapshot,
            convert::queue_persistence_snapshot_from_proto(request.get_ref())
        );
        Ok(Response::new(proto::SaveQueueSnapshotResponse {}))
    }

    async fn set_favorite(
        &self,
        request: Request<proto::FavoriteRequest>,
    ) -> Result<Response<proto::SetFavoriteResponse>, Status> {
        let request = request.get_ref();
        api_call!(self, set_favorite, request.key.clone(), request.favorite);
        Ok(Response::new(proto::SetFavoriteResponse {}))
    }

    async fn start_job(
        &self,
        request: Request<proto::StartJobRequest>,
    ) -> Result<Response<proto::JobRef>, Status> {
        let kind = convert::job_kind_from_proto(request.get_ref().kind);
        let job = api_call!(self, start_job, kind);
        Ok(Response::new(proto::JobRef { job_id: job.job_id }))
    }

    async fn cancel_job(
        &self,
        request: Request<proto::JobId>,
    ) -> Result<Response<proto::CancelJobResponse>, Status> {
        api_call!(self, cancel_job, request.get_ref().id.clone());
        Ok(Response::new(proto::CancelJobResponse {}))
    }

    async fn start_downloads(
        &self,
        request: Request<proto::DownloadRequest>,
    ) -> Result<Response<proto::JobRef>, Status> {
        let job = api_call!(self, download, request.get_ref().keys.clone());
        Ok(Response::new(proto::JobRef { job_id: job.job_id }))
    }

    async fn remove_download(
        &self,
        request: Request<proto::TrackRef>,
    ) -> Result<Response<proto::RemoveDownloadResponse>, Status> {
        api_call!(self, remove_download, request.get_ref().key.clone());
        Ok(Response::new(proto::RemoveDownloadResponse {}))
    }

    async fn cancel_download_item(
        &self,
        request: Request<proto::TrackRef>,
    ) -> Result<Response<proto::CancelDownloadItemResponse>, Status> {
        api_call!(self, cancel_download_item, request.get_ref().key.clone());
        Ok(Response::new(proto::CancelDownloadItemResponse {}))
    }

    async fn start_ytdlp(
        &self,
        request: Request<proto::YtdlpRequest>,
    ) -> Result<Response<proto::JobRef>, Status> {
        let job = api_call!(
            self,
            start_ytdlp,
            convert::ytdlp_request_from_proto(request.get_ref()).map_err(failed)?
        );
        Ok(Response::new(proto::JobRef { job_id: job.job_id }))
    }

    async fn patch_config(
        &self,
        request: Request<proto::ConfigPatch>,
    ) -> Result<Response<proto::ConfigView>, Status> {
        let patch: serde_json::Value = serde_json::from_str(&request.get_ref().merge_patch_json)
            .map_err(|error| Status::invalid_argument(format!("invalid merge patch: {error}")))?;
        let view = api_call!(self, patch_config, patch);
        Ok(Response::new(convert::config_view_to_proto(&view)))
    }

    async fn create_playlist(
        &self,
        request: Request<proto::CreatePlaylistRequest>,
    ) -> Result<Response<proto::EntityRef>, Status> {
        let request = request.get_ref();
        let id = api_call!(
            self,
            create_playlist,
            request.name.clone(),
            request.keys.clone()
        );
        Ok(Response::new(proto::EntityRef { id }))
    }

    async fn rename_playlist(
        &self,
        request: Request<proto::NamedEntity>,
    ) -> Result<Response<proto::RenamePlaylistResponse>, Status> {
        let request = request.get_ref();
        api_call!(
            self,
            rename_playlist,
            request.id.clone(),
            request.name.clone()
        );
        Ok(Response::new(proto::RenamePlaylistResponse {}))
    }

    async fn delete_playlist(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::DeletePlaylistResponse>, Status> {
        api_call!(self, delete_playlist, request.get_ref().id.clone());
        Ok(Response::new(proto::DeletePlaylistResponse {}))
    }

    async fn add_playlist_tracks(
        &self,
        request: Request<proto::PlaylistKeysRequest>,
    ) -> Result<Response<proto::AddPlaylistTracksResponse>, Status> {
        let request = request.get_ref();
        api_call!(
            self,
            add_playlist_tracks,
            request.id.clone(),
            request.keys.clone()
        );
        Ok(Response::new(proto::AddPlaylistTracksResponse {}))
    }

    async fn remove_playlist_tracks(
        &self,
        request: Request<proto::PlaylistKeysRequest>,
    ) -> Result<Response<proto::RemovePlaylistTracksResponse>, Status> {
        let request = request.get_ref();
        api_call!(
            self,
            remove_playlist_tracks,
            request.id.clone(),
            request.keys.clone()
        );
        Ok(Response::new(proto::RemovePlaylistTracksResponse {}))
    }

    async fn reorder_playlist_tracks(
        &self,
        request: Request<proto::PlaylistKeysRequest>,
    ) -> Result<Response<proto::ReorderPlaylistTracksResponse>, Status> {
        let request = request.get_ref();
        api_call!(
            self,
            reorder_playlist_tracks,
            request.id.clone(),
            request.keys.clone()
        );
        Ok(Response::new(proto::ReorderPlaylistTracksResponse {}))
    }

    async fn create_playlist_folder(
        &self,
        request: Request<proto::Name>,
    ) -> Result<Response<proto::EntityRef>, Status> {
        let id = api_call!(self, create_playlist_folder, request.get_ref().name.clone());
        Ok(Response::new(proto::EntityRef { id }))
    }

    async fn rename_playlist_folder(
        &self,
        request: Request<proto::NamedEntity>,
    ) -> Result<Response<proto::RenamePlaylistFolderResponse>, Status> {
        let request = request.get_ref();
        api_call!(
            self,
            rename_playlist_folder,
            request.id.clone(),
            request.name.clone()
        );
        Ok(Response::new(proto::RenamePlaylistFolderResponse {}))
    }

    async fn delete_playlist_folder(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::DeletePlaylistFolderResponse>, Status> {
        api_call!(self, delete_playlist_folder, request.get_ref().id.clone());
        Ok(Response::new(proto::DeletePlaylistFolderResponse {}))
    }

    async fn move_playlist(
        &self,
        request: Request<proto::MovePlaylistRequest>,
    ) -> Result<Response<proto::MovePlaylistResponse>, Status> {
        let request = request.get_ref();
        api_call!(
            self,
            move_playlist,
            request.id.clone(),
            request.folder_id.clone()
        );
        Ok(Response::new(proto::MovePlaylistResponse {}))
    }

    async fn switch_source(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::SourceInfo>, Status> {
        let source = api_call!(self, switch_source, request.get_ref().id.clone());
        Ok(Response::new(convert::source_info_to_proto(&source)))
    }

    async fn upsert_local_source(
        &self,
        request: Request<proto::LocalSourceDraft>,
    ) -> Result<Response<proto::SourceInfo>, Status> {
        let source = api_call!(
            self,
            upsert_local_source,
            convert::local_source_draft_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::source_info_to_proto(&source)))
    }

    async fn delete_local_source(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::DeleteLocalSourceResponse>, Status> {
        api_call!(self, delete_local_source, request.get_ref().id.clone());
        Ok(Response::new(proto::DeleteLocalSourceResponse {}))
    }

    async fn set_source_directories(
        &self,
        request: Request<proto::SourceDirectories>,
    ) -> Result<Response<proto::SourceInfo>, Status> {
        let request = request.get_ref();
        let source = api_call!(
            self,
            set_source_directories,
            request.id.clone(),
            request.directories.clone()
        );
        Ok(Response::new(convert::source_info_to_proto(&source)))
    }

    async fn upsert_server(
        &self,
        request: Request<proto::ServerDraft>,
    ) -> Result<Response<proto::SourceInfo>, Status> {
        let source = api_call!(
            self,
            upsert_server,
            convert::server_draft_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::source_info_to_proto(&source)))
    }

    async fn delete_server(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::DeleteServerResponse>, Status> {
        api_call!(self, delete_server, request.get_ref().id.clone());
        Ok(Response::new(proto::DeleteServerResponse {}))
    }

    async fn provision_credentials(
        &self,
        request: Request<proto::CredentialProvision>,
    ) -> Result<Response<proto::SourceInfo>, Status> {
        let source = api_call!(
            self,
            provision_credentials,
            convert::credential_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::source_info_to_proto(&source)))
    }

    async fn login_source(
        &self,
        request: Request<proto::SourceLoginRequest>,
    ) -> Result<Response<proto::SourceInfo>, Status> {
        let source = api_call!(
            self,
            login_source,
            convert::source_login_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::source_info_to_proto(&source)))
    }

    async fn clear_credentials(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::ClearCredentialsResponse>, Status> {
        api_call!(self, clear_credentials, request.get_ref().id.clone());
        Ok(Response::new(proto::ClearCredentialsResponse {}))
    }

    async fn authenticate_source(
        &self,
        request: Request<proto::EntityRef>,
    ) -> Result<Response<proto::SourceInfo>, Status> {
        let source = api_call!(self, authenticate_source, request.get_ref().id.clone());
        Ok(Response::new(convert::source_info_to_proto(&source)))
    }

    async fn provision_integration_credentials(
        &self,
        request: Request<proto::IntegrationCredentialProvision>,
    ) -> Result<Response<proto::IntegrationCredentialStatus>, Status> {
        let status = api_call!(
            self,
            provision_integration_credentials,
            convert::integration_provision_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::integration_status_to_proto(&status)))
    }

    async fn clear_integration_credentials(
        &self,
        request: Request<proto::IntegrationRef>,
    ) -> Result<Response<proto::ClearIntegrationCredentialsResponse>, Status> {
        api_call!(
            self,
            clear_integration_credentials,
            convert::integration_kind_from_proto(request.get_ref().kind)
        );
        Ok(Response::new(proto::ClearIntegrationCredentialsResponse {}))
    }

    async fn authenticate_integration(
        &self,
        request: Request<proto::IntegrationCredentialProvision>,
    ) -> Result<Response<proto::IntegrationCredentialStatus>, Status> {
        let status = api_call!(
            self,
            authenticate_integration,
            convert::integration_provision_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::integration_status_to_proto(&status)))
    }

    async fn set_external_playback(
        &self,
        request: Request<proto::SetExternalPlaybackRequest>,
    ) -> Result<Response<proto::SetExternalPlaybackResponse>, Status> {
        let external = request
            .get_ref()
            .external
            .as_ref()
            .map(|external| api::ExternalPlayback {
                kind: external.kind.clone(),
                device: external.device.clone(),
            });
        api_call!(self, set_external_playback, external);
        Ok(Response::new(proto::SetExternalPlaybackResponse {}))
    }

    async fn claim_external_playback(
        &self,
        request: Request<proto::ExternalPlayback>,
    ) -> Result<Response<proto::ExternalPlaybackLease>, Status> {
        let lease = api_call!(
            self,
            claim_external_playback,
            convert::external_playback_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::external_lease_to_proto(&lease)))
    }

    async fn report_external_playback(
        &self,
        request: Request<proto::ExternalPlaybackReport>,
    ) -> Result<Response<proto::ReportExternalPlaybackResponse>, Status> {
        api_call!(
            self,
            report_external_playback,
            convert::external_report_from_proto(request.get_ref())
        );
        Ok(Response::new(proto::ReportExternalPlaybackResponse {}))
    }

    async fn release_external_playback(
        &self,
        request: Request<proto::ExternalPlaybackLease>,
    ) -> Result<Response<proto::ReleaseExternalPlaybackResponse>, Status> {
        api_call!(
            self,
            release_external_playback,
            request.get_ref().lease_id.clone()
        );
        Ok(Response::new(proto::ReleaseExternalPlaybackResponse {}))
    }

    async fn add_radio_registry(
        &self,
        request: Request<proto::RegistryRequest>,
    ) -> Result<Response<proto::AddRadioRegistryResponse>, Status> {
        api_call!(self, add_radio_registry, request.get_ref().url.clone());
        Ok(Response::new(proto::AddRadioRegistryResponse {}))
    }

    async fn remove_radio_registry(
        &self,
        request: Request<proto::RegistryRequest>,
    ) -> Result<Response<proto::RemoveRadioRegistryResponse>, Status> {
        api_call!(self, remove_radio_registry, request.get_ref().url.clone());
        Ok(Response::new(proto::RemoveRadioRegistryResponse {}))
    }

    async fn set_radio_registry_enabled(
        &self,
        request: Request<proto::SetRegistryEnabledRequest>,
    ) -> Result<Response<proto::SetRadioRegistryEnabledResponse>, Status> {
        let request = request.get_ref();
        api_call!(
            self,
            set_radio_registry_enabled,
            request.url.clone(),
            request.enabled
        );
        Ok(Response::new(proto::SetRadioRegistryEnabledResponse {}))
    }

    async fn pin_radio_station(
        &self,
        request: Request<proto::PinRadioStationRequest>,
    ) -> Result<Response<proto::PinRadioStationResponse>, Status> {
        let request = request.get_ref();
        let station = request
            .station
            .as_ref()
            .map(convert::radio_station_from_proto)
            .ok_or_else(|| Status::invalid_argument("missing radio station"))?;
        api_call!(self, pin_radio_station, station, request.pinned);
        Ok(Response::new(proto::PinRadioStationResponse {}))
    }

    async fn update_track_metadata(
        &self,
        request: Request<proto::TrackMetadataPatch>,
    ) -> Result<Response<proto::TrackInfo>, Status> {
        let track = api_call!(
            self,
            update_track_metadata,
            convert::metadata_patch_from_proto(request.get_ref())
        );
        Ok(Response::new(convert::track_info_to_proto(&track)))
    }

    async fn delete_tracks(
        &self,
        request: Request<proto::DeleteTracksRequest>,
    ) -> Result<Response<proto::DeleteTracksResponse>, Status> {
        let request = request.get_ref();
        api_call!(self, delete_tracks, request.keys.clone(), request.from_disk);
        Ok(Response::new(proto::DeleteTracksResponse {}))
    }

    async fn delete_album(
        &self,
        request: Request<proto::DeleteAlbumRequest>,
    ) -> Result<Response<proto::DeleteAlbumResponse>, Status> {
        let request = request.get_ref();
        api_call!(self, delete_album, request.id.clone(), request.from_disk);
        Ok(Response::new(proto::DeleteAlbumResponse {}))
    }

    async fn upload_artwork(
        &self,
        request: Request<proto::ArtworkUpload>,
    ) -> Result<Response<proto::UploadArtworkResponse>, Status> {
        api_call!(
            self,
            upload_artwork,
            convert::artwork_upload_from_proto(request.get_ref())
        );
        Ok(Response::new(proto::UploadArtworkResponse {}))
    }

    async fn remove_artwork(
        &self,
        request: Request<proto::RemoveArtworkRequest>,
    ) -> Result<Response<proto::RemoveArtworkResponse>, Status> {
        let target = convert::remove_artwork_from_proto(request.get_ref())
            .ok_or_else(|| Status::invalid_argument("artwork target is required"))?;
        api_call!(self, remove_artwork, target);
        Ok(Response::new(proto::RemoveArtworkResponse {}))
    }

    async fn shutdown(
        &self,
        _request: Request<proto::ShutdownRequest>,
    ) -> Result<Response<proto::ShutdownResponse>, Status> {
        let Some(shutdown) = &self.0.shutdown else {
            return Err(failed(ApiError::unsupported(
                "this daemon cannot be shut down remotely",
            )));
        };
        shutdown.notify_one();
        Ok(Response::new(proto::ShutdownResponse {}))
    }

    #[allow(clippy::result_large_err)]
    async fn get_artwork(
        &self,
        request: Request<proto::ArtworkRequest>,
    ) -> Result<Response<Self::GetArtworkStream>, Status> {
        let request = convert::artwork_request_from_proto(request.get_ref());
        if request.entity.is_none() {
            return Err(Status::invalid_argument(
                "pass one of track, album, artist, or playlist",
            ));
        }
        let payload = api_call!(self, artwork, request);
        let content_type = payload.content_type;
        let stream = futures_util::stream::unfold(
            (payload.data, 0usize, content_type),
            |(bytes, offset, content_type)| async move {
                if offset >= bytes.len() {
                    return None;
                }
                let end = (offset + ARTWORK_CHUNK).min(bytes.len());
                let chunk = proto::ArtworkChunk {
                    content_type: if offset == 0 {
                        content_type.clone()
                    } else {
                        String::new()
                    },
                    data: bytes[offset..end].to_vec(),
                };
                Some((Ok(chunk), (bytes, end, content_type)))
            },
        );
        Ok(Response::new(Box::pin(stream)))
    }
}

/// Serve the daemon on `listener` until the future is dropped. Reflection
/// (v1 and v1alpha) is registered so `grpcurl` works out of the box.
/// `result_large_err` is tonic's own Status type; nothing to shrink here.
#[allow(clippy::result_large_err)]
pub async fn serve(
    listener: tokio::net::TcpListener,
    state: Arc<GrpcState>,
) -> std::io::Result<()> {
    let token = state.token.clone();
    let auth = move |request: Request<()>| -> Result<Request<()>, Status> {
        let provided = request
            .metadata()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "));
        if provided.is_some_and(|value| constant_time_eq(value.as_bytes(), token.as_bytes())) {
            Ok(request)
        } else {
            Err(Status::unauthenticated("missing or invalid bearer token"))
        }
    };
    let reflection_v1 = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .map_err(std::io::Error::other)?;
    let reflection_v1alpha = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .map_err(std::io::Error::other)?;
    let kopuz = KopuzServer::new(KopuzGrpc(state))
        .max_decoding_message_size(33 * 1024 * 1024)
        .max_encoding_message_size(33 * 1024 * 1024);
    let kopuz = tonic::service::interceptor::InterceptedService::new(kopuz, auth);
    tonic::transport::Server::builder()
        .add_service(reflection_v1)
        .add_service(reflection_v1alpha)
        .add_service(kopuz)
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
        .await
        .map_err(std::io::Error::other)
}
