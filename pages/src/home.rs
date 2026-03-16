use config::{AppConfig, MusicSource};
use dioxus::prelude::*;
use reader::{FavoritesStore, Library, PlaylistStore};

use crate::jellyfin::home::JellyfinHome;
use crate::local::home::LocalHome;

#[component]
pub fn Home(
    library: Signal<Library>,
    playlist_store: Signal<PlaylistStore>,
    favorites_store: Signal<FavoritesStore>,
    on_select_album: EventHandler<String>,
    on_play_album: EventHandler<String>,
    on_select_playlist: EventHandler<String>,
    on_search_artist: EventHandler<String>,
) -> Element {
    let config = use_context::<Signal<AppConfig>>();
    let is_jellyfin = config.read().active_source == MusicSource::Jellyfin;
    let is_mobile = cfg!(any(target_os = "android", target_os = "ios"));

    rsx! {
        div {
            class: if is_mobile {
                "px-4 animate-fade-in w-full overflow-x-hidden"
            } else {
                "p-4 md:p-8 md:pb-8 space-y-10 md:space-y-12 animate-fade-in w-full max-w-[1600px] mx-auto"
            },

            if !is_mobile {
                div { class: "flex items-center justify-between mb-4 md:mb-2 pl-12 md:pl-0",
                    h1 { class: "text-4xl font-black text-white tracking-tight", "Home" }
                }
            } else {
                div { class: "flex items-center justify-between mb-6 mt-1",
                    h1 { class: "text-4xl font-black text-white tracking-tight", "Home" }
                }
            }

            if is_jellyfin {
                JellyfinHome {
                    library,
                    playlist_store,
                    favorites_store,
                    on_select_album,
                    on_play_album,
                    on_select_playlist,
                    on_search_artist,
                }
            } else {
                LocalHome {
                    library,
                    playlist_store,
                    favorites_store,
                    on_select_album,
                    on_play_album,
                    on_select_playlist,
                    on_search_artist,
                }
            }
        }
    }
}
