
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::fmt::{Display, Formatter, Error};
use std::cmp::PartialEq;
use std::f64::consts::PI;
use crate::math;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {

    /// Tetermines whether or not the vector is the null-vector (0.,0.,0.)
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
        self.x == 0. &&
        self.y == 0. &&
        self.z == 0.
    }

    /// tetermines whether or not the vector is normalized (of length 1)
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let mut v: Vector3 = Vector3 { x: 0.75, y: 0.5, z: 0. };
    /// assert!( !(v.is_normalized()) );
    /// v.x = v.x.sqrt();
    /// assert!(v.is_normalized());
    /// ```
    #[inline]
    pub fn is_normalized(&self) -> bool {
        (1.0 - self.magn_sq()).abs() < math::EPSILON
    }

    /// tetermines whether or not one vector is a multiple of the other. Inputs must not be null-vectors
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1: Vector3 = Vector3 { x: 0.75, y: 0.5, z: 6. };
    /// let mut v2: Vector3 = &v1 * 3.6;
    /// assert!( Vector3::is_collinear(&v1, &v2) );
    /// v2.x += 10.;
    /// assert!( !(Vector3::is_collinear(&v1, &v2)) );
    /// ```
    #[inline]
    pub fn is_collinear(v1:&Self, v2:&Self) -> bool {
        v1.y*v2.z - v1.z*v2.y == 0.0 &&
        v1.z*v2.x - v1.x*v2.z == 0.0 &&
        v1.x*v2.y - v1.y*v2.x == 0.0
    }
    /// tetermines whether or not there exists a plane that contains all 3 vectors. Inputs must not be null-vectors
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
    #[inline]
    pub fn is_coplanar(v1: &Self, v2: &Self, v3: &Self) -> bool {
        // scalar product of v3 and (crossproduct of v1 and v2) needs to be 0
        (v1.y*v2.z - v1.z*v2.y)*v3.x +
        (v1.z*v2.x - v1.x*v2.z)*v3.y +
        (v1.x*v2.y - v1.y*v2.x)*v3.z == 0.0
    }

    /// vector scalar/dot-product
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
        v1.x * v2.x +
        v1.y * v2.y +
        v1.z * v2.z
    }

    /// set the vector's length to 1 while keeping the direction intact
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

    // calculate the square of the length of the vector
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 1.5, y: 0.5, z: 6. };
    /// assert_eq!(v1.magn_sq(), 38.5 );
    /// ```
    #[inline]
    pub fn magn_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    // calculate the length of the vector
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 1.5, y: 0.5, z: 6. };
    /// let num: f64 = 38.5;
    /// assert_eq!(v1.magn(), num.sqrt() );
    /// ```
    #[inline]
    pub fn magn(&self) -> f64 {
        ( self.magn_sq() ).sqrt()
    }

    // the the square of distance between two vector's point-equivalents 
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 3.5, y: 2.5, z: 4. };
    /// let v2 = Vector3 { x: 2., y: 2., z: -2. };
    /// let num: f64 = 38.5;
    /// assert_eq!(Vector3::dist_sq(&v1, &v2), num );
    /// ```
    #[inline]
    pub fn dist_sq(v1: &Self, v2: &Self) -> f64 {
        let dx: f64 = v1.x-v2.x;
        let dy: f64 = v1.y-v2.y;
        let dz: f64 = v1.z-v2.z;
        dx*dx + dy*dy + dz*dz
    }
    
    // the distance between two vector's point-equivalents 
    /// # Examples
    /// ```
    /// use cute_gorl::vector3::*;
    /// let v1 = Vector3 { x: 3.5, y: 2.5, z: 4. };
    /// let v2 = Vector3 { x: 2., y: 2., z: -2. };
    /// let num: f64 = 38.5;
    /// assert_eq!(Vector3::dist(&v1, &v2), num.sqrt() );
    /// ```
    #[inline]
    pub fn dist(v1: &Self, v2: &Self) -> f64 {
        Vector3::dist_sq(v1, v2).sqrt()
    }
    pub fn clamp_max(&mut self, max: f64) {
        let len: f64 = self.magn();
        if len > max { *self *= max / len; }
    }
    pub fn clamp_min(&mut self, min: f64) {
        let len: f64 = self.magn();
        if len < min { *self *= min / len; }
    }
    #[inline]
    pub fn lerp(v1: &Self, v2: &Self, factor: f64) -> Self {
        let temp = 1.0 - factor;
        Vector3 {
            x: v1.x*temp + v2.x*factor,
            y: v1.y*temp + v2.y*factor,
            z: v1.z*temp + v2.z*factor
        }
    }
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
    // crossproduct specific to 3 dimentional vectors
    #[inline]
    pub fn crossp(v1: &Self, v2: &Self) -> Self {
        Vector3 {
            x: v1.y*v2.z - v1.z*v2.y,
            y: v1.z*v2.x - v1.x*v2.z,
            z: v1.x*v2.y - v1.y*v2.x
        }
    }
    pub fn rotate(&self, angle: f64, n0: &Self) -> Self {
        //right hand rule
        let sin_a: f64 = angle.sin();
        let cos_a: f64 = math::cosq(sin_a);
        let mut temp1: Self = Self::crossp(n0, self);
        let mut f: f64      = Self::scalar(n0, self);

        temp1 *= sin_a;
        temp1 += &( self * cos_a );
        f *= 1.0 - cos_a;
        &temp1 + &(n0 * f) 
    }
    pub fn rotate_right(&self, n0: &Self) -> Self {
        //right hand rule

        let temp1: Self = Self::crossp(n0, self);
        let f: f64      = Self::scalar(n0, self);

        &temp1 + &(n0 * f) 
    }
    pub fn rotate_left(&self, n0: &Self) -> Self {
        //right hand rule

        let temp1: Self = -Self::crossp(n0, self);
        let f: f64      =  Self::scalar(n0, self);
        
        &temp1 + &(n0 * f) 
    }
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
impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}