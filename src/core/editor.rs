use sola_raylib::{RaylibHandle, camera::Camera2D, color::Color, drawing::{RaylibDraw}, ffi::MouseButton};

use crate::{core::{asset_manager::get_asset_manager, layer::{self, Layer}}, globals::glob};

pub struct Editor {
    selected_texture: i32,
    selected_texture_name: String,
}

impl Editor {
    pub fn new() -> Self {        
        Self {
            selected_texture: 0,
            selected_texture_name: get_asset_manager().get_texture_names()[0].to_string(),
        }
    }
    
    // Handles selecting textures with scroll wheel
    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: f32, camera: &Camera2D) {
        if rl.get_mouse_wheel_move() > 0. {
            self.selected_texture += 1;
            self.selected_texture = self.selected_texture.clamp(0, (get_asset_manager().get_texture_names().len()-1) as i32);
        }
        else if rl.get_mouse_wheel_move() < 0. {
            self.selected_texture -= 1;
            self.selected_texture = self.selected_texture.clamp(0, (get_asset_manager().get_texture_names().len()-1) as i32);
        }
        
        self.selected_texture_name = get_asset_manager().get_texture_names()[self.selected_texture as usize].clone()
    }
    
    // Handles adding/destroying textures with mouse buttons
    pub fn update_current_layer(&self, rl: &mut RaylibHandle, selected_layer: &mut Layer) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = glob().world_mouse_position;
            let texture_name = get_asset_manager().get_texture_names()[self.selected_texture as usize].to_string();
            selected_layer.add_sprite(mouse_pos.x.trunc(), mouse_pos.y.trunc(), &texture_name);
        }
        
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            selected_layer.remove_sprite();
        }
    }
    
    // Handles texture deletion after update (so it doesnt conflict with the for loops)
    pub fn after_update(&mut self, rl: &mut RaylibHandle, delta_time: f32, selected_layer: &mut Layer) {
        selected_layer.process_sprite_deletion();
    }
    
    // Render the selected texture in the corner
    pub fn render(&mut self, batch: &mut impl RaylibDraw) {
        batch.draw_texture(&get_asset_manager().textures[&self.selected_texture_name], 0, 0, Color::new(255, 255, 255, 50));
    }
    
    // Render the selected texture at the mouse position
    pub fn render_mode2d(&mut self, batch: &mut impl RaylibDraw) {
        let wmp = glob().world_mouse_position;
        
        batch.draw_texture(&get_asset_manager().textures[&self.selected_texture_name], wmp.x as i32, wmp.y as i32, Color::new(255, 255, 255, 35));
    }
}