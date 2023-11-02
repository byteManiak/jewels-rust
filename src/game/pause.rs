use sdl2::{keyboard::Keycode, render::WindowCanvas, rect::Rect};

use crate::engine::{input::Input, assets::AssetManager};

pub(crate) struct PauseMenu {
    menu_cursor: i8,
    sound_muted: bool,
    music_muted: bool
}

pub(crate) enum PauseReturn {
    None,
    NewGame,
    Quit
}

impl PauseMenu {
    pub fn new() -> Self {
        Self {menu_cursor: 0, sound_muted: false, music_muted: false}
    }

    pub fn update(&mut self, renderer: &mut WindowCanvas, manager: &mut AssetManager, input: &Input) -> PauseReturn {
        if input.is_pressed(Keycode::Down) {
            self.menu_cursor += 1;
        }
        if input.is_pressed(Keycode::Up) {
            self.menu_cursor -= 1;
        }
        if self.menu_cursor < 0 {
            self.menu_cursor = 0;
        }
        if self.menu_cursor > 3 {
            self.menu_cursor = 3;
        }

        if input.is_pressed(Keycode::Return) {
            match self.menu_cursor {
                0 => {
                    return PauseReturn::NewGame;
                }
                1 => {
                    self.sound_muted = !self.sound_muted;
                    // TODO: manager.mute_sounds();
                }
                2 => {
                    self.music_muted = !self.music_muted;
                    // TODO: manager.mute_music();
                }
                _ => { return PauseReturn::Quit; }
            }
        }

        // TODO: draw everything

        PauseReturn::None
    }
}