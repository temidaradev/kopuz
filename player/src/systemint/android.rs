use jni::objects::JObject;
use jni::{JNIEnv, JavaVM};
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Clone, Copy)]
pub enum SystemEvent {
    Play,
    Pause,
    Toggle,
    Next,
    Prev,
}

static JVM: OnceLock<JavaVM> = OnceLock::new();
static BACKGROUND_HANDLER: OnceLock<Arc<Mutex<Option<Box<dyn Fn(SystemEvent) + Send + Sync>>>>> =
    OnceLock::new();

fn get_bg_handler() -> Arc<Mutex<Option<Box<dyn Fn(SystemEvent) + Send + Sync>>>> {
    BACKGROUND_HANDLER
        .get_or_init(|| Arc::new(Mutex::new(None)))
        .clone()
}

pub fn set_background_handler(handler: impl Fn(SystemEvent) + Send + Sync + 'static) {
    let handler_lock = get_bg_handler();
    let mut guard = handler_lock.lock().unwrap();
    *guard = Some(Box::new(handler));
    println!("[android] Background event handler registered");
}

fn dispatch_event(event: SystemEvent) {
    println!("[android] Dispatching system event: {:?}", event);
    if let Ok(guard) = get_bg_handler().lock() {
        if let Some(ref handler) = *guard {
            handler(event);
        }
    }
}

/// Initialize Android Media integration.
pub fn init() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        println!("[android] Initializing system integration...");
        let ctx = ndk_context::android_context();
        // Safety: We must ensure the VM pointer is valid. Dioxus/ndk-context usually provides a valid VM.
        let vm_ptr = ctx.vm();
        if !vm_ptr.is_null() {
            match unsafe { JavaVM::from_raw(vm_ptr.cast()) } {
                Ok(vm) => {
                    let _ = JVM.set(vm);
                    println!("[android] JVM captured successfully");
                }
                Err(e) => eprintln!("[android] Failed to capture JVM: {}", e),
            }
        }
    });
}

/// Get the app's internal files directory path.
/// We use environment variables or standard defaults to avoid complex JNI calls
/// during initialization which are prone to crashing in multi-threaded environments.
pub fn get_files_dir() -> Option<String> {
    // Android apps usually have their internal files dir at /data/data/package.name/files
    // Or /data/user/0/package.name/files.
    // We prefer checking common environment variables first.
    std::env::var("FILES_DIR").ok().or_else(|| {
        // Fallback: check if we are in a standard android structure
        let home = std::env::var("HOME").ok()?;
        if home.contains("com.temidaradev.rusic") {
            Some(format!("{}/files", home))
        } else {
            None
        }
    })
}

pub fn update_now_playing(
    title: &str,
    artist: &str,
    album: &str,
    duration: f64,
    position: f64,
    playing: bool,
    _artwork_path: Option<&str>,
) {
    init();

    let _vm = match JVM.get() {
        Some(v) => v,
        None => return,
    };

    // Logging for verification. Full MediaSession integration usually requires
    // a background Service in Java to be robust.
    println!(
        "[android] Update MediaSession (Stub): {} - {} (playing: {}) @ {:.1}/{:.1}",
        artist, title, playing, position, duration
    );
}

// JNI Entry points for the MediaSession Callbacks.
// These match the package name defined in AndroidManifest.xml: com.temidaradev.rusic
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_temidaradev_rusic_MediaCallback_onPlay(
    _env: JNIEnv,
    _class: JObject,
) {
    dispatch_event(SystemEvent::Play);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_temidaradev_rusic_MediaCallback_onPause(
    _env: JNIEnv,
    _class: JObject,
) {
    dispatch_event(SystemEvent::Pause);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_temidaradev_rusic_MediaCallback_onSkipToNext(
    _env: JNIEnv,
    _class: JObject,
) {
    dispatch_event(SystemEvent::Next);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_temidaradev_rusic_MediaCallback_onSkipToPrevious(
    _env: JNIEnv,
    _class: JObject,
) {
    dispatch_event(SystemEvent::Prev);
}
