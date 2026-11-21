use sola_raylib::drawing::RaylibDrawHandle;

use crate::{GameContext, core::{entity::Entity, sprite::Sprite}, entities::player::Player, scenes::scene::Scene};

pub struct StartScene {
    entities: Vec<Box<dyn Entity>>,
}

impl Scene for StartScene {
    fn new() -> Self {
        let mut entities: Vec<Box<dyn Entity>> = Vec::new();
        
        Self {
            entities: entities,
        }
    }

    fn on_start(&mut self) {
        // ENTITIES
        self.entities.push(Box::new(Player::new()));
    }

    fn on_update(&mut self, ctx: &mut GameContext) {
        // Update entities
    }

    fn on_render(&mut self, batch: &mut RaylibDrawHandle) {
        // Render entities
    }
    
    fn get_entities(&mut self) -> &mut Vec<Box<dyn Entity>> {
        &mut self.entities
    }
}