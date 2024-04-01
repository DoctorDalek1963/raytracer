//! This module provides the [`Dielectric`] material.

use crate::{
    material::{reflect, reflectance, refract, Material, Reflection},
    object::Hit,
    ray::Ray,
    vector::Colour,
};
use rand::random;

/// A transparent material like glass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dielectric {
    /// The colour tint of the material.
    pub colour: Colour,

    /// The ratio of the refractive index of the external material to the refractive index of the
    /// internal material.
    pub refraction_ratio: f64,
}

impl Dielectric {
    pub fn new(colour: Colour, refraction_ratio: f64) -> Self {
        Self {
            colour,
            refraction_ratio,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let ratio = if hit.front_face {
            self.refraction_ratio.recip()
        } else {
            self.refraction_ratio
        };
        let incoming = incident_ray.direction.normalise();

        let cos_theta = -incoming.dot(hit.surface_normal);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let scatter_direction =
            if (sin_theta * ratio > 1.) || (reflectance(cos_theta, ratio) > random()) {
                reflect(incoming, hit.surface_normal)
            } else {
                refract(incoming, hit.surface_normal, ratio)
            };

        Some(Reflection {
            reflected_ray: Ray::new(hit.intersection_point, scatter_direction),
            colour_attenuation: self.colour,
        })
    }
}
