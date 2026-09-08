//! Conversion boilerplate shared by the domain modules: the enum and
//! plain-struct mappings, and the artwork-target oneof that three messages
//! repeat with different variant paths.

macro_rules! enum_conversion {
    (
        $to_proto:ident, $from_proto:ident,
        $api_type:path, $proto_type:path,
        default $default:path, unspecified $unspecified:path,
        { $($api_variant:path => $proto_variant:path),+ $(,)? }
        $(, unknown $unknown:path)?
    ) => {
        pub fn $to_proto(value: $api_type) -> $proto_type {
            match value {
                $($api_variant => $proto_variant,)+
                $($unknown => $unspecified,)?
            }
        }

        pub fn $from_proto(value: i32) -> $api_type {
            match <$proto_type>::try_from(value).unwrap_or($unspecified) {
                $($proto_variant => $api_variant,)+
                $unspecified => $default,
            }
        }
    };
}

macro_rules! struct_conversion {
    (
        $to_proto:ident, $from_proto:ident,
        $api_type:path, $proto_type:path,
        copy { $($copy:ident),* $(,)? },
        clone { $($clone:ident),* $(,)? }
    ) => {
        pub fn $to_proto(value: &$api_type) -> $proto_type {
            $proto_type {
                $($copy: value.$copy,)*
                $($clone: value.$clone.clone(),)*
            }
        }

        pub fn $from_proto(value: &$proto_type) -> $api_type {
            $api_type {
                $($copy: value.$copy,)*
                $($clone: value.$clone.clone(),)*
            }
        }
    };
}

macro_rules! artwork_target_conversion {
    (
        $to_proto:ident, $from_proto:ident, $proto_type:path,
        track $track:path, album $album:path,
        artist $artist:path, playlist $playlist:path
    ) => {
        fn $to_proto(value: &api::ArtworkTarget) -> $proto_type {
            match value {
                api::ArtworkTarget::Track { key } => $track(key.clone()),
                api::ArtworkTarget::Album { id } => $album(id.clone()),
                api::ArtworkTarget::Artist { name } => $artist(name.clone()),
                api::ArtworkTarget::Playlist { id } => $playlist(id.clone()),
            }
        }

        fn $from_proto(value: &$proto_type) -> api::ArtworkTarget {
            match value {
                $track(key) => api::ArtworkTarget::Track { key: key.clone() },
                $album(id) => api::ArtworkTarget::Album { id: id.clone() },
                $artist(name) => api::ArtworkTarget::Artist { name: name.clone() },
                $playlist(id) => api::ArtworkTarget::Playlist { id: id.clone() },
            }
        }
    };
}

pub(super) use {artwork_target_conversion, enum_conversion, struct_conversion};
