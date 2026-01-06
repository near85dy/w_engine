use crate::engine::renderer::transform::Transform;



pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<i32>,
    pub transform: Transform,
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<i32>) -> Self {
        Self {
            transform: Transform::new(),
            vertices,
            indices,
        }
    }
}