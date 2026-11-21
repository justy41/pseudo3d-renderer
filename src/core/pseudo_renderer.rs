use sola_raylib::{ffi::GetScreenToWorld2D, prelude::*};

use crate::{
    GameContext,
    core::{
        editor::Editor,
        layer::{self, Layer},
        tilemap::Tilemap,
    },
    globals::glob,
};

pub struct PseudoRenderer {
    pub layers: Vec<Layer>,
    pub camera: Camera2D,
    selected_layer: usize,
    tilemap: Tilemap,
}

impl PseudoRenderer {
    pub fn new(ctx: &mut GameContext) -> Self {
        let layers = vec![
            Layer::new("Background", 0.8),
            Layer::new("Ground", 1.0),
            Layer::new("Foreground", 2.),
        ];

        let mut renderer = Self {
            layers,
            camera: Camera2D {
                offset: Vector2::new(0., 0.),
                target: Vector2::new(0., 0.),
                rotation: 0.,
                zoom: 1.,
            },
            selected_layer: 0,
            tilemap: Tilemap::new(8),
        };

        renderer.load_layers();

        renderer
    }

    pub fn save_layers(&self) {
        for layer in &self.layers {
            match layer.save() {
                Ok(path) => println!("Saved layer '{}' to {}", layer.name, path.display()),
                Err(error) => eprintln!("Failed to save layer '{}': {}", layer.name, error),
            }
        }
    }

    pub fn load_layers(&mut self) {
        for layer in &mut self.layers {
            match layer.load() {
                Ok(_) => println!("Loaded saved entities for layer '{}'", layer.name),
                Err(error) => eprintln!("Failed to load layer '{}': {}", layer.name, error),
            }
        }
    }

    pub fn update(&mut self, ctx: &mut GameContext) {
        // Processing input
        glob().mouse_position = ctx.rl.get_mouse_position();

        if ctx.rl.is_key_pressed(KeyboardKey::KEY_F5) {
            self.save_layers();
        }

        if ctx.rl.is_key_down(KeyboardKey::KEY_A) {
            self.camera.target.x -= 1000. * ctx.dt;
        } else if ctx.rl.is_key_down(KeyboardKey::KEY_D) {
            self.camera.target.x += 1000. * ctx.dt;
        }
        if ctx.rl.is_key_down(KeyboardKey::KEY_W) {
            self.camera.target.y -= 1000. * ctx.dt;
        } else if ctx.rl.is_key_down(KeyboardKey::KEY_S) {
            self.camera.target.y += 1000. * ctx.dt;
        }

        if glob().debug {
            if ctx.rl.is_key_pressed(KeyboardKey::KEY_UP) {
                self.selected_layer = self.selected_layer.saturating_sub(1);
                self.selected_layer = self.selected_layer.clamp(0, self.layers.len() - 1);
                println!(
                    "Current texture layer: {}",
                    self.layers[self.selected_layer].name
                );
            } else if ctx.rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
                self.selected_layer = self.selected_layer.saturating_add(1);
                self.selected_layer = self.selected_layer.clamp(0, self.layers.len() - 1);
                println!(
                    "Current texture layer: {}",
                    self.layers[self.selected_layer].name
                );
            }
        }

        // MANAGE WORLD MOUSE POSITION
        let selected_camera = self.get_layer_camera(self.selected_layer);

        // Adding the letterbox scaling into consideration for the world mouse position
        {
            let mut g = glob();

            g.mouse_position.x = (g.mouse_position.x
                - (ctx.rl.get_screen_width() as f32 - (g.game_width as f32 * g.scale)) * 0.5)
                / g.scale;
            g.mouse_position.y = (g.mouse_position.y
                - (ctx.rl.get_screen_height() as f32 - (g.game_height as f32 * g.scale)) * 0.5)
                / g.scale;
        }

        // Adding the camera scaling into consideration
        glob().world_mouse_position =
            unsafe { GetScreenToWorld2D(glob().mouse_position.into(), selected_camera.into()) }
                .into();

        // Updating tilemap
        self.tilemap.update(ctx.dt);

        // Updating editor
        if glob().debug {
            ctx.editor.update(&mut ctx.rl, ctx.dt, &selected_camera);
            ctx.editor
                .update_current_layer(&mut ctx.rl, &mut self.layers[self.selected_layer]);
        }
    }

    pub fn after_update(&mut self, ctx: &mut GameContext) {
        if glob().debug {
            ctx.editor
                .after_update(&mut ctx.rl, ctx.dt, &mut self.layers[self.selected_layer]);
        }
    }

    pub fn render(&mut self, batch: &mut impl RaylibDraw, editor: &mut Editor) {
        for i in 0..self.layers.len() {
            let c = self.get_layer_camera(i);

            // Draw camera canvas
            let mut mode2d = batch.begin_mode2D(c);

            if glob().debug {
                if i == self.selected_layer {
                    self.layers[i].draw_textures(&mut mode2d, None); // Draw selected layer normally
                } else {
                    self.layers[i].draw_textures(&mut mode2d, Some(Color::new(245, 245, 245, 50))); // Draw unselected layers semi-transparent
                }

                // Drawing the map on the Base Layer (the one that has the camera zoom = 1.0)
                if self.layers[i].scale == 1.0 {
                    self.tilemap.draw_map(&mut mode2d);
                }
            } else {
                self.layers[i].draw_textures(&mut mode2d, None); // Draw all layers normally

                // Drawing the map on the Base Layer (the one that has the camera zoom = 1.0)
                if self.layers[i].scale == 1.0 {
                    self.tilemap.draw_map(&mut mode2d);
                }
            }
        }

        // Debug rendering the editor
        if glob().debug {
            editor.render(batch); // Draw using the normal batch (for showing textures in the corner)

            let c = self.get_layer_camera(self.selected_layer);
            let mut mode2d = batch.begin_mode2D(c);

            editor.render_mode2d(&mut mode2d); // Draw using the camera zoom (for texture placement)
        }
    }

    fn get_layer_camera(&self, index: usize) -> Camera2D {
        let gw = glob().game_width;
        let gh = glob().game_height;
        Camera2D {
            offset: Vector2::new(0., 0.),
            target: self.camera.target,
            rotation: 0.,
            zoom: self.camera.zoom * self.layers[index].scale,
        }
    }
}
