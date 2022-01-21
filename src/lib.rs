mod local_tests;
pub mod vector2;
pub mod vector3;
mod quadtree;

pub mod cute {
    pub fn cuter()->i32{
        println!("gwa");
        4
    }
}

mod math {
    pub const EPSILON: f64 = 1e-8;
    pub fn cosq(sin_a: f64) -> f64 { (1.0 - sin_a*sin_a).sqrt() }
}