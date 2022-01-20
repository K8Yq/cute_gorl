#[cfg(test)]
pub mod tests {

    use crate::vector2::*;
    use crate::vector3::*;  
    
    #[test]
    pub fn gwa(){
        println!("Katy Time!!");
    }
    #[test]
    pub fn is_funcs(){
        assert!(Vector2{ x: 1., y: 0. }.is_normalized());
        assert!(Vector3{ x: 0., y: 0., z: 1.0 }.is_normalized());
    }
}
