//! This module handles various objects that can exist in a scene.

use crate::{
    ray::Ray,
    vector::{Point, Vec3},
};

/// An object which a ray could hit.
pub trait Object {
    /// Does the give ray hit this object? If so, return information about the hit.
    ///
    /// It is assumed that `bounds.0 <= bounds.1`.
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit>;
}

/// Information about how a ray hit an object.
pub struct Hit {
    /// The point at which the ray hit the object.
    pub intersection_point: Point,

    /// Th surface normal vector at the intersection point. This vector should always be
    /// pre-normalised.
    pub surface_normal: Vec3,

    /// The parameter `t` where the ray intersected the object. See [`Ray::at`].
    pub t: f64,
}

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
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        let centre_to_ray_origin = ray.origin - self.centre;

        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(centre_to_ray_origin);
        let c = centre_to_ray_origin.dot(centre_to_ray_origin) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant >= 0. {
            let root1 = (-b + discriminant.sqrt()) / (2. * a);
            let root2 = (-b - discriminant.sqrt()) / (2. * a);
            let (lower, upper) = bounds;

            let r1 = (lower..=upper).contains(&root1).then_some(root1);
            let r2 = (lower..=upper).contains(&root2).then_some(root2);

            let t = match (r1, r2) {
                (Some(r1), Some(r2)) => r1.min(r2),
                (Some(r1), None) => r1,
                (None, Some(r2)) => r2,
                (None, None) => return None,
            };

            let intersection_point = ray.at(t);
            let surface_normal = (intersection_point - self.centre).normalise();

            Some(Hit {
                intersection_point,
                surface_normal,
                t,
            })
        } else {
            None
        }
    }
}
