//! This module handles various objects that can exist in a scene.

use crate::{ray::Ray, vector::Point};

/// A simple sphere.
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    /// The centre of the sphere.
    centre: Point,

    /// The radius of the sphere.
    radius: f64,
}

impl Sphere {
    /// Create a new sphere.
    pub fn new(centre: Point, radius: f64) -> Self {
        Self { centre, radius }
    }

    /// Does the given ray hit this sphere?
    pub fn hit(&self, ray: &Ray) -> bool {
        let centre_to_ray_origin = ray.origin - self.centre;

        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(centre_to_ray_origin);
        let c = centre_to_ray_origin.dot(centre_to_ray_origin) - self.radius * self.radius;

        b * b - 4. * a * c >= 0.
    }
}
