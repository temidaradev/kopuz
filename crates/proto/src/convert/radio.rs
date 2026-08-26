//! Radio stations and the registries they come from.

use super::macros::struct_conversion;
use crate::*;

pub fn radio_station_to_proto(value: &api::RadioStationInfo) -> RadioStationInfo {
    RadioStationInfo {
        id: value.id.clone(),
        name: value.name.clone(),
        description: value.description.clone(),
        icon: value.icon.clone(),
        artwork: value.artwork.clone(),
        tags: value.tags.clone(),
        streams: value
            .streams
            .iter()
            .map(|stream| RadioStreamInfo {
                id: stream.id.clone(),
                name: stream.name.clone(),
                url: stream.url.clone(),
                icon: stream.icon.clone(),
            })
            .collect(),
        pinned: value.pinned,
    }
}

pub fn radio_station_from_proto(value: &RadioStationInfo) -> api::RadioStationInfo {
    api::RadioStationInfo {
        id: value.id.clone(),
        name: value.name.clone(),
        description: value.description.clone(),
        icon: value.icon.clone(),
        artwork: value.artwork.clone(),
        tags: value.tags.clone(),
        streams: value
            .streams
            .iter()
            .map(|stream| api::RadioStreamInfo {
                id: stream.id.clone(),
                name: stream.name.clone(),
                url: stream.url.clone(),
                icon: stream.icon.clone(),
            })
            .collect(),
        pinned: value.pinned,
    }
}

struct_conversion!(
    radio_registry_to_proto,
    radio_registry_from_proto,
    api::RadioRegistryInfo,
    RadioRegistryInfo,
    copy { enabled, built_in },
    clone { url }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stations_and_registries_round_trip() {
        let station = api::RadioStationInfo {
            id: "station".into(),
            name: "Station".into(),
            description: "Description".into(),
            icon: "https://example.com/icon.png".into(),
            artwork: Some("https://example.com/art.png".into()),
            tags: vec!["tag".into()],
            streams: vec![api::RadioStreamInfo {
                id: "main".into(),
                name: "Main".into(),
                url: "https://example.com/stream".into(),
                icon: Some("https://example.com/stream.png".into()),
            }],
            pinned: true,
        };
        assert_eq!(
            station,
            radio_station_from_proto(&radio_station_to_proto(&station))
        );

        let registry = api::RadioRegistryInfo {
            url: "https://example.com/index.json".into(),
            enabled: true,
            built_in: false,
        };
        assert_eq!(
            registry,
            radio_registry_from_proto(&radio_registry_to_proto(&registry))
        );
    }
}
