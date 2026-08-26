use crate::*;

pub fn phase_to_proto(value: api::Phase) -> Phase {
    match value {
        api::Phase::Idle => Phase::Idle,
        api::Phase::Playing => Phase::Playing,
        api::Phase::Paused => Phase::Paused,
        api::Phase::Ended => Phase::Ended,
    }
}

pub fn phase_from_proto(value: i32) -> api::Phase {
    match Phase::try_from(value).unwrap_or(Phase::Unspecified) {
        Phase::Playing => api::Phase::Playing,
        Phase::Paused => api::Phase::Paused,
        Phase::Ended => api::Phase::Ended,
        Phase::Idle | Phase::Unspecified => api::Phase::Idle,
    }
}

pub fn loop_to_proto(value: api::LoopMode) -> LoopMode {
    match value {
        api::LoopMode::None => LoopMode::None,
        api::LoopMode::Queue => LoopMode::Queue,
        api::LoopMode::Track => LoopMode::Track,
    }
}

pub fn loop_from_proto(value: i32) -> api::LoopMode {
    match LoopMode::try_from(value).unwrap_or(LoopMode::Unspecified) {
        LoopMode::Queue => api::LoopMode::Queue,
        LoopMode::Track => api::LoopMode::Track,
        LoopMode::None | LoopMode::Unspecified => api::LoopMode::None,
    }
}

pub fn track_kind_to_proto(value: api::TrackKind) -> TrackKind {
    match value {
        api::TrackKind::Normal => TrackKind::Normal,
        api::TrackKind::Radio => TrackKind::Radio,
    }
}

pub fn track_kind_from_proto(value: i32) -> api::TrackKind {
    match TrackKind::try_from(value).unwrap_or(TrackKind::Unspecified) {
        TrackKind::Radio => api::TrackKind::Radio,
        TrackKind::Normal | TrackKind::Unspecified => api::TrackKind::Normal,
    }
}

pub fn queue_mode_to_proto(value: api::QueueMode) -> QueueMode {
    match value {
        api::QueueMode::Replace => QueueMode::Replace,
        api::QueueMode::Append => QueueMode::Append,
        api::QueueMode::PlayNext => QueueMode::PlayNext,
        api::QueueMode::Insert => QueueMode::Insert,
    }
}

pub fn queue_mode_from_proto(value: i32) -> api::QueueMode {
    match QueueMode::try_from(value).unwrap_or(QueueMode::Unspecified) {
        QueueMode::Append => api::QueueMode::Append,
        QueueMode::PlayNext => api::QueueMode::PlayNext,
        QueueMode::Insert => api::QueueMode::Insert,
        QueueMode::Replace | QueueMode::Unspecified => api::QueueMode::Replace,
    }
}

pub fn table_to_proto(value: api::Table) -> Table {
    match value {
        api::Table::Tracks => Table::Tracks,
        api::Table::Albums => Table::Albums,
        api::Table::Playlists => Table::Playlists,
        api::Table::Favorites => Table::Favorites,
        api::Table::Folders => Table::Folders,
        api::Table::Servers => Table::Servers,
        api::Table::Recents => Table::Recents,
        api::Table::Unknown => Table::Unspecified,
    }
}

pub fn table_from_proto(value: i32) -> api::Table {
    match Table::try_from(value).unwrap_or(Table::Unspecified) {
        Table::Tracks => api::Table::Tracks,
        Table::Albums => api::Table::Albums,
        Table::Playlists => api::Table::Playlists,
        Table::Favorites => api::Table::Favorites,
        Table::Folders => api::Table::Folders,
        Table::Servers => api::Table::Servers,
        Table::Recents => api::Table::Recents,
        Table::Unspecified => api::Table::Unknown,
    }
}

pub fn job_kind_to_proto(value: api::JobKind) -> JobKind {
    match value {
        api::JobKind::Scan => JobKind::Scan,
        api::JobKind::LibrarySync => JobKind::LibrarySync,
        api::JobKind::FavoritesSync => JobKind::FavoritesSync,
        api::JobKind::PlaylistSync => JobKind::PlaylistSync,
        api::JobKind::Download => JobKind::Download,
        api::JobKind::Ytdlp => JobKind::Ytdlp,
        api::JobKind::Unknown => JobKind::Unspecified,
    }
}

