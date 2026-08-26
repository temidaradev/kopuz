use super::macros::enum_conversion;
use crate::*;

enum_conversion!(phase_to_proto, phase_from_proto, api::Phase, Phase,
    default api::Phase::Idle, unspecified Phase::Unspecified, {
        api::Phase::Idle => Phase::Idle,
        api::Phase::Playing => Phase::Playing,
        api::Phase::Paused => Phase::Paused,
        api::Phase::Ended => Phase::Ended,
    }
);

enum_conversion!(loop_to_proto, loop_from_proto, api::LoopMode, LoopMode,
    default api::LoopMode::None, unspecified LoopMode::Unspecified, {
        api::LoopMode::None => LoopMode::None,
        api::LoopMode::Queue => LoopMode::Queue,
        api::LoopMode::Track => LoopMode::Track,
    }
);

enum_conversion!(track_kind_to_proto, track_kind_from_proto, api::TrackKind, TrackKind,
    default api::TrackKind::Normal, unspecified TrackKind::Unspecified, {
        api::TrackKind::Normal => TrackKind::Normal,
        api::TrackKind::Radio => TrackKind::Radio,
    }
);

enum_conversion!(queue_mode_to_proto, queue_mode_from_proto, api::QueueMode, QueueMode,
    default api::QueueMode::Replace, unspecified QueueMode::Unspecified, {
        api::QueueMode::Replace => QueueMode::Replace,
        api::QueueMode::Append => QueueMode::Append,
        api::QueueMode::PlayNext => QueueMode::PlayNext,
        api::QueueMode::Insert => QueueMode::Insert,
    }
);

enum_conversion!(table_to_proto, table_from_proto, api::Table, Table,
    default api::Table::Unknown, unspecified Table::Unspecified, {
        api::Table::Tracks => Table::Tracks,
        api::Table::Albums => Table::Albums,
        api::Table::Playlists => Table::Playlists,
        api::Table::Favorites => Table::Favorites,
        api::Table::Folders => Table::Folders,
        api::Table::Servers => Table::Servers,
        api::Table::Recents => Table::Recents,
    }, unknown api::Table::Unknown
);

enum_conversion!(job_kind_to_proto, job_kind_from_proto, api::JobKind, JobKind,
    default api::JobKind::Unknown, unspecified JobKind::Unspecified, {
        api::JobKind::Scan => JobKind::Scan,
        api::JobKind::LibrarySync => JobKind::LibrarySync,
        api::JobKind::FavoritesSync => JobKind::FavoritesSync,
        api::JobKind::PlaylistSync => JobKind::PlaylistSync,
        api::JobKind::Download => JobKind::Download,
        api::JobKind::Ytdlp => JobKind::Ytdlp,
    }, unknown api::JobKind::Unknown
);

enum_conversion!(job_state_to_proto, job_state_from_proto, api::JobState, JobState,
    default api::JobState::Unknown, unspecified JobState::Unspecified, {
        api::JobState::Running => JobState::Running,
        api::JobState::Finished => JobState::Finished,
        api::JobState::Failed => JobState::Failed,
        api::JobState::Cancelled => JobState::Cancelled,
    }, unknown api::JobState::Unknown
);

enum_conversion!(source_state_to_proto, source_state_from_proto, api::SourceState, SourceState,
    default api::SourceState::Offline, unspecified SourceState::Unspecified, {
        api::SourceState::Online => SourceState::Online,
        api::SourceState::AuthExpired => SourceState::AuthExpired,
        api::SourceState::Offline => SourceState::Offline,
    }
);

enum_conversion!(notice_level_to_proto, notice_level_from_proto, api::NoticeLevel, NoticeLevel,
    default api::NoticeLevel::Unknown, unspecified NoticeLevel::Unspecified, {
        api::NoticeLevel::Info => NoticeLevel::Info,
        api::NoticeLevel::Warning => NoticeLevel::Warning,
        api::NoticeLevel::Error => NoticeLevel::Error,
    }, unknown api::NoticeLevel::Unknown
);

