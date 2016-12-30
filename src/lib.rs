// Library Copyright 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Prompto, Final Fantasy, and associated images and media are property of and
// copyrighted by SQUARE ENIX CO., LTD.

//! A frivolous joke of a library that turns your ordinary images into selfies
//! featuring Final Fantasy XV's Prompto.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

#[macro_use] extern crate lazy_static;
extern crate image;

mod asset;
mod error;
mod selfie;

pub use error::PromptoError;
pub use selfie::add_selfie_to_image;
pub use selfie::selfie_from_file;
pub use selfie::selfie_from_image;
