//! This module handles rays.

use crate::{
    object::Object,
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
    pub fn colour(&self, object: &impl Object) -> Colour {
        if let Some(hit) = object.hit(self, (0., f64::INFINITY)) {
            hit.surface_normal.normal_to_colour()
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
