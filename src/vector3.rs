
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt::{Display, Formatter, Error};
use std::cmp::PartialEq;
use std::f64::consts::PI;
#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}
pub struct Spherical {
    pub r:   f64,
    pub azm: f64,
    pub plr: f64
    // radial distance, azimuthal angle, and polar angle
}
/*
pub struct Plane {
    pub anchor: Vector3,
    pub normal: Vector3
}
pub struct Line {
    pub anchor: Vector3,
    pub direct: Vector3
} */
const D_SIGNIFICANT: f64 = 0.0000001;
#[inline]
fn cosq(sin_a: f64) -> f64 {
    (1.0 - sin_a*sin_a).sqrt()
}

impl Vector3 {
    
    // TODO: distance plane, point
    pub fn from_spherical(s: &Spherical) -> Vector3 {
        let sin_plr: f64 = s.plr.sin();
        let sin_azm: f64 = s.azm.sin();
        let cos_plr: f64 = cosq(sin_plr);
        let cos_azm: f64 = cosq(sin_azm);
        Vector3 {
            x: s.r * sin_plr * cos_azm,
            y: s.r * sin_plr * sin_azm,
            z: s.r * cos_plr
        }
    }
    // tetermines whether or not the vector is is the null-vector (0,0)
    #[inline]
    pub fn is_nullvector(&self) -> bool { 
        self.x == 0.0 &&
        self.y == 0.0 &&
        self.z == 0.0
    }
    // tetermines whether or not the vector is normalized (of length 1)
    #[inline]
    pub fn is_normalized(&self) -> bool {
        (1.0 - self.magn_sq()).abs() < D_SIGNIFICANT
    }
    // tetermines whether or not one vector is a multiple of the other. Inputs must not be null-vectors
    #[inline]
    pub fn is_collinear(v1:&Self, v2:&Self) -> bool {
        v1.y*v2.z - v1.z*v2.y == 0.0 &&
        v1.z*v2.x - v1.x*v2.z == 0.0 &&
        v1.x*v2.y - v1.y*v2.x == 0.0
    }
    // tetermines whether or not there exists a plane that contains all 3 vectors. Inputs must not be null-vectors
    #[inline]
    pub fn is_coplanar(v1:&Self, v2:&Self, v3:&Self) -> bool {
        // scalar product of v3 and (crossproduct of v1 and v2) needs to be 0
        (v1.y*v2.z - v1.z*v2.y)*v3.x +
        (v1.z*v2.x - v1.x*v2.z)*v3.y +
        (v1.x*v2.y - v1.y*v2.x)*v3.z == 0.0
    }
    // vector scalar/dot-product
    #[inline]
    pub fn scalar(v1:&Self, v2:&Self) -> f64 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }
    // scales the vector's length to 1 while keeping the same orientation
    pub fn normalize(&mut self) -> () {
        let inv_magn = 1.0 / self.magn();
        self.x *= inv_magn;
        self.y *= inv_magn;
        self.z *= inv_magn;
    }
    // calculate (the square of) the length of the vector
    // the square is computationally quicker
    #[inline]
    pub fn magn_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    #[inline]
    pub fn magn(&self) -> f64 {
        ( self.magn_sq() ).sqrt()
    }
    // the (the square of) distance between two vector's point-equivalents 
    // the square is computationally quicker
    #[inline]
    pub fn dist_sq(v1: &Self, v2: &Self) -> f64 {
        let dx: f64 = v1.x-v2.x;
        let dy: f64 = v1.y-v2.y;
        let dz: f64 = v1.z-v2.z;
        dx*dx + dy*dy + dz*dz
    }
    #[inline]
    pub fn dist(v1: &Self, v2: &Self) -> f64 {
        Vector3::dist_sq(v1, v2).sqrt()
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
        let cos_a: f64 = cosq(sin_a);
        let mut temp1: Self = Self::crossp(n0, self);
        let mut f: f64      = Self::scalar(n0, self);

        temp1 *= sin_a;
        temp1 += &( self * cos_a );
        f *= 1.0 - cos_a;
        &temp1 + &(n0 * f) 
    }
    pub fn reflect(&self, n0: &Self) -> Self {
        let mut f: f64 = Self::scalar(self, n0);
        f *= 2.0;
        self - &(n0 * f)
    }
}

impl Spherical {
    #[inline]
    pub fn from_vector3(v: &Vector3) -> Spherical {
        let t: f64 = v.x*v.x + v.y*v.y;
        Spherical {
            r: (t * v.z*v.z).sqrt(),
            azm: (v.y / v.x).atan(),
            plr: (t   / v.z).atan()
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
        Vector3 { x: self.x/quotient,
                  y: self.y/quotient,
                  z: self.z/quotient }
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
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}
impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}