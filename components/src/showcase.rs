use crate::track_row::TrackRow;
use config::{AppConfig, MusicSource};
use dioxus::prelude::*;
use reader::{Library, Track};

#[derive(Props, Clone, PartialEq)]
pub struct ShowcaseProps {
    pub name: String,
    pub description: String,
    pub cover_url: Option<String>,
    pub tracks: Vec<Track>,
    pub library: Signal<Library>,
    pub on_play: EventHandler<usize>,
    pub on_add_to_playlist: Option<EventHandler<usize>>,
    pub on_delete_track: Option<EventHandler<usize>>,
    pub active_track: Option<std::path::PathBuf>,
    pub on_click_menu: Option<EventHandler<usize>>,
    pub on_close_menu: Option<EventHandler<()>>,
    pub actions: Option<Element>,
}

#[component]
pub fn Showcase(props: ShowcaseProps) -> Element {
    let config = use_context::<Signal<AppConfig>>();
    let total_seconds: u64 = props.tracks.iter().map(|t| t.duration).sum();
    let duration_min = total_seconds / 60;

    let lib = props.library.read();
    let is_jellyfin = config.read().active_source == MusicSource::Jellyfin;

    rsx! {
         div { class: "w-full",
             div {
                 class: "flex flex-col md:flex-row items-center md:items-end gap-6 md:gap-8 mb-8 md:mb-12 px-4 md:px-0 w-full",
                 div { class: "w-56 h-56 md:w-80 md:h-80 rounded-2xl md:rounded-[32px] bg-stone-800 overflow-hidden relative flex-shrink-0 shadow-2xl",
                     if let Some(url) = &props.cover_url {
                         img { src: "{url}", class: "w-full h-full object-cover" }
                     } else {
                         div { class: "w-full h-full flex flex-col items-center justify-center text-white/20",
                             i { class: "fa-solid fa-music text-6xl mb-4" }
                         }
                     }
                 }
                 div { class: "flex-1 w-full text-center md:text-left",
                     if !props.description.is_empty() {
                         h5 { class: "text-[11px] md:text-sm font-black tracking-[0.2em] text-white/40 uppercase mb-2", "{props.description}" }
                     }
                     h1 { class: "text-3xl md:text-7xl font-extrabold text-white mb-4 md:mb-6 tracking-tight leading-tight", "{props.name}" }
                     div { class: "flex items-center justify-center md:justify-start gap-4 text-[13px] font-medium text-white/40 mb-6 md:mb-0",
                         p { "{props.tracks.len()} songs" }
                         span { class: "text-white/20", "•" }
                         p { "{duration_min} min" }
                     }
                 }

                div { class: "flex items-center gap-4 w-full md:w-auto justify-center md:justify-start pr-2 md:pr-0",
                     if !props.tracks.is_empty() {
                         button {
                             class: "w-14 h-14 md:w-16 md:h-16 rounded-full bg-indigo-500 hover:bg-indigo-400 text-black flex items-center justify-center transition-all hover:scale-105 shadow-lg shadow-indigo-500/20 active:scale-95",
                             onclick: move |_| props.on_play.call(0),
                             i { class: "fa-solid fa-play text-xl md:text-2xl ml-1" }
                         }
                     }
                     if let Some(actions) = props.actions {
                         {actions}
                     }
                 }
             }

             div { class: "space-y-1",
                 if props.tracks.is_empty() {
                     div { class: "py-12 flex flex-col items-center justify-center text-slate-600",
                         i { class: "fa-regular fa-folder-open text-4xl mb-4" }
                         p { class: "text-lg", "No songs here." }
                     }
                 } else {
                     div { class: "hidden md:grid grid-cols-[auto_1fr_1fr_auto_auto] gap-4 px-4 py-2 border-b border-white/5 text-sm font-medium text-slate-500 mb-2 uppercase tracking-wider",
                          div { class: "w-8 text-center", "#" }
                          div { "Title" }
                          div { "Album" }
                     }

                     for (idx, track) in props.tracks.iter().enumerate() {
                         {
                             let cover_url = if is_jellyfin {
                                 if let Some(server) = &config.read().server {
                                     let path_str = track.path.to_string_lossy();
                                     let parts: Vec<&str> = path_str.split(':').collect();
                                     if parts.len() >= 2 {
                                         let id = parts[1];
                                         let mut url = format!("{}/Items/{}/Images/Primary", server.url, id);
                                         let mut params = Vec::new();
                                         if parts.len() >= 3 { params.push(format!("tag={}", parts[2])); }
                                         if let Some(token) = &server.access_token { params.push(format!("api_key={}", token)); }
                                         if !params.is_empty() {
                                             url.push('?');
                                             url.push_str(&params.join("&"));
                                         }
                                         Some(url)
                                     } else { None }
                                 } else { None }
                             } else {
                                 lib.albums.iter()
                                    .find(|a| a.id == track.album_id)
                                    .and_then(|a| utils::format_artwork_url(a.cover_path.as_ref()))
                             };

                             rsx! {
                                 TrackRow {
                                     key: "{track.path.display()}",
                                     track: track.clone(),
                                     cover_url: cover_url,
                                     is_menu_open: props.active_track.as_ref() == Some(&track.path),
                                     on_click_menu: move |_| {
                                        if let Some(handler) = &props.on_click_menu {
                                            handler.call(idx);
                                        }
                                     },
                                     on_add_to_playlist: move |_| {
                                        if let Some(handler) = &props.on_add_to_playlist {
                                            handler.call(idx);
                                        }
                                     },
                                     on_close_menu: move |_| {
                                        if let Some(handler) = &props.on_close_menu {
                                            handler.call(());
                                        }
                                     },
                                     on_delete: move |_| {
                                        if let Some(handler) = &props.on_delete_track {
                                            handler.call(idx);
                                        }
                                     },
                                     on_play: move |_| props.on_play.call(idx)
                                 }
                             }
                         }
                     }
                 }
             }
         }
    }
}
