//! Albums, artists, search, and the provider discover catalog.

use super::macros::struct_conversion;
use super::*;
use crate::*;

struct_conversion!(
    album_filter_to_proto,
    album_filter_from_proto,
    api::AlbumFilter,
    AlbumFilter,
    copy {},
    clone {
        search,
        artist,
        genre,
        sort
    }
);

struct_conversion!(
    album_info_to_proto,
    album_info_from_proto,
    api::AlbumInfo,
    AlbumInfo,
    copy {
        year,
        manual_artwork
    },
    clone {
        id,
        title,
        artist,
        genre,
        artwork
    }
);

pub fn album_page_to_proto(value: &api::AlbumPage) -> AlbumPage {
    AlbumPage {
        total: value.total,
        offset: value.offset,
        items: value.items.iter().map(album_info_to_proto).collect(),
    }
}

pub fn album_page_from_proto(value: &AlbumPage) -> api::AlbumPage {
    api::AlbumPage {
        total: value.total,
        offset: value.offset,
        items: value.items.iter().map(album_info_from_proto).collect(),
    }
}

struct_conversion!(
    artist_info_to_proto,
    artist_info_from_proto,
    api::ArtistInfo,
    ArtistInfo,
    copy {
        track_count,
        album_count,
        manual_artwork
    },
    clone { name, artwork }
);

pub fn artist_page_to_proto(value: &api::ArtistPage) -> ArtistPage {
    ArtistPage {
        total: value.total,
        offset: value.offset,
        items: value.items.iter().map(artist_info_to_proto).collect(),
    }
}

pub fn artist_page_from_proto(value: &ArtistPage) -> api::ArtistPage {
    api::ArtistPage {
        total: value.total,
        offset: value.offset,
        items: value.items.iter().map(artist_info_from_proto).collect(),
    }
}

pub fn search_results_to_proto(value: &api::SearchResults) -> SearchResults {
    SearchResults {
        tracks: value.tracks.iter().map(track_info_to_proto).collect(),
        albums: value.albums.iter().map(album_info_to_proto).collect(),
    }
}

pub fn search_results_from_proto(value: &SearchResults) -> api::SearchResults {
    api::SearchResults {
        tracks: value.tracks.iter().map(track_info_from_proto).collect(),
        albums: value.albums.iter().map(album_info_from_proto).collect(),
    }
}

pub fn catalog_item_to_proto(value: &api::CatalogItem) -> CatalogItem {
    CatalogItem {
        kind: catalog_item_kind_to_proto(value.kind) as i32,
        id: value.id.clone(),
        title: value.title.clone(),
        subtitle: value.subtitle.clone(),
        artwork: value.artwork.clone(),
        track: value.track.as_ref().map(track_info_to_proto),
    }
}

pub fn catalog_item_from_proto(value: &CatalogItem) -> api::CatalogItem {
    api::CatalogItem {
        kind: catalog_item_kind_from_proto(value.kind),
        id: value.id.clone(),
        title: value.title.clone(),
        subtitle: value.subtitle.clone(),
        artwork: value.artwork.clone(),
        track: value.track.as_ref().map(track_info_from_proto),
    }
}

pub fn catalog_page_to_proto(value: &api::CatalogPage) -> CatalogPage {
    CatalogPage {
        shelves: value
            .shelves
            .iter()
            .map(|shelf| CatalogShelf {
                title: shelf.title.clone(),
                strapline: shelf.strapline.clone(),
                items: shelf.items.iter().map(catalog_item_to_proto).collect(),
                more_ref: shelf.more_ref.clone(),
                list: shelf.list,
            })
            .collect(),
        continuation: value.continuation.clone(),
    }
}

pub fn catalog_page_from_proto(value: &CatalogPage) -> api::CatalogPage {
    api::CatalogPage {
        shelves: value
            .shelves
            .iter()
            .map(|shelf| api::CatalogShelf {
                title: shelf.title.clone(),
                strapline: shelf.strapline.clone(),
                items: shelf.items.iter().map(catalog_item_from_proto).collect(),
                more_ref: shelf.more_ref.clone(),
                list: shelf.list,
            })
            .collect(),
        continuation: value.continuation.clone(),
    }
}

pub fn catalog_detail_request_to_proto(value: &api::CatalogDetailRequest) -> CatalogDetailRequest {
    CatalogDetailRequest {
        kind: catalog_item_kind_to_proto(value.kind) as i32,
        id: value.id.clone(),
        continuation: value.continuation.clone(),
    }
}

pub fn catalog_detail_request_from_proto(
    value: &CatalogDetailRequest,
) -> api::CatalogDetailRequest {
    api::CatalogDetailRequest {
        kind: catalog_item_kind_from_proto(value.kind),
        id: value.id.clone(),
        continuation: value.continuation.clone(),
    }
}

