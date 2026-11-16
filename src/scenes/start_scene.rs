use crate::{core::entity::Entity, scenes::scene::Scene};

pub struct StartScene {
    entities: Vec<Entity>,
}

impl Scene for StartScene {
    fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    fn start(&self) {
        // Load entities
    }

    fn on_update(&mut self, delta_time: f32) {
        // Update entities
    }

    fn on_render(&mut self) {
        // Render entities
    }
    
    fn get_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.entities
    }
}