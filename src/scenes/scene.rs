use sola_raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, math::Vector2};

use crate::{GameContext, core::entity::Entity, globals::glob};

pub trait Scene {
    fn new() -> Self;
    fn on_start(&mut self);
    fn update(&mut self, ctx: &mut GameContext) {
        for entity in self.get_entities() {
            // entity.update ...
        }
        
        self.on_update(ctx);
    }
    fn on_update(&mut self, ctx: &mut GameContext);
    fn render(&mut self, batch: &mut impl RaylibDraw) {
        for entity in self.get_entities() {
            if glob().debug {
                batch.draw_rectangle_pro(entity.get_rect(), Vector2::zero(), 0., Color::RED);
            }
        }
        
        self.on_render(batch);
    }
    fn on_render(&mut self, batch: &mut impl RaylibDraw);
    fn get_entities(&mut self) -> &mut Vec<Box<dyn Entity>>;
}