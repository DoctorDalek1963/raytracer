//! This module provides the [`Metal`] material.

use crate::{
    material::{reflect, Material, Reflection},
    object::Hit,
    ray::Ray,
    vector::{Colour, Vec3},
};

/// A reflective metal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metal {
    /// The colour of this material.
    pub colour: Colour,

    /// The fuzziness of this material. It should be in `[0, 1]` and will be clamped down to this
    /// range.
    pub fuzz: f64,
}

impl Metal {
    pub fn new(colour: Colour, fuzz: f64) -> Self {
        Self {
            colour,
            fuzz: fuzz.clamp(0., 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let reflection_direction = reflect(incident_ray.direction, hit.surface_normal)
            + self.fuzz * Vec3::random_unit_vector();
        let reflected_ray = Ray::new(hit.intersection_point, reflection_direction);

        if reflected_ray.direction.dot(hit.surface_normal) > 0. {
            Some(Reflection {
                reflected_ray,
                colour_attenuation: self.colour,
            })
        } else {
            None
        }
    }
}
