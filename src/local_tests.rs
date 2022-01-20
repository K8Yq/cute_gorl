mod vector2;
mod vector3;
use vector2::*;
use vector3::*;

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn gwa(){
        println!("Katy Time!!");
    }
    #[test]
    pub fn it_works() {
        
        let mut v1 = Vector3 { x:0.1, y:1.0, z:0.0 };
        v1.normalize();
        //let v2 = Vector3 { x:0.0, y:0.0, z:0.0 };
        let i1:f32 = 0.0/0.0;
        //let mut v3 = Vector3 { x:1.0, y:2.0, z:3.0 };
        //let f = (&v2).magn();
        //&v2 *= (&v2).magn();
        println!("Katy Time!!");
        println!("{} {}", i1.is_nan(), v1);
        //println!("{}", Vector3::is_collinear(&v1, &v2));
        //println!("..{}", x + y);
    }
}
