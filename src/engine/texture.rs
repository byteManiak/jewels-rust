use std::collections::HashMap;

use sdl2::{render::{TextureCreator, Texture}, video::WindowContext, image::LoadSurface, surface::Surface, pixels::{Palette, Color, PixelFormatEnum}};

struct TextureData<'a> {
    pub surface: Surface<'a>,
    pub texture: Texture<'a>
}

pub struct TexManager<'a> {
    loader: &'a TextureCreator<WindowContext>,
    texture_data: HashMap<String, TextureData<'a>>,
    palettes: HashMap<String, Palette>,
    current_palette: String
}

impl<'a> TexManager<'a> {
    pub(crate) fn new(creator: &'a TextureCreator<WindowContext>, palette: Palette) -> Self {
        let mut palettes = HashMap::new();
        palettes.insert("default".to_string(), palette);
        Self { loader: creator, texture_data: HashMap::new(), palettes, current_palette: "default".to_string() }
    }

    pub(crate) fn create_texture(&mut self, path: &str, name: &str) -> Result<(), String> {
        let name_str = name.to_string();

        if self.texture_data.contains_key(&name_str) {
            return Err(format!("A texture with the same name ({name_str}) has already been loaded"));
        }

        let mut surface = Surface::from_file(path)?;
        surface.set_palette(&self.get_current_palette())?;

        let mut temp_surface = surface.convert_format(PixelFormatEnum::RGBA32)?;
        temp_surface.set_color_key(true, Color::RGBA(0xFF, 0xFF, 0xFF, 0))?;
        let texture = temp_surface.as_texture(&self.loader).map_err(|e| e.to_string())?;

        self.texture_data.insert(name_str.clone(), TextureData { surface, texture });

        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture<'a>> {
        if let Some(data) = self.texture_data.get(name) {
            return Some(&data.texture);
        } else {
            return None;
        }
    }

    pub fn add_palette(&mut self, name: &str, palette: Palette) {
        self.palettes.insert(name.to_string(), palette);
    }

    fn get_current_palette(&self) -> &Palette {
        self.palettes.get(&self.current_palette).unwrap()
    }

    pub fn set_palette(&mut self, name: &str) {
        self.current_palette = name.to_string();
    }
}