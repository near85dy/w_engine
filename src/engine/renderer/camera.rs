use glam::{Mat4, Quat, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov: f32,
    pub aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            fov: 90.0_f32.to_radians(),
            aspect: 16.0/9.0,
            near: 0.1, // > 0
            far: 1000.0
        }
    }

    pub fn projection(&self) -> Mat4 {
        return Mat4::perspective_rh(
            self.fov, 
            self.aspect, 
            self.near, 
            self.far
        );
    }

    pub fn view(&self) -> Mat4 {
        let forward = self.rotation * Vec3::NEG_Z;
        return Mat4::look_at_rh(self.position, self.position+forward, Vec3::Y);
    }

}