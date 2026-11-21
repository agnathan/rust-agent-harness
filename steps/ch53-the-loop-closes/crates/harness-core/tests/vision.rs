use std::collections::HashMap;

use harness_core::browser::public_resolver as browser_resolver;
use harness_core::vision::{
    analyze, prepare, sniff, tiny_png, FakeImages, VisionError, MAX_IMAGE_BYTES, MAX_PNG_PIXELS,
};

fn source_with(url: &str, bytes: Vec<u8>) -> FakeImages {
    let mut images = HashMap::new();
    images.insert(url.into(), bytes);
    let mut captions = HashMap::new();
    captions.insert(url.into(), "caption ".repeat(8));
    FakeImages::new(images, captions)
}

#[test]
fn a_png_header_is_png_and_a_bmp_is_not() {
    assert_eq!(sniff(&tiny_png()), Some("image/png"));
    assert_eq!(sniff(b"BM"), None);
    assert_eq!(sniff(b"<svg"), None);
    assert_eq!(prepare(&tiny_png()).unwrap(), "image/png");
    assert_eq!(prepare(b"BM....").unwrap_err(), VisionError::Unsupported);
}

#[test]
fn an_oversized_buffer_is_refused() {
    let mut bytes = tiny_png();
    bytes.resize(MAX_IMAGE_BYTES + 1, 0);
    assert!(matches!(
        prepare(&bytes),
        Err(VisionError::TooBig {
            cap: MAX_IMAGE_BYTES,
            ..
        })
    ));
}

#[test]
fn a_huge_png_is_over_the_pixel_cap() {
    let mut bytes = tiny_png();
    let side = ((MAX_PNG_PIXELS as f64).sqrt() as u32).saturating_add(2);
    bytes[16..20].copy_from_slice(&side.to_be_bytes());
    bytes[20..24].copy_from_slice(&side.to_be_bytes());
    assert!(matches!(prepare(&bytes), Err(VisionError::TooBig { .. })));
}

#[test]
fn a_metadata_image_url_is_not_loaded() {
    struct Spy;
    impl harness_core::vision::ImageSource for Spy {
        fn load(&self, _url: &str) -> Result<Vec<u8>, VisionError> {
            panic!("source must not run");
        }
        fn caption(&self, _url: &str) -> String {
            "no".into()
        }
    }
    let error = analyze(
        &Spy,
        "http://169.254.169.254/latest/meta-data",
        &browser_resolver(),
    )
    .unwrap_err();
    assert!(matches!(error, VisionError::UnsafeUrl(_)));
}

#[test]
fn a_canned_png_is_wrapped() {
    let url = "https://example.com/a.png";
    let source = source_with(url, tiny_png());
    let text = analyze(&source, url, &browser_resolver()).unwrap();
    assert!(text.contains("source=\"vision_analyze\""));
    assert!(text.contains("image/png"));
    assert!(text.contains("DATA") || text.contains("data"));
}

#[test]
fn the_demo_accepts_the_canned_png() {
    assert_eq!(harness_core::vision::demo(), "vision: png");
}
