//! Playlists and the folders that hold them.

use crate::*;

pub fn playlist_info_to_proto(value: &api::PlaylistInfo) -> PlaylistInfo {
    PlaylistInfo {
        id: value.id.clone(),
        name: value.name.clone(),
        track_count: value.track_count,
        artwork: value.artwork.clone(),
        track_keys: value.track_keys.clone(),
        manual_artwork: value.manual_artwork,
    }
}

pub fn playlist_info_from_proto(value: &PlaylistInfo) -> api::PlaylistInfo {
    api::PlaylistInfo {
        id: value.id.clone(),
        name: value.name.clone(),
        track_count: value.track_count,
        artwork: value.artwork.clone(),
        track_keys: value.track_keys.clone(),
        manual_artwork: value.manual_artwork,
    }
}

pub fn playlist_folder_to_proto(value: &api::PlaylistFolderInfo) -> PlaylistFolderInfo {
    PlaylistFolderInfo {
        id: value.id.clone(),
        name: value.name.clone(),
        playlist_ids: value.playlist_ids.clone(),
    }
}

pub fn playlist_folder_from_proto(value: &PlaylistFolderInfo) -> api::PlaylistFolderInfo {
    api::PlaylistFolderInfo {
        id: value.id.clone(),
        name: value.name.clone(),
        playlist_ids: value.playlist_ids.clone(),
    }
}

pub fn playlist_catalog_to_proto(value: &api::PlaylistCatalog) -> PlaylistCatalog {
    PlaylistCatalog {
        playlists: value.playlists.iter().map(playlist_info_to_proto).collect(),
        folders: value.folders.iter().map(playlist_folder_to_proto).collect(),
    }
}

pub fn playlist_catalog_from_proto(value: &PlaylistCatalog) -> api::PlaylistCatalog {
    api::PlaylistCatalog {
        playlists: value
            .playlists
            .iter()
            .map(playlist_info_from_proto)
            .collect(),
        folders: value
            .folders
            .iter()
            .map(playlist_folder_from_proto)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_round_trips_with_folders() {
        let catalog = api::PlaylistCatalog {
            playlists: vec![api::PlaylistInfo {
                id: "playlist".into(),
                name: "Playlist".into(),
                track_count: 2,
                track_keys: vec!["a".into(), "b".into()],
                artwork: Some("playlist".into()),
                manual_artwork: false,
            }],
            folders: vec![api::PlaylistFolderInfo {
                id: "folder".into(),
                name: "Folder".into(),
                playlist_ids: vec!["playlist".into()],
            }],
        };
        assert_eq!(
            catalog,
            playlist_catalog_from_proto(&playlist_catalog_to_proto(&catalog))
        );
    }
}
