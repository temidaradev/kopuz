use std::sync::{Arc, Mutex, OnceLock};
use jni::objects::{JObject, JValue};
use jni::{JavaVM, JNIEnv};

#[derive(Debug, Clone, Copy)]
pub enum SystemEvent {
    Play,
    Pause,
    Toggle,
    Next,
    Prev,
}

static JVM: OnceLock<JavaVM> = OnceLock::new();
static BACKGROUND_HANDLER: OnceLock<Arc<Mutex<Option<Box<dyn Fn(SystemEvent) + Send + Sync>>>>> = OnceLock::new();

fn get_bg_handler() -> Arc<Mutex<Option<Box<dyn Fn(SystemEvent) + Send + Sync>>>> {
    BACKGROUND_HANDLER.get_or_init(|| Arc::new(Mutex::new(None))).clone()
}

pub fn set_background_handler(handler: impl Fn(SystemEvent) + Send + Sync + 'static) {
    let mut guard = get_bg_handler().lock().unwrap();
    *guard = Some(Box::new(handler));
}

fn dispatch_event(event: SystemEvent) {
    if let Ok(guard) = get_bg_handler().lock() {
        if let Some(ref handler) = *guard {
            handler(event);
        }
    }
}

/// Initialize Android MediaSession integration.
/// This expects to be called from a context where JNI is available.
pub fn init() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        // In a real Dioxus/Mobile environment, the JVM is typically provided via
        // ndk_context or similar hooks.
        if let Some(ctx) = ndk_context::android_context() {
            let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()).expect("valid jvm") };
            let _ = JVM.set(vm);
        }

        println!("[android] MediaSession system integration initialized");
        // Implementation note: On Android, actual MediaSession callbacks usually
        // require a MediaSession.Callback object created via JNI and attached
        // to the active MediaSession.
    });
}

pub fn update_now_playing(
    title: &str,
    artist: &str,
    album: &str,
    duration: f64,
    position: f64,
    playing: bool,
    artwork_path: Option<&str>,
) {
    init();

    let vm = match JVM.get() {
        Some(v) => v,
        None => return,
    };

    let _ = vm.attach_current_thread().map(|env| {
        // This is a simplified representation of the JNI calls required to update
        // a MediaSession's Metadata and PlaybackState.
        // In a production app, you would fetch your singleton MediaSession instance
        // and call setMetadata() and setPlaybackState() on it.

        // Example logic for Title:
        // let metadata_builder = env.new_object("android/support/v4/media/MediaMetadataCompat$Builder", "()V", &[]).unwrap();
        // let title_key = env.new_string("android.media.metadata.TITLE").unwrap();
        // let title_val = env.new_string(title).unwrap();
        // env.call_method(metadata_builder, "putString", "(Ljava/lang/String;Ljava/lang/String;)Landroid/support/v4/media/MediaMetadataCompat$Builder;", &[title_key.into(), title_val.into()]).unwrap();

        // Due to the complexity of Android MediaSession API (requiring Builders and specific Constants),
        // most developers use a small Java helper class or a simplified JNI wrapper.

        if playing {
            // Update state to STATE_PLAYING
        } else {
            // Update state to STATE_PAUSED
        }
    });
}

// JNI Entry points for the MediaSession Callbacks
#[no_mangle]
pub extern "system" fn Java_com_temidaradev_rusic_MediaCallback_onPlay(_env: JNIEnv, _class: JObject) {
    dispatch_event(SystemEvent::Play);
}

#[no_mangle]
pub extern "system" fn Java_com_temidaradev_rusic_MediaCallback_onPause(_env: JNIEnv, _class: JObject) {
    dispatch_event(SystemEvent::Pause);
}

#[no_mangle]
pub extern "system" fn Java_com_temidaradev_rusic_MediaCallback_onSkipToNext(_env: JNIEnv, _class: JObject) {
    dispatch_event(SystemEvent::Next);
}

#[no_mangle]
pub extern "system" fn Java_com_temidaradev_rusic_MediaCallback_onSkipToPrevious(_env: JNIEnv, _class: JObject) {
    dispatch_event(SystemEvent::Prev);
}
