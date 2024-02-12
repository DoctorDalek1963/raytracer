//! This crate is a simple raytracer based on [this UWCS project](https://rs118.uwcs.co.uk/raytracer.html).

#![cfg_attr(debug_assertions, allow(dead_code))]

mod object;
mod ray;
mod vector;

use self::{ray::Ray, vector::v};
use color_eyre::{eyre::Context, Result};
use image::RgbImage;
use rayon::iter::ParallelIterator;

/// The width of the whole image.
const IMG_WIDTH: u32 = 1920;

/// The height of the whole image.
const IMG_HEIGHT: u32 = IMG_WIDTH * 9 / 16;

fn main() -> Result<()> {
    color_eyre::install()?;

    let vec_height = v!(0, 2, 0);
    let vec_width = v!(32. / 9., 0, 0);
    let vec_focal_length = v!(0, 0, 1);

    let viewport_top_left = -vec_width / 2. + vec_height / 2. - vec_focal_length;

    let mut img = RgbImage::new(IMG_WIDTH, IMG_HEIGHT);
    img.par_enumerate_pixels_mut().for_each(|(i, j, pixel)| {
        let x_prop = i as f64 / IMG_WIDTH as f64;
        debug_assert!(
            (0.0..=1.0).contains(&x_prop),
            "The x proportion must be in [0, 1]: {x_prop}"
        );

        let y_prop = j as f64 / IMG_HEIGHT as f64;
        debug_assert!(
            (0.0..=1.0).contains(&y_prop),
            "The y proportion must be in [0, 1]: {y_prop}"
        );

        let pixel_pos_vec = viewport_top_left + x_prop * vec_width - y_prop * vec_height;

        let ray = Ray::new(v!(0), pixel_pos_vec);
        *pixel = ray.colour().into();
    });

    img.save("./out.png")
        .wrap_err("When trying to save image buffer")?;

    Ok(())
}
