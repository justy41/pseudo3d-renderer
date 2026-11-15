use std::collections::HashMap;

use sola_raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::GetRandomValue, math::{Rectangle, Vector2}, texture::Texture2D};

use crate::core::{asset_manager::get_asset_manager, entity::Entity};

pub struct Layer {
    pub name: String,
    pub scale: f32,
    pub entities: Vec<Entity>,
    rand_x: i32,
}

impl Layer {
    pub fn new(name: &str, scale: f32) -> Self {
        Self {
            name: name.to_string(),
            scale,
            entities: Vec::new(),
            rand_x: unsafe{GetRandomValue(0, 400)},
        }
    }
    
    pub fn add_entity(&mut self, x: f32, y: f32, tex: &str) {
        println!("added entity");
        self.entities.push(Entity::new(x, y, tex));
    }
    
    pub fn draw_textures(&mut self, batch: &mut impl RaylibDraw, tint: Option<Color>) {
        for entity in &self.entities {
            match get_asset_manager().textures.get(&entity.texture_id) {
                //Some(tex) => batch.draw_texture(tex, entity.x as i32, entity.y as i32, tint.unwrap_or(Color::WHITE)),
                Some(tex) => batch.draw_texture_pro(
                    tex,
                    Rectangle::new(0., 0., tex.width as f32, tex.height as f32),
                    Rectangle::new(entity.x, entity.y, tex.width as f32, tex.height as f32),
                    Vector2::zero(),
                    entity.rotation,
                    tint.unwrap_or(Color::WHITE)
                ),
                None => eprintln!("Missing texture '{}'", entity.texture_id),
            }
        }
    }
}