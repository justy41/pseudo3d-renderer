use crate::{GameContext, core::sprite::Sprite, scenes::scene::Scene};

pub struct StartScene {
    sprites: Vec<Sprite>,
}

impl Scene for StartScene {
    fn new() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }

    fn start(&self) {
        // Load sprites
    }

    fn on_update(&mut self, ctx: &mut GameContext) {
        // Update sprites
    }

    fn on_render(&mut self) {
        // Render entities
    }
    
    fn get_sprites(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
    }
}