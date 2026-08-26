//! The one cover-resize policy, shared by the app's `artwork://` handler and
//! the daemon's `GetArtwork`. Both serve the same covers to the same UI, so
//! the dimensions and JPEG quality live here rather than in each.

/// Thumbnail edge, for grids and the now-playing strip.
pub const THUMB_MAX: u32 = 400;
/// Full-view edge, for the fullscreen player background.
pub const HQ_MAX: u32 = 1920;

pub const THUMB_QUALITY: u8 = 75;
pub const HQ_QUALITY: u8 = 85;

/// Below this an HQ original is already small enough to serve untouched.
pub const HQ_REENCODE_THRESHOLD: usize = 2 * 1024 * 1024;

/// Decode bounds. A cover is attacker-supplied once a remote source is in
/// play, and a header claiming huge dimensions otherwise buys an allocation
/// far past anything a real cover needs. Both sit well above the largest
/// artwork any source serves.
const MAX_INPUT_DIMENSION: u32 = 8192;
const MAX_DECODE_ALLOC: u64 = 192 * 1024 * 1024;
const _: () = assert!(MAX_INPUT_DIMENSION >= 6000);
const _: () = assert!(MAX_DECODE_ALLOC >= 6000 * 6000 * 4);

/// Re-encode `raw` as JPEG, downscaling only when it exceeds `max_dimension`.
/// `None` when the bytes do not decode as an image within the bounds above,
/// which callers treat as "serve the original".
pub fn shrink_jpeg(raw: &[u8], max_dimension: u32, quality: u8) -> Option<Vec<u8>> {
    use image::codecs::jpeg::JpegEncoder;
    let mut reader = image::ImageReader::new(std::io::Cursor::new(raw))
        .with_guessed_format()
        .ok()?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_INPUT_DIMENSION);
    limits.max_image_height = Some(MAX_INPUT_DIMENSION);
    limits.max_alloc = Some(MAX_DECODE_ALLOC);
    reader.limits(limits);
    let img = reader.decode().ok()?;
    let img = if img.width() > max_dimension || img.height() > max_dimension {
        img.thumbnail(max_dimension, max_dimension)
    } else {
        img
    };
    let mut out = Vec::new();
    img.write_with_encoder(JpegEncoder::new_with_quality(&mut out, quality))
        .ok()?;
    Some(out)
}

/// The thumbnail policy: 400 px at q75.
pub fn thumbnail(raw: &[u8]) -> Option<Vec<u8>> {
    shrink_jpeg(raw, THUMB_MAX, THUMB_QUALITY)
}

/// The full-view policy: 1920 px at q85, and only worth doing above
/// [`HQ_REENCODE_THRESHOLD`].
pub fn hq_image(raw: &[u8]) -> Option<Vec<u8>> {
    if raw.len() <= HQ_REENCODE_THRESHOLD {
        return None;
    }
    shrink_jpeg(raw, HQ_MAX, HQ_QUALITY)
}
