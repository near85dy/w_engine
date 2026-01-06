use uuid::Uuid;

use crate::engine::renderer::transform::Transform;

pub enum RenderCommand {
    CreateMesh {
        id: Uuid,
        vertices: Vec<f32>,
        indices: Vec<i32>
    },
    CreateTexture {
        id: Uuid,
    },
    DrawMesh {
        id: Uuid,
        transform: Transform,
    },
    ApplyTransform {
        id: Uuid,
        transform: Transform,
    },
}