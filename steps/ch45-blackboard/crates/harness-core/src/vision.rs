//! Vision behind an `ImageSource`. Bytes are sniffed. The caption is untrusted.
//! A remote URL uses the same secret and SSRF floors as web extract.

use std::collections::HashMap;
use std::net::IpAddr;

use crate::web::{
    is_safe_url, url_has_secret, wrap_untrusted, HostResolver, MapResolver, WebError,
};

/// Hermes `_MAX_BASE64_BYTES`. Inline images stop here.
pub const MAX_IMAGE_BYTES: usize = 20 * 1024 * 1024;
/// Hermes `_VISION_MAX_DOWNLOAD_BYTES`. A download larger than this is refused
/// before the inline cap is considered.
pub const MAX_DOWNLOAD_BYTES: usize = 50 * 1024 * 1024;
pub const MAX_PNG_PIXELS: u64 = 100_000_000;

const PNG: &[u8] = b"\x89PNG\r\n\x1a\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisionError {
    SecretUrl,
    UnsafeUrl(String),
    TooBig { bytes: usize, cap: usize },
    Unsupported,
    Empty,
    Backend(String),
}

impl std::fmt::Display for VisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SecretUrl => {
                f.write_str("Blocked: URL contains what appears to be an API key or token.")
            }
            Self::UnsafeUrl(text) => f.write_str(text),
            Self::TooBig { bytes, cap } => write!(f, "image is {bytes} bytes, cap is {cap}"),
            Self::Unsupported => f.write_str("image type is not jpeg, png, gif, or webp"),
            Self::Empty => f.write_str("image is empty"),
            Self::Backend(text) => f.write_str(text),
        }
    }
}

/// Magic-byte sniff. SVG, BMP, and unknown bytes are `None`.
pub fn sniff(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(PNG) {
        return Some("image/png");
    }
    if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        return Some("image/jpeg");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("image/gif");
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    None
}

/// PNG IHDR width and height, when the buffer is long enough. Overflow is `None`.
pub fn png_pixels(bytes: &[u8]) -> Option<u64> {
    if !bytes.starts_with(PNG) || bytes.len() < 24 {
        return None;
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let height = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    Some(u64::from(width) * u64::from(height))
}

pub fn prepare(bytes: &[u8]) -> Result<&'static str, VisionError> {
    if bytes.is_empty() {
        return Err(VisionError::Empty);
    }
    if bytes.len() > MAX_DOWNLOAD_BYTES {
        return Err(VisionError::TooBig {
            bytes: bytes.len(),
            cap: MAX_DOWNLOAD_BYTES,
        });
    }
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err(VisionError::TooBig {
            bytes: bytes.len(),
            cap: MAX_IMAGE_BYTES,
        });
    }
    let Some(mime) = sniff(bytes) else {
        return Err(VisionError::Unsupported);
    };
    if mime == "image/png" {
        if let Some(pixels) = png_pixels(bytes) {
            if pixels > MAX_PNG_PIXELS {
                return Err(VisionError::TooBig {
                    bytes: bytes.len(),
                    cap: MAX_IMAGE_BYTES,
                });
            }
        }
    }
    Ok(mime)
}

pub trait ImageSource: Send + Sync {
    fn load(&self, url: &str) -> Result<Vec<u8>, VisionError>;
    fn caption(&self, url: &str) -> String;
}

#[derive(Clone, Default)]
pub struct FakeImages {
    images: HashMap<String, Vec<u8>>,
    captions: HashMap<String, String>,
}

impl FakeImages {
    pub fn new(images: HashMap<String, Vec<u8>>, captions: HashMap<String, String>) -> Self {
        Self { images, captions }
    }

    pub fn example() -> Self {
        let mut images = HashMap::new();
        images.insert("https://example.com/a.png".into(), tiny_png());
        let mut captions = HashMap::new();
        captions.insert(
            "https://example.com/a.png".into(),
            "a small green square on a white field".into(),
        );
        Self { images, captions }
    }
}

impl ImageSource for FakeImages {
    fn load(&self, url: &str) -> Result<Vec<u8>, VisionError> {
        self.images
            .get(url)
            .cloned()
            .ok_or_else(|| VisionError::Backend(format!("no canned image for {url}")))
    }

    fn caption(&self, url: &str) -> String {
        self.captions
            .get(url)
            .cloned()
            .unwrap_or_else(|| "no caption".into())
    }
}

pub fn public_resolver() -> MapResolver {
    let mut hosts = HashMap::new();
    hosts.insert(
        "example.com".into(),
        Ok(vec!["93.184.216.34".parse::<IpAddr>().expect("ip")]),
    );
    MapResolver::new(hosts)
}

fn check_remote(url: &str, resolver: &dyn HostResolver) -> Result<(), VisionError> {
    if url.starts_with("data:") {
        return Ok(());
    }
    if url_has_secret(url) {
        return Err(VisionError::SecretUrl);
    }
    is_safe_url(url, false, resolver).map_err(|error| match error {
        WebError::Message(text) => VisionError::UnsafeUrl(text),
    })
}

/// Load, cap, sniff, then wrap the caption. The model sees data, not the page's instructions.
pub fn analyze(
    source: &dyn ImageSource,
    url: &str,
    resolver: &dyn HostResolver,
) -> Result<String, VisionError> {
    check_remote(url, resolver)?;
    let bytes = source.load(url)?;
    let mime = prepare(&bytes)?;
    let caption = source.caption(url);
    let body = format!("{mime}\n{caption}");
    Ok(wrap_untrusted("vision_analyze", &body))
}

/// A 1x1 PNG. IHDR sits at byte 16.
pub fn tiny_png() -> Vec<u8> {
    let mut bytes = PNG.to_vec();
    bytes.extend_from_slice(b"\x00\x00\x00\x0dIHDR");
    bytes.extend_from_slice(&1u32.to_be_bytes());
    bytes.extend_from_slice(&1u32.to_be_bytes());
    bytes.extend_from_slice(&[8, 2, 0, 0, 0]);
    bytes
}

pub fn demo() -> String {
    let source = FakeImages::example();
    let text = analyze(&source, "https://example.com/a.png", &public_resolver()).expect("png");
    if text.contains("image/png") {
        "vision: png".into()
    } else {
        "vision: rejected".into()
    }
}
