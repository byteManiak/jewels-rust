use std::time::Instant;

use sdl2::{keyboard::Keycode, mixer::Channel};

use crate::engine::{input::Input, assets::AssetManager};

const LOGO_SIZE_X: i16 = 47;
const LOGO_SIZE_Y: i16 = 26;

pub struct MainMenu {
    start_tick: Instant,
    logo_moving: bool,
    logo_y: i16,
    sound_channel: Channel
}

impl MainMenu {
    pub(crate) fn new(manager: &AssetManager) -> Self {
        let channel = manager.play_sound("intro").unwrap();

        Self {
            start_tick: Instant::now(),
            logo_moving: true,
            logo_y: -LOGO_SIZE_Y*2,
            sound_channel: channel
        }
    }

    pub(crate) fn update(&mut self, input: &Input, manager: &AssetManager) -> bool {
        let mut ret = false;

        if self.logo_moving {
            if input.is_pressed(Keycode::Return) {
                self.logo_moving = false;
            }
        } else {
            if input.is_pressed(Keycode::Return) {
                manager.stop_sound(self.sound_channel);
                ret = true;
            }
        }

        ret
    }
}