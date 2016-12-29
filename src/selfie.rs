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

// TODO: Test
/// Add Prompto to the image.
pub fn selfie_from_image(image: &DynamicImage)
    -> Result<DynamicImage, PromptoError> {
  let source = image.to_rgba();
  let mut source = DynamicImage::ImageRgba8(source);
  let _r = add_selfie_to_image(&mut source)?;
  Ok(source)
}

// TODO: Test
/// Load an image file into memory and add a Prompto selfie.
pub fn selfie_from_file(filename: &str) -> Result<DynamicImage, PromptoError> {
  let source = image::open(&Path::new(filename))?
      .to_rgba();
  let mut source = DynamicImage::ImageRgba8(source);
  let _r = add_selfie_to_image(&mut source)?;
  Ok(source)
}

// TODO: Test
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

// TODO: Test permutations.
/// Get the resize dimensions. The selfie mask is a square, so it returns the
/// length of one side.
fn get_bounding_size(source: &DynamicImage) -> u32 {
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
