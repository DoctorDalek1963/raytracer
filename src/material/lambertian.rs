//! This module provides the [`Lambertian`] material.

use crate::{
    material::{Material, Reflection},
    object::Hit,
    ray::Ray,
    vector::{Colour, Vec3},
};

/// A material with Lambertian diffuse reflection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lambertian {
    /// The colour of this material.
    pub colour: Colour,
}

impl Lambertian {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let random_scatter_direction = hit.surface_normal + Vec3::random_unit_vector();
        Some(Reflection {
            reflected_ray: Ray::new(
                hit.intersection_point,
                if random_scatter_direction.is_zero() {
                    hit.surface_normal
                } else {
                    random_scatter_direction
                },
            ),
            colour_attenuation: self.colour,
        })
    }
}
