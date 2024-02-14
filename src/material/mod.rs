//! This module deals with materials.

mod lambertian;

use crate::{object::Hit, ray::Ray, vector::Vec3};

pub use self::lambertian::Lambertian;

/// A trait to represent a material.
pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

/// Information about the reflection.
#[derive(Clone, Debug, PartialEq)]
pub struct Reflection {
    /// The newly reflected ray.
    pub reflected_ray: Ray,

    /// How the colour gets attenuated by the reflection.
    pub colour_attenuation: Vec3,
}
