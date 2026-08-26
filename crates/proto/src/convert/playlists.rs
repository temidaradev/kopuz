//! Playlists and the folders that hold them.

use super::macros::struct_conversion;
use crate::*;

struct_conversion!(
    playlist_info_to_proto,
    playlist_info_from_proto,
    api::PlaylistInfo,
    PlaylistInfo,
    copy {
        track_count,
        manual_artwork
    },
    clone {
        id,
        name,
        artwork,
        track_keys
    }
);

struct_conversion!(
    playlist_folder_to_proto,
    playlist_folder_from_proto,
    api::PlaylistFolderInfo,
    PlaylistFolderInfo,
    copy {},
    clone {
        id,
        name,
        playlist_ids
    }
);

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
