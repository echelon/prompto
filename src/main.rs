extern crate image;
#[macro_use] extern crate lazy_static;

use std::path::Path;
use image::GenericImage;
use std::env;
use image::ImageResult;
use image::ImageFormat;
use image::DynamicImage;
use image::load_from_memory;
use image::load_from_memory_with_format;

lazy_static! {
  // NB: Image is included with the library.
  // Loading and decoding is only done once.
  static ref IMAGE: DynamicImage = load_from_memory_with_format(
      include_bytes!("img/prompto_selfie.png"),
      ImageFormat::PNG)
    .ok()
    .unwrap();
}

fn main() {
  let mut args = env::args();
  if args.len() < 2 {
    println!("Must supply filename.");
    panic!();
  }

  let filename = args.nth(1).unwrap();

  let source = open(&filename).unwrap();


  println!("Source Width: {}, Height: {}", source.width(), source.height());
  println!("Prompto Width: {}, Height: {}", IMAGE.width(), IMAGE.height());

  /*println!("Opening!");
  //let img = image::open(&Path::new("img/prompto_selfie.png")).unwrap();
  let img = IMAGE.clone();
  println!("Opened.");

  println!("Opening again!");
  let img = IMAGE.clone();
  println!("Opened.");

  println!("Opening again!");
  let img = IMAGE.clone();
  println!("Opened.");*/
}

fn open(filename: &str) -> ImageResult<DynamicImage> {
  image::open(&Path::new(filename))
}


fn load() -> ImageResult<DynamicImage> {
  let bytes = include_bytes!("img/prompto_selfie.png");
  load_from_memory_with_format(bytes, ImageFormat::PNG)
}