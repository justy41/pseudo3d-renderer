use sola_raylib::{RaylibHandle, check_collision_point_poly, color::Color, drawing::RaylibDraw, ffi::CheckCollisionPointRec, math::{Rectangle, Vector2}};

use crate::{core::{asset_manager::get_asset_manager, entity::Entity}, globals::glob};

pub struct Layer {
    pub name: String,
    pub scale: f32,
    pub entities: Vec<Entity>,
    pub remove_entity_indexes: Vec<usize>,
}

impl Layer {
    pub fn new(name: &str, scale: f32) -> Self {
        Self {
            name: name.to_string(),
            scale,
            entities: Vec::new(),
            remove_entity_indexes: Vec::new(),
        }
    }
    
    pub fn add_entity(&mut self, x: f32, y: f32, tex: &str) {
        println!("added entity");
        let texture_w = get_asset_manager().textures[tex].width;
        let texture_h = get_asset_manager().textures[tex].height;
        self.entities.push(Entity::new(x, y, texture_w as f32, texture_h as f32, tex));
    }
    
    pub fn remove_entity(&mut self) {
        for (i, entity) in self.entities.iter_mut().enumerate() {
            let wmp = glob().world_mouse_position;
            if Rectangle::new(entity.x, entity.y, entity.width, entity.height).check_collision_point_rec(wmp) {
                self.remove_entity_indexes.push(i);
                break;
            }
        }
    }
    
    pub fn process_entity_deletion(&mut self) {
        if !self.remove_entity_indexes.is_empty() {
                for i in self.remove_entity_indexes.iter() {
                    self.entities.swap_remove(*i);
                }
            
            self.remove_entity_indexes.clear();
        }
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