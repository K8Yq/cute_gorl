//! Mathematical vectors in 2 dimentional space. 
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::PartialEq;
use std::f64::consts::PI;
use crate::{math};

///2D Vector
#[derive(Copy, Clone, Debug, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

/// The Nullvector (0 0)
pub const NULL: Vector2 = Vector2 { x: 0., y: 0. };

impl Vector2 {
    
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }

    /// Tetermines whether or not a [`Vector2`]'s komponents are all equal to `0.0`.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let mut v: Vector2 = Vector2 { x: 0., y: 0. };
    /// assert!(v.is_nullvector());
    /// v.x += 0.125;
    /// assert!( !(v.is_nullvector()) );
    /// ```
    #[inline]
    pub fn is_nullvector(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    /// Tetermines whether or not a [`Vector2`] is normalized (of length `1`).
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let mut v: Vector2 = Vector2 { x: 0.75, y: 0.5 };
    /// assert!( !(v.is_normalized()) );
    /// v.x = v.x.sqrt();
    /// assert!(v.is_normalized());
    /// ```
    pub fn is_normalized(&self) -> bool {
        let diff = 1.0 - self.magn_sq();
        diff.abs() < math::EPSILON
    }

    /// Tetermines whether or not one [`Vector2`] is a multiple of the other. The inputs must not be null-vectors.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let v1: Vector2 = Vector2 { x: 0.75, y: 0.5 };
    /// let mut v2: Vector2 = &v1 * 3.6;
    /// assert!( Vector2::is_collinear(&v1, &v2) );
    /// v2.x += 10.;
    /// assert!( !(Vector2::is_collinear(&v1, &v2)) );
    /// ```
    #[inline]
    pub fn is_collinear(v1:&Self, v2:&Self) -> bool {
        v1.x*v2.y - v1.y*v2.x == 0.0
    }

    /// Calculates the scalar/dot-product of two [`Vector2`]s.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let v1 = Vector2 { x: 1.25, y: 0.5 };
    /// let v2 = Vector2 { x: 2.0, y: 1. };
    /// let scalar: f64 = Vector2::scalar(&v1, &v2);
    /// assert_eq!( scalar, 3. );
    /// ```
    #[inline]
    pub fn scalar(v1:&Self, v2:&Self) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }

    /// Scales a [`Vector2`] to a magnitude of 1.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let mut v1 = Vector2 { x: 1.25, y: 0.5 };
    /// let mut v2 = Vector2 { x: 10.0, y: 1. };
    /// v1.normalize();
    /// v2.normalize();
    /// assert!(v1.is_normalized() && v2.is_normalized());
    /// ```
    pub fn normalize(&mut self) -> () {
        let inv_magn = 1.0 / self.magn();
        self.x *= inv_magn;
        self.y *= inv_magn;
    }

    /// Calculates the square of the magnitude of a [`Vector2`].
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let v1 = Vector2 { x: 1.5, y: 0.5 };
    /// assert_eq!(v1.magn_sq(), 2.5 );
    /// ```
    #[inline]
    pub fn magn_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }

    /// Calculates the magnitude of a [`Vector2`].
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let v1 = Vector2 { x: 1.5, y: 0.5 };
    /// assert_eq!(v1.magn(), 2.5_f64.sqrt() );
    /// ```
    pub fn magn(&self) -> f64 {
        ( self.x*self.x + self.y*self.y ).sqrt()
    }

    /// Calculates the the square of distance between two [`Vector2`]s interpreted as points.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let v1 = Vector2 { x: 3.5, y: 2.5 };
    /// let v2 = Vector2 { x: 2., y: 2. };
    /// assert_eq!(Vector2::dist_sq(&v1, &v2), 2.5 );
    /// ```
    pub fn dist_sq(v1:&Self, v2:&Self) -> f64 {
        let dx: f64 = v1.x-v2.x;
        let dy: f64 = v1.y-v2.y;
        dx*dx + dy*dy
    }

    /// Calculates the distance between two [`Vector2`]s interpreted as points.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let v1 = Vector2 { x: 3.5, y: 2.5 };
    /// let v2 = Vector2 { x: 2., y: 2. };
    /// assert_eq!(Vector2::dist(&v1, &v2), 2.5_f64.sqrt() );
    /// ```
    pub fn dist(v1:&Self, v2:&Self) -> f64 {
        Self::dist_sq(v1, v2).sqrt()
    }

    /// Scales down a [`Vector2`] to a magnitude if it exceeds that magnitude.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let mut v = Vector2 { x: 4., y: 4. };
    /// v.clamp_max(1_f64);
    /// assert!(v.is_normalized());
    /// ```
    pub fn clamp_max(&mut self, max: f64) {
        let len: f64 = self.magn();
        if len > max { *self *= max / len; }
    }

    /// Scales up a [`Vector2`] to a magnitude if it is shorter than that magnitude.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let mut v = Vector2 { x: 1., y: 1. };
    /// v.clamp_max(1_f64);
    /// assert!(v.is_normalized());
    /// ```
    pub fn clamp_min(&mut self, min: f64) {
        let len: f64 = self.magn();
        if len < min { *self *= min / len; }
    }

    /// Linearly interpolate between two [`Vector2`]s interpreted as points.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let v1 = Vector2 { x: 0.2, y: 0.1 };
    /// let v2 = -v1;
    /// assert_eq!(Vector2::lerp(&v1, &v2, 0.5_f64), NULL);
    /// ```
    pub fn lerp(v1: &Self, v2: &Self, factor: f64) -> Self {
        let temp = 1.0 - factor;
        Self {
            x: v1.x*temp + v2.x*factor,
            y: v1.y*temp + v2.y*factor
        }
    }

    /// calculates the angle between two [`Vector2`]s.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// use std::f64::consts::PI;
    /// let v1 = Vector2 { x: 1., y: 1. };
    /// let v2 = -v1;
    /// assert_eq!( Vector2::angle_between(&v1, &v2), PI );
    /// ```
    pub fn angle_between(v1: &Self, v2: &Self) -> f64 {
        let o: f64 = v1.y * v2.x - v1.x * v2.y;
        let a: f64 = v1.x * v2.x + v1.y * v2.y;
        let res: f64 = (o / a).atan();
        if o < 0.0 {
            if a <= 0.0 { PI - res } else { -res }
        } else {
            if a <  0.0 { PI + res } else { res }
        }
    }

    /// Rotate (anti-clockwise) a [`Vector2`] by an angle.
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// use std::f64::consts::PI;
    /// let mut v = Vector2 { x: 1., y: 1. };
    /// assert_eq!(
    ///     v.rotate(0.5*PI),
    ///     Vector2 { x: -1., y: 1. }
    /// );
    /// ```
    pub fn rotate(&self, angle: f64) -> Self {
        let sin_a: f64 = angle.sin();
        let cos_a: f64 = math::cosq(sin_a);
        Self {
            x: self.x*cos_a - self.y*sin_a,
            y: self.y*cos_a + self.x*sin_a
        }
    }

    /// Rotate a [`Vector2`] by 90 degrees.
    #[inline(always)]
    pub fn rotate_right(&self) -> Self {
        Self { x: -self.y, y: self.x }
    }

    /// Rotate (clockwise) a [`Vector2`] by -90 degrees.
    #[inline(always)]
    pub fn rotate_left(&self) -> Self {
        Self { x: self.y, y: -self.x }
    }

    /// Reflect a [`Vector2`] off of a surface with a certain normal [`Vector2`].
    /// # Examples
    /// ```
    /// use cute_gorl::vector2::*;
    /// let mut v = Vector2 { x: 1., y: 1. };
    /// let n = Vector2 { x: 0., y: 1. };
    /// assert_eq!(v.reflect(&n), Vector2 { x: 1., y: -1. });
    /// ```
    pub fn reflect(&self, n0: &Self) -> Self {
        let factor: f64 = 2.0 * &Self::scalar(self, n0);
        let temp: Self = n0 * factor;
        self - &temp
    }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output {
        Self::Output { x: -self.x, y: -self.y }
    }
}
impl PartialEq for Vector2{
    fn eq(&self, other: &Vector2) -> bool{
        (self.x == other.x) &&
        (self.y == other.y)
    }
    fn ne(&self, other: &Vector2) -> bool{
        (self.x != other.x) &&
        (self.y != other.y)
    }
}
impl Add for &Vector2{
    type Output = Vector2;
    fn add(self, other: &Vector2) -> Self::Output {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }
}
impl Sub for &Vector2{
    type Output = Vector2;
    fn sub(self, other: &Vector2) -> Self::Output {
        Vector2 { x: self.x-other.x, y: self.y-other.y }
    }
}
impl Mul<f64> for &Vector2{
    type Output = Vector2;
    fn mul(self, factor: f64) -> Self::Output {
        Vector2 { x: self.x*factor, y: self.y*factor }
    }
}
impl Mul<&Vector2> for f64{
    type Output = Vector2;
    fn mul(self, vector: &Vector2) -> Self::Output {
        Vector2 { x: vector.x*self, y: vector.y*self }
    }
}
impl Div<f64> for &Vector2{
    type Output = Vector2;
    fn div(self, quotient: f64) -> Self::Output {
        let f: f64 = 1./quotient;
        Vector2 { x: self.x*f, y: self.y*f }
    }
}
impl AddAssign<Self> for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl AddAssign<&Self> for Vector2 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign<Self> for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl SubAssign<&Self> for Vector2 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
}
impl DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, other: f64) {
        let f: f64 = 1./other;
        self.x *= f;
        self.y *= f;
    }
}