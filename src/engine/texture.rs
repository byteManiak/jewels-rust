use std::collections::HashMap;

use sdl2::{render::{TextureCreator, Texture}, video::WindowContext, image::LoadSurface, surface::Surface, pixels::{Palette, Color, PixelFormatEnum}};

struct TextureData<'a> {
    pub surface: Surface<'a>,
    pub texture: Texture<'a>
}

pub struct ColorPalette {
    pub(crate) palette: Palette,
    pub(crate) colors: [Color; 4]
}

pub struct TexManager<'a> {
    pub(crate) loader: &'a TextureCreator<WindowContext>,
    texture_data: HashMap<String, TextureData<'a>>,
    palettes: Vec<ColorPalette>,
    current_palette: usize
}

impl<'a> TexManager<'a> {
    pub(crate) fn new(creator: &'a TextureCreator<WindowContext>, palette: ColorPalette) -> Self {
        let mut palettes = Vec::new();
        palettes.push(palette);

        Self { loader: creator, texture_data: HashMap::new(), palettes, current_palette: 0 }
    }

    pub(crate) fn create_texture(&mut self, path: &str, name: &str) -> Result<(), String> {
        let name_str = name.to_string();

        if self.texture_data.contains_key(&name_str) {
            return Err(format!("A texture with the same name ({name_str}) has already been loaded"));
        }

        let mut surface = Surface::from_file(path)?;
        let palette = &self.palettes[self.current_palette];
        surface.set_palette(&palette.palette)?;

        let mut temp_surface = surface.convert_format(PixelFormatEnum::RGBA32)?;
        temp_surface.set_color_key(true, Color::RGBA(0xFF, 0xFF, 0xFF, 0))?;
        let texture = temp_surface.as_texture(self.loader).map_err(|e| e.to_string())?;

        self.texture_data.insert(name_str.clone(), TextureData { surface, texture });

        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture<'a>> {
        if let Some(data) = self.texture_data.get(name) {
            Some(&data.texture)
        } else {
            None
        }
    }

    pub fn update_textures(&mut self) -> Result<(), String> {
        for tex_data in self.texture_data.values_mut() {
            let surface = &mut tex_data.surface;
            let palette = &self.palettes[self.current_palette];
            surface.set_palette(&palette.palette)?;
            let mut temp_surface = surface.convert_format(PixelFormatEnum::RGBA32)?;
            temp_surface.set_color_key(true, Color::RGBA(0xFF, 0xFF, 0xFF, 0))?;
            tex_data.texture = temp_surface.as_texture(self.loader).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn add_palette(&mut self, palette: ColorPalette) {
        self.palettes.push(palette);
    }

    pub fn get_index_color(&self, index: usize) -> Color {
        self.palettes[self.current_palette].colors[index]
    }

    pub fn set_next_palette(&mut self) {
        self.current_palette += 1;
        self.current_palette %= self.palettes.len();

        let _ = self.update_textures();
    }
}