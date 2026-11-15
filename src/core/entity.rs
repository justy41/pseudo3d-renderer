pub struct Entity {
    pub texture_id: String,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl Entity {
    pub fn new(x: f32, y: f32, texture_id: &str) -> Self {
        Self {
            texture_id: texture_id.to_string(),
            x: x,
            y: y,
            rotation: 0.,
        }
    }
}