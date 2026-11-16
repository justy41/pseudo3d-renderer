mod Game;
mod scenes;
mod core;
mod globals;
use std::rc::Rc;

use sola_raylib::{prelude::*};
use crate::{core::{asset_manager::get_asset_manager, editor::Editor, pseudo_renderer::PseudoRenderer}, globals::glob};

/*
TODO:
    Check scene system
    Saving placed Textures (that means + Layers)
    
    Fix Background layer pixel jitter problem
*/       

pub struct GameContext {
    rl: RaylibHandle,
    thread: RaylibThread,
    editor: Editor,
    dt: f32,
}

impl GameContext {
    pub fn new(sw: u32, sh: u32) -> Self {
        let (mut rl, thread) = sola_raylib::init()
            .size(sw as i32, sh as i32)
            .title("renderer 3d")
            .resizable()
            .build();
        rl.set_target_fps(60);
        
        // Order matters here
        // This...
        get_asset_manager().get_pngs_from_folder(&mut rl, &thread, "assets").unwrap();
        get_asset_manager().get_pngs_from_folder(&mut rl, &thread, "assets/decorations").unwrap();
        get_asset_manager().get_pngs_from_folder(&mut rl, &thread, "assets/tilesets").unwrap();
        
        let mut delta_time: f32 = 0.0;
        let mut editor = Editor::new(); // ... before this!
        
        let mut ctx = GameContext {
            rl: rl,
            thread: thread,
            editor: editor,
            dt: delta_time,
        };
        
        ctx
    }
}

fn main() {
    let sw = glob().screen_width; // Window Width
    let sh = glob().screen_height; // Window Height
    let gw = glob().game_width; // In-Game Width Resolution
    let gh = glob().game_height; // In-Game Height Resolution
    let mut scale; // Scaling ratio
    
    let mut ctx = GameContext::new(sw, sh);
    let mut pseudo_renderer = PseudoRenderer::new(&mut ctx);
    
    let mut target = ctx.rl.load_render_texture(&ctx.thread, gw, gh).unwrap();
    target.set_texture_filter(&ctx.thread, TextureFilter::TEXTURE_FILTER_POINT);

    while !ctx.rl.window_should_close() {
        // UPDATE
        scale = (ctx.rl.get_screen_width() as f32/gw as f32).min(ctx.rl.get_screen_height() as f32/gh as f32);
        glob().scale = scale;
        
        ctx.dt = ctx.rl.get_frame_time();
        if ctx.rl.is_key_pressed(KeyboardKey::KEY_GRAVE) {
            let mut debug = glob().debug;
            debug = !debug;
            glob().debug = debug;
            println!("Debug Mode: {}", debug);
        }
        
        pseudo_renderer.update(&mut ctx);
        pseudo_renderer.after_update(&mut ctx);
        
        // DRAW on render texture
        {
            let mut batch = ctx.rl.begin_texture_mode(&ctx.thread, &mut target);
            batch.clear_background(Color::WHITE);
            pseudo_renderer.render(&mut batch, &mut ctx.editor);
        }
        
        // DRAW render texture on the screen
        let mut batch = ctx.rl.begin_drawing(&ctx.thread);
        batch.clear_background(Color::BLACK);
        
        batch.draw_texture_pro(
            target.texture(),
            Rectangle::new(0.0, 0.0, target.texture.width as f32, -target.texture.height as f32),
            Rectangle::new((batch.get_screen_width() as f32 - (gw as f32 * scale))*0.5, (batch.get_screen_height() as f32 - (gh as f32 * scale))*0.5, gw as f32 * scale, gh as f32 *scale),
            Vector2::new(0., 0.),
            0.0,
            Color::WHITE
        );
    }
    
    get_asset_manager().unload_textures();
}