pub fn job_kind_from_proto(value: i32) -> api::JobKind {
    match JobKind::try_from(value).unwrap_or(JobKind::Unspecified) {
        JobKind::Scan => api::JobKind::Scan,
        JobKind::LibrarySync => api::JobKind::LibrarySync,
        JobKind::FavoritesSync => api::JobKind::FavoritesSync,
        JobKind::PlaylistSync => api::JobKind::PlaylistSync,
        JobKind::Download => api::JobKind::Download,
        JobKind::Ytdlp => api::JobKind::Ytdlp,
        JobKind::Unspecified => api::JobKind::Unknown,
    }
}

pub fn job_state_to_proto(value: api::JobState) -> JobState {
    match value {
        api::JobState::Running => JobState::Running,
        api::JobState::Finished => JobState::Finished,
        api::JobState::Failed => JobState::Failed,
        api::JobState::Cancelled => JobState::Cancelled,
        api::JobState::Unknown => JobState::Unspecified,
    }
}

pub fn job_state_from_proto(value: i32) -> api::JobState {
    match JobState::try_from(value).unwrap_or(JobState::Unspecified) {
        JobState::Running => api::JobState::Running,
        JobState::Finished => api::JobState::Finished,
        JobState::Cancelled => api::JobState::Cancelled,
        JobState::Failed => api::JobState::Failed,
        JobState::Unspecified => api::JobState::Unknown,
    }
}

pub fn source_state_to_proto(value: api::SourceState) -> SourceState {
    match value {
        api::SourceState::Online => SourceState::Online,
        api::SourceState::AuthExpired => SourceState::AuthExpired,
        api::SourceState::Offline => SourceState::Offline,
    }
}

pub fn source_state_from_proto(value: i32) -> api::SourceState {
    match SourceState::try_from(value).unwrap_or(SourceState::Unspecified) {
        SourceState::Online => api::SourceState::Online,
        SourceState::AuthExpired => api::SourceState::AuthExpired,
        SourceState::Offline | SourceState::Unspecified => api::SourceState::Offline,
    }
}

pub fn notice_level_to_proto(value: api::NoticeLevel) -> NoticeLevel {
    match value {
        api::NoticeLevel::Info => NoticeLevel::Info,
        api::NoticeLevel::Warning => NoticeLevel::Warning,
        api::NoticeLevel::Error => NoticeLevel::Error,
        api::NoticeLevel::Unknown => NoticeLevel::Unspecified,
    }
}

pub fn notice_level_from_proto(value: i32) -> api::NoticeLevel {
    match NoticeLevel::try_from(value).unwrap_or(NoticeLevel::Unspecified) {
        NoticeLevel::Info => api::NoticeLevel::Info,
        NoticeLevel::Warning => api::NoticeLevel::Warning,
        NoticeLevel::Error => api::NoticeLevel::Error,
        NoticeLevel::Unspecified => api::NoticeLevel::Unknown,
    }
}

pub fn music_service_to_proto(value: api::MusicService) -> MusicService {
    match value {
        api::MusicService::Jellyfin => MusicService::Jellyfin,
        api::MusicService::Subsonic => MusicService::Subsonic,
        api::MusicService::Custom => MusicService::Custom,
        api::MusicService::YtMusic => MusicService::YtMusic,
        api::MusicService::AppleMusic => MusicService::AppleMusic,
        api::MusicService::SoundCloud => MusicService::SoundCloud,
        api::MusicService::Spotify => MusicService::Spotify,
        api::MusicService::Nextcloud => MusicService::Nextcloud,
        api::MusicService::Unknown => MusicService::Unspecified,
    }
}

pub fn music_service_from_proto(value: i32) -> api::MusicService {
    match MusicService::try_from(value).unwrap_or(MusicService::Unspecified) {
        MusicService::Jellyfin => api::MusicService::Jellyfin,
        MusicService::Subsonic => api::MusicService::Subsonic,
        MusicService::Custom => api::MusicService::Custom,
        MusicService::YtMusic => api::MusicService::YtMusic,
        MusicService::AppleMusic => api::MusicService::AppleMusic,
        MusicService::SoundCloud => api::MusicService::SoundCloud,
        MusicService::Spotify => api::MusicService::Spotify,
        MusicService::Nextcloud => api::MusicService::Nextcloud,
        MusicService::Unspecified => api::MusicService::Unknown,
    }
}

