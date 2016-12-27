extern crate image;
#[macro_use] extern crate lazy_static;

mod error;
mod asset;

use std::cmp::max;
use std::fs::File;
use std::io::Write;
use error::PromptoError;
use image::DynamicImage;
use image::FilterType;
use image::GenericImage;
use image::ImageFormat;
use image::load_from_memory_with_format;
use std::env;
use std::path::Path;
use asset::prompto_for_box;

struct Dimensions {
  pub width: u32,
  pub height: u32,
}

fn main() {
  let mut args = env::args();
  if args.len() < 2 {
    println!("Must supply filename.");
    panic!();
  }

  let filename = args.nth(1).unwrap();
  println!("Filename: {}", filename);

  let mut source = open(&filename).ok().unwrap();
  println!("Source Width: {}, Height: {}", source.width(), source.height());

  let prompto = prompto_for_box(source.width(), source.height());
  println!("Prompto Width: {}, Height: {}", prompto.width(), prompto.height());

  let dimensions = get_bounding_size(&source);
  println!("Resized Prompto Width: {}, Height: {}", dimensions.width, dimensions.height);

  let prompto = prompto.resize(dimensions.width, dimensions.height, FilterType::Lanczos3);

  println!("Adding prompto");
  //source.copy_from(&prompto, 0, 0);

  let x = source.width() - prompto.width();
  let y = source.height() - prompto.height();

  mask(&mut source, &prompto, x, y);

  println!("Writing image");
  let ref mut buffer = File::create("out.png").unwrap();
  //let ref mut buffer = File::create(&Path::new("out.jpg")).unwrap();

  source.save(buffer, ImageFormat::PNG).unwrap();

  // Source Image
  // Source Image => bounding size ~ width / 2

  /*println!("Opening!");
  //let img = image::open(&Path::new("img/prompto_selfie.png")).unwrap();
  let img = PROMPTO_IMAGE.clone();
  println!("Opened.");

  println!("Opening again!");
  let img = PROMPTO_IMAGE.clone();
  println!("Opened.");

  println!("Opening again!");
  let img = PROMPTO_IMAGE.clone();
  println!("Opened.");*/
}

fn open(filename: &str) -> Result<DynamicImage, PromptoError> {
  let img = image::open(&Path::new(filename))?
      .to_rgba();
  Ok(DynamicImage::ImageRgba8(img))
}

/*fn load() -> Result<DynamicImage, PromptoError> {
  let bytes = include_bytes!("img/prompto_selfie.png");
  let img = load_from_memory_with_format(bytes, ImageFormat::PNG)?;
  Ok(img)
}*/

fn get_dimensions(image: &DynamicImage) -> Dimensions {
  Dimensions {
    width: image.width(),
    height: image.height(),
  }
}

/// Get the resize dimensions.
fn get_bounding_size(source: &DynamicImage) -> Dimensions {
  let p_width = source.width() / 2;
  let p_height = source.height() / 2;
  let side = max(p_width, p_height);
  // TODO REFACTOR OUT DIMENSION
  let side = if side > source.width() {
    p_width
  } else if side > source.height() {
    p_height
  } else {
    side
  };

  Dimensions {
    width: side,
    height: side,
  }
}


fn mask(source: &mut DynamicImage, other: &DynamicImage, x: u32, y:u32) -> bool {
  if source.width() < other.width() + x {
    return false;
  } else if source.height() < other.height() + y {
    return false;
  }

  for i in 0 .. other.width() {
    for k in 0 .. other.height() {
      unsafe {
        let p = other.unsafe_get_pixel(i, k);
        //source.unsafe_put_pixel(i + x, k + y, p);
        source.blend_pixel(i + x, k + y, p);
      }
    }
  }
  true
}

