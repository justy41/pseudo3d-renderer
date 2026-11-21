use std::{sync::{Mutex, MutexGuard, OnceLock}};

use sola_raylib::math::Vector2;

#[derive(Debug)]
pub struct Globals {
    pub mouse_position: Vector2,
    pub world_mouse_position: Vector2,
    pub debug: bool,
    pub scale: f32,
    pub screen_width: u32,
    pub screen_height: u32,
    pub game_width: u32,
    pub game_height: u32,
    pub pix_per_unit: i32,
}

static GLOBAL: OnceLock<Mutex<Globals>> = OnceLock::new();

pub fn glob() -> MutexGuard<'static, Globals> {
    GLOBAL.get_or_init(|| Mutex::new(Globals::new())).lock().unwrap()
}

impl Globals {
    pub fn new() -> Self {
        Self {
            mouse_position: Vector2::new(0., 0.),
            world_mouse_position: Vector2::new(0., 0.),
            debug: false,
            scale: 0.,
            screen_width: 1280,
            screen_height: 720,
            game_width: 2560,
            game_height: 1440,
            pix_per_unit: 8,
        }
    }
    
    pub fn get_filename_from_path(&self, path: String) -> String {
        let mut result = String::from("");
        for c in path.chars().into_iter().rev() {
            if c != '/' {
                result += &c.to_string().clone();
            }
            else {
                break;
            }
        }
        
        result = result.chars().rev().collect();
        result
    }
}