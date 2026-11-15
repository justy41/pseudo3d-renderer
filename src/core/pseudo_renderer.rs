use std::{rc::Rc};

use sola_raylib::{ffi::{GetScreenToWorld2D}, prelude::*};

use crate::{core::{editor::Editor, layer::{self, Layer}, tilemap::Tilemap}, globals::glob};

pub struct PseudoRenderer {
    pub layers: Vec<Layer>,
    pub camera: Camera2D,
    selected_layer: usize,
    editor: Rc<Editor>,
    tilemap: Tilemap,
}

impl PseudoRenderer {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, editor: Editor) -> Self {
        let layers = vec![
            Layer::new("Background", 0.8),
            Layer::new("Ground", 1.0),
            Layer::new("Foreground", 2.)
        ];
        
        Self {
            layers: layers,
            camera: Camera2D { offset: Vector2::new(0., 0.), target: Vector2::new(0., 0.), rotation: 0., zoom: 1. },
            selected_layer: 0,
            editor: Rc::new(editor),
            tilemap: Tilemap::new(8),
        }
    }
    
    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: f32) {
        // Processing input
        glob().mouse_position = rl.get_mouse_position();
        
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.camera.target.x -= 100. * delta_time;
        }
        else if rl.is_key_down(KeyboardKey::KEY_D) {
            self.camera.target.x += 100. * delta_time;
        }
        
        if glob().debug {
            if rl.is_key_pressed(KeyboardKey::KEY_UP) {
                self.selected_layer = self.selected_layer.saturating_sub(1);
                self.selected_layer = self.selected_layer.clamp(0, self.layers.len()-1);
                println!("Current texture layer: {}", self.layers[self.selected_layer].name);
            }
            else if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
                self.selected_layer = self.selected_layer.saturating_add(1);
                self.selected_layer = self.selected_layer.clamp(0, self.layers.len()-1);
                println!("Current texture layer: {}", self.layers[self.selected_layer].name);
            }
        }
        
        // MANAGE WORLD MOUSE POSITION
        let selected_camera = self.get_layer_camera(self.selected_layer);
        
            // Adding the letterbox scaling into consideration for the world mouse position
        {
            let mut g = glob();
            
            g.mouse_position.x = (g.mouse_position.x - (rl.get_screen_width() as f32 - (g.game_width as f32 * g.scale))*0.5)/g.scale;
            g.mouse_position.y = (g.mouse_position.y - (rl.get_screen_height() as f32 - (g.game_height as f32 * g.scale))*0.5)/g.scale;
        }
        
        // Adding the camera scaling into consideration
        glob().world_mouse_position = unsafe {
            GetScreenToWorld2D(glob().mouse_position.into(), selected_camera.into())
        }.into();
        
        // Updating tilemap
        self.tilemap.update(delta_time);
        
        // Updating editor
        if glob().debug {
            Rc::get_mut(&mut self.editor).unwrap().update(rl, delta_time, &selected_camera);
            self.editor.update_current_layer(rl, &mut self.layers[self.selected_layer]);
        }
    }
    
    pub fn after_update(&mut self, rl: &mut RaylibHandle, delta_time: f32) {
        if glob().debug {
            Rc::get_mut(&mut self.editor).unwrap().after_update(rl, delta_time, &mut self.layers[self.selected_layer]);
        }
    }
    
    pub fn render(&mut self, batch: &mut impl RaylibDraw) {
        for i in 0..self.layers.len() {
            let c = self.get_layer_camera(i);
                        
            // Draw camera canvas
            let mut mode2d = batch.begin_mode2D(c);
            
            if glob().debug {
                if i == self.selected_layer {
                    self.layers[i].draw_textures(&mut mode2d, None); // Draw selected layer normally
                }
                else {
                    self.layers[i].draw_textures(&mut mode2d, Some(Color::new(245, 245, 245, 50))); // Draw unselected layers semi-transparent
                }
                
                // Drawing the map on the Base Layer (the one that has the camera zoom = 1.0)
                if self.layers[i].scale == 1.0 {
                    self.tilemap.draw_map(&mut mode2d);
                }
            }
            else {
                self.layers[i].draw_textures(&mut mode2d, None); // Draw all layers normally
                
                // Drawing the map on the Base Layer (the one that has the camera zoom = 1.0)
                if self.layers[i].scale == 1.0 {
                    self.tilemap.draw_map(&mut mode2d);
                }
            }
        }
        
        // Debug rendering the editor
        if glob().debug {
            Rc::get_mut(&mut self.editor).unwrap().render(batch); // Draw using the normal batch (for showing textures in the corner)
            
            let c = self.get_layer_camera(self.selected_layer);
            let mut mode2d = batch.begin_mode2D(c);
            
            Rc::get_mut(&mut self.editor).unwrap().render_mode2d(&mut mode2d); // Draw using the camera zoom (for texture placement)
        }
    }
    
    fn get_layer_camera(&self, index: usize) -> Camera2D {
        let gw = glob().game_width;
        let gh = glob().game_height;
        Camera2D {
            offset: Vector2::new(0., 0.),
            target: self.camera.target,
            rotation: 0.,
            zoom: self.camera.zoom * self.layers[index].scale
        }
    }
}
