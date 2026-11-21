use crate::core::sprite::Sprite;

pub trait Scene {
    fn new() -> Self;
    fn start(&self);
    fn update(&mut self, ctx: &mut crate::GameContext) {
        for sprite in self.get_sprites() {
            // sprite.update ...
        }
        
        self.on_update(ctx);
    }
    fn on_update(&mut self, ctx: &mut crate::GameContext);
    fn render(&mut self) {
        for sprite in self.get_sprites() {
            // sprite.draw ...
        }
        
        self.on_render();
    }
    fn on_render(&mut self);
    fn get_sprites(&mut self) -> &mut Vec<Sprite>;
}