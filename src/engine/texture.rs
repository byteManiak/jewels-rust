use std::collections::HashMap;

use sdl2::{render::{TextureCreator, Texture}, video::WindowContext, image::LoadSurface, surface::Surface};

pub struct TexManager<'a> {
    loader: &'a TextureCreator<WindowContext>,
    textures: HashMap<String, Texture<'a>>
}

impl<'a> TexManager<'a> {
    pub fn new(creator: &'a TextureCreator<WindowContext>) -> Self {
        Self { loader: creator, textures: HashMap::new() }
    }

    pub fn create_texture(&mut self, path: &str, name: &str) -> Result<(), String> {
        let name_str = name.to_string();

        if self.textures.contains_key(&name_str) {
            return Err(format!("A texture with the same name ({name_str}) has already been loaded"));
        }

        let surface = Surface::from_file(path)?;
        let texture = self.loader.create_texture_from_surface(surface).map_err(|e| e.to_string())?;
        self.textures.insert(name_str, texture);

        Ok(())
    }

    pub fn add_texture(&mut self, name: &str, texture: Texture<'a>) {
        self.textures.insert(name.to_string(), texture);
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture<'a>> {
        self.textures.get(name)
    }
}