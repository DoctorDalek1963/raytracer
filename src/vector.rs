//! This module handles vectors.

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// An RGB colour.
pub type Colour = Vec3;

/// A point in 3D space.
pub type Point = Vec3;

/// A vector of three floats.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Create a new vector with the given coordinates.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Create a new vector where every component is the given value.
    pub const fn splat(value: f64) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }

    /// The dot product of this vector with another.
    #[inline]
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// The cross product of this vector with another.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - other.y * self.z,
            y: other.x * self.z - self.x * other.z,
            z: self.x * other.y - other.x * self.y,
        }
    }

    /// Multiply this vector elementwise with another.
    #[inline]
    pub fn mul_elementwise(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    /// Get the length (magnitude) of this vector.
    #[inline]
    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Map a function over all the elements of this vector.
    #[inline]
    pub fn map(self, f: impl Fn(f64) -> f64) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    /// Normalise the vector so it has unit length.
    #[inline]
    pub fn normalise(self) -> Self {
        self.map(|x| x / self.len())
    }

    /// Convert this vector into an array of three `u8`s to make it an RGB array.
    #[inline]
    pub fn into_u8_array(self) -> [u8; 3] {
        debug_assert!(
            (0.0..=1.0).contains(&self.x),
            "The x value must be in [0, 1]: {}",
            self.x
        );
        debug_assert!(
            (0.0..=1.0).contains(&self.y),
            "The y value must be in [0, 1]: {}",
            self.y
        );
        debug_assert!(
            (0.0..=1.0).contains(&self.z),
            "The z value must be in [0, 1]: {}",
            self.z
        );
        [
            (self.x * 255.).round() as u8,
            (self.y * 255.).round() as u8,
            (self.z * 255.).round() as u8,
        ]
    }
}

impl From<Vec3> for [u8; 3] {
    fn from(value: Vec3) -> Self {
        value.into_u8_array()
    }
}

impl From<Vec3> for image::Rgb<u8> {
    fn from(value: Vec3) -> Self {
        image::Rgb(value.into_u8_array())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Construct a vector.
macro_rules! v {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vector::Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };
    ($x:expr) => {
        $crate::vector::Vec3::splat(f64::from($x))
    };
}

pub(crate) use v;
