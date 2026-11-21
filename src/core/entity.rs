use sola_raylib::{drawing::{RaylibDrawHandle}, math::Rectangle};

use crate::GameContext;

pub trait Entity {
    fn start(&self);
    fn update(&mut self, ctx: &mut GameContext);
    fn render(&mut self, batch: &mut RaylibDrawHandle);
    fn get_rect(&self) -> Rectangle;
}