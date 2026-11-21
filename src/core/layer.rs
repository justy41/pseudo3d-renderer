use sola_raylib::{
    color::Color,
    drawing::RaylibDraw,
    math::{Rectangle, Vector2},
};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    core::{asset_manager::get_asset_manager, sprite::Sprite},
    globals::glob,
};

pub struct Layer {
    pub name: String,
    pub scale: f32,
    pub sprites: Vec<Sprite>,
    pub remove_sprite_indexes: Vec<usize>,
}

impl Layer {
    pub fn new(name: &str, scale: f32) -> Self {
        Self {
            name: name.to_string(),
            scale,
            sprites: Vec::new(),
            remove_sprite_indexes: Vec::new(),
        }
    }

    pub fn save(&self) -> io::Result<PathBuf> {
        let directory = Path::new("assets").join("saves");
        fs::create_dir_all(&directory)?;

        let filename = self
            .name
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect::<String>();
        let path = directory.join(format!("{filename}.json"));
        let json = serde_json::to_vec_pretty(&self.sprites)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        fs::write(&path, json)?;
        Ok(path)
    }

    pub fn load(&mut self) -> io::Result<()> {
        let filename = self
            .name
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect::<String>();
        let path = Path::new("assets")
            .join("saves")
            .join(format!("{filename}.json"));

        if !path.exists() {
            return Ok(());
        }

        let json = fs::read_to_string(&path)?;
        self.sprites = serde_json::from_str(&json)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        self.remove_sprite_indexes.clear();
        Ok(())
    }

    pub fn add_sprite(&mut self, x: f32, y: f32, tex: &str) {
        println!("added sprite");
        let texture_w = get_asset_manager().textures[tex].width;
        let texture_h = get_asset_manager().textures[tex].height;
        self.sprites
            .push(Sprite::new(x, y, texture_w as f32, texture_h as f32, tex));
    }

    pub fn remove_sprite(&mut self) {
        for (i, sprite) in self.sprites.iter_mut().enumerate() {
            let wmp = glob().world_mouse_position;
            if Rectangle::new(sprite.x, sprite.y, sprite.width, sprite.height)
                .check_collision_point_rec(wmp)
            {
                self.remove_sprite_indexes.push(i);
                break;
            }
        }
    }

    pub fn process_sprite_deletion(&mut self) {
        if !self.remove_sprite_indexes.is_empty() {
            for i in self.remove_sprite_indexes.iter() {
                self.sprites.swap_remove(*i);
            }

            self.remove_sprite_indexes.clear();
        }
    }

    pub fn draw_textures(&mut self, batch: &mut impl RaylibDraw, tint: Option<Color>) {
        for sprite in &self.sprites {
            match get_asset_manager().textures.get(&sprite.texture_id) {
                //Some(tex) => batch.draw_texture(tex, sprite.x as i32, sprite.y as i32, tint.unwrap_or(Color::WHITE)),
                Some(tex) => batch.draw_texture_pro(
                    tex,
                    Rectangle::new(0., 0., tex.width as f32, tex.height as f32),
                    Rectangle::new(sprite.x, sprite.y, tex.width as f32, tex.height as f32),
                    Vector2::zero(),
                    sprite.rotation,
                    tint.unwrap_or(Color::WHITE),
                ),
                None => eprintln!("Missing texture '{}'", sprite.texture_id),
            }
        }
    }
}
