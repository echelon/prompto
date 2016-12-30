// Library Copyright 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Prompto, Final Fantasy, and associated images and media are property of and
// copyrighted by SQUARE ENIX CO., LTD.

//! Image processing functions.

use asset::prompto_for_box;
use error::PromptoError;
use image::DynamicImage;
use image::FilterType;
use image::GenericImage;
use image;
use std::cmp::max;
use std::path::Path;

/// Load an image file into memory and add a Prompto selfie.
pub fn selfie_from_file(filename: &str) -> Result<DynamicImage, PromptoError> {
  let source = image::open(&Path::new(filename))?
      .to_rgba();
  let mut source = DynamicImage::ImageRgba8(source);
  let _r = add_selfie_to_image(&mut source)?;
  Ok(source)
}

/// Add Prompto to the image.
pub fn selfie_from_image(image: &DynamicImage)
                         -> Result<DynamicImage, PromptoError> {
  let source = image.to_rgba();
  let mut source = DynamicImage::ImageRgba8(source);
  let _r = add_selfie_to_image(&mut source)?;
  Ok(source)
}

/// Add Prompto to the image buffer.
/// The image must be an RGBA image.
pub fn add_selfie_to_image(mut image: &mut DynamicImage)
    -> Result<(), PromptoError> {

  match *image {
    DynamicImage::ImageRgba8(_) => {},
    _ => return Err(PromptoError::NotRgba),
  }

  let size = get_bounding_size(&image);

  let prompto = prompto_for_box(image.width(), image.height());
  let prompto = prompto.resize(size, size, FilterType::Lanczos3);

  let x = image.width() - prompto.width();
  let y = image.height() - prompto.height();

  let _r = mask(&mut image, &prompto, x, y)?;

  Ok(())
}

/// Get the resize dimensions. The selfie mask is a square, so it returns the
/// length of one side.
pub fn get_bounding_size(source: &DynamicImage) -> u32 {
  let p_width = source.width() / 2;
  let p_height = source.height() / 2;

  let side = max(p_width, p_height);

  if side > source.width() {
    p_width
  } else if side > source.height() {
    p_height
  } else {
    side
  }
}

/// Mask the source image with the Prompto selfie mask.
fn mask(source: &mut DynamicImage, other: &DynamicImage, x: u32, y:u32)
    -> Result<(), PromptoError> {
  // Code adapted from `image` library.
  if source.width() < other.width() + x {
    return Err(PromptoError::MaskingError);
  } else if source.height() < other.height() + y {
    return Err(PromptoError::MaskingError);
  }

  for i in 0 .. other.width() {
    for k in 0 .. other.height() {
      unsafe {
        let p = other.unsafe_get_pixel(i, k);
        source.blend_pixel(i + x, k + y, p);
      }
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  extern crate tempfile;

  use super::*;
  use image::DynamicImage;
  use image::GenericImage;
  use image::ImageBuffer;
  use image::ImageFormat;
  use image::Rgb;
  use image::RgbImage;
  use self::tempfile::NamedTempFile;
  use self::tempfile::NamedTempFileOptions;

  // TODO: Actually assert pixel values of output images.

  #[test]
  fn test_selfie_from_jpeg_file() {
    let file = create_image_file(800, 600, ImageFormat::JPEG, ".jpg");
    let path = file.path().to_str().unwrap();

    let selfie = selfie_from_file(path).unwrap();

    assert_eq!(800, selfie.width());
    assert_eq!(600, selfie.height());
  }

  #[test]
  fn test_selfie_from_png_file() {
    // Test another image format
    let file = create_image_file(250, 250, ImageFormat::PNG, ".png");
    let path = file.path().to_str().unwrap();

    let selfie = selfie_from_file(path).unwrap();

    assert_eq!(250, selfie.width());
    assert_eq!(250, selfie.height());
  }

  #[test]
  fn test_selfie_from_wide_image() {
    let image = create_image(200, 400);
    let selfie = selfie_from_image(&image).unwrap();

    assert_eq!(200, selfie.width());
    assert_eq!(400, selfie.height());
  }

  #[test]
  fn test_selfie_from_tall_image() {
    // Make sure other dimensions work
    let image = create_image(600, 100);
    let selfie = selfie_from_image(&image).unwrap();

    assert_eq!(600, selfie.width());
    assert_eq!(100, selfie.height());
  }

  #[test]
  fn test_add_selfie_to_image() {
    // NB: Only tests that the function doesn't error.
    let mut image = DynamicImage::ImageRgba8(ImageBuffer::new(250, 250));
    let _selfie = add_selfie_to_image(&mut image).unwrap();
  }

  #[test]
  fn test_bounding_size() {
    // Square
    let image = create_image(1000, 1000);
    let side = get_bounding_size(&image);
    assert_eq!(500, side);

    // Wide
    let image = create_image(1000, 600);
    let side = get_bounding_size(&image);
    assert_eq!(500, side);

    // Really Wide
    let image = create_image(1000, 400);
    let side = get_bounding_size(&image);
    assert_eq!(200, side);

    // Tall
    let image = create_image(800, 1200);
    let side = get_bounding_size(&image);
    assert_eq!(600, side);

    // Really Tall
    let image = create_image(200, 1200);
    let side = get_bounding_size(&image);
    assert_eq!(100, side);
  }

  // Create an in-memory image for use in tests.
  fn create_image(width: u32, height: u32) -> DynamicImage {
    let img: RgbImage = ImageBuffer::from_fn(width, height, |x, _y| {
      if x % 2 == 0 {
        Rgb([0u8, 0u8, 0u8])
      } else {
        Rgb([255u8, 255u8, 255u8])
      }
    });

    DynamicImage::ImageRgb8(img)
  }

  // Create a temporary image file for use in tests.
  fn create_image_file(width: u32,
                       height: u32,
                       format: ImageFormat,
                       suffix: &str) -> NamedTempFile {
    let image = create_image(width, height);

    let mut file = NamedTempFileOptions::new()
        .prefix("selfie_")
        .suffix(suffix) // Sadly used by the image library to determine the type
        .create()
        .unwrap();

    let _ = image.save(&mut file, format);
    file
  }
}