pub fn source_kind_to_proto(value: api::SourceKind) -> SourceKind {
    match value {
        api::SourceKind::Local => SourceKind::Local,
        api::SourceKind::LocalLibrary => SourceKind::LocalLibrary,
        api::SourceKind::Server => SourceKind::Server,
        api::SourceKind::Unknown => SourceKind::Unspecified,
    }
}

pub fn source_kind_from_proto(value: i32) -> api::SourceKind {
    match SourceKind::try_from(value).unwrap_or(SourceKind::Unspecified) {
        SourceKind::Local => api::SourceKind::Local,
        SourceKind::LocalLibrary => api::SourceKind::LocalLibrary,
        SourceKind::Server => api::SourceKind::Server,
        SourceKind::Unspecified => api::SourceKind::Unknown,
    }
}

pub fn playlist_capability_to_proto(value: api::PlaylistCapability) -> PlaylistCapability {
    match value {
        api::PlaylistCapability::None => PlaylistCapability::None,
        api::PlaylistCapability::AddRemove => PlaylistCapability::AddRemove,
        api::PlaylistCapability::Reorder => PlaylistCapability::Reorder,
        api::PlaylistCapability::Unknown => PlaylistCapability::Unspecified,
    }
}

pub fn playlist_capability_from_proto(value: i32) -> api::PlaylistCapability {
    match PlaylistCapability::try_from(value).unwrap_or(PlaylistCapability::Unspecified) {
        PlaylistCapability::AddRemove => api::PlaylistCapability::AddRemove,
        PlaylistCapability::Reorder => api::PlaylistCapability::Reorder,
        PlaylistCapability::None | PlaylistCapability::Unspecified => api::PlaylistCapability::None,
    }
}

pub fn artist_presentation_to_proto(value: api::ArtistPresentation) -> ArtistPresentation {
    match value {
        api::ArtistPresentation::Library => ArtistPresentation::Library,
        api::ArtistPresentation::Remote => ArtistPresentation::Remote,
        api::ArtistPresentation::Unknown => ArtistPresentation::Unspecified,
    }
}

pub fn artist_presentation_from_proto(value: i32) -> api::ArtistPresentation {
    match ArtistPresentation::try_from(value).unwrap_or(ArtistPresentation::Unspecified) {
        ArtistPresentation::Remote => api::ArtistPresentation::Remote,
        ArtistPresentation::Library | ArtistPresentation::Unspecified => {
            api::ArtistPresentation::Library
        }
    }
}

pub fn album_presentation_to_proto(value: api::AlbumPresentation) -> AlbumPresentation {
    match value {
        api::AlbumPresentation::Standard => AlbumPresentation::Standard,
        api::AlbumPresentation::Remote => AlbumPresentation::Remote,
        api::AlbumPresentation::Unknown => AlbumPresentation::Unspecified,
    }
}

pub fn album_presentation_from_proto(value: i32) -> api::AlbumPresentation {
    match AlbumPresentation::try_from(value).unwrap_or(AlbumPresentation::Unspecified) {
        AlbumPresentation::Remote => api::AlbumPresentation::Remote,
        AlbumPresentation::Standard | AlbumPresentation::Unspecified => {
            api::AlbumPresentation::Standard
        }
    }
}

pub fn integration_kind_to_proto(value: api::IntegrationKind) -> IntegrationKind {
    match value {
        api::IntegrationKind::ListenBrainz => IntegrationKind::ListenBrainz,
        api::IntegrationKind::LastFm => IntegrationKind::LastFm,
        api::IntegrationKind::LibreFm => IntegrationKind::LibreFm,
        api::IntegrationKind::Unknown => IntegrationKind::Unspecified,
    }
}

pub fn integration_kind_from_proto(value: i32) -> api::IntegrationKind {
    match IntegrationKind::try_from(value).unwrap_or(IntegrationKind::Unspecified) {
        IntegrationKind::ListenBrainz => api::IntegrationKind::ListenBrainz,
        IntegrationKind::LastFm => api::IntegrationKind::LastFm,
        IntegrationKind::LibreFm => api::IntegrationKind::LibreFm,
        IntegrationKind::Unspecified => api::IntegrationKind::Unknown,
    }
}

