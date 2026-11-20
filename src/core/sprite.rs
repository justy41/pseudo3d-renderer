use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub texture_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
}

impl Sprite {
    pub fn new(x: f32, y: f32, width: f32, height: f32, texture_id: &str) -> Self {
        Self {
            texture_id: texture_id.to_string(),
            x: x,
            y: y,
            width: width,
            height: height,
            rotation: 0.,
        }
    }
}
