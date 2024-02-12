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

    /// Does the given ray hit this sphere? If so, return the `t` value such that [`Ray::at`]
    /// returns the point where the ray first intersects the sphere.
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let centre_to_ray_origin = ray.origin - self.centre;

        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(centre_to_ray_origin);
        let c = centre_to_ray_origin.dot(centre_to_ray_origin) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant >= 0. {
            // The point which is closest to the camera will always be the smaller t value, and
            // since t is always positive, this means we only need the minus branch
            debug_assert!(
                (-b - discriminant.sqrt()) / (2. * a) <= (-b + discriminant.sqrt()) / (2. * a),
                "The minus branch of the quadratic formula should always give a smaller t value"
            );
            Some((-b - discriminant.sqrt()) / (2. * a))
        } else {
            None
        }
    }
}
