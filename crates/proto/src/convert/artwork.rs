//! Track metadata edits and the artwork attached to entities.

use crate::*;

pub fn metadata_patch_to_proto(value: &api::TrackMetadataPatch) -> TrackMetadataPatch {
    TrackMetadataPatch {
        key: value.key.clone(),
        title: value.title.clone(),
        artist: value.artist.clone(),
        album: value.album.clone(),
        track_number: value.track_number,
        clear_track_number: value.clear_track_number,
        disc_number: value.disc_number,
        clear_disc_number: value.clear_disc_number,
    }
}

pub fn metadata_patch_from_proto(value: &TrackMetadataPatch) -> api::TrackMetadataPatch {
    api::TrackMetadataPatch {
        key: value.key.clone(),
        title: value.title.clone(),
        artist: value.artist.clone(),
        album: value.album.clone(),
        track_number: value.track_number,
        clear_track_number: value.clear_track_number,
        disc_number: value.disc_number,
        clear_disc_number: value.clear_disc_number,
    }
}

pub fn artwork_request_to_proto(value: &api::ArtworkRequest) -> ArtworkRequest {
    ArtworkRequest {
        entity: value.entity.as_ref().map(|entity| match entity {
            api::ArtworkEntity::Track { key } => artwork_request::Entity::Track(key.clone()),
            api::ArtworkEntity::Album { id } => artwork_request::Entity::Album(id.clone()),
            api::ArtworkEntity::Artist { name } => artwork_request::Entity::Artist(name.clone()),
            api::ArtworkEntity::Playlist { id } => artwork_request::Entity::Playlist(id.clone()),
        }),
        hq: value.hq,
    }
}

pub fn artwork_request_from_proto(value: &ArtworkRequest) -> api::ArtworkRequest {
    api::ArtworkRequest {
        entity: value.entity.as_ref().map(|entity| match entity {
            artwork_request::Entity::Track(key) => api::ArtworkEntity::Track { key: key.clone() },
            artwork_request::Entity::Album(id) => api::ArtworkEntity::Album { id: id.clone() },
            artwork_request::Entity::Artist(name) => {
                api::ArtworkEntity::Artist { name: name.clone() }
            }
            artwork_request::Entity::Playlist(id) => {
                api::ArtworkEntity::Playlist { id: id.clone() }
            }
        }),
        hq: value.hq,
    }
}

pub fn artwork_upload_to_proto(value: &api::ArtworkUpload) -> ArtworkUpload {
    ArtworkUpload {
        target: value.target.as_ref().map(|target| match target {
            api::ArtworkTarget::Track { key } => artwork_upload::Target::TrackKey(key.clone()),
            api::ArtworkTarget::Album { id } => artwork_upload::Target::AlbumId(id.clone()),
            api::ArtworkTarget::Artist { name } => artwork_upload::Target::ArtistName(name.clone()),
            api::ArtworkTarget::Playlist { id } => artwork_upload::Target::PlaylistId(id.clone()),
        }),
        content_type: value.content_type.clone(),
        data: value.data.clone(),
    }
}

pub fn artwork_upload_from_proto(value: &ArtworkUpload) -> api::ArtworkUpload {
    api::ArtworkUpload {
        target: value.target.as_ref().map(|target| match target {
            artwork_upload::Target::TrackKey(key) => api::ArtworkTarget::Track { key: key.clone() },
            artwork_upload::Target::AlbumId(id) => api::ArtworkTarget::Album { id: id.clone() },
            artwork_upload::Target::ArtistName(name) => {
                api::ArtworkTarget::Artist { name: name.clone() }
            }
            artwork_upload::Target::PlaylistId(id) => {
                api::ArtworkTarget::Playlist { id: id.clone() }
            }
        }),
        content_type: value.content_type.clone(),
        data: value.data.clone(),
    }
}

pub fn remove_artwork_to_proto(value: &api::ArtworkTarget) -> RemoveArtworkRequest {
    let target = match value {
        api::ArtworkTarget::Track { key } => remove_artwork_request::Target::TrackKey(key.clone()),
        api::ArtworkTarget::Album { id } => remove_artwork_request::Target::AlbumId(id.clone()),
        api::ArtworkTarget::Artist { name } => {
            remove_artwork_request::Target::ArtistName(name.clone())
        }
        api::ArtworkTarget::Playlist { id } => {
            remove_artwork_request::Target::PlaylistId(id.clone())
        }
    };
    RemoveArtworkRequest {
        target: Some(target),
    }
}

pub fn remove_artwork_from_proto(value: &RemoveArtworkRequest) -> Option<api::ArtworkTarget> {
    Some(match value.target.as_ref()? {
        remove_artwork_request::Target::TrackKey(key) => {
            api::ArtworkTarget::Track { key: key.clone() }
        }
        remove_artwork_request::Target::AlbumId(id) => api::ArtworkTarget::Album { id: id.clone() },
        remove_artwork_request::Target::ArtistName(name) => {
            api::ArtworkTarget::Artist { name: name.clone() }
        }
        remove_artwork_request::Target::PlaylistId(id) => {
            api::ArtworkTarget::Playlist { id: id.clone() }
        }
    })
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
