use std::{
    collections::HashMap,
    fs,
    path::Path,
};

// This code wrote AI
// In future add FBX sdk or gltf parsing
 
#[derive(Clone, Debug)]
pub struct ModelData {
    pub vertices: Vec<[f32; 5]>,
    pub indices: Vec<i32>,
}

impl ModelData {
    
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
        let mut positions = Vec::new();
        let mut texcoords = Vec::new(); 
        let mut name = "unknown".to_string();

        for line in model_content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("o ") {
                name = line[2..].trim().to_string();
            } else if line.starts_with("v ") {
                let parts: Vec<&str> = line[2..].trim().split_whitespace().collect();
                if parts.len() >= 3 {
                    for &part in &parts[0..3] {
                        if let Ok(coord) = part.parse::<f32>() {
                            positions.push(coord);
                        }
                    }
                }
            } else if line.starts_with("vt ") {
                let parts: Vec<&str> = line[3..].trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    for &part in &parts[0..2] {
                        if let Ok(coord) = part.parse::<f32>() {
                            texcoords.push(coord);
                        }
                    }
                }
            }
        }

        let mut vertices: Vec<[f32; 5]> = Vec::new();
        let mut indices: Vec<i32> = Vec::new();
        let mut vert_cache: HashMap<(usize, usize), usize> = HashMap::new();

        for line in model_content.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with("f ") {
                continue;
            }

            let face_parts: Vec<&str> = line[2..].trim().split_whitespace().collect();
            if face_parts.len() < 3 {
                continue;
            }

            let mut fan_indices = Vec::new();
            for &face_part in &face_parts {
                let parts: Vec<&str> = face_part.split('/').collect();
                
                let v_str = parts.get(0).copied().unwrap_or("0");
                let vt_str = parts.get(1).copied(); 
                
                let v_idx = v_str.parse::<usize>().unwrap_or(1).saturating_sub(1);
                let vt_idx = vt_str.map_or(0usize, |s| s.parse::<usize>().unwrap_or(1).saturating_sub(1));

                let key = (v_idx, vt_idx);
                if let Some(&cached_idx) = vert_cache.get(&key) {
                    fan_indices.push(cached_idx as i32);
                    continue;
                }

                let pos_base = v_idx * 3;
                let uv_base = vt_idx * 2;
                
                let px = if pos_base + 2 < positions.len() {
                    [
                        positions.get(pos_base).copied().unwrap_or(0.0),
                        positions.get(pos_base + 1).copied().unwrap_or(0.0),
                        positions.get(pos_base + 2).copied().unwrap_or(0.0)
                    ]
                } else {
                    [0.0, 0.0, 0.0]
                };
                
                let uv = if uv_base + 1 < texcoords.len() {
                    [
                        texcoords.get(uv_base).copied().unwrap_or(0.0),
                        texcoords.get(uv_base + 1).copied().unwrap_or(0.0)
                    ]
                } else {
                    [0.0, 0.0]
                };

                let vert = [px[0], px[1], px[2], uv[0], uv[1]];
                let new_idx = vertices.len();
                vertices.push(vert);
                vert_cache.insert(key, new_idx);
                fan_indices.push(new_idx as i32);
            }

            if fan_indices.len() >= 3 {
                for i in 1..fan_indices.len().saturating_sub(1) {
                    indices.push(fan_indices[0]);
                    indices.push(fan_indices[i]);
                    indices.push(fan_indices[i + 1]);
                }
            }
        }

        Ok(ModelData { vertices, indices })
    }
}
