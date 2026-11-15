use ldtk_rust::{LayerInstance, Level, Project};
use sola_raylib::{color::Color, drawing::{RaylibDraw}, math::{Rectangle, Vector2}};

use crate::{core::asset_manager::get_asset_manager, globals::glob};

pub struct Tilemap {
    ldtk: Project,
    tile_size: i32,
}

impl Tilemap {
    pub fn new(tile_size: i32) -> Self {
        let ldtk = Project::new("assets/tilemaps/test_map.ldtk");
        Self {
            ldtk: ldtk,
            tile_size: tile_size,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Nothing...
    }
    
    pub fn draw_map(&self, batch: &mut impl RaylibDraw) {
        for level in self.ldtk.levels.iter() {
            self.draw_level(batch, level);
        }
    }
    
    pub fn draw_level(&self, batch: &mut impl RaylibDraw, level: &Level) {
        for layer in level.layer_instances.iter().flatten() {
            self.draw_layer(batch, layer);
        }
    }
    
    pub fn draw_layer(&self, batch: &mut impl RaylibDraw, layer: &LayerInstance) {
        for tile in layer.auto_layer_tiles.iter() {
            batch.draw_texture_pro(
                &get_asset_manager().textures[&glob().get_filename_from_path(layer.tileset_rel_path.clone().unwrap())],
                Rectangle::new(tile.src[0] as f32, tile.src[1] as f32, self.tile_size as f32, self.tile_size as f32),
                Rectangle::new(tile.px[0] as f32 + layer.px_offset_x as f32, tile.px[1] as f32 + layer.px_offset_y as f32, self.tile_size as f32, self.tile_size as f32),
                Vector2::zero(),
                0.,
                Color::WHITE
            );
        }
    }
}