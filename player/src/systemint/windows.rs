use std::sync::OnceLock;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use windows::core::{BOOL, Ref, w};
use windows::{
    Foundation::{TimeSpan, TypedEventHandler, Uri},
    Media::{
        MediaPlaybackStatus, MediaPlaybackType, PlaybackPositionChangeRequestedEventArgs,
        SystemMediaTransportControls, SystemMediaTransportControlsButton,
        SystemMediaTransportControlsButtonPressedEventArgs,
        SystemMediaTransportControlsTimelineProperties,
    },
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream, RandomAccessStreamReference},
    Win32::{
        Foundation::{HWND, LPARAM},
        System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx},
        System::Threading::GetCurrentProcessId,
        System::WinRT::RoGetActivationFactory,
        UI::WindowsAndMessaging::{
            CreateWindowExW, EnumWindows, GetWindowThreadProcessId, HWND_MESSAGE, IsWindowVisible,
            WINDOW_EX_STYLE, WINDOW_STYLE,
        },
    },
};

#[derive(Debug)]
pub enum SystemEvent {
    Play,
    Pause,
    Toggle,
    Next,
    Prev,
    Seek(f64),
}

struct SendableSmtc(SystemMediaTransportControls);
unsafe impl Send for SendableSmtc {}
unsafe impl Sync for SendableSmtc {}

static EVENT_SENDER: OnceLock<UnboundedSender<SystemEvent>> = OnceLock::new();
static EVENT_RECEIVER: OnceLock<Mutex<UnboundedReceiver<SystemEvent>>> = OnceLock::new();
static SMTC: OnceLock<SendableSmtc> = OnceLock::new();

fn get_tx() -> UnboundedSender<SystemEvent> {
    EVENT_SENDER
        .get_or_init(|| {
            let (tx, rx) = mpsc::unbounded_channel();
            let _ = EVENT_RECEIVER.set(Mutex::new(rx));
            tx
        })
        .clone()
}

pub fn poll_event() -> Option<SystemEvent> {
    EVENT_RECEIVER.get()?.try_lock().ok()?.try_recv().ok()
}

pub async fn wait_event() -> Option<SystemEvent> {
    if let Some(rx) = EVENT_RECEIVER.get() {
        let mut guard = rx.lock().await;
        guard.recv().await
    } else {
        None
    }
}

// HWND discovery
struct EnumData {
    pid: u32,
    hwnd: HWND,
    // fallback for when no visible window exists yet
    any_hwnd: HWND,
}

unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data = unsafe { &mut *(lparam.0 as *mut EnumData) };
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    if pid == data.pid && unsafe { IsWindowVisible(hwnd).as_bool() } {
        data.hwnd = hwnd;
        BOOL(0) // stop enumeration
    } else {
        if pid == data.pid && data.any_hwnd.0.is_null() {
            data.any_hwnd = hwnd;
        }
        BOOL(1)
    }
}

fn create_message_window() -> Option<HWND> {
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("STATIC"),
            w!("KopuzSMTC"),
            WINDOW_STYLE::default(),
            0,
            0,
            0,
            0,
            Some(HWND_MESSAGE),
            None,
            None,
            None,
        )
    };
    match hwnd {
        Ok(h) if !h.0.is_null() => Some(h),
        _ => None,
    }
}

fn find_main_hwnd() -> Option<HWND> {
    let mut data = EnumData {
        pid: unsafe { GetCurrentProcessId() },
        hwnd: HWND(std::ptr::null_mut()),
        any_hwnd: HWND(std::ptr::null_mut()),
    };

    // return Err even on success
    let _ = unsafe { EnumWindows(Some(enum_proc), LPARAM(&mut data as *mut EnumData as isize)) };

    if !data.hwnd.0.is_null() {
        return Some(data.hwnd);
    }

    // hacky
    if !data.any_hwnd.0.is_null() {
        return Some(data.any_hwnd);
    }
    create_message_window()
}

// SMTC setup
use windows::Win32::System::WinRT::ISystemMediaTransportControlsInterop;

fn setup_smtc(hwnd: HWND) {
    if SMTC.get().is_some() {
        return;
    }

    let result = (|| unsafe {
        let class_id = windows::core::HSTRING::from("Windows.Media.SystemMediaTransportControls");
        let interop: ISystemMediaTransportControlsInterop = RoGetActivationFactory(&class_id)?;
        let smtc: SystemMediaTransportControls = interop.GetForWindow(hwnd)?;

        smtc.SetIsEnabled(true)?;
        smtc.SetIsPlayEnabled(true)?;
        smtc.SetIsPauseEnabled(true)?;
        smtc.SetIsNextEnabled(true)?;
        smtc.SetIsPreviousEnabled(true)?;
        smtc.SetIsStopEnabled(true)?;

        let tx = get_tx();
        let seek_tx = tx.clone();

        smtc.ButtonPressed(&TypedEventHandler::new(
            move |_: Ref<SystemMediaTransportControls>,
                  args: Ref<SystemMediaTransportControlsButtonPressedEventArgs>|
                  -> windows::core::Result<()> {
                if let Some(args) = args.as_ref() {
                    let btn: SystemMediaTransportControlsButton = args.Button()?;
                    let evt = if btn == SystemMediaTransportControlsButton::Play {
                        Some(SystemEvent::Play)
                    } else if btn == SystemMediaTransportControlsButton::Pause {
                        Some(SystemEvent::Pause)
                    } else if btn == SystemMediaTransportControlsButton::Next {
                        Some(SystemEvent::Next)
                    } else if btn == SystemMediaTransportControlsButton::Previous {
                        Some(SystemEvent::Prev)
                    } else {
                        None
                    };
                    if let Some(e) = evt {
                        let _ = tx.send(e);
                    }
                }
                Ok(())
            },
        ))?;

        smtc.PlaybackPositionChangeRequested(&TypedEventHandler::new(
            move |_: Ref<SystemMediaTransportControls>,
                  args: Ref<PlaybackPositionChangeRequestedEventArgs>|
                  -> windows::core::Result<()> {
                if let Some(args) = args.as_ref() {
                    let pos = args.RequestedPlaybackPosition()?;
                    let secs = pos.Duration as f64 / 10_000_000.0;
                    let _ = seek_tx.send(SystemEvent::Seek(secs));
                }
                Ok(())
            },
        ))?;

        windows::core::Result::Ok(smtc)
    })();

    match result {
        Ok(smtc) => {
            if SMTC.set(SendableSmtc(smtc)).is_ok() {
                println!("[windows] SMTC initialised");
            }
        }
        Err(e) => eprintln!("[windows] SMTC setup failed: {e:?}"),
    }
}

