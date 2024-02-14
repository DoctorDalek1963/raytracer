//! This module provides the [`Sphere`] type.

use super::{Hit, Object};
use crate::{material::Material, ray::Ray, vector::Point};

/// A simple sphere.
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere<M: Material> {
    /// The centre of the sphere.
    centre: Point,

    /// The radius of the sphere.
    radius: f64,

    material: M,
}

impl<M: Material> Sphere<M> {
    /// Create a new sphere.
    pub fn new(centre: Point, radius: f64, material: M) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
}

impl<M: Material> Object for Sphere<M> {
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
            let front_face = ray.direction.dot(surface_normal) <= 0.;

            let mut hit = Hit {
                intersection_point,
                surface_normal,
                front_face,
                t,
                reflection: None,
            };

            hit.reflection = self.material.scatter(ray, &hit);

            Some(hit)
        } else {
            None
        }
    }
}
