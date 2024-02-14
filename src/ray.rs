//! This module handles rays.

use crate::{
    material::Reflection,
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
        Self {
            origin,
            direction: direction.normalise(),
        }
    }

    /// Get the point along this ray with parameter `t`.
    #[inline]
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    /// Trace this ray and determine its colour.
    pub fn colour(&self, object: &impl Object, bounces: u16) -> Colour {
        if bounces == 0 {
            return v!(0);
        }

        if let Some(hit) = object.hit(self, (1e-200, f64::INFINITY)) {
            if let Some(Reflection {
                reflected_ray,
                colour_attenuation,
            }) = hit.reflection
            {
                colour_attenuation.mul_elementwise(reflected_ray.colour(object, bounces - 1))
            } else {
                v!(0)
            }
        } else {
            let height = 0.5 * (self.direction.normalise().y + 1.);
            debug_assert!(
                (0.0..=1.0).contains(&height),
                "The height must be in [0, 1]: {height}"
            );
            (1. - height) * v!(1) + height * v!(0.5, 0.7, 1)
        }
    }
}
