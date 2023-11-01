use std::collections::HashSet;

use sdl2::{Sdl, render::{WindowCanvas, Texture, TextureAccess}, mixer::{AUDIO_S16LSB, InitFlag, Sdl2MixerContext}, keyboard::Keycode, pixels::{Color, PixelFormat, PixelFormatEnum}};

use super::input::Input;

pub struct Context {
    pub sdl: Sdl,
    pub renderer: WindowCanvas,
    pub audio: Sdl2MixerContext,

    pub input: Input
}

impl Context {
    pub fn create_ctx(title: &str, width: u32, height: u32, int_scaling: bool) -> Result<Context, String> {
        let sdl = sdl2::init()?;

        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, 2, 4096)?;
        let audio = sdl2::mixer::init(InitFlag::MP3)?;
        sdl2::mixer::allocate_channels(8);

        let mut window = sdl.video()?.window(title, width*3, height*3).resizable().build().map_err(|e| e.to_string())?;
        window.show();

        let mut renderer = window.into_canvas().accelerated().present_vsync().build().map_err(|e| e.to_string())?;

        if int_scaling {
            renderer.set_integer_scale(true)?;
            renderer.set_logical_size(width, height).map_err(|e| e.to_string())?;
        }

        renderer.clear();
        renderer.present();

        Ok(Context {
            sdl, audio, renderer,
            input: Input::new()
        })
    }

    pub fn update_events(&mut self) {
        let mut events = self.sdl.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                _ => continue
            }
        }

        let keys: HashSet<Keycode> = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        self.input.update(&keys);
    }
}