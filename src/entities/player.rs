use sola_raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, math::{Rectangle, Vector2}};

use crate::core::entity::Entity;

pub struct Player {
    pub position: Vector2,
    pub velocity: Vector2,
    pub rect: Rectangle,
    pub rotation: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(0., 0.),
            velocity: Vector2::new(0., 0.),
            rect: Rectangle::new(0., 0., 16., 16.),
            rotation: 0.,
        }
    }
}

impl Entity for Player {
    fn start(&self) {
        // Nothing...
    }

    fn update(&mut self, ctx: &mut crate::GameContext) {
        self.position.x += self.velocity.x * ctx.dt;
        self.position.y += self.velocity.y * ctx.dt;
    }

    fn render(&mut self, batch: &mut RaylibDrawHandle) {
        batch.draw_rectangle_pro(self.rect, Vector2::zero(), self.rotation, Color::WHITE);
    }

    fn get_rect(&self) -> Rectangle {
        self.rect
    }
}