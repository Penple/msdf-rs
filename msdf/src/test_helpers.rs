use image::RgbaImage;
use std::{env, iter};

pub(super) fn compare_images(image: &RgbaImage, path: &str) {
    // i'm hoping that floating point optimizations and whatnot do not affect the results enough that a bitwise comparison fails
    let path = env::current_dir()
        .unwrap()
        .join("test_resources")
        .join(path)
        .with_extension("png");
    let truth = image::open(path).expect("Unable to open test resource");
    let truth = truth.into_rgba8(); // make sure we're working in rgb8

    if image.len() != truth.len() {
        panic!("Image sizes do not match.");
    }

    for pixel in iter::zip(image.pixels(), truth.pixels()) {
        if pixel.0 != pixel.1 {
            panic!("Images do not match.");
        }
    }
}
