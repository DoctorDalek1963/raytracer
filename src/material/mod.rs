//! This module deals with materials.

mod lambertian;
mod metal;

use crate::{object::Hit, ray::Ray, vector::Vec3};

pub use self::{lambertian::Lambertian, metal::Metal};

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

/// Perfectly reflect a ray from a surface, given the normal vector of the tangent plane.
pub fn reflect(incident_ray: Vec3, normal: Vec3) -> Vec3 {
    incident_ray - 2. * normal * incident_ray.dot(normal)
}
