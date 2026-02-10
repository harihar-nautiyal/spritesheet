use crate::models::sheet::ImageType;
use crate::models::sheet::Sheet;
use std::collections::HashMap;

impl Sheet {
    pub fn new(
        name: String,
        height: u32,
        width: u32,
        image_type: ImageType,
        pixels: Vec<u8>,
    ) -> Self {
        todo!("Implement sheet detection system")
    }

    pub fn most_used_colors(height: u32, width: u32, pixels: Vec<u8>) -> ([u8; 4], f32) {
        let mut map: HashMap<u32, u32> = HashMap::new();

        let total_pixels = (width * height) as f32;

        for chunk in pixels.chunks_exact(4) {
            let key = (chunk[0] as u32) << 24
                | (chunk[1] as u32) << 16
                | (chunk[2] as u32) << 8
                | (chunk[3] as u32);

            *map.entry(key).or_insert(0) += 1;
        }

        let (best_key, best_count) = map.into_iter().max_by_key(|(_, c)| *c).unwrap();

        let color = [
            ((best_key >> 24) & 0xFF) as u8,
            ((best_key >> 16) & 0xFF) as u8,
            ((best_key >> 8) & 0xFF) as u8,
            (best_key & 0xFF) as u8,
        ];

        let percent = (best_count as f32 / total_pixels) * 100.0;

        (color, percent)
    }
}
