//! This module handles various objects that can exist in a scene.

mod sphere;

use crate::{
    material::Reflection,
    ray::Ray,
    vector::{Point, Vec3},
};

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

    /// A possible reflection from the object.
    pub reflection: Option<Reflection>,
}

/// A collection of objects. If your scene only contains objects of one type, just use a vec of
/// that type.
pub type Scene = Vec<Box<dyn Object + Sync + Send>>;

/// Create a `Vec<Box<dyn `[`Object`]` + Sync + Send>>` without having to wrap every element in a [`Box`].
#[allow(unused_macros)]
macro_rules! dyn_scene_vec {
    ($($elem:expr),*$(,)?) => {
        vec![$((
            ::std::boxed::Box::new($elem)
            as ::std::boxed::Box<dyn $crate::object::Object + ::std::marker::Sync + ::std::marked::Send>
        )),*]
    };
}

#[allow(unused_imports)]
pub(crate) use dyn_scene_vec;

impl Object for Scene {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        self.iter()
            .map(|object| object.hit(ray, bounds))
            .fold(None, |a, b| match (a, b) {
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
            })
    }
}

/// Generate a random scene.
pub fn random_scene() -> Scene {
    use crate::{
        material::{Dielectric, Lambertian, Metal},
        vector::v,
    };

    fn rand_f64() -> f64 {
        rand::random()
    }

    let mut objects: Scene = Vec::with_capacity(4 + (11usize + 11 + 1).pow(2));

    // Ground
    objects.push(Box::new(Sphere::new(
        v!(0, -1000, 0),
        1000.0,
        Lambertian::new(v!(0.5, 0.5, 0.5)),
    )));

    for a in -11..=11 {
        for b in -11..=11 {
            let a = a as f64;
            let b = b as f64;
            let material_choice = rand_f64();
            let centre = v!(a + 0.75 * rand_f64(), 0.2, b + 0.75 * rand_f64());

            #[allow(illegal_floating_point_literal_pattern)]
            objects.push(match material_choice {
                0.0..=0.8 => Box::new(Sphere::new(centre, 0.2, Lambertian::new(v!(rand_f64())))),
                0.8..=0.95 => Box::new(Sphere::new(
                    centre,
                    0.2,
                    Metal::new(v!(0.2 + rand_f64() * 0.8), rand_f64() / 1.5),
                )),
                0.95..=1.0 => Box::new(Sphere::new(
                    centre,
                    0.2,
                    Dielectric::new(v!(0.5 + rand_f64() * 0.5), 1.5),
                )),
                _ => panic!("material_choice should always be in 0.0..=1.0"),
            });
        }
    }

    objects.push(Box::new(Sphere::new(
        v!(0, 1, 0),
        1.0,
        Dielectric::new(v!(1), 1.5),
    )));
    objects.push(Box::new(Sphere::new(
        v!(-4, 1, 0),
        1.0,
        Lambertian::new(v!(0.4, 0.2, 0.1)),
    )));
    objects.push(Box::new(Sphere::new(
        v!(4, 1, 0),
        1.0,
        Metal::new(v!(0.7, 0.6, 0.5), 0.0),
    )));
    objects
}

// This impl uses generics rather than trait objects to allow for more efficent compiler
// optimisations.
impl<T> Object for Vec<T>
where
    T: Object + Sync,
{
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        self.iter()
            .map(|object| object.hit(ray, bounds))
            .fold(None, |a, b| match (a, b) {
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
            })
    }
}
