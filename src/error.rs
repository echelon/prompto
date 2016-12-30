// Library Copyright 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Prompto, Final Fantasy, and associated images and media are property of and
// copyrighted by SQUARE ENIX CO., LTD.

//! Library errors.

use image::ImageError;
use std::io;

/// Library errors.
#[derive(Debug)]
pub enum PromptoError {
  /// Failure to process an image.
  ImageError {
    /// Source of the error.
    cause: ImageError,
  },
  /// Failure to load an image.
  IoError {
    /// Source of the error.
    cause: io::Error,
  },
  /// Problem encountered when adding the image mask.
  MaskingError,
  /// Image is not an RGB image with an alpha channel.
  NotRgba,
}

impl From<ImageError> for PromptoError {
  fn from(error: ImageError) -> Self {
    match error {
      ImageError::IoError(cause) => PromptoError::IoError { cause: cause },
      _ => PromptoError::ImageError { cause: error },
    }
  }
}
