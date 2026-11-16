use crate::core::entity::Entity;

pub trait Scene {
    fn new() -> Self;
    fn start(&self);
    fn update(&mut self, delta_time: f32) {
        for entity in self.get_entities() {
            // entity.update ...
        }
        
        self.on_update(delta_time);
    }
    fn on_update(&mut self, delta_time: f32);
    fn render(&mut self) {
        for entity in self.get_entities() {
            // entity.draw ...
        }
        
        self.on_render();
    }
    fn on_render(&mut self);
    fn get_entities(&mut self) -> &mut Vec<Entity>;
}