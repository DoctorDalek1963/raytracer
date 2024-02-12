//! This crate is a simple raytracer based on [this UWCS project](https://rs118.uwcs.co.uk/raytracer.html).

mod vector;

use self::vector::Vec3;
use color_eyre::{eyre::Context, Result};
use image::RgbImage;

/// The width of the whole image.
const IMG_WIDTH: u32 = 512;

/// The height of the whole image.
const IMG_HEIGHT: u32 = 512;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut img = RgbImage::new(IMG_WIDTH, IMG_HEIGHT);
    for (i, j, pixel) in img.enumerate_pixels_mut() {
        let rgb = Vec3::new(
            i as f64 / IMG_WIDTH as f64,
            j as f64 / IMG_HEIGHT as f64,
            0.25,
        );
        *pixel = rgb.into();
    }

    img.save("./out.png")
        .wrap_err("When trying to save image buffer")?;

    Ok(())
}
