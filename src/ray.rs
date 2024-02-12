//! This module handles rays.

use crate::{
    object::Sphere,
    vector::{v, Colour, Point, Vec3},
};

/// A ray, starting at an origin and pointing in a direction.
#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    /// The point where the ray starts.
    pub origin: Point,

    /// The direction of the ray.
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray with the given origin and direction.
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Get the point along this ray with parameter `t`.
    #[inline]
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    /// Trace this ray and determine its colour.
    pub fn colour(&self) -> Colour {
        let sphere_centre = v!(0, 0, -1);
        if let Some(t) = Sphere::new(sphere_centre, 0.5).hit(self) {
            let intersection = self.at(t);
            let surface_normal = (intersection - sphere_centre).normalise();
            surface_normal.map(|x| (x + 1.) / 2.)
        } else {
            let height = ((self.direction / -self.direction.z).y + 1.) / 2.;
            debug_assert!(
                (0.0..=1.0).contains(&height),
                "The height must be in [0, 1]: {height}"
            );
            (1. - height) * v!(1) + height * v!(0.5, 0.7, 1)
        }
    }
}
