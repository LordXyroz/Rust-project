extern crate cgmath;

use cgmath::{Vector3, Deg, Euler, Quaternion};

pub struct Transform {
    positon: Vector3<f32>,
    rotation: Quaternion<f32>,
}

impl Transform {
    
}
 
impl Default for Transform {
    fn default() -> Transform {
        let _pos = Vector3::new(0.0, 0.0, 0.0);
        Transform {
            positon: _pos,
            rotation: Quaternion::from(Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)))
        }
    }
}