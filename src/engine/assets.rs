use sdl2::{render::{TextureCreator, Texture, WindowCanvas}, video::WindowContext, mixer::Channel, rect::Rect};

use super::{sound::SoundManager, texture::TexManager};

pub struct AssetManager<'a> {
    tex_manager: TexManager<'a>,
    snd_manager: SoundManager
}

impl<'a> AssetManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let tex_manager = TexManager::new(texture_creator);
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

    pub fn draw_texture(&self, renderer: &mut WindowCanvas, name: &str, x: i32, y: i32, w: u32, h: u32) {
        if let Some(tex) = self.tex_manager.get_texture(name) {
            let texcoords = sdl2::rect::Rect::new(x, y, w, h);
            renderer.copy(tex, None, Some(texcoords)).unwrap();
        }
    }
}