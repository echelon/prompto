//! Library errors.

use image::ImageError;
use std::io;

/// Library errors.
pub enum PromptoError {
  /// Failure to load an image.
  IoError { cause: io::Error },
  /// Uncategorized error
  MiscError,
}

impl From<ImageError> for PromptoError {
  fn from(error: ImageError) -> Self {
    match error {
      ImageError::IoError(cause) => PromptoError::IoError { cause: cause },
      _ => PromptoError::MiscError,
    }
  }
}
