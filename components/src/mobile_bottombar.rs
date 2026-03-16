use dioxus::prelude::*;
use hooks::use_player_controller::PlayerController;

#[component]
pub fn MobileBottombar(
    current_song_title: Signal<String>,
    current_song_artist: Signal<String>,
    current_song_cover_url: Signal<String>,
    mut is_playing: Signal<bool>,
    mut is_fullscreen: Signal<bool>,
    mut current_song_progress: Signal<u64>,
    current_song_duration: Signal<u64>,
) -> Element {
    let mut ctrl = use_context::<PlayerController>();

    let progress_percent = if *current_song_duration.read() > 0 {
        (*current_song_progress.read() as f64 / *current_song_duration.read() as f64) * 100.0
    } else {
        0.0
    };

    rsx! {
        div {
            class: "fixed bottom-[env(safe-area-inset-bottom)] left-2 right-2 h-[72px] bg-[#121212]/95 backdrop-blur-3xl border border-white/10 rounded-[28px] flex items-center px-3 gap-3 z-[90] shadow-[0_12px_40px_rgba(0,0,0,0.8)] overflow-hidden",
            onclick: move |_| is_fullscreen.set(true),

            // Progress bar at the top
            div {
                class: "absolute top-0 left-0 h-[2px] bg-white/10 w-full",
                div {
                    class: "h-full bg-white shadow-[0_0_10px_rgba(255,255,255,0.5)] transition-all duration-300",
                    style: "width: {progress_percent}%"
                }
            }

            div {
                class: "w-11 h-11 bg-white/5 rounded-xl flex-shrink-0 overflow-hidden shadow-md",
                if current_song_cover_url.read().is_empty() {
                    i { class: "fa-solid fa-music text-white/20 m-auto" }
                } else {
                    img { src: "{current_song_cover_url}", class: "w-full h-full object-cover" }
                }
            }

            div {
                class: "flex-1 min-w-0 flex flex-col justify-center gap-0.5 pl-1",
                span { class: "text-[13px] font-bold text-white truncate drop-shadow-sm", "{current_song_title}" }
                span { class: "text-[11px] font-medium text-white/60 truncate", "{current_song_artist}" }
            }

            div {
                class: "flex items-center gap-1 pr-1",
                button {
                    class: "w-12 h-12 flex items-center justify-center text-white text-xl active:scale-90 transition-transform",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        ctrl.toggle();
                    },
                    i { class: if *is_playing.read() { "fa-solid fa-pause" } else { "fa-solid fa-play ml-1" } }
                }
                button {
                    class: "w-12 h-12 flex items-center justify-center text-white text-lg active:scale-90 transition-transform",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        ctrl.play_next();
                    },
                    i { class: "fa-solid fa-forward-step" }
                }
            }
        }
    }
}
