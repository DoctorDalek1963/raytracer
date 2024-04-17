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
use image::{DynamicImage, Rgb32FImage, RgbImage};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressIterator, ProgressStyle};
use rand::{distributions::Distribution, thread_rng};
use rayon::iter::ParallelIterator;
use std::{
    num::NonZeroU32,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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

    let float_img = &mut Rgb32FImage::new(args.width, args.height) as *mut _;
    let scene = random_scene();

    let offset_distribution = rand::distributions::Uniform::new_inclusive(-0.5, 0.5);

    let progress_bar = ProgressBar::with_draw_target(
        Some(args.samples as u64),
        ProgressDrawTarget::stdout_with_hz(10),
    )
    .with_style(
        ProgressStyle::with_template(
            "[{bar}] {percent}% - {elapsed_precise} / {duration_precise} {msg}",
        )
        .expect("We should be able to create the progress bar")
        .progress_chars("=> "),
    );

    let preview_scale_factor = if args.width > args.height {
        args.width as f64 / 1280.
    } else {
        args.height as f64 / 720.
    };

    let preview_width = args.width as f64 / preview_scale_factor;
    let preview_height = args.height as f64 / preview_scale_factor;

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_inner_size(LogicalSize::new(preview_width, preview_height))
            .with_resizable(false)
            .with_title("Raytracer")
            .build(&event_loop)?,
    );
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    println!("Rendering scene...");
    let start_time = Instant::now();

    // Thread to actually do the raytracing
    thread::spawn({
        let float_img = unsafe { &mut *float_img } as &mut Rgb32FImage;
        move || {
            for sample_idx in (0..args.samples).progress_with(progress_bar) {
                float_img
                    .par_enumerate_pixels_mut()
                    .for_each(|(i, j, pixel)| {
                        let mut rng = thread_rng();
                        let sampled_colour = camera
                            .get_ray(
                                (i as f64 + offset_distribution.sample(&mut rng))
                                    / args.width as f64,
                                (j as f64 + offset_distribution.sample(&mut rng))
                                    / args.height as f64,
                            )
                            .colour(&scene, args.bounces);
                        let current_colour = Colour::from(*pixel);

                        let avg_colour = (current_colour * sample_idx as f64 + sampled_colour)
                            / (sample_idx + 1) as f64;

                        *pixel = avg_colour.into();
                    });
            }

            let time_taken = start_time.elapsed();
            println!("Rendering took {time_taken:?}");

            RgbImage::from(DynamicImage::from(float_img.to_owned()))
                .save(&args.output)
                .wrap_err("When trying to save image buffer")
                .unwrap();
            println!("Rendered to {}", args.output);
        }
    });

    // Redraw the window at 30 fps
    thread::spawn({
        let window = window.clone();
        move || loop {
            thread::sleep(Duration::from_millis(100));
            window.request_redraw();
        }
    });

    event_loop.run(move |event, event_loop_window_target| {
        event_loop_window_target.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                let float_img = unsafe { &*float_img as &Rgb32FImage };

                let height = preview_height.floor() as u32;
                let width = preview_width.floor() as u32;

                for y in 0..height {
                    for x in 0..width {
                        let index = y * width + x;
                        // TODO: Average out pixels in area?
                        if let Some(pixel) = float_img.get_pixel_checked(
                            (x as f64 * preview_scale_factor) as u32,
                            (y as f64 * preview_scale_factor) as u32,
                        ) {
                            let [r, g, b] = pixel.0;

                            let r = (r.clamp(0., 1.) * 255.).floor() as u32;
                            let g = (g.clamp(0., 1.) * 255.).floor() as u32;
                            let b = (b.clamp(0., 1.) * 255.).floor() as u32;

                            buffer[index as usize] = (r << 16) | (g << 8) | b;
                        } else {
                            buffer[index as usize] = 0;
                        }
                    }
                }

                buffer.present().unwrap();
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } if window_id == window.id() => event_loop_window_target.exit(),
            _ => {}
        };
    })?;

    Ok(())
}
