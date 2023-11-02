use sdl2::{keyboard::Keycode, render::WindowCanvas, rect::Rect};

use crate::engine::{input::Input, assets::AssetManager};

use super::game::XCENTRE;

pub(crate) struct PauseMenu {
    menu_cursor: i32,
    sound_muted: bool,
    music_muted: bool
}

#[derive(PartialEq)]
pub(crate) enum PauseReturn {
    None,
    NewGame,
    Quit
}

const PAUSEY: i32 = 42;

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

        self.menu_cursor = self.menu_cursor.clamp(0, 3);

        if input.is_pressed(Keycode::Return) {
            match self.menu_cursor {
                0 => {
                    return PauseReturn::NewGame;
                }
                1 => {
                    self.sound_muted = !self.sound_muted;
                    manager.mute_sounds(self.sound_muted);
                }
                2 => {
                    self.music_muted = !self.music_muted;
                    manager.mute_music(self.music_muted);
                }
                _ => { return PauseReturn::Quit; }
            }
        }

        manager.draw_rectangle(renderer, 0, PAUSEY-2, 160, 1, 1, true);
        manager.draw_rectangle(renderer, 0, PAUSEY-1, 160, 49, 2, true);
        manager.draw_rectangle(renderer, 0, PAUSEY+47, 160, 1, 1, true);

        manager.draw_text(renderer, "pause", XCENTRE-20, PAUSEY);
        manager.draw_text(renderer, "new game", XCENTRE-32, PAUSEY+8);
        manager.draw_text(renderer, "sounds", XCENTRE-24, PAUSEY+16);
        if !self.sound_muted {
            manager.draw_text(renderer, "x", XCENTRE+32, PAUSEY+16);
        }
        manager.draw_text(renderer, "music", XCENTRE-20, PAUSEY+24);
        if !self.music_muted {
            manager.draw_text(renderer, "x", XCENTRE+24, PAUSEY+24);
        }
        manager.draw_text(renderer, "save and quit", XCENTRE-48, PAUSEY+32);
        manager.draw_text(renderer, "-", 24, PAUSEY+(self.menu_cursor+1)*8);

        PauseReturn::None
    }
}