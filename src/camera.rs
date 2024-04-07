//! This module provides the [`Camera`] type.

use crate::{
    ray::Ray,
    vector::{Point, Vec3},
};

/// A simple camera.
pub struct Camera {
    /// The position of the camera itself. This is where rays will originate from.
    position: Point,

    /// The position of the top left corner of the viewport.
    viewport_top_left: Point,

    /// The height of the viewport.
    viewport_height: Vec3,

    /// The width of the viewport.
    viewport_width: Vec3,
}

/// The options needed to construct a camera.
pub struct CameraOpts {
    /// The full width of the final image.
    pub width: u32,

    /// The full height of the final image.
    pub height: u32,

    /// The vertical field of view, in degrees.
    pub vertical_fov_degrees: f64,

    /// The point where the camera is looking from.
    pub look_from: Point,

    /// The point that the camera is looking at.
    pub look_at: Point,

    /// A vector that decides which way is up for the camera.
    pub view_up: Vec3,
}

impl From<CameraOpts> for Camera {
    fn from(value: CameraOpts) -> Self {
        Self::from_camera_opts(value)
    }
}

impl Camera {
    /// Create a new camera from the given options.
    pub fn from_camera_opts(
        CameraOpts {
            width,
            height,
            vertical_fov_degrees,
            look_from,
            look_at,
            view_up,
        }: CameraOpts,
    ) -> Self {
        let w = (look_from - look_at).normalise();
        let u = view_up.cross(w).normalise();
        let v = w.cross(u).normalise();

        let h = f64::tan(vertical_fov_degrees.to_radians() / 2.);
        let two_h = 2. * h;
        let aspect_ratio = width as f64 / height as f64;

        let viewport_height = v * two_h;
        let viewport_width = u * two_h * aspect_ratio;

        Self {
            position: look_from,
            viewport_top_left: -viewport_width / 2. + viewport_height / 2. - w,
            viewport_height,
            viewport_width,
        }
    }

    /// Return the ray from this camera going through the given pixel.
    ///
    /// The position of the pixel in each direction is given as a proportion of the total viewport
    /// size in that direction. The given proportions should be in the range `[0, 1]` and will be
    /// clamped down to that if they exceed it.
    pub fn get_ray(&self, x_prop: f64, y_prop: f64) -> Ray {
        let pixel_pos_vec = self.viewport_top_left + x_prop.clamp(0., 1.) * self.viewport_width
            - y_prop.clamp(0., 1.) * self.viewport_height;

        Ray::new(self.position, pixel_pos_vec)
    }
}
