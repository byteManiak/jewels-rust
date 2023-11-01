use sdl2::{render::{TextureCreator, WindowCanvas, Texture}, video::WindowContext, mixer::Channel, pixels::{Palette, Color, PixelFormatEnum}, rect::Rect};

use super::{sound::SoundManager, texture::TexManager};

pub struct AssetManager<'a> {
    tex_manager: TexManager<'a>,
    snd_manager: SoundManager<'a>
}

pub fn u32_palette(c1: u32, c2: u32, c3: u32, c4:u32) -> Palette {
    let color1 = Color::RGBA(
        ((c1 & 0xFF0000) >> 16) as u8,
        ((c1 & 0xFF00) >> 8) as u8,
        ((c1 & 0xFF)) as u8,
        0,
    );
    let color2 = Color::RGBA(
        ((c2 & 0xFF0000) >> 16) as u8,
        ((c2 & 0xFF00) >> 8) as u8,
        ((c2 & 0xFF)) as u8,
        0,
    );
    let color3 = Color::RGBA(
        ((c3 & 0xFF0000) >> 16) as u8,
        ((c3 & 0xFF00) >> 8) as u8,
        ((c3 & 0xFF)) as u8,
        0,
    );
    let color4 = Color::RGBA(
        ((c4 & 0xFF0000) >> 16) as u8,
        ((c4 & 0xFF00) >> 8) as u8,
        ((c4 & 0xFF)) as u8,
        0,
    );

    Palette::with_colors(&[color1, color2, color3, color4]).unwrap()
}

impl<'a> AssetManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, palette: Palette, width: u32, height: u32) -> Self {
        let tex_manager = TexManager::new(texture_creator, palette, width, height);
        let snd_manager = SoundManager::new();
        Self {tex_manager, snd_manager}
    }

    pub fn load_sound(&mut self, path: &str, name: &str) -> Result<(), String> {
        self.snd_manager.load_sound(path, name)
    }

    pub fn load_texture(&mut self, path: &str, name: &str) -> Result<(), String> {
        self.tex_manager.create_texture(path, name)
    }

    pub fn play_sound(&self, name: &str) -> Result<Channel, String> {
        self.snd_manager.play_sound(name)
    }

    pub fn stop_sound(&self, channel: Channel) {
        self.snd_manager.stop_sound(channel)
    }

    pub fn draw_texture(
        &self, renderer: &mut WindowCanvas, name: &str,
        x: i32, y: i32, w: u32, h: u32,
        sx: i32, sy: i32, sw: u32, sh: u32) {
        if let Some(tex) = self.tex_manager.get_texture(name) {
            let src = Rect::new(sx, sy, sw, sh);
            let dst = Rect::new(x, y, w, h);
            renderer.copy(tex, Some(src), Some(dst)).unwrap();
        }
    }

    pub fn add_palette(&mut self, name: &str, palette: Palette) {
        self.tex_manager.add_palette(name, palette)
    }

    pub fn set_palette(&mut self, name: &str) {
        self.tex_manager.set_palette(name);
    }

    pub fn load_music(&mut self, path: &str) {
        self.snd_manager.load_music(path)
    }

    pub fn play_music(&self) {
        self.snd_manager.play_music()
    }

    pub fn update_textures(&mut self) -> Result<(), String> {
        self.tex_manager.update_textures()
    }
}