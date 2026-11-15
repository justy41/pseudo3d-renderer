mod core;
mod globals;
use sola_raylib::{prelude::*};
use crate::{core::{asset_manager::get_asset_manager, editor::Editor, pseudo_renderer::PseudoRenderer}, globals::glob};

/*
TODO:
    Saving placed Textures (that means + Layers)
    
    Fix Background layer pixel jitter problem
*/       

fn main() {
    let sw = glob().screen_width; // Window Width
    let sh = glob().screen_height; // Window Height
    let gw = glob().game_width; // In-Game Width Resolution
    let gh = glob().game_height; // In-Game Height Resolution
    let mut scale = 0.0; // Scaling ratio
    
    let (mut rl, thread) = sola_raylib::init()
        .size(sw as i32, sh as i32)
        .title("renderer 3d")
        .resizable()
        .build();
    rl.set_target_fps(60);
    let mut delta_time: f32;
    
    let mut target = rl.load_render_texture(&thread, gw, gh).unwrap();
    target.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);
    
    get_asset_manager().get_pngs_from_folder(&mut rl, &thread, "assets").unwrap();
    get_asset_manager().get_pngs_from_folder(&mut rl, &thread, "assets/decorations").unwrap();
    get_asset_manager().get_pngs_from_folder(&mut rl, &thread, "assets/tilesets").unwrap();
    
    let mut editor = Editor::new();
    let mut pseudo_renderer = PseudoRenderer::new(&mut rl, &thread, editor);

    while !rl.window_should_close() {
        // UPDATE
        scale = (rl.get_screen_width() as f32/gw as f32).min(rl.get_screen_height() as f32/gh as f32);
        glob().scale = scale;
        
        delta_time = rl.get_frame_time();
        if rl.is_key_pressed(KeyboardKey::KEY_GRAVE) {
            let mut debug = glob().debug;
            debug = !debug;
            glob().debug = debug;
            println!("Debug Mode: {}", debug);
        }
        
        pseudo_renderer.update(&mut rl, delta_time);
        pseudo_renderer.after_update(&mut rl, delta_time);
        
        // DRAW on render texture
        {
            let mut batch = rl.begin_texture_mode(&thread, &mut target);
            batch.clear_background(Color::WHITE);
            pseudo_renderer.render(&mut batch);
        }
        
        // DRAW render texture on the screen
        let mut batch = rl.begin_drawing(&thread);
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