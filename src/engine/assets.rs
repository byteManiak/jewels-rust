use std::{cell::RefCell, rc::Rc};

use sdl2::{render::{TextureCreator, WindowCanvas}, video::WindowContext, pixels::{Palette, Color}, rect::Rect, mixer::Channel, ttf::Font};

use super::{sound::SoundManager, texture::{TexManager, ColorPalette}};

pub struct AssetManager<'a, 'b> {
    tex_manager: TexManager<'a>,
    snd_manager: SoundManager<'a>,
    font: &'a Font<'a, 'b>,
    renderer: Rc<RefCell<WindowCanvas>>
}

impl<'a, 'b> AssetManager<'a, 'b> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, renderer: &Rc<RefCell<WindowCanvas>>, font: &'a Font<'a, 'b>, palette: ColorPalette) -> Self {
        let tex_manager = TexManager::new(texture_creator, palette);
        let snd_manager = SoundManager::new();
        Self {tex_manager, snd_manager, font, renderer: renderer.clone()}
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
        &self, name: &str,
        x: i32, y: i32, w: u32, h: u32,
        sx: i32, sy: i32, sw: u32, sh: u32) {
        if let Some(tex) = self.tex_manager.get_texture(name) {
            let src = if sx == 0 && sy == 0 && sw == 0 && sh == 0 {None} else {Some(Rect::new(sx, sy, sw, sh))} ;
            let dst = Rect::new(x, y, w, h);
            self.renderer.borrow_mut().copy(tex, src, Some(dst)).unwrap();
        }
    }

    pub fn add_palette(&mut self, palette: ColorPalette) {
        self.tex_manager.add_palette(palette)
    }

    pub fn set_next_palette(&mut self) {
        self.tex_manager.set_next_palette();
    }

    pub fn load_music(&mut self, path: &str) {
        self.snd_manager.load_music(path)
    }

    pub fn play_music(&self) {
        self.snd_manager.play_music()
    }

    pub fn pause_music(&self) {
        self.snd_manager.pause_music()
    }

    pub fn resume_music(&self) {
        self.snd_manager.resume_music()
    }

    pub fn mute_sounds(&mut self, mute: bool) {
        self.snd_manager.mute_sounds(mute)
    }

    pub fn mute_music(&self, mute: bool) {
        self.snd_manager.mute_music(mute)
    }

    pub fn update_textures(&mut self) -> Result<(), String> {
        self.tex_manager.update_textures()
    }

    pub fn draw_text(&self, text: &str, x: i32, y: i32) {
        let surface = self.font.render(text).solid(self.tex_manager.get_index_color(3));

        if let Ok(surface) = surface {
            let (w, h) = surface.size();
            let texture = self.tex_manager.loader.create_texture_from_surface(surface);

            if let Ok(tex) = texture {
                let dst = Rect::new(x, y, w, h);
                let _ = self.renderer.borrow_mut().copy(&tex, None, dst);
            }
        }
    }

    pub fn draw_rectangle(&self, x: i32, y: i32, w: u32, h: u32, color_index: usize, fill: bool) {
        let r = Rect::new(x, y, w, h);

        let mut renderer = self.renderer.borrow_mut();
        renderer.set_draw_color(self.tex_manager.get_index_color(color_index));

        if fill {
            let _ = renderer.fill_rect(r);
        } else {
            let _ = renderer.draw_rect(r);
        }
    }

    pub fn begin_draw(&mut self) {
        let mut renderer = self.renderer.borrow_mut();

        renderer.set_draw_color(Color::RGBA(0, 0, 0, 0));
        renderer.clear();
    }
}

pub fn u32_palette(c1: u32, c2: u32, c3: u32, c4:u32) -> ColorPalette {
    let color1 = Color::RGBA(
        ((c1 & 0xFF0000) >> 16) as u8,
        ((c1 & 0xFF00) >> 8) as u8,
        (c1 & 0xFF) as u8,
        0,
    );
    let color2 = Color::RGBA(
        ((c2 & 0xFF0000) >> 16) as u8,
        ((c2 & 0xFF00) >> 8) as u8,
        (c2 & 0xFF) as u8,
        0,
    );
    let color3 = Color::RGBA(
        ((c3 & 0xFF0000) >> 16) as u8,
        ((c3 & 0xFF00) >> 8) as u8,
        (c3 & 0xFF) as u8,
        0,
    );
    let color4 = Color::RGBA(
        ((c4 & 0xFF0000) >> 16) as u8,
        ((c4 & 0xFF00) >> 8) as u8,
        (c4 & 0xFF) as u8,
        0,
    );

    let palette = Palette::with_colors(&[color1, color2, color3, color4]).unwrap();

    ColorPalette {
        palette,
        colors: [color1, color2, color3, color4]
    }
}