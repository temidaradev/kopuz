use dioxus::prelude::*;

#[component]
pub fn AddPlaylistPopup(
    playlist_name: Signal<String>,
    error: Signal<Option<String>>,
    on_close: EventHandler<()>,
    on_save: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "overlay",
            onclick: move |_| on_close.call(()),

            div {
                class: "popup",
                onclick: |e| e.stop_propagation(),

                h2 { "Add playlist" }

                if let Some(err) = error() {
                    p { class: "error", "{err}" }
                }

                input {
                    placeholder: "Playlist name",
                    value: "{playlist_name()}",
                    oninput: move |e| playlist_name.set(e.value()),
                    onkeydown: move |e| e.stop_propagation()
                }

                div { class: "actions",
                    button {
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }
                    button {
                        onclick: move |_| on_save.call(()),
                        "Save"
                    }
                }
            }
        }
    }
}
