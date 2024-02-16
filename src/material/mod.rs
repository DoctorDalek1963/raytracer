//! This module deals with materials.

mod dielectric;
mod lambertian;
mod metal;

use crate::{object::Hit, ray::Ray, vector::Vec3};

pub use self::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

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
#[inline]
pub fn reflect(incident_ray: Vec3, normal: Vec3) -> Vec3 {
    incident_ray - 2. * normal * incident_ray.dot(normal)
}

/// Refract a ray through a surface, given the refraction ratio.
///
/// This function *requires* that the `incident_ray` and `normal` vectors are normalised, otherwise
/// it could produce nonsense.
pub fn refract(incident_ray: Vec3, normal: Vec3, refraction_ratio: f64) -> Vec3 {
    let cos = -incident_ray.dot(normal);
    let r_perp = refraction_ratio * (incident_ray + cos * normal);
    let r_par = -(1. - r_perp.dot(r_perp)).abs().sqrt() * normal;
    r_perp + r_par
}

/// Use the Shlick approximation to calculate the reflectance of a material with the given
/// refractive index and the cosine of the incident angle.
pub fn reflectance(cos_theta: f64, refractive_index: f64) -> f64 {
    let frac = (1. - refractive_index) / (1. + refractive_index);
    let r_0 = frac * frac;
    r_0 + (1. - r_0) * (1. - cos_theta).powi(5)
}
