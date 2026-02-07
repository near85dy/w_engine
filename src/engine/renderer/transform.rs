use glam::{Mat4, Quat, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug)]  
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self { 
            position: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 
            rotation: Quat::from_euler(
                glam::EulerRot::YXZ,
                0.0_f32.to_radians(),
                0.0,
                0.0,
            ), 
            scale: Vec3::ONE,
        }
    } 

    pub fn rotate_local_x(&mut self, angle: f32) {
        let rot_x = Quat::from_rotation_x(angle);
        self.rotation = (self.rotation * rot_x).normalize();  // ← self * local!
    }
    
    pub fn rotate_local_y(&mut self, angle: f32) {
        let rot_y = Quat::from_rotation_y(angle);
        self.rotation = (self.rotation * rot_y).normalize();
    }
    
    pub fn rotate_local_z(&mut self, angle: f32) {
        let rot_z = Quat::from_rotation_z(angle);
        self.rotation = (self.rotation * rot_z).normalize();
    }

    pub fn get_transform_matrix(&self) -> Mat4
    {
        return Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        );
    }
}