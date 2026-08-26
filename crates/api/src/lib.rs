//! Wire types and the client-facing trait for the Kopuz daemon API.
//!
//! Everything here is transport-neutral: `LocalApi` (daemon crate) implements
//! [`KopuzApi`] with direct in-process calls; `GrpcApi` (client crate)
//! implements it over the gRPC wire. The protobuf schema in `kopuz-proto` is
//! the versioned wire contract.
//!
//! The trait starts with the playback core and grows one resource group at a
//! time as the daemon services land.

mod error;
mod events;
mod frontend;
mod library;
mod player;
mod queue;

pub use error::{ApiError, ErrorBody, ErrorCode};
pub use events::{ApiEvent, JobKind, JobProgress, NoticeLevel, SourceState, Table};
pub use frontend::{
    AlbumFilter, AlbumInfo, AlbumPage, AlbumPresentation, ArtistInfo, ArtistPage,
    ArtistPresentation, ArtworkData, ArtworkEntity, ArtworkRequest, ArtworkTarget, ArtworkUpload,
    CatalogDetail, CatalogDetailRequest, CatalogItem, CatalogItemKind, CatalogPage, CatalogShelf,
    CredentialProvision, ExternalAccess, FavoritesSyncMode, IntegrationCredentialProvision,
    IntegrationCredentialStatus, IntegrationKind, LocalSourceDraft, MusicService,
    PlaylistCapability, PlaylistCatalog, PlaylistFolderInfo, PlaylistInfo, PlaylistTracksRequest,
    RadioRegistryInfo, RadioStationInfo, RadioStreamInfo, SearchResults, ServerDraft,
    SourceCapabilities, SourceFolderEntry, SourceInfo, SourceKind, SourceLoginRequest,
    TrackMetadataPatch, YtdlpAudioFormat, YtdlpRequest,
};
pub use library::{
    DEFAULT_PAGE_LIMIT, LyricChunkView, LyricLineView, LyricsView, Page, StatsView, TrackFilter,
    TrackInfo, TrackPage,
};
pub use player::{
    BufferedRange, ExternalPlayback, ExternalPlaybackLease, ExternalPlaybackReport, FadingState,
    Intent, LoopMode, NowPlaying, Phase, PlayerCommand, PlayerState, PositionAnchor, QueueSummary,
    TrackKind,
};
pub use queue::{
    QueueContext, QueueEdit, QueueItem, QueueMode, QueuePersistenceSnapshot, QueueWindow,
    SetQueueRequest,
};

pub const API_VERSION: u32 = 1;

/// The config view: the layered config with credential keys
/// stripped, plus the keys a managed settings file pins (rendered locked in
/// settings UIs).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConfigView {
    pub config: serde_json::Value,
    pub locked_keys: Vec<String>,
}

/// Returned by every command; `rev` names the state revision that includes
/// the command's effect, so a client can wait for the event stream to catch
/// up before trusting its local mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandAck {
    pub rev: u64,
}

