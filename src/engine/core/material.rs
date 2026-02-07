use glam::Vec3;
use uuid::Uuid;

pub struct Material {
    pub base_color: Vec3,
    pub metallic: f32,
    pub roughness: f32,
    pub albedo_id: Uuid,
    pub roughness_id: Uuid,
    pub normal_id: Uuid,
}

impl Material {
    
}