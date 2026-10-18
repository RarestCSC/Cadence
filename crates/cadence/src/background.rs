use std::{io::Cursor, path::Path, sync::Arc};

use anyhow::{Context as _, Result};
use gpui::{Image, ImageFormat};
use image::{ImageReader, imageops::FilterType};

const MAX_DIMENSION: u32 = 2560;

pub(crate) fn load(path: &Path, blur: u32) -> Result<Arc<Image>> {
    let image = ImageReader::open(path)
        .context("无法打开背景图片")?
        .with_guessed_format()
        .context("无法识别背景图片格式")?
        .decode()
        .context("无法解码背景图片")?;
    let image = if image.width() > MAX_DIMENSION || image.height() > MAX_DIMENSION {
        image.resize(MAX_DIMENSION, MAX_DIMENSION, FilterType::Triangle)
    } else {
        image
    };
    let blurred = if blur > 0 {
        image.blur(blur as f32)
    } else {
        image
    };
    let mut bytes = Cursor::new(Vec::new());
    blurred
        .write_to(&mut bytes, image::ImageFormat::Png)
        .context("无法编码背景图片")?;
    Ok(Arc::new(Image::from_bytes(
        ImageFormat::Png,
        bytes.into_inner(),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("cadence-background-test-{name}"))
    }

    #[test]
    fn missing_background_is_reported() {
        assert!(load(&temp_path("missing.png"), 28).is_err());
    }

    #[test]
    fn loads_small_background_images() {
        let path = temp_path("small.png");
        let image = image::RgbaImage::from_pixel(8, 8, image::Rgba([32, 96, 200, 255]));
        image.save(&path).expect("write fixture");
        let loaded = load(&path, 8).expect("load background");
        let _ = std::fs::remove_file(&path);
        assert!(Arc::strong_count(&loaded) == 1);
    }
}