pub type EventStream = futures_util::stream::BoxStream<'static, ApiEvent>;
pub type LyricsStream = futures_util::stream::BoxStream<'static, Result<LyricsView, ApiError>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobRef {
    pub job_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Running,
    Finished,
    Failed,
    Cancelled,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JobStatus {
    pub id: String,
    pub kind: JobKind,
    pub state: JobState,
    pub phase: String,
    pub current: Option<u64>,
    pub total: Option<u64>,
    pub message: Option<String>,
    pub error: Option<ErrorBody>,
    pub request: Option<String>,
    pub title: Option<String>,
    pub format: Option<String>,
    pub speed: Option<String>,
    pub eta: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FavoritesView {
    pub refs: Vec<String>,
    pub generation: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DownloadItemState {
    #[default]
    Queued,
    Downloading,
    Finished,
    Failed,
    Cancelled,
    Unknown,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DownloadItemStatus {
    pub key: String,
    pub state: DownloadItemState,
    pub bytes_done: u64,
    pub total_bytes: Option<u64>,
    pub error: Option<String>,
}

#[async_trait::async_trait]
pub trait KopuzApi: Send + Sync {
    async fn player_state(&self) -> Result<PlayerState, ApiError>;

    async fn player_command(&self, cmd: PlayerCommand) -> Result<CommandAck, ApiError>;

    async fn queue_window(&self, page: Page) -> Result<QueueWindow, ApiError>;

    async fn set_queue(&self, req: SetQueueRequest) -> Result<CommandAck, ApiError>;

    async fn queue_edit(&self, edit: QueueEdit) -> Result<CommandAck, ApiError>;

    async fn queue_snapshot(&self) -> Result<QueuePersistenceSnapshot, ApiError>;

    /// The session actor's current queue and shuffle permutation. Unlike
    /// `queue_snapshot`, this is not a read of the durable restore row.
    async fn live_queue(&self) -> Result<QueuePersistenceSnapshot, ApiError>;

    async fn save_queue_snapshot(&self, snapshot: QueuePersistenceSnapshot)
    -> Result<(), ApiError>;

    async fn config(&self) -> Result<ConfigView, ApiError>;

    /// RFC 7396 merge patch against the config surface. Credential and
    /// locked keys are refused with `invalid_input`.
    async fn patch_config(&self, patch: serde_json::Value) -> Result<ConfigView, ApiError>;

    /// Apply an equalizer value to the live engine without persisting it.
    async fn preview_equalizer(&self, equalizer: serde_json::Value) -> Result<(), ApiError>;

    async fn favorites(&self) -> Result<FavoritesView, ApiError>;

    /// Optimistic set: recorded locally and reflected immediately, pushed to
    /// the remote in the background of the call; a rejected push reverts the
    /// local state and surfaces the error.
    async fn set_favorite(&self, key: String, favorite: bool) -> Result<(), ApiError>;

    /// Start a long-running job (`scan`, `library_sync`, `favorites_sync`).
    /// Progress arrives as `job.progress` / `job.finished` events; a second
    /// start of an already-running kind returns `conflict`.
    async fn start_job(&self, kind: JobKind) -> Result<JobRef, ApiError>;

    /// Cache server tracks for offline playback; returns the download job.
    async fn download(&self, keys: Vec<String>) -> Result<JobRef, ApiError>;

    /// Item ids with a registered offline copy.
    async fn downloads(&self) -> Result<Vec<String>, ApiError>;

    async fn download_statuses(&self) -> Result<Vec<DownloadItemStatus>, ApiError>;

    async fn cancel_download_item(&self, key: String) -> Result<(), ApiError>;

    async fn remove_download(&self, key: String) -> Result<(), ApiError>;

    async fn jobs(&self) -> Result<Vec<JobStatus>, ApiError>;

    async fn cancel_job(&self, id: String) -> Result<(), ApiError>;

    async fn tracks(&self, filter: TrackFilter, page: Page) -> Result<TrackPage, ApiError>;

    /// Local tracks under a directory prefix, path-ordered.
    async fn folder_tracks(&self, prefix: String, page: Page) -> Result<TrackPage, ApiError>;

    async fn lyrics(&self, key: String) -> Result<LyricsView, ApiError>;

    fn lyrics_stream(&self, key: String) -> LyricsStream;

    async fn stats(&self) -> Result<StatsView, ApiError>;

    async fn albums(&self, filter: AlbumFilter, page: Page) -> Result<AlbumPage, ApiError>;

    async fn album(&self, id: String) -> Result<AlbumInfo, ApiError>;

    async fn artists(&self, page: Page) -> Result<ArtistPage, ApiError>;

    async fn refresh_artist_artwork(&self, names: Vec<String>) -> Result<Vec<String>, ApiError>;

    async fn genres(&self) -> Result<Vec<String>, ApiError>;

    async fn recent_tracks(&self, page: Page) -> Result<TrackPage, ApiError>;

    async fn album_tracks(&self, id: String, page: Page) -> Result<TrackPage, ApiError>;

    async fn artist_tracks(&self, name: String, page: Page) -> Result<TrackPage, ApiError>;

    async fn genre_tracks(&self, name: String, page: Page) -> Result<TrackPage, ApiError>;

    async fn artist_sample_tracks(&self, page: Page) -> Result<TrackPage, ApiError>;

    async fn tracks_by_keys(&self, keys: Vec<String>) -> Result<Vec<TrackInfo>, ApiError>;

    async fn track_web_url(&self, key: String) -> Result<Option<String>, ApiError>;

    async fn album_web_url(&self, id: String) -> Result<Option<String>, ApiError>;

    async fn top_genre(&self) -> Result<Option<String>, ApiError>;

    async fn search(&self, query: String) -> Result<SearchResults, ApiError>;

    async fn playlists(&self) -> Result<PlaylistCatalog, ApiError>;

    async fn playlist_tracks(&self, request: PlaylistTracksRequest) -> Result<TrackPage, ApiError>;

    async fn refresh_playlist(&self, request: PlaylistTracksRequest)
    -> Result<TrackPage, ApiError>;

    async fn create_playlist(&self, name: String, keys: Vec<String>) -> Result<String, ApiError>;

    async fn rename_playlist(&self, id: String, name: String) -> Result<(), ApiError>;

    async fn delete_playlist(&self, id: String) -> Result<(), ApiError>;

    async fn add_playlist_tracks(&self, id: String, keys: Vec<String>) -> Result<(), ApiError>;

    async fn remove_playlist_tracks(&self, id: String, keys: Vec<String>) -> Result<(), ApiError>;

    async fn reorder_playlist_tracks(&self, id: String, keys: Vec<String>) -> Result<(), ApiError>;

    async fn create_playlist_folder(&self, name: String) -> Result<String, ApiError>;

    async fn rename_playlist_folder(&self, id: String, name: String) -> Result<(), ApiError>;

    async fn delete_playlist_folder(&self, id: String) -> Result<(), ApiError>;

    async fn move_playlist(&self, id: String, folder_id: Option<String>) -> Result<(), ApiError>;

    async fn sources(&self) -> Result<Vec<SourceInfo>, ApiError>;

    async fn switch_source(&self, id: String) -> Result<SourceInfo, ApiError>;

    async fn upsert_local_source(&self, source: LocalSourceDraft) -> Result<SourceInfo, ApiError>;

    async fn delete_local_source(&self, id: String) -> Result<(), ApiError>;

    async fn set_source_directories(
        &self,
        id: String,
        directories: Vec<String>,
    ) -> Result<SourceInfo, ApiError>;

    async fn upsert_server(&self, server: ServerDraft) -> Result<SourceInfo, ApiError>;

    async fn delete_server(&self, id: String) -> Result<(), ApiError>;

    async fn provision_credentials(
        &self,
        provision: CredentialProvision,
    ) -> Result<SourceInfo, ApiError>;

    async fn login_source(&self, request: SourceLoginRequest) -> Result<SourceInfo, ApiError>;

    async fn clear_credentials(&self, id: String) -> Result<(), ApiError>;

    /// Launch a daemon-owned browser authentication flow and store the
    /// resulting source credentials without returning them to the caller.
    async fn authenticate_source(&self, id: String) -> Result<SourceInfo, ApiError>;

    async fn browse_source(
        &self,
        id: String,
        path: String,
    ) -> Result<Vec<SourceFolderEntry>, ApiError>;

    async fn integration_credentials(&self) -> Result<Vec<IntegrationCredentialStatus>, ApiError>;

    async fn provision_integration_credentials(
        &self,
        provision: IntegrationCredentialProvision,
    ) -> Result<IntegrationCredentialStatus, ApiError>;

    async fn clear_integration_credentials(&self, kind: IntegrationKind) -> Result<(), ApiError>;

    async fn authenticate_integration(
        &self,
        provision: IntegrationCredentialProvision,
    ) -> Result<IntegrationCredentialStatus, ApiError>;

    async fn validate_source(&self, id: String) -> Result<SourceState, ApiError>;

    async fn external_access(&self, kind: String) -> Result<ExternalAccess, ApiError>;

    async fn set_external_playback(
        &self,
        external: Option<ExternalPlayback>,
    ) -> Result<(), ApiError>;

    async fn claim_external_playback(
        &self,
        external: ExternalPlayback,
    ) -> Result<ExternalPlaybackLease, ApiError>;

    async fn report_external_playback(
        &self,
        report: ExternalPlaybackReport,
    ) -> Result<(), ApiError>;

    async fn release_external_playback(&self, lease_id: String) -> Result<(), ApiError>;

    async fn start_ytdlp(&self, request: YtdlpRequest) -> Result<JobRef, ApiError>;

    async fn catalog(&self, continuation: Option<String>) -> Result<CatalogPage, ApiError>;

    async fn catalog_detail(
        &self,
        request: CatalogDetailRequest,
    ) -> Result<CatalogDetail, ApiError>;

    async fn radio_stations(&self) -> Result<Vec<RadioStationInfo>, ApiError>;

    async fn track_radio(&self, key: String) -> Result<Vec<TrackInfo>, ApiError>;

    async fn playlist_radio(&self, id: String) -> Result<Vec<TrackInfo>, ApiError>;

    async fn search_radio(
        &self,
        query: String,
        limit: u32,
    ) -> Result<Vec<RadioStationInfo>, ApiError>;

    async fn radio_registries(&self) -> Result<Vec<RadioRegistryInfo>, ApiError>;

    async fn add_radio_registry(&self, url: String) -> Result<(), ApiError>;

    async fn remove_radio_registry(&self, url: String) -> Result<(), ApiError>;

    async fn set_radio_registry_enabled(&self, url: String, enabled: bool) -> Result<(), ApiError>;

    async fn pin_radio_station(
        &self,
        station: RadioStationInfo,
        pinned: bool,
    ) -> Result<(), ApiError>;

    async fn update_track_metadata(&self, patch: TrackMetadataPatch)
    -> Result<TrackInfo, ApiError>;

    async fn delete_tracks(&self, keys: Vec<String>, from_disk: bool) -> Result<(), ApiError>;

    async fn delete_album(&self, id: String, from_disk: bool) -> Result<(), ApiError>;

    async fn upload_artwork(&self, upload: ArtworkUpload) -> Result<(), ApiError>;

    async fn remove_artwork(&self, target: ArtworkTarget) -> Result<(), ApiError>;

    async fn artwork(&self, request: ArtworkRequest) -> Result<ArtworkData, ApiError>;

    /// Subscribe to the state stream. Every subscriber gets every event from
    /// the moment of subscription; a snapshot fetch plus this stream is the
    /// complete synchronization story.
    fn events(&self) -> EventStream;
}
