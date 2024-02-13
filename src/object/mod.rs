//! This module handles various objects that can exist in a scene.

mod sphere;

use crate::{
    ray::Ray,
    vector::{Point, Vec3},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub use self::sphere::Sphere;

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
    /// pre-normalised and should point away from the inside of the object.
    pub surface_normal: Vec3,

    /// Did the ray hit the outside of the object?
    pub front_face: bool,

    /// The parameter `t` where the ray intersected the object. See [`Ray::at`].
    pub t: f64,
}

/// A collection of objects. If your scene only contains objects of one type, just use a vec of
/// that type.
pub type Scene = Vec<Box<dyn Object + Sync>>;

impl Object for Scene {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        self.par_iter()
            .map(|object| object.hit(ray, bounds))
            .reduce(
                || None,
                |a, b| match (a, b) {
                    (Some(a), Some(b)) => {
                        if a.t < b.t {
                            Some(a)
                        } else {
                            Some(b)
                        }
                    }
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                },
            )
    }
}

// This impl uses generics rather than trait objects to allow for more efficent compiler
// optimisations.
impl<T> Object for Vec<T>
where
    T: Object + Sync,
{
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        self.par_iter()
            .map(|object| object.hit(ray, bounds))
            .reduce(
                || None,
                |a, b| match (a, b) {
                    (Some(a), Some(b)) => {
                        if a.t < b.t {
                            Some(a)
                        } else {
                            Some(b)
                        }
                    }
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                },
            )
    }
}
