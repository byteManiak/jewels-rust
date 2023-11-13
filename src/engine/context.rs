use std::{collections::HashSet, cell::RefCell, rc::Rc};

use sdl2::{Sdl, render::WindowCanvas, mixer::{AUDIO_S16LSB, InitFlag}, keyboard::Keycode, mouse::MouseButton, event::Event};

use super::input::Input;

pub struct Context {
    pub sdl: Sdl,
    pub renderer: Rc<RefCell<WindowCanvas>>,

    pub input: Input
}

impl Context {
    pub fn create_ctx(title: &str, width: u32, height: u32, int_scaling: bool) -> Result<Context, String> {
        let sdl = sdl2::init()?;

        sdl2::mixer::open_audio(44100, AUDIO_S16LSB, 2, 4096)?;
        let _ = sdl2::mixer::init(InitFlag::MP3)?;
        sdl2::mixer::allocate_channels(8);
        sdl2::mixer::Music::set_volume(48);

        let mut window = sdl.video()?.window(title, width*3, height*3).resizable().build().map_err(|e| e.to_string())?;
        window.show();

        let mut renderer = window.into_canvas().accelerated().present_vsync().build().map_err(|e| e.to_string())?;

        if int_scaling {
            renderer.set_integer_scale(true)?;
            renderer.set_logical_size(width, height).map_err(|e| e.to_string())?;
        }

        renderer.clear();
        renderer.present();

        Ok(Context { sdl, renderer: Rc::new(RefCell::new(renderer)), input: Input::new() })
    }

    pub fn update_events(&mut self) {
        let mut events = self.sdl.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x, y, xrel: _, yrel: _ } => {
                    self.input.update_mouse_coords((x, y));
                }
                _ => continue
            }
        }

        let keys: HashSet<Keycode> = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        let mouse_buttons: HashSet<MouseButton> = events.mouse_state().pressed_mouse_buttons().collect();
        self.input.update(keys, mouse_buttons);
    }
}
