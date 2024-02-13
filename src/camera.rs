//! This module provides the [`Camera`] type.

use crate::{
    ray::Ray,
    vector::{v, Point, Vec3},
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

impl Camera {
    /// Create a new camera with the given width and height in pixels.
    pub fn new(width: u32, height: u32) -> Self {
        let viewport_height = v!(0, 2, 0);
        let viewport_width = v!(2. * width as f64 / height as f64, 0, 0);

        Self {
            position: v!(0),
            viewport_top_left: -viewport_width / 2. + viewport_height / 2. - v!(0, 0, 1),
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
