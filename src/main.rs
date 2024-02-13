//! This crate is a simple raytracer based on [this UWCS project](https://rs118.uwcs.co.uk/raytracer.html).

#![cfg_attr(debug_assertions, allow(dead_code))]

mod camera;
mod object;
mod ray;
mod vector;

use self::vector::v;
use camera::Camera;
use clap::Parser;
use color_eyre::{eyre::Context, Result};
use image::RgbImage;
use object::Sphere;
use rand::{distributions::Distribution, thread_rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vector::Colour;

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Args {
    /// The full width of the image.
    #[arg(long, short, default_value_t = 1920)]
    width: u32,

    /// The full height of the image.
    #[arg(long, short = 'H', default_value_t = 1080)]
    height: u32,

    /// How many samples should we use per pixel for the antialiasing?
    #[arg(long, short, default_value_t = 100)]
    samples: u16,

    /// The path to the output image file.
    #[arg(long, short, default_value = "./out.png")]
    output: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let camera = Camera::new(args.width, args.height);

    let mut img = RgbImage::new(args.width, args.height);
    let scene = vec![
        Sphere::new(v!(0, 0, -1), 0.5),
        Sphere::new(v!(0, -100.5, -1), 100.),
    ];

    let offset_distribution = rand::distributions::Uniform::new_inclusive(-0.5, 0.5);

    img.par_enumerate_pixels_mut().for_each(|(i, j, pixel)| {
        let colour_sum: Colour = (0..args.samples)
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                camera
                    .get_ray(
                        (i as f64 + offset_distribution.sample(&mut rng)) / args.width as f64,
                        (j as f64 + offset_distribution.sample(&mut rng)) / args.height as f64,
                    )
                    .colour(&scene)
            })
            .sum();
        let avg_colour = colour_sum / args.samples as f64;

        *pixel = avg_colour.into();
    });

    img.save(args.output)
        .wrap_err("When trying to save image buffer")?;

    Ok(())
}
