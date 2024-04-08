//! This crate is a simple raytracer based on [this UWCS project](https://rs118.uwcs.co.uk/raytracer.html).

#![cfg_attr(debug_assertions, allow(dead_code))]

mod camera;
mod material;
mod object;
mod ray;
mod vector;

use self::{
    camera::{Camera, CameraOpts},
    object::random_scene,
    vector::{v, Colour},
};
use clap::Parser;
use color_eyre::{eyre::Context, Result};
use image::RgbImage;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressDrawTarget, ProgressStyle};
use rand::{distributions::Distribution, thread_rng};
use rayon::iter::ParallelIterator;
use std::time::Instant;

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

    /// How many times should each ray bounce?
    #[arg(long, short, default_value_t = 50)]
    bounces: u16,

    /// The path to the output image file.
    #[arg(long, short, default_value = "./out.png")]
    output: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let look_from = v!(13, 2, 3);
    let look_at = v!(0, 0, 0);

    let camera = Camera::from(CameraOpts {
        width: args.width,
        height: args.height,
        vertical_fov_degrees: 20.,
        look_from,
        look_at,
        view_up: v!(0, 1, 0),
        aperture_width: 0.1,
        focus_distance: 10.,
    });

    let mut img = RgbImage::new(args.width, args.height);
    let scene = random_scene();

    let offset_distribution = rand::distributions::Uniform::new_inclusive(-0.5, 0.5);

    let progress_bar = ProgressBar::with_draw_target(
        Some(args.width as u64 * args.height as u64),
        ProgressDrawTarget::stdout_with_hz(10),
    )
    .with_style(
        ProgressStyle::with_template(
            "[{bar}] {percent}% - {elapsed_precise} / {duration_precise} {msg}",
        )
        .expect("We should be able to create the progress bar")
        .progress_chars("=> "),
    );

    println!("Rendering scene...");
    let start_time = Instant::now();

    img.par_enumerate_pixels_mut()
        .progress_with(progress_bar)
        .for_each(|(i, j, pixel)| {
            let colour_sum: Colour = (0..args.samples)
                .map(|_| {
                    let mut rng = thread_rng();
                    camera
                        .get_ray(
                            (i as f64 + offset_distribution.sample(&mut rng)) / args.width as f64,
                            (j as f64 + offset_distribution.sample(&mut rng)) / args.height as f64,
                        )
                        .colour(&scene, args.bounces)
                })
                .sum();
            let avg_colour = colour_sum / args.samples as f64;

            *pixel = avg_colour.into();
        });

    let time_taken = start_time.elapsed();
    println!("Rendering took {time_taken:?}");
    println!("Rendered to {}", args.output);

    img.save(args.output)
        .wrap_err("When trying to save image buffer")?;

    Ok(())
}
