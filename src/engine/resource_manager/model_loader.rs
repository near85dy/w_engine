use std::{
    fs,
    path::Path,
};

pub struct ModelData {
    pub name: String,
    pub vertices: Vec<f32>,
    pub indices: Vec<i32>,
}

pub struct ModelLoader {}

impl ModelLoader {
    pub fn load_model(filename: String) -> std::io::Result<ModelData> {
        let full_path = format!("assets/models/{}", filename);
        let model_path = Path::new(&full_path);

        if !model_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File {} does not exist", model_path.display())
            ));
        }

        let model_content = fs::read_to_string(&full_path)?;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut name = "unknown".to_string();

        for line in model_content.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }

            if line.starts_with("o ") {
                name = line[2..].trim().to_string();
            } else if line.starts_with("v ") {
                let parts: Vec<&str> = line[2..].trim().split_whitespace().collect();
                if parts.len() >= 3 {
                    for &part in &parts[0..3] {
                        if let Ok(coord) = part.parse::<f32>() {
                            vertices.push(coord);
                        }
                    }
                }
            } else if line.starts_with("f ") {
                let face_parts: Vec<&str> = line[2..].trim().split_whitespace().collect();
                let mut face_indices = Vec::new();

                for &face_part in &face_parts {
                    let idx_str = face_part.split('/').next().unwrap();
                    if let Ok(idx) = idx_str.parse::<i32>() {
                        face_indices.push(idx - 1);
                    }
                }

                for i in 1..(face_indices.len() - 1) {
                    indices.push(face_indices[0]);
                    indices.push(face_indices[i]);
                    indices.push(face_indices[i + 1]);
                }
            }
        }

        Ok(ModelData { name, vertices, indices })
    }
}