enum_conversion!(music_service_to_proto, music_service_from_proto, api::MusicService, MusicService,
    default api::MusicService::Unknown, unspecified MusicService::Unspecified, {
        api::MusicService::Jellyfin => MusicService::Jellyfin,
        api::MusicService::Subsonic => MusicService::Subsonic,
        api::MusicService::Custom => MusicService::Custom,
        api::MusicService::YtMusic => MusicService::YtMusic,
        api::MusicService::AppleMusic => MusicService::AppleMusic,
        api::MusicService::SoundCloud => MusicService::SoundCloud,
        api::MusicService::Spotify => MusicService::Spotify,
        api::MusicService::Nextcloud => MusicService::Nextcloud,
    }, unknown api::MusicService::Unknown
);

enum_conversion!(source_kind_to_proto, source_kind_from_proto, api::SourceKind, SourceKind,
    default api::SourceKind::Unknown, unspecified SourceKind::Unspecified, {
        api::SourceKind::Local => SourceKind::Local,
        api::SourceKind::LocalLibrary => SourceKind::LocalLibrary,
        api::SourceKind::Server => SourceKind::Server,
    }, unknown api::SourceKind::Unknown
);

enum_conversion!(playlist_capability_to_proto, playlist_capability_from_proto, api::PlaylistCapability, PlaylistCapability,
    default api::PlaylistCapability::None, unspecified PlaylistCapability::Unspecified, {
        api::PlaylistCapability::None => PlaylistCapability::None,
        api::PlaylistCapability::AddRemove => PlaylistCapability::AddRemove,
        api::PlaylistCapability::Reorder => PlaylistCapability::Reorder,
    }, unknown api::PlaylistCapability::Unknown
);

enum_conversion!(artist_presentation_to_proto, artist_presentation_from_proto, api::ArtistPresentation, ArtistPresentation,
    default api::ArtistPresentation::Library, unspecified ArtistPresentation::Unspecified, {
        api::ArtistPresentation::Library => ArtistPresentation::Library,
        api::ArtistPresentation::Remote => ArtistPresentation::Remote,
    }, unknown api::ArtistPresentation::Unknown
);

enum_conversion!(album_presentation_to_proto, album_presentation_from_proto, api::AlbumPresentation, AlbumPresentation,
    default api::AlbumPresentation::Standard, unspecified AlbumPresentation::Unspecified, {
        api::AlbumPresentation::Standard => AlbumPresentation::Standard,
        api::AlbumPresentation::Remote => AlbumPresentation::Remote,
    }, unknown api::AlbumPresentation::Unknown
);

enum_conversion!(integration_kind_to_proto, integration_kind_from_proto, api::IntegrationKind, IntegrationKind,
    default api::IntegrationKind::Unknown, unspecified IntegrationKind::Unspecified, {
        api::IntegrationKind::ListenBrainz => IntegrationKind::ListenBrainz,
        api::IntegrationKind::LastFm => IntegrationKind::LastFm,
        api::IntegrationKind::LibreFm => IntegrationKind::LibreFm,
    }, unknown api::IntegrationKind::Unknown
);

enum_conversion!(catalog_item_kind_to_proto, catalog_item_kind_from_proto, api::CatalogItemKind, CatalogItemKind,
    default api::CatalogItemKind::Unknown, unspecified CatalogItemKind::Unspecified, {
        api::CatalogItemKind::Track => CatalogItemKind::Track,
        api::CatalogItemKind::Album => CatalogItemKind::Album,
        api::CatalogItemKind::Playlist => CatalogItemKind::Playlist,
        api::CatalogItemKind::Artist => CatalogItemKind::Artist,
        api::CatalogItemKind::Mood => CatalogItemKind::Mood,
    }, unknown api::CatalogItemKind::Unknown
);

enum_conversion!(ytdlp_format_to_proto, ytdlp_format_from_proto, api::YtdlpAudioFormat, YtdlpAudioFormat,
    default api::YtdlpAudioFormat::Best, unspecified YtdlpAudioFormat::Unspecified, {
        api::YtdlpAudioFormat::Best => YtdlpAudioFormat::Best,
        api::YtdlpAudioFormat::Mp3 => YtdlpAudioFormat::Mp3,
        api::YtdlpAudioFormat::M4a => YtdlpAudioFormat::M4a,
        api::YtdlpAudioFormat::Opus => YtdlpAudioFormat::Opus,
        api::YtdlpAudioFormat::Flac => YtdlpAudioFormat::Flac,
        api::YtdlpAudioFormat::Wav => YtdlpAudioFormat::Wav,
        api::YtdlpAudioFormat::Video => YtdlpAudioFormat::Video,
    }, unknown api::YtdlpAudioFormat::Unknown
);

