use crate::core::sprite::Sprite;

pub trait Scene {
    fn new() -> Self;
    fn start(&self);
    fn update(&mut self, delta_time: f32) {
        for sprite in self.get_sprites() {
            // sprite.update ...
        }
        
        self.on_update(delta_time);
    }
    fn on_update(&mut self, delta_time: f32);
    fn render(&mut self) {
        for sprite in self.get_sprites() {
            // sprite.draw ...
        }
        
        self.on_render();
    }
    fn on_render(&mut self);
    fn get_sprites(&mut self) -> &mut Vec<Sprite>;
}