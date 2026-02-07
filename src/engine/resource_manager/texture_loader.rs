use std::path::Path;

use image::GenericImageView;

pub struct TextureLoader {

}

impl TextureLoader {
    pub fn load(name: &str) -> (image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, u32, u32)
    {
        // Создаём путь
        let path = format!("./assets/textures/{}", name);
        let img = image::open(Path::new(&path))
            .expect("Failed to load texture")
            .flipv();
        
        let (width, height) = img.dimensions();
        let data = img.into_rgba8();

        (data, width, height)
    }
}