pub fn catalog_item_kind_to_proto(value: api::CatalogItemKind) -> CatalogItemKind {
    match value {
        api::CatalogItemKind::Track => CatalogItemKind::Track,
        api::CatalogItemKind::Album => CatalogItemKind::Album,
        api::CatalogItemKind::Playlist => CatalogItemKind::Playlist,
        api::CatalogItemKind::Artist => CatalogItemKind::Artist,
        api::CatalogItemKind::Mood => CatalogItemKind::Mood,
        api::CatalogItemKind::Unknown => CatalogItemKind::Unspecified,
    }
}

pub fn catalog_item_kind_from_proto(value: i32) -> api::CatalogItemKind {
    match CatalogItemKind::try_from(value).unwrap_or(CatalogItemKind::Unspecified) {
        CatalogItemKind::Track => api::CatalogItemKind::Track,
        CatalogItemKind::Album => api::CatalogItemKind::Album,
        CatalogItemKind::Playlist => api::CatalogItemKind::Playlist,
        CatalogItemKind::Artist => api::CatalogItemKind::Artist,
        CatalogItemKind::Mood => api::CatalogItemKind::Mood,
        CatalogItemKind::Unspecified => api::CatalogItemKind::Unknown,
    }
}

pub fn ytdlp_format_to_proto(value: api::YtdlpAudioFormat) -> YtdlpAudioFormat {
    match value {
        api::YtdlpAudioFormat::Best => YtdlpAudioFormat::Best,
        api::YtdlpAudioFormat::Mp3 => YtdlpAudioFormat::Mp3,
        api::YtdlpAudioFormat::M4a => YtdlpAudioFormat::M4a,
        api::YtdlpAudioFormat::Opus => YtdlpAudioFormat::Opus,
        api::YtdlpAudioFormat::Flac => YtdlpAudioFormat::Flac,
        api::YtdlpAudioFormat::Wav => YtdlpAudioFormat::Wav,
        api::YtdlpAudioFormat::Video => YtdlpAudioFormat::Video,
        api::YtdlpAudioFormat::Unknown => YtdlpAudioFormat::Unspecified,
    }
}

pub fn ytdlp_format_from_proto(value: i32) -> api::YtdlpAudioFormat {
    match YtdlpAudioFormat::try_from(value).unwrap_or(YtdlpAudioFormat::Unspecified) {
        YtdlpAudioFormat::Mp3 => api::YtdlpAudioFormat::Mp3,
        YtdlpAudioFormat::M4a => api::YtdlpAudioFormat::M4a,
        YtdlpAudioFormat::Opus => api::YtdlpAudioFormat::Opus,
        YtdlpAudioFormat::Flac => api::YtdlpAudioFormat::Flac,
        YtdlpAudioFormat::Wav => api::YtdlpAudioFormat::Wav,
        YtdlpAudioFormat::Video => api::YtdlpAudioFormat::Video,
        YtdlpAudioFormat::Best | YtdlpAudioFormat::Unspecified => api::YtdlpAudioFormat::Best,
    }
}

pub fn download_item_state_to_proto(value: api::DownloadItemState) -> DownloadItemState {
    match value {
        api::DownloadItemState::Queued => DownloadItemState::Queued,
        api::DownloadItemState::Downloading => DownloadItemState::Downloading,
        api::DownloadItemState::Finished => DownloadItemState::Finished,
        api::DownloadItemState::Failed => DownloadItemState::Failed,
        api::DownloadItemState::Cancelled => DownloadItemState::Cancelled,
        api::DownloadItemState::Unknown => DownloadItemState::Unspecified,
    }
}

pub fn download_item_state_from_proto(value: i32) -> api::DownloadItemState {
    match DownloadItemState::try_from(value).unwrap_or(DownloadItemState::Unspecified) {
        DownloadItemState::Queued => api::DownloadItemState::Queued,
        DownloadItemState::Downloading => api::DownloadItemState::Downloading,
        DownloadItemState::Finished => api::DownloadItemState::Finished,
        DownloadItemState::Failed => api::DownloadItemState::Failed,
        DownloadItemState::Cancelled => api::DownloadItemState::Cancelled,
        DownloadItemState::Unspecified => api::DownloadItemState::Unknown,
    }
}

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
