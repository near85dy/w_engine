use crate::engine::renderer::{gpu_mesh::GpuMesh, transform::Transform};

pub struct RenderObjectData {
    pub mesh: GpuMesh,
    pub transform: Transform,
}

impl RenderObjectData {
    pub fn new(mesh: GpuMesh) -> Self {
        Self { 
            mesh, 
            transform: Transform::new(),
        }
    }
}
