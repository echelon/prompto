// Library Copyright 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Prompto, Final Fantasy, and associated images and media are property of and
// copyrighted by SQUARE ENIX CO., LTD.

//! A functional example of how to use the library.

extern crate image;
extern crate prompto;

use prompto::selfie_from_file;
use image::ImageFormat;
use std::env;
use std::fs::File;

fn main() {
  let mut args = env::args();
  if args.len() < 2 {
    let program = args.nth(0).unwrap_or("./make_selfie".to_string());
    println!("Usage: {} input_filename [output_filename]", program);
    panic!();
  }

  let filename = args.nth(1).unwrap();
  let output_filename = args.nth(2).unwrap_or_else(|| "selfie.jpg".to_string());

  let selfie = match selfie_from_file(&filename) {
    Ok(selfie) => selfie,
    Err(e) => {
      println!("There was an error: {:?}", e);
      panic!();
    },
  };

  println!("Saving output to file: {}", output_filename);

  let ref mut buffer = File::create(&output_filename).unwrap();
  selfie.save(buffer, ImageFormat::JPEG).unwrap();
}
