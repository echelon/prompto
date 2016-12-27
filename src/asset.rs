use error::PromptoError;
use image::DynamicImage;
use image::ImageFormat;
use image::load_from_memory_with_format;

lazy_static! {
  // NB: Image bytes are statically compiled into the library.
  // Decoding cost is payed only once at runtime upon first use.
  // TODO: Precalculated sizes based on ratio of image
  pub static ref PROMPTO_IMAGE_200: DynamicImage = load_from_memory_with_format(
      include_bytes!("img/prompto_selfie_200x200.png"),
      ImageFormat::PNG)
    .ok()
    .unwrap();

  static ref PROMPTO_IMAGE_400: DynamicImage = load_from_memory_with_format(
      include_bytes!("img/prompto_selfie_400x400.png"),
      ImageFormat::PNG)
    .ok()
    .unwrap();

  static ref PROMPTO_IMAGE_600: DynamicImage = load_from_memory_with_format(
      include_bytes!("img/prompto_selfie_600x600.png"),
      ImageFormat::PNG)
    .ok()
    .unwrap();

  static ref PROMPTO_IMAGE_800: DynamicImage = load_from_memory_with_format(
      include_bytes!("img/prompto_selfie_800x800.png"),
      ImageFormat::PNG)
    .ok()
    .unwrap();

  static ref PROMPTO_IMAGE_1000: DynamicImage = load_from_memory_with_format(
      include_bytes!("img/prompto_selfie_1000x1000.png"),
      ImageFormat::PNG)
    .ok()
    .unwrap();

  static ref PROMPTO_IMAGE_1200: DynamicImage = load_from_memory_with_format(
      include_bytes!("img/prompto_selfie_1200x1200.png"),
      ImageFormat::PNG)
    .ok()
    .unwrap();
}

pub fn prompto_for_box(width: u32, height: u32) -> &'static DynamicImage {
  if width < 400 || height < 400 {
    &PROMPTO_IMAGE_200
  } else if width < 600|| height < 600 {
    &PROMPTO_IMAGE_400
  } else if width < 800 || height < 800 {
    &PROMPTO_IMAGE_600
  } else if width < 1000 || height < 1000 {
    &PROMPTO_IMAGE_800
  } else if width < 1200 || height < 1200 {
    &PROMPTO_IMAGE_1000
  } else {
    &PROMPTO_IMAGE_1200
  }
}
