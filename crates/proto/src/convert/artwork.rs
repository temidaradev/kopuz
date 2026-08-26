//! Track metadata edits and the artwork attached to entities.

use super::macros::{artwork_target_conversion, struct_conversion};
use crate::*;

artwork_target_conversion!(
    artwork_target_to_upload,
    artwork_target_from_upload,
    artwork_upload::Target,
    track artwork_upload::Target::TrackKey,
    album artwork_upload::Target::AlbumId,
    artist artwork_upload::Target::ArtistName,
    playlist artwork_upload::Target::PlaylistId
);

artwork_target_conversion!(
    artwork_target_to_request,
    artwork_target_from_request,
    artwork_request::Entity,
    track artwork_request::Entity::Track,
    album artwork_request::Entity::Album,
    artist artwork_request::Entity::Artist,
    playlist artwork_request::Entity::Playlist
);

artwork_target_conversion!(
    artwork_target_to_remove,
    artwork_target_from_remove,
    remove_artwork_request::Target,
    track remove_artwork_request::Target::TrackKey,
    album remove_artwork_request::Target::AlbumId,
    artist remove_artwork_request::Target::ArtistName,
    playlist remove_artwork_request::Target::PlaylistId
);

struct_conversion!(
    metadata_patch_to_proto,
    metadata_patch_from_proto,
    api::TrackMetadataPatch,
    TrackMetadataPatch,
    copy {
        track_number,
        clear_track_number,
        disc_number,
        clear_disc_number
    },
    clone {
        key,
        title,
        artist,
        album
    }
);

pub fn artwork_request_to_proto(value: &api::ArtworkRequest) -> ArtworkRequest {
    ArtworkRequest {
        entity: value.entity.as_ref().map(artwork_target_to_request),
        hq: value.hq,
    }
}

pub fn artwork_request_from_proto(value: &ArtworkRequest) -> api::ArtworkRequest {
    api::ArtworkRequest {
        entity: value.entity.as_ref().map(artwork_target_from_request),
        hq: value.hq,
    }
}

pub fn artwork_upload_to_proto(value: &api::ArtworkUpload) -> ArtworkUpload {
    ArtworkUpload {
        target: value.target.as_ref().map(artwork_target_to_upload),
        content_type: value.content_type.clone(),
        data: value.data.clone(),
    }
}

pub fn artwork_upload_from_proto(value: &ArtworkUpload) -> api::ArtworkUpload {
    api::ArtworkUpload {
        target: value.target.as_ref().map(artwork_target_from_upload),
        content_type: value.content_type.clone(),
        data: value.data.clone(),
    }
}

pub fn remove_artwork_to_proto(value: &api::ArtworkTarget) -> RemoveArtworkRequest {
    RemoveArtworkRequest {
        target: Some(artwork_target_to_remove(value)),
    }
}

pub fn remove_artwork_from_proto(value: &RemoveArtworkRequest) -> Option<api::ArtworkTarget> {
    value.target.as_ref().map(artwork_target_from_remove)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_patch_round_trips_with_clear_flags() {
        let metadata = api::TrackMetadataPatch {
            key: "track".into(),
            title: Some("Title".into()),
            artist: None,
            album: Some("Album".into()),
            track_number: Some(3),
            clear_track_number: false,
            disc_number: None,
            clear_disc_number: true,
        };
        assert_eq!(
            metadata,
            metadata_patch_from_proto(&metadata_patch_to_proto(&metadata))
        );
    }

    #[test]
    fn every_artwork_target_and_entity_round_trips() {
        for target in [
            api::ArtworkTarget::Track {
                key: "track".into(),
            },
            api::ArtworkTarget::Album { id: "album".into() },
            api::ArtworkTarget::Artist {
                name: "artist".into(),
            },
            api::ArtworkTarget::Playlist {
                id: "playlist".into(),
            },
        ] {
            let upload = api::ArtworkUpload {
                target: Some(target.clone()),
                content_type: "image/png".into(),
                data: vec![1, 2, 3],
            };
            assert_eq!(
                upload,
                artwork_upload_from_proto(&artwork_upload_to_proto(&upload))
            );
            assert_eq!(
                target,
                remove_artwork_from_proto(&remove_artwork_to_proto(&target))
                    .expect("artwork target survives")
            );
        }
        assert_eq!(
            api::ArtworkUpload::default(),
            artwork_upload_from_proto(&artwork_upload_to_proto(&api::ArtworkUpload::default()))
        );

        for entity in [
            api::ArtworkEntity::Track {
                key: "track".into(),
            },
            api::ArtworkEntity::Album { id: "album".into() },
            api::ArtworkEntity::Artist {
                name: "artist".into(),
            },
            api::ArtworkEntity::Playlist {
                id: "playlist".into(),
            },
        ] {
            let request = api::ArtworkRequest {
                entity: Some(entity),
                hq: true,
            };
            assert_eq!(
                request,
                artwork_request_from_proto(&artwork_request_to_proto(&request))
            );
        }
        assert_eq!(
            api::ArtworkRequest::default(),
            artwork_request_from_proto(&artwork_request_to_proto(&api::ArtworkRequest::default()))
        );
    }
}
