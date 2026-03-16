#[cfg(any(target_os = "macos", target_os = "ios"))]
mod macos;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use macos::{SystemEvent, init, set_background_handler, update_now_playing, wake_run_loop};

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub use android::{SystemEvent, init, set_background_handler, update_now_playing};

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::{SystemEvent, poll_event, update_now_playing};
