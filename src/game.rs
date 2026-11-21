use sola_raylib::drawing::RaylibDrawHandle;

use crate::{GameContext, scenes::{scene::Scene, start_scene::StartScene}};

pub struct Game {
    start_scene: StartScene,
}

impl Game {
    pub fn new() -> Self {
        let mut start_scene = StartScene::new();
        start_scene.on_start();
        
        Self {
            start_scene: start_scene,
        }
    }
    
    pub fn update(&mut self, ctx: &mut GameContext) {
        self.start_scene.update(ctx);
    }
    
    pub fn render(&mut self, batch: &mut RaylibDrawHandle) {
        self.start_scene.render(batch);
    }
}