use config::{AppConfig, SortOrder};
use dioxus::prelude::*;
use reader::Library;
use reader::models::Track;
use std::collections::HashMap;

pub struct LibraryItems {
    pub all_tracks: Memo<Vec<(Track, Option<String>)>>,
    pub artist_count: Memo<usize>,
    pub sort_order: Signal<SortOrder>,
}

pub fn use_library_items(library: Signal<Library>) -> LibraryItems {
    let config = use_context::<Signal<AppConfig>>();

    let initial_sort_order = config.read().sort_order.clone();
    let sort_order = use_signal(move || initial_sort_order);

    let artist_count = use_memo(move || {
        let lib = library.read();
        let mut artists = std::collections::HashSet::new();
        for album in &lib.albums {
            artists.insert(&album.artist);
        }
        for track in &lib.tracks {
            artists.insert(&track.artist);
        }
        artists.len()
    });

    let all_tracks = use_memo(move || {
        let lib = library.read();

        let album_covers: HashMap<_, _> = lib
            .albums
            .iter()
            .map(|a| {
                (
                    &a.id,
                    a.cover_path
                        .as_ref()
                        .and_then(|p| utils::format_artwork_url(Some(p))),
                )
            })
            .collect();

        let mut tracks: Vec<(Track, Option<String>)> = lib
            .tracks
            .iter()
            .map(|track| {
                let cover_url = album_covers.get(&track.album_id).cloned().flatten();
                (track.clone(), cover_url)
            })
            .collect();

        match *sort_order.read() {
            SortOrder::Title => tracks.sort_by_cached_key(|(a, _)| {
                (a.title.to_lowercase(), a.artist.to_lowercase(), a.album.to_lowercase(), a.disc_number, a.track_number)
            }),
            SortOrder::Artist => tracks.sort_by_cached_key(|(a, _)| {
                (a.artist.to_lowercase(), a.album.to_lowercase(), a.disc_number, a.track_number, a.title.to_lowercase())
            }),
            SortOrder::Album => tracks.sort_by_cached_key(|(a, _)| {
                (a.album.to_lowercase(), a.disc_number, a.track_number, a.title.to_lowercase())
            }),
        }

        tracks
    });

    LibraryItems {
        all_tracks,
        artist_count,
        sort_order,
    }
}