pub fn catalog_detail_to_proto(value: &api::CatalogDetail) -> CatalogDetail {
    CatalogDetail {
        kind: catalog_item_kind_to_proto(value.kind) as i32,
        id: value.id.clone(),
        title: value.title.clone(),
        subtitle: value.subtitle.clone(),
        description: value.description.clone(),
        artwork: value.artwork.clone(),
        playback_id: value.playback_id.clone(),
        year: value.year.clone(),
        tracks: value.tracks.iter().map(track_info_to_proto).collect(),
        shelves: value
            .shelves
            .iter()
            .map(|shelf| CatalogShelf {
                title: shelf.title.clone(),
                strapline: shelf.strapline.clone(),
                items: shelf.items.iter().map(catalog_item_to_proto).collect(),
                more_ref: shelf.more_ref.clone(),
                list: shelf.list,
            })
            .collect(),
        continuation: value.continuation.clone(),
    }
}

pub fn catalog_detail_from_proto(value: &CatalogDetail) -> api::CatalogDetail {
    api::CatalogDetail {
        kind: catalog_item_kind_from_proto(value.kind),
        id: value.id.clone(),
        title: value.title.clone(),
        subtitle: value.subtitle.clone(),
        description: value.description.clone(),
        artwork: value.artwork.clone(),
        playback_id: value.playback_id.clone(),
        year: value.year.clone(),
        tracks: value.tracks.iter().map(track_info_from_proto).collect(),
        shelves: value
            .shelves
            .iter()
            .map(|shelf| api::CatalogShelf {
                title: shelf.title.clone(),
                strapline: shelf.strapline.clone(),
                items: shelf.items.iter().map(catalog_item_from_proto).collect(),
                more_ref: shelf.more_ref.clone(),
                list: shelf.list,
            })
            .collect(),
        continuation: value.continuation.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn album_and_artist_pages_round_trip() {
        let album = api::AlbumInfo {
            id: "album".into(),
            title: "Title".into(),
            artist: "Artist".into(),
            genre: "Genre".into(),
            year: 2025,
            artwork: Some("album".into()),
            manual_artwork: true,
        };
        assert_eq!(album, album_info_from_proto(&album_info_to_proto(&album)));

        let album_page = api::AlbumPage {
            total: 4,
            offset: 2,
            items: vec![album],
        };
        assert_eq!(
            album_page,
            album_page_from_proto(&album_page_to_proto(&album_page))
        );

        let artist_page = api::ArtistPage {
            total: 1,
            offset: 0,
            items: vec![api::ArtistInfo {
                name: "Artist".into(),
                track_count: 8,
                album_count: 2,
                artwork: Some("Artist".into()),
                manual_artwork: true,
            }],
        };
        assert_eq!(
            artist_page,
            artist_page_from_proto(&artist_page_to_proto(&artist_page))
        );
    }

    #[test]
    fn catalog_round_trips_for_every_item_kind() {
        let track = api::TrackInfo {
            key: "track".into(),
            title: "Track".into(),
            ..Default::default()
        };
        for kind in [
            api::CatalogItemKind::Track,
            api::CatalogItemKind::Album,
            api::CatalogItemKind::Playlist,
            api::CatalogItemKind::Artist,
            api::CatalogItemKind::Mood,
        ] {
            let page = api::CatalogPage {
                shelves: vec![api::CatalogShelf {
                    title: "Shelf".into(),
                    strapline: Some("Strapline".into()),
                    items: vec![api::CatalogItem {
                        kind,
                        id: "item".into(),
                        title: "Item".into(),
                        subtitle: Some("Subtitle".into()),
                        artwork: Some("item".into()),
                        track: Some(track.clone()),
                    }],
                    more_ref: Some("more".into()),
                    list: true,
                }],
                continuation: Some("next".into()),
            };
            assert_eq!(page, catalog_page_from_proto(&catalog_page_to_proto(&page)));

            let request = api::CatalogDetailRequest {
                kind,
                id: "detail".into(),
                continuation: Some("cursor".into()),
            };
            assert_eq!(
                request,
                catalog_detail_request_from_proto(&catalog_detail_request_to_proto(&request))
            );

            let detail = api::CatalogDetail {
                kind,
                id: "detail".into(),
                title: "Detail".into(),
                subtitle: Some("Subtitle".into()),
                description: Some("Description".into()),
                artwork: Some("artwork".into()),
                playback_id: Some("playback".into()),
                year: Some("2026".into()),
                tracks: vec![track.clone()],
                shelves: page.shelves,
                continuation: Some("next".into()),
            };
            assert_eq!(
                detail,
                catalog_detail_from_proto(&catalog_detail_to_proto(&detail))
            );
        }
    }
}
