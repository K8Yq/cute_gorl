//! Mathematical vectors in 3 dimentional space. 
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::PartialEq;
use crate::{math, vector2::Vector2};

///3D Vector
#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// The Nullvector (0 0 0)
pub const NULL: Vector3 = Vector3 { x: 0., y: 0., z:0. };

impl Vector3 {

    #[inline(always)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x: x, y: y, z: z}
    }

    /// Tetermines whether or not a [`Vector3`]'s komponents are all equal to `0.0`.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let mut v: Vector3 = Vector3 { x: 0., y: 0., z: 0. };
    /// assert!(v.is_nullvector());
    /// v.x += 0.125;
    /// assert!( !(v.is_nullvector()) );
    /// ```
    #[inline]
    pub fn is_nullvector(&self) -> bool { 
           self.x == 0.
        && self.y == 0.
        && self.z == 0.
    }

    /// Tetermines whether or not a [`Vector3`] is normalized (of length `1`).
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let mut v: Vector3 = Vector3 { x: 0.75, y: 0.5, z: 0. };
    /// assert!( !(v.is_normalized()) );
    /// v.x = v.x.sqrt();
    /// assert!(v.is_normalized());
    /// ```
    pub fn is_normalized(&self) -> bool {
        // allows for a slim margin of error to account for inaccuracy of floating-point math
        let diff = 1.0 - self.magn_sq();
        diff.abs() < math::EPSILON
    }

    /// Tetermines whether or not one [`Vector3`] is a multiple of the other. The inputs must not be null-vectors.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1: Vector3 = Vector3 { x: 0.75, y: 0.5, z: 6. };
    /// let mut v2: Vector3 = &v1 * 3.6;
    /// assert!( Vector3::is_collinear(&v1, &v2) );
    /// v2.x += 10.;
    /// assert!( !(Vector3::is_collinear(&v1, &v2)) );
    /// ```
    pub fn is_collinear(v1:&Self, v2:&Self) -> bool {
        // checks if the crossproduct is the nullvector
        v1.y*v2.z - v1.z*v2.y == 0.0 &&
        v1.z*v2.x - v1.x*v2.z == 0.0 &&
        v1.x*v2.y - v1.y*v2.x == 0.0
    }

    /// Tetermines whether or not there exists a plane that contains all three vectors. The inputs must not be null-vectors.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 0.75, y: 0.5, z: 6. };
    /// let v2 = Vector3 { x: 2.5, y: 1., z: -5. };
    /// let mut v3 = Vector3 { x: 1., y: 0., z: -17. };
    /// assert!(Vector3::is_coplanar(&v1, &v2, &v3));
    /// v3.x += 1.;
    /// assert!( !(Vector3::is_coplanar(&v1, &v2, &v3)) );
    /// ```
    pub fn is_coplanar(v1: &Self, v2: &Self, v3: &Self) -> bool {
        // checks if scalar product of v3 and (crossproduct of v1 and v2) is 0
        (v1.y*v2.z - v1.z*v2.y)*v3.x +
        (v1.z*v2.x - v1.x*v2.z)*v3.y +
        (v1.x*v2.y - v1.y*v2.x)*v3.z == 0.0
    }

    /// Calculates the scalar/dot-product of two [`Vector3`]s.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 1.25, y: 0.5, z: 6. };
    /// let v2 = Vector3 { x: 2.0, y: 1., z: -5. };
    /// let scalar: f64 = Vector3::scalar(&v1, &v2);
    /// assert_eq!( scalar, -27. );
    /// ```
    #[inline]
    pub fn scalar(v1: &Self, v2: &Self) -> f64 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }

    /// Scales a [`Vector3`] to a magnitude of 1.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let mut v1 = Vector3 { x: 1.25, y: 0.5, z: 6. };
    /// let mut v2 = Vector3 { x: 10.0, y: 1., z: -5. };
    /// v1.normalize();
    /// v2.normalize();
    /// assert!(v1.is_normalized() && v2.is_normalized());
    /// ```
    pub fn normalize(&mut self) -> () {
        let inv_magn = 1. / self.magn();
        self.x *= inv_magn;
        self.y *= inv_magn;
        self.z *= inv_magn;
    }

    /// Calculates the square of the magnitude of a [`Vector3`].
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 1.5, y: 0.5, z: 6. };
    /// assert_eq!(v1.magn_sq(), 38.5_f64 );
    /// ```
    #[inline]
    pub fn magn_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    /// Calculates the magnitude of a [`Vector3`].
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 1.5, y: 0.5, z: 6. };
    /// assert_eq!(v1.magn(), 38.5_f64.sqrt() );
    /// ```
    #[inline]
    pub fn magn(&self) -> f64 {
        self.magn_sq().sqrt()
    }

    /// Calculates the the square of distance between two [`Vector3`]s interpreted as points.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 3.5, y: 2.5, z: 4. };
    /// let v2 = Vector3 { x: 2., y: 2., z: -2. };
    /// assert_eq!(Vector3::dist_sq(&v1, &v2), 38.5_f64 );
    /// ```
    #[inline]
    pub fn dist_sq(v1: &Self, v2: &Self) -> f64 {
        let dx: f64 = v1.x-v2.x;
        let dy: f64 = v1.y-v2.y;
        let dz: f64 = v1.z-v2.z;
        dx*dx + dy*dy + dz*dz
    }
    
    /// Calculates the distance between two [`Vector3`]s interpreted as points.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 3.5, y: 2.5, z: 4. };
    /// let v2 = Vector3 { x: 2., y: 2., z: -2. };
    /// assert_eq!(Vector3::dist(&v1, &v2), 38.5_f64.sqrt() );
    /// ```
    #[inline]
    pub fn dist(v1: &Self, v2: &Self) -> f64 {
        Self::dist_sq(v1, v2).sqrt()
    }

    /// Scales down a [`Vector3`] to a magnitude if it exceeds that magnitude.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let mut v = Vector3 { x: 4., y: 4., z: 4. };
    /// v.clamp_max(1_f64);
    /// assert!(v.is_normalized());
    /// ```
    pub fn clamp_max(&mut self, max: f64) {
        let len: f64 = self.magn();
        if len > max { *self *= max / len; }
    }

    /// Scales up a [`Vector3`] to a magnitude if it is shorter than that magnitude.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let mut v = Vector3 { x: 1., y: 1., z: 1. };
    /// v.clamp_max(1_f64);
    /// assert!(v.is_normalized());
    /// ```
    pub fn clamp_min(&mut self, min: f64) {
        let len: f64 = self.magn();
        if len < min { *self *= min / len; }
    }

    /// Linearly interpolate between two [`Vector3`]s interpreted as points.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 0.2, y: 0.1, z: 0.3 };
    /// let v2 = -v1;
    /// assert_eq!(Vector3::lerp(&v1, &v2, 0.5_f64), NULL);
    /// ```
    pub fn lerp(v1: &Self, v2: &Self, factor: f64) -> Self {
        let temp = 1.0 - factor;
        Vector3 {
            x: v1.x*temp + v2.x*factor,
            y: v1.y*temp + v2.y*factor,
            z: v1.z*temp + v2.z*factor
        }
    }

    /// calculates the angle between two [`Vector3`]s.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 1., y: 1., z: 1. };
    /// let v2 = Vector3 { x: 1., y: 1., z: -1. };
    /// assert_eq!(
    ///     Vector3::angle_between(&v1, &v2),
    ///     1.2309594173407747_f64
    /// );
    /// ```
    pub fn angle_between(v1: &Self, v2: &Self) -> f64 {
        let a: f64 = Self::scalar(v1, v2);
        let h: f64 = ( v1.magn_sq() * v2.magn_sq() ).sqrt();
        (a / h).acos()
    }

    /// The crossproduct of two [`Vector3`]s.
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 1., y: 2., z: 3. };
    /// let v2 = Vector3 { x: 4., y: -5., z: 0. };
    /// assert!(!Vector3::is_coplanar(
    ///     &v1,
    ///     &v2,
    ///     & ( Vector3::crossp(&v1, &v2) )
    /// ));
    /// ```
    pub fn crossp(v1: &Self, v2: &Self) -> Self {
        Vector3 {
            x: v1.y*v2.z - v1.z*v2.y,
            y: v1.z*v2.x - v1.x*v2.z,
            z: v1.x*v2.y - v1.y*v2.x
        }
    }

    /// Rotate a [`Vector3`] by an angle around another [`Vector3`].
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// use std::f64::consts::PI;
    /// let mut v = Vector3 { x: 1., y: 1., z: 1. };
    /// let mut n = Vector3 { x: 1., y: -1., z: 1. };
    /// n.normalize();
    /// assert_eq!(
    ///     v.rotate(0.5*PI, &n),
    ///     Vector3 { x: -0.8213672050459182, y: -0.3333333333333334, z: 1.4880338717125852 }
    /// );
    /// ```
    pub fn rotate(&self, angle: f64, n0: &Self) -> Self {
        //vrot = vcos0 + (kxv)sin0 + k(k*v)(1-cos0) 
        
        let sin_a: f64 = angle.sin();
        let cos_a: f64 = math::cosq(sin_a);
        let mut temp1: Self = Self::crossp(n0, self);
        let mut f: f64      = Self::scalar(n0, self);

        temp1 *= sin_a;
        temp1 += &( self * cos_a );
        f *= 1.0 - cos_a;
        &temp1 + &(n0 * f) 
    }
    /// Rotate a [`Vector3`] by 90 degrees around another [`Vector3`].
    pub fn rotate_right(&self, n0: &Self) -> Self {
        //right hand rule

        let crossp: Self = Self::crossp(n0, self);
        let scalar: f64  = Self::scalar(n0, self);

        &crossp + &(n0 * scalar) 
    }

    /// Rotate (clockwise) a [`Vector3`] by -90 degrees around another [`Vector3`].
    pub fn rotate_left(&self, n0: &Self) -> Self {
        //right hand rule

        let crossp: Self = -Self::crossp(n0, self);
        let scalar: f64  =  Self::scalar(n0, self);
        
        &crossp + &(n0 * scalar) 
    }
    
    /// Reflect a [`Vector3`] off of a surface with a certain normal [`Vector3`].
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let mut v = Vector3 { x: 1., y: 1., z: 1. };
    /// let n = Vector3 { x: 0., y: 0., z: 1. };
    /// v.reflect(&n);
    /// assert_eq!(v, Vector3 { x: 1., y: 1., z: -1. });
    /// ```
    pub fn reflect(&mut self, n0: &Self) -> () {
        let mut f: f64 = Self::scalar(self, n0);
        f *= 2.0;
        *self -= &(n0 * f);
    }
}
impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}
impl PartialEq for Vector3{
    fn eq(&self, other: &Vector3) -> bool{
        (self.x == other.x) &&
        (self.y == other.y) &&
        (self.z == other.z)
    }
    fn ne(&self, other: &Vector3) -> bool{
        (self.x != other.x) &&
        (self.y != other.y) &&
        (self.z != other.z)
    }
}
impl Add for &Vector3{
    type Output = Vector3;
    fn add(self, other: &Vector3) -> Self::Output {
        Vector3 { x: self.x + other.x,
                  y: self.y + other.y,
                  z: self.z + other.z }
    }
}
impl Sub for &Vector3{
    type Output = Vector3;
    fn sub(self, other: &Vector3) -> Self::Output {
        Vector3 { x: self.x-other.x,
                  y: self.y-other.y,
                  z: self.z-other.z }
    }
}
impl Mul<f64> for &Vector3{
    type Output = Vector3;
    fn mul(self, factor: f64) -> Self::Output {
        Vector3 { x: self.x*factor, 
                  y: self.y*factor, 
                  z: self.z*factor }
    }
}
impl Mul<&Vector3> for f64{
    type Output = Vector3;
    fn mul(self, vector: &Vector3) -> Self::Output {
        Vector3 { x: vector.x*self,
                  y: vector.y*self,
                  z: vector.z*self }
    }
}
impl Div<f64> for &Vector3{
    type Output = Vector3;
    fn div(self, quotient: f64) -> Self::Output {
        let f: f64 = 1./quotient;
        Vector3 { x: self.x*f,
                  y: self.y*f,
                  z: self.z*f }
    }
}
impl AddAssign<&Self> for Vector3 {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl SubAssign<&Self> for Vector3 {
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}
impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        let f: f64 = 1./other;
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}
impl From<Vector2> for Vector3 {
    #[inline]
    fn from(v2: Vector2) -> Vector3 {
        Vector3 {
            x: v2.x,
            y: v2.y,
            z: 0.0
        }
    }
}