pub fn init() {
    if SMTC.get().is_some() {
        return;
    }
    static INIT_ONCE: OnceLock<()> = OnceLock::new();
    INIT_ONCE.get_or_init(|| {
        std::thread::spawn(|| {
            // CoInitializeEx must be called on the thread that uses WinRT/COM.
            // The tokio thread pool does not do this, so setup_smtc must run here.
            unsafe {
                let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            }

            match find_main_hwnd() {
                Some(hwnd) => setup_smtc(hwnd),
                None => eprintln!("[windows] Could not find main HWND for SMTC"),
            }
        });
    });
}

// convert seconds to a Windows TimeSpan (unit is 100-nanosecond ticks)
#[inline]
fn secs_to_timespan(secs: f64) -> TimeSpan {
    TimeSpan {
        Duration: (secs * 10_000_000.0) as i64,
    }
}

// helper funcs: wrap raw bytes in an in-memory stream SMTC can read
// or fetch image bytes from either a local path or an url
fn stream_ref_from_bytes(bytes: &[u8]) -> Option<RandomAccessStreamReference> {
    let stream = InMemoryRandomAccessStream::new().ok()?;
    let writer = DataWriter::CreateDataWriter(&stream).ok()?;
    writer.WriteBytes(bytes).ok()?;
    tokio::runtime::Builder::new_current_thread()
        .build()
        .ok()?
        .block_on(async { writer.StoreAsync().ok()?.await.ok() })?;
    writer.DetachStream().ok()?;
    stream.Seek(0).ok()?; // rewind so SMTC reads from the start
    RandomAccessStreamReference::CreateFromStream(&stream).ok()
}

fn fetch_artwork_bytes(path: &str) -> Option<Vec<u8>> {
    if path.starts_with("http://") || path.starts_with("https://") {
        let resp = reqwest::blocking::get(path).ok()?;
        if resp.status().is_success() {
            resp.bytes().ok().map(|b| b.to_vec())
        } else {
            None
        }
    } else {
        std::fs::read(path).ok()
    }
}

pub fn update_now_playing(
    title: &str,
    artist: &str,
    album: &str,
    _duration: f64,
    _position: f64,
    playing: bool,
    artwork_path: Option<&str>,
) {
    // init in case init() wasn't called before the first track plays
    if SMTC.get().is_none() {
        init();
    }

    let Some(smtc) = SMTC.get() else { return };
    let smtc = &smtc.0;

    let _ = smtc.SetPlaybackStatus(if playing {
        MediaPlaybackStatus::Playing
    } else {
        MediaPlaybackStatus::Paused
    });

    if let Ok(updater) = smtc.DisplayUpdater() {
        let _ = updater.SetType(MediaPlaybackType::Music);
        if let Ok(props) = updater.MusicProperties() {
            let _ = props.SetTitle(&windows::core::HSTRING::from(title));
            let _ = props.SetArtist(&windows::core::HSTRING::from(artist));
            let _ = props.SetAlbumTitle(&windows::core::HSTRING::from(album));
        }

        if let Some(art) = artwork_path {
            if art.starts_with("http://") || art.starts_with("https://") {
                // Jellyfin: give the url directly to SMTC, it fetches lazily
                if let Ok(uri) = Uri::CreateUri(&windows::core::HSTRING::from(art)) {
                    if let Ok(stream_ref) = RandomAccessStreamReference::CreateFromUri(&uri) {
                        let _ = updater.SetThumbnail(&stream_ref);
                    }
                }
            } else {
                // Local: read bytes on a background thread, then apply thumbnail
                let art_owned = art.to_string();
                std::thread::spawn(move || {
                    if let Some(bytes) = fetch_artwork_bytes(&art_owned) {
                        if let Some(stream_ref) = stream_ref_from_bytes(&bytes) {
                            if let Some(smtc) = SMTC.get() {
                                if let Ok(updater) = smtc.0.DisplayUpdater() {
                                    let _ = updater.SetThumbnail(&stream_ref);
                                    let _ = updater.Update();
                                }
                            }
                        }
                    }
                });
            }
        }

        let _ = updater.Update();
    }

    let duration = _duration;
    let position = _position;
    if duration > 0.0 {
        if let Ok(timeline) = SystemMediaTransportControlsTimelineProperties::new() {
            let _ = timeline.SetStartTime(secs_to_timespan(0.0));
            let _ = timeline.SetEndTime(secs_to_timespan(duration));
            let _ = timeline.SetPosition(secs_to_timespan(position));
            let _ = timeline.SetMinSeekTime(secs_to_timespan(0.0));
            let _ = timeline.SetMaxSeekTime(secs_to_timespan(duration));
            let _ = smtc.UpdateTimelineProperties(&timeline);
        }
    }
}
