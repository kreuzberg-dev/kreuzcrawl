//! Image extraction from HTML documents.

use tl::VDom;
use url::Url;

use crate::types::{ImageInfo, ImageSource};

use super::get_attr;
use super::resolve_url;
use super::selectors::{SEL_IMG_SRC, SEL_OG_IMAGE, SEL_SOURCE_SRCSET, SEL_TWITTER_IMAGE};

/// Extract all images from a parsed HTML document.
pub(crate) fn extract_images(dom: &VDom<'_>, base_url: &Url) -> Vec<ImageInfo> {
    let parser = dom.parser();
    let mut images = Vec::new();

    if let Some(iter) = dom.query_selector(SEL_IMG_SRC) {
        for handle in iter {
            let Some(tag) = handle.get(parser).and_then(|n| n.as_tag()) else {
                continue;
            };
            let src = get_attr(tag, "src").unwrap_or("");
            if src.is_empty() || src.starts_with("data:") {
                continue;
            }
            let resolved = resolve_url(src, base_url);
            let alt = get_attr(tag, "alt").map(String::from);
            let width = get_attr(tag, "width").and_then(|w| w.parse::<u32>().ok());
            let height = get_attr(tag, "height").and_then(|h| h.parse::<u32>().ok());
            images.push(ImageInfo {
                url: resolved,
                alt,
                width,
                height,
                source: ImageSource::Img,
            });
        }
    }

    if let Some(iter) = dom.query_selector(SEL_SOURCE_SRCSET) {
        for handle in iter {
            let Some(tag) = handle.get(parser).and_then(|n| n.as_tag()) else {
                continue;
            };
            let srcset = get_attr(tag, "srcset").unwrap_or("");
            if !srcset.is_empty() {
                let first_url = srcset.split(',').next().unwrap_or("").trim();
                let raw_url = first_url.split_whitespace().next().unwrap_or("");
                if !raw_url.is_empty() {
                    let resolved = resolve_url(raw_url, base_url);
                    images.push(ImageInfo {
                        url: resolved,
                        alt: None,
                        width: None,
                        height: None,
                        source: ImageSource::PictureSource,
                    });
                }
            }
        }
    }

    if let Some(iter) = dom.query_selector(SEL_OG_IMAGE) {
        for handle in iter {
            let Some(tag) = handle.get(parser).and_then(|n| n.as_tag()) else {
                continue;
            };
            if let Some(content) = get_attr(tag, "content")
                && !content.is_empty()
            {
                let resolved = resolve_url(content, base_url);
                images.push(ImageInfo {
                    url: resolved,
                    alt: None,
                    width: None,
                    height: None,
                    source: ImageSource::OgImage,
                });
            }
        }
    }

    if let Some(iter) = dom.query_selector(SEL_TWITTER_IMAGE) {
        for handle in iter {
            let Some(tag) = handle.get(parser).and_then(|n| n.as_tag()) else {
                continue;
            };
            if let Some(content) = get_attr(tag, "content")
                && !content.is_empty()
            {
                let resolved = resolve_url(content, base_url);
                images.push(ImageInfo {
                    url: resolved,
                    alt: None,
                    width: None,
                    height: None,
                    source: ImageSource::TwitterImage,
                });
            }
        }
    }

    images
}
