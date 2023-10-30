use std::collections::HashSet;

use sdl2::{Sdl, render::WindowCanvas, mixer::{AUDIO_S16LSB, InitFlag, Sdl2MixerContext}, keyboard::Keycode};

struct Input {
    _prev_keys_state: HashSet<Keycode>,
    new_keys: HashSet<Keycode>,
    old_keys: HashSet<Keycode>
}

impl Input {
    fn new() -> Self {
        Self {_prev_keys_state: HashSet::new(), new_keys: HashSet::new(), old_keys: HashSet::new()}
    }
}

pub struct Context {
    pub sdl: Sdl,
    pub renderer: WindowCanvas,
    pub audio: Sdl2MixerContext,

    input: Input
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

        Ok(Context {sdl, audio, renderer, input: Input::new()})
    }

    pub fn update_events(&mut self) {
        let mut events = self.sdl.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                _ => continue
            }
        }
        let keys: HashSet<Keycode> = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        self.input.new_keys = &keys - &self.input._prev_keys_state;
        self.input.old_keys = &self.input._prev_keys_state - &keys;

        self.input._prev_keys_state = keys;
    }

    pub fn is_pressed(&self, key: Keycode) -> bool {
        self.input.new_keys.contains(&key) && !self.input.old_keys.contains(&key)
    }

    pub fn is_released(&self, key: Keycode) -> bool {
        self.input.old_keys.contains(&key) && !self.input.new_keys.contains(&key)
    }
}