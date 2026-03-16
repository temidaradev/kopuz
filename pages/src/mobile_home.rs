use dioxus::prelude::*;
use reader::{FavoritesStore, Library, PlaylistStore};

#[component]
pub fn MobileHome(
    library: Signal<Library>,
    playlist_store: Signal<PlaylistStore>,
    favorites_store: Signal<FavoritesStore>,
    on_select_album: EventHandler<String>,
    on_play_album: EventHandler<String>,
    on_select_playlist: EventHandler<String>,
    on_search_artist: EventHandler<String>,
) -> Element {
    rsx! {
        div { "Mobile Home" }
    }
}
