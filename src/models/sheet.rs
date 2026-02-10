use crate::models::frame::Frame;

pub struct Sheet {
    pub name: String,
    pub uploaded_at: u64,
    pub height: u32,
    pub width: u32,
    pub columns: u32,
    pub rows: u32,
    pub image_type: ImageType,
    pub pixels: Vec<u8>,
    pub frames: Vec<Frame>,
}

pub enum ImageType {
    PNG,
    JPG,
    GIF,
    BMP,
    SVG,
    WEBP,
}
