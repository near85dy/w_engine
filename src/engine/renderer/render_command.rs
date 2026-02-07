use glam::{Quat, Vec3};
use uuid::Uuid;

use crate::engine::renderer::transform::Transform;

pub enum RenderCommand {
    LoadMesh {
        id: Uuid,
        vertices: Vec<[f32; 5]>,
        indices: Vec<i32>
    },
    LoadTexture {
        id: Uuid,
        width: i32,
        height: i32,
        image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
    },
    BindTexture {
        id: Uuid,
    },
    DrawMesh {
        id: Uuid,
        transform: Transform,
    },
    TransformCamera {
        position: Vec3,
        rotation: Quat,
    },
    ApplyTransform {
        id: Uuid,
        transform: Transform,
    },
}