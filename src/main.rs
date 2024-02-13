//! This crate is a simple raytracer based on [this UWCS project](https://rs118.uwcs.co.uk/raytracer.html).

#![cfg_attr(debug_assertions, allow(dead_code))]

mod object;
mod ray;
mod vector;

use self::{ray::Ray, vector::v};
use clap::Parser;
use color_eyre::{eyre::Context, Result};
use image::RgbImage;
use object::Sphere;
use rayon::iter::ParallelIterator;

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Args {
    /// The full width of the image.
    #[arg(long, short, default_value_t = 1920)]
    width: u32,

    /// The full height of the image.
    #[arg(long, short = 'H', default_value_t = 1080)]
    height: u32,

    /// The path to the output image file.
    #[arg(long, short, default_value = "./out.png")]
    output: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let vec_height = v!(0, 2, 0);
    let vec_width = v!(2. * args.width as f64 / args.height as f64, 0, 0);
    let vec_focal_length = v!(0, 0, 1);

    let viewport_top_left = -vec_width / 2. + vec_height / 2. - vec_focal_length;

    let mut img = RgbImage::new(args.width, args.height);
    let scene = vec![
        Sphere::new(v!(0, 0, -1), 0.5),
        Sphere::new(v!(0, -100.5, -1), 100.),
    ];

    img.par_enumerate_pixels_mut().for_each(|(i, j, pixel)| {
        let x_prop = i as f64 / args.width as f64;
        debug_assert!(
            (0.0..=1.0).contains(&x_prop),
            "The x proportion must be in [0, 1]: {x_prop}"
        );

        let y_prop = j as f64 / args.height as f64;
        debug_assert!(
            (0.0..=1.0).contains(&y_prop),
            "The y proportion must be in [0, 1]: {y_prop}"
        );

        let pixel_pos_vec = viewport_top_left + x_prop * vec_width - y_prop * vec_height;

        let ray = Ray::new(v!(0), pixel_pos_vec);
        *pixel = ray.colour(&scene).into();
    });

    img.save(args.output)
        .wrap_err("When trying to save image buffer")?;

    Ok(())
}
