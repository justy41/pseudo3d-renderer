use std::{collections::HashMap, fs, sync::{Mutex, MutexGuard, OnceLock}};
use sola_raylib::{RaylibHandle, RaylibThread, texture::Texture2D};

#[derive(Debug)]
pub struct AssetManager {
    pub textures: HashMap<String, Texture2D>,
    texture_names: Vec<String>,
}

static INSTANCE: OnceLock<Mutex<AssetManager>> = OnceLock::new();

pub fn get_asset_manager() -> MutexGuard<'static, AssetManager> {
    INSTANCE.get_or_init(|| Mutex::new(AssetManager::new())).lock().unwrap()
}

impl AssetManager {
    pub fn new() -> Self {
        let textures = HashMap::new();
        
        Self {
            textures: textures,
            texture_names: Vec::new(),
        }
    }
    
    pub fn get_pngs_from_folder(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, folder_path: &str) -> Result<(), std::io::Error> {
        if let Ok(entries) = fs::read_dir(folder_path) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext.eq_ignore_ascii_case("png") {
                            self.textures.insert(path.file_name().unwrap().to_str().unwrap().to_string(), rl.load_texture(thread, path.to_str().unwrap()).unwrap());
                            self.texture_names.push(path.file_name().unwrap().to_str().unwrap().to_string());
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn get_texture_names(&self) -> Vec<String> {
        return self.texture_names.clone();
    }
    
    pub fn unload_textures(&mut self) {
        // Texture2D implements the Drop trait so it will automatically unload textures
        self.textures.clear();
    }
}