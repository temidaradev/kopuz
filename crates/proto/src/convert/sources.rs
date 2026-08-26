//! Sources, servers, credentials, and the integrations attached to them.

use super::*;
use crate::*;

pub fn source_capabilities_to_proto(value: &api::SourceCapabilities) -> SourceCapabilities {
    SourceCapabilities {
        edit_tags: value.edit_tags,
        delete_from_disk: value.delete_from_disk,
        scan_folders: value.scan_folders,
        folders: value.folders,
        sync: value.sync,
        downloads: value.downloads,
        discover: value.discover,
        track_radio: value.track_radio,
        playlist_radio: value.playlist_radio,
        playlists: playlist_capability_to_proto(value.playlists) as i32,
        artists: artist_presentation_to_proto(value.artists) as i32,
        albums: album_presentation_to_proto(value.albums) as i32,
        favorites_sync: favorites_sync_to_proto(value.favorites_sync) as i32,
    }
}

pub fn source_capabilities_from_proto(
    value: Option<&SourceCapabilities>,
) -> api::SourceCapabilities {
    let value = value.cloned().unwrap_or_default();
    api::SourceCapabilities {
        edit_tags: value.edit_tags,
        delete_from_disk: value.delete_from_disk,
        scan_folders: value.scan_folders,
        folders: value.folders,
        sync: value.sync,
        downloads: value.downloads,
        discover: value.discover,
        track_radio: value.track_radio,
        playlist_radio: value.playlist_radio,
        playlists: playlist_capability_from_proto(value.playlists),
        artists: artist_presentation_from_proto(value.artists),
        albums: album_presentation_from_proto(value.albums),
        favorites_sync: favorites_sync_from_proto(value.favorites_sync),
    }
}

pub fn source_info_to_proto(value: &api::SourceInfo) -> SourceInfo {
    SourceInfo {
        id: value.id.clone(),
        name: value.name.clone(),
        kind: source_kind_to_proto(value.kind) as i32,
        service: value
            .service
            .map(|service| music_service_to_proto(service) as i32),
        active: value.active,
        authenticated: value.authenticated,
        capabilities: Some(source_capabilities_to_proto(&value.capabilities)),
        url: value.url.clone(),
        browser: value.browser.clone(),
        anonymous: value.anonymous,
        storefront: value.storefront.clone(),
        language: value.language.clone(),
        directories: value.directories.clone(),
    }
}

pub fn source_info_from_proto(value: &SourceInfo) -> api::SourceInfo {
    api::SourceInfo {
        id: value.id.clone(),
        name: value.name.clone(),
        kind: source_kind_from_proto(value.kind),
        service: value.service.map(music_service_from_proto),
        active: value.active,
        authenticated: value.authenticated,
        capabilities: source_capabilities_from_proto(value.capabilities.as_ref()),
        url: value.url.clone(),
        browser: value.browser.clone(),
        anonymous: value.anonymous,
        storefront: value.storefront.clone(),
        language: value.language.clone(),
        directories: value.directories.clone(),
    }
}

pub fn server_draft_to_proto(value: &api::ServerDraft) -> ServerDraft {
    ServerDraft {
        id: value.id.clone(),
        name: value.name.clone(),
        url: value.url.clone(),
        service: music_service_to_proto(value.service) as i32,
        browser: value.browser.clone(),
        anonymous: value.anonymous,
        storefront: value.storefront.clone(),
        language: value.language.clone(),
    }
}

pub fn server_draft_from_proto(value: &ServerDraft) -> api::ServerDraft {
    api::ServerDraft {
        id: value.id.clone(),
        name: value.name.clone(),
        url: value.url.clone(),
        service: music_service_from_proto(value.service),
        browser: value.browser.clone(),
        anonymous: value.anonymous,
        storefront: value.storefront.clone(),
        language: value.language.clone(),
    }
}

pub fn credential_to_proto(value: &api::CredentialProvision) -> CredentialProvision {
    CredentialProvision {
        server_id: value.server_id.clone(),
        secret: value.secret.clone(),
        user_id: value.user_id.clone(),
        browser: value.browser.clone(),
    }
}

pub fn credential_from_proto(value: &CredentialProvision) -> api::CredentialProvision {
    api::CredentialProvision {
        server_id: value.server_id.clone(),
        secret: value.secret.clone(),
        user_id: value.user_id.clone(),
        browser: value.browser.clone(),
    }
}

pub fn integration_status_to_proto(
    value: &api::IntegrationCredentialStatus,
) -> IntegrationCredentialStatus {
    IntegrationCredentialStatus {
        kind: integration_kind_to_proto(value.kind) as i32,
        configured: value.configured,
    }
}

pub fn integration_status_from_proto(
    value: &IntegrationCredentialStatus,
) -> api::IntegrationCredentialStatus {
    api::IntegrationCredentialStatus {
        kind: integration_kind_from_proto(value.kind),
        configured: value.configured,
    }
}

pub fn integration_provision_to_proto(
    value: &api::IntegrationCredentialProvision,
) -> IntegrationCredentialProvision {
    IntegrationCredentialProvision {
        kind: integration_kind_to_proto(value.kind) as i32,
        token: value.token.clone(),
        api_key: value.api_key.clone(),
        api_secret: value.api_secret.clone(),
        session_key: value.session_key.clone(),
    }
}