enum_conversion!(
    download_item_state_to_proto,
    download_item_state_from_proto,
    api::DownloadItemState,
    DownloadItemState,
    default api::DownloadItemState::Unknown,
    unspecified DownloadItemState::Unspecified,
    {
        api::DownloadItemState::Queued => DownloadItemState::Queued,
        api::DownloadItemState::Downloading => DownloadItemState::Downloading,
        api::DownloadItemState::Finished => DownloadItemState::Finished,
        api::DownloadItemState::Failed => DownloadItemState::Failed,
        api::DownloadItemState::Cancelled => DownloadItemState::Cancelled,
    },
    unknown api::DownloadItemState::Unknown
);

enum_conversion!(favorites_sync_to_proto, favorites_sync_from_proto, api::FavoritesSyncMode, FavoritesSyncMode,
    default api::FavoritesSyncMode::Instant, unspecified FavoritesSyncMode::Unspecified, {
        api::FavoritesSyncMode::Instant => FavoritesSyncMode::Instant,
        api::FavoritesSyncMode::Paginated => FavoritesSyncMode::Paginated,
    }, unknown api::FavoritesSyncMode::Unknown
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unspecified_status_values_are_unknown() {
        assert_eq!(job_state_from_proto(0), api::JobState::Unknown);
        assert_eq!(notice_level_from_proto(0), api::NoticeLevel::Unknown);
    }

    #[test]
    fn every_frontend_enum_round_trips() {
        for service in [
            api::MusicService::Jellyfin,
            api::MusicService::Subsonic,
            api::MusicService::Custom,
            api::MusicService::YtMusic,
            api::MusicService::AppleMusic,
            api::MusicService::SoundCloud,
            api::MusicService::Spotify,
            api::MusicService::Nextcloud,
        ] {
            assert_eq!(
                service,
                music_service_from_proto(music_service_to_proto(service) as i32)
            );
        }
        for kind in [
            api::SourceKind::Local,
            api::SourceKind::LocalLibrary,
            api::SourceKind::Server,
        ] {
            assert_eq!(
                kind,
                source_kind_from_proto(source_kind_to_proto(kind) as i32)
            );
        }
        for capability in [
            api::PlaylistCapability::None,
            api::PlaylistCapability::AddRemove,
            api::PlaylistCapability::Reorder,
        ] {
            assert_eq!(
                capability,
                playlist_capability_from_proto(playlist_capability_to_proto(capability) as i32)
            );
        }
        for presentation in [
            api::ArtistPresentation::Library,
            api::ArtistPresentation::Remote,
        ] {
            assert_eq!(
                presentation,
                artist_presentation_from_proto(artist_presentation_to_proto(presentation) as i32)
            );
        }
        for presentation in [
            api::AlbumPresentation::Standard,
            api::AlbumPresentation::Remote,
        ] {
            assert_eq!(
                presentation,
                album_presentation_from_proto(album_presentation_to_proto(presentation) as i32)
            );
        }
        for kind in [
            api::IntegrationKind::LastFm,
            api::IntegrationKind::LibreFm,
            api::IntegrationKind::ListenBrainz,
        ] {
            assert_eq!(
                kind,
                integration_kind_from_proto(integration_kind_to_proto(kind) as i32)
            );
        }
        for format in [
            api::YtdlpAudioFormat::Best,
            api::YtdlpAudioFormat::Mp3,
            api::YtdlpAudioFormat::M4a,
            api::YtdlpAudioFormat::Opus,
            api::YtdlpAudioFormat::Flac,
            api::YtdlpAudioFormat::Wav,
            api::YtdlpAudioFormat::Video,
        ] {
            assert_eq!(
                format,
                ytdlp_format_from_proto(ytdlp_format_to_proto(format) as i32)
            );
        }
        for state in [
            api::DownloadItemState::Queued,
            api::DownloadItemState::Downloading,
            api::DownloadItemState::Finished,
            api::DownloadItemState::Failed,
            api::DownloadItemState::Cancelled,
        ] {
            assert_eq!(
                state,
                download_item_state_from_proto(download_item_state_to_proto(state) as i32)
            );
        }
    }
}
