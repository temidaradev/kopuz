use crate::{Page, TrackInfo};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MusicService {
    Jellyfin,
    Subsonic,
    Custom,
    YtMusic,
    AppleMusic,
    SoundCloud,
    Spotify,
    Nextcloud,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SourceKind {
    Local,
    LocalLibrary,
    Server,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PlaylistCapability {
    #[default]
    None,
    AddRemove,
    Reorder,
    Unknown,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ArtistPresentation {
    #[default]
    Library,
    Remote,
    Unknown,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlbumPresentation {
    #[default]
    Standard,
    Remote,
    Unknown,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceCapabilities {
    pub edit_tags: bool,
    pub delete_from_disk: bool,
    pub scan_folders: bool,
    pub folders: bool,
    pub sync: bool,
    pub downloads: bool,
    pub discover: bool,
    pub track_radio: bool,
    pub playlist_radio: bool,
    pub playlists: PlaylistCapability,
    pub artists: ArtistPresentation,
    pub albums: AlbumPresentation,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceInfo {
    pub id: String,
    pub name: String,
    pub kind: SourceKind,
    pub service: Option<MusicService>,
    pub active: bool,
    pub authenticated: bool,
    pub capabilities: SourceCapabilities,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ServerDraft {
    pub id: Option<String>,
    pub name: String,
    pub url: String,
    pub service: MusicService,
    pub browser: Option<String>,
    pub anonymous: bool,
    pub storefront: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CredentialProvision {
    pub server_id: String,
    pub secret: String,
    pub user_id: Option<String>,
    pub browser: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExternalAccess {
    pub kind: String,
    pub access_token: String,
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IntegrationKind {
    ListenBrainz,
    LastFm,
    LibreFm,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IntegrationCredentialStatus {
    pub kind: IntegrationKind,
    pub configured: bool,
}

/// Write-only scrobbling credentials. No API response contains these values.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IntegrationCredentialProvision {
    pub kind: IntegrationKind,
    pub token: Option<String>,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub session_key: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceFolderEntry {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum YtdlpAudioFormat {
    #[default]
    Best,
    Mp3,
    M4a,
    Opus,
    Flac,
    Wav,
    Video,
    Unknown,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct YtdlpRequest {
    pub url: String,
    pub output_dir: String,
    pub format: YtdlpAudioFormat,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AlbumInfo {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub genre: String,
    pub year: u32,
    pub artwork: Option<String>,
    pub manual_artwork: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AlbumFilter {
    pub search: Option<String>,
    pub artist: Option<String>,
    pub genre: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AlbumPage {
    pub total: u32,
    pub offset: u32,
    pub items: Vec<AlbumInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtistInfo {
    pub name: String,
    pub track_count: u32,
    pub album_count: u32,
    pub artwork: Option<String>,
    pub manual_artwork: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtistPage {
    pub total: u32,
    pub offset: u32,
    pub items: Vec<ArtistInfo>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SearchResults {
    pub tracks: Vec<TrackInfo>,
    pub albums: Vec<AlbumInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlaylistInfo {
    pub id: String,
    pub name: String,
    pub track_count: u32,
    pub track_keys: Vec<String>,
    pub artwork: Option<String>,
    pub manual_artwork: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlaylistFolderInfo {
    pub id: String,
    pub name: String,
    pub playlist_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlaylistCatalog {
    pub playlists: Vec<PlaylistInfo>,
    pub folders: Vec<PlaylistFolderInfo>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CatalogItemKind {
    Track,
    Album,
    Playlist,
    Artist,
    Mood,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CatalogItem {
    pub kind: CatalogItemKind,
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub artwork: Option<String>,
    pub track: Option<TrackInfo>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CatalogShelf {
    pub title: String,
    pub strapline: Option<String>,
    pub items: Vec<CatalogItem>,
    pub more_ref: Option<String>,
    pub list: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CatalogPage {
    pub shelves: Vec<CatalogShelf>,
    pub continuation: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogDetailRequest {
    pub kind: CatalogItemKind,
    pub id: String,
    pub continuation: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CatalogDetail {
    pub kind: CatalogItemKind,
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub artwork: Option<String>,
    pub playback_id: Option<String>,
    pub year: Option<String>,
    pub tracks: Vec<TrackInfo>,
    pub shelves: Vec<CatalogShelf>,
    pub continuation: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RadioStreamInfo {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RadioStationInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub streams: Vec<RadioStreamInfo>,
    pub pinned: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RadioRegistryInfo {
    pub url: String,
    pub enabled: bool,
    pub built_in: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrackMetadataPatch {
    pub key: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<u32>,
    pub clear_track_number: bool,
    pub disc_number: Option<u32>,
    pub clear_disc_number: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtworkTarget {
    Track { key: String },
    Album { id: String },
    Artist { name: String },
    Playlist { id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtworkEntity {
    Track { key: String },
    Album { id: String },
    Artist { name: String },
    Playlist { id: String },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtworkRequest {
    pub entity: Option<ArtworkEntity>,
    pub hq: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtworkData {
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtworkUpload {
    pub target: Option<ArtworkTarget>,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlaylistTracksRequest {
    pub id: String,
    pub page: Page,
}