pub fn integration_provision_from_proto(
    value: &IntegrationCredentialProvision,
) -> api::IntegrationCredentialProvision {
    api::IntegrationCredentialProvision {
        kind: integration_kind_from_proto(value.kind),
        token: value.token.clone(),
        api_key: value.api_key.clone(),
        api_secret: value.api_secret.clone(),
        session_key: value.session_key.clone(),
    }
}

pub fn source_folder_to_proto(value: &api::SourceFolderEntry) -> SourceFolderEntry {
    SourceFolderEntry {
        path: value.path.clone(),
        name: value.name.clone(),
    }
}

pub fn source_folder_from_proto(value: &SourceFolderEntry) -> api::SourceFolderEntry {
    api::SourceFolderEntry {
        path: value.path.clone(),
        name: value.name.clone(),
    }
}

pub fn external_access_to_proto(value: &api::ExternalAccess) -> ExternalAccess {
    ExternalAccess {
        kind: value.kind.clone(),
        access_token: value.access_token.clone(),
        client_id: value.client_id.clone(),
    }
}

pub fn external_access_from_proto(value: &ExternalAccess) -> api::ExternalAccess {
    api::ExternalAccess {
        kind: value.kind.clone(),
        access_token: value.access_token.clone(),
        client_id: value.client_id.clone(),
    }
}

pub fn local_source_draft_to_proto(value: &api::LocalSourceDraft) -> LocalSourceDraft {
    LocalSourceDraft {
        id: value.id.clone(),
        name: value.name.clone(),
        directories: value.directories.clone(),
    }
}

pub fn local_source_draft_from_proto(value: &LocalSourceDraft) -> api::LocalSourceDraft {
    api::LocalSourceDraft {
        id: value.id.clone(),
        name: value.name.clone(),
        directories: value.directories.clone(),
    }
}

pub fn source_login_to_proto(value: &api::SourceLoginRequest) -> SourceLoginRequest {
    SourceLoginRequest {
        server_id: value.server_id.clone(),
        username: value.username.clone(),
        password: value.password.clone(),
    }
}

pub fn source_login_from_proto(value: &SourceLoginRequest) -> api::SourceLoginRequest {
    api::SourceLoginRequest {
        server_id: value.server_id.clone(),
        username: value.username.clone(),
        password: value.password.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_and_credential_dtos_round_trip() {
        let source = api::SourceInfo {
            id: "server".into(),
            name: "Server".into(),
            kind: api::SourceKind::Server,
            service: Some(api::MusicService::Subsonic),
            active: true,
            authenticated: true,
            capabilities: api::SourceCapabilities {
                edit_tags: true,
                delete_from_disk: true,
                scan_folders: true,
                folders: true,
                sync: true,
                downloads: true,
                discover: true,
                track_radio: true,
                playlist_radio: true,
                playlists: api::PlaylistCapability::Reorder,
                artists: api::ArtistPresentation::Remote,
                albums: api::AlbumPresentation::Remote,
                favorites_sync: api::FavoritesSyncMode::Paginated,
            },
            url: Some("https://example.com".into()),
            browser: Some("chrome".into()),
            anonymous: true,
            storefront: Some("tr".into()),
            language: Some("tr".into()),
            directories: vec!["/Music".into()],
        };
        assert_eq!(
            source,
            source_info_from_proto(&source_info_to_proto(&source))
        );

        let server = api::ServerDraft {
            id: Some("server".into()),
            name: "Server".into(),
            url: "https://example.com".into(),
            service: api::MusicService::YtMusic,
            browser: Some("chrome".into()),
            anonymous: true,
            storefront: Some("tr".into()),
            language: Some("tr".into()),
        };
        assert_eq!(
            server,
            server_draft_from_proto(&server_draft_to_proto(&server))
        );

        let credential = api::CredentialProvision {
            server_id: "server".into(),
            secret: "secret".into(),
            user_id: Some("user".into()),
            browser: Some("chrome".into()),
        };
        assert_eq!(
            credential,
            credential_from_proto(&credential_to_proto(&credential))
        );

        let folder = api::SourceFolderEntry {
            path: "/Music".into(),
            name: "Music".into(),
        };
        assert_eq!(
            folder,
            source_folder_from_proto(&source_folder_to_proto(&folder))
        );
    }

    #[test]
    fn integration_and_access_dtos_round_trip() {
        let provision = api::IntegrationCredentialProvision {
            kind: api::IntegrationKind::LastFm,
            token: None,
            api_key: Some("key".into()),
            api_secret: Some("secret".into()),
            session_key: Some("session".into()),
        };
        assert_eq!(
            provision,
            integration_provision_from_proto(&integration_provision_to_proto(&provision))
        );

        let status = api::IntegrationCredentialStatus {
            kind: api::IntegrationKind::LibreFm,
            configured: true,
        };
        assert_eq!(
            status,
            integration_status_from_proto(&integration_status_to_proto(&status))
        );

        let external = api::ExternalAccess {
            kind: "spotify".into(),
            access_token: "token".into(),
            client_id: Some("client".into()),
        };
        assert_eq!(
            external,
            external_access_from_proto(&external_access_to_proto(&external))
        );
    }
}
