// 2 and 3 dimentional component focused mathematical vectors 
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::fmt::{Display, Formatter, Error};
use std::cmp::PartialEq;
use std::f64::consts::PI;
use crate::math;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}
impl Vector2 {
    
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }
    // tetermines whether or not the vector is is the null-vector (0,0)
    #[inline]
    pub fn is_nullvector(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
    // tetermines whether or not the vector is normalized (of length 1) 
    #[inline]
    pub fn is_normalized(&self) -> bool {
        (1.0 - self.magn_sq()).abs() < math::EPSILON
    }
    // tetermines whether or not one vector is a multiple of the other. Inputs must not be null-vectors
    #[inline]
    pub fn is_collinear(v1:&Self, v2:&Self) -> bool {
        v1.x*v2.y - v1.y*v2.x == 0.0
    }
    // vector scalar/dot-product 
    #[inline]
    pub fn scalar(v1:&Self, v2:&Self) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }
    // scales the vector's length to 1 while keeping the same orientation 
    #[inline]
    pub fn normalize(&mut self) -> () {
        let inv_magn = 1.0 / self.magn();
        self.x *= inv_magn;
        self.y *= inv_magn;
    }
    // calculate (the square of) the length of the vector
    // the square is computationally quicker
    #[inline]
    pub fn magn_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }
    pub fn magn(&self) -> f64 {
        ( self.x*self.x + self.y*self.y ).sqrt()
    }
    #[inline]
    // the (the square of) distance between two vector's point-equivalents 
    // the square is computationally quicker
    pub fn dist_sq(v1:&Self, v2:&Self) -> f64 {
        let dx: f64 = v1.x-v2.x;
        let dy: f64 = v1.y-v2.y;
        dx*dx + dy*dy
    }
    pub fn dist(v1:&Self, v2:&Self) -> f64 {
        Self::dist_sq(v1, v2).sqrt()
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
        Self {
            x: v1.x*temp + v2.x*factor,
            y: v1.y*temp + v2.y*factor
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
    pub fn rotate(&self, angle: f64) -> Self {
        let sin_a: f64 = angle.sin();
        let cos_a: f64 = math::cosq(sin_a);
        Self {
            x: self.x*cos_a - self.y*sin_a,
            y: self.y*cos_a + self.x*sin_a
        }
    }
    pub fn rotate_right(&self) -> Self {
        Self { x: -self.y, y:  self.x }
    }
    pub fn rotate_left(&self) -> Self {
        Self { x:  self.y, y: -self.x }
    }
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
impl Display for Vector2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}