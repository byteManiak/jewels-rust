use std::time::Instant;

use sdl2::{keyboard::Keycode, mixer::Channel};

use crate::engine::{input::Input, assets::AssetManager, sprite::Sprite};

use super::game::{YCENTRE, XCENTRE};

const LOGO_SIZE_X: i32 = 47;
const LOGO_SIZE_Y: i32 = 26;

struct SpinnyGem {
    sprite: Sprite,
    is_looping: bool,
    start_tick: Instant
}

impl SpinnyGem {
    pub fn new(gem_type: u32) -> Self {
        let name = format!("gem{:?}", gem_type);
        Self {sprite: Sprite::new(name, 15, 15, 6, 100), is_looping: false, start_tick: Instant::now()}
    }

    pub fn update(&mut self, manager: &AssetManager) {
        let mut x;
        let y;
        if !self.is_looping {
            x = -15 + (self.start_tick.elapsed().as_millis() as f32 / 20.0f32) as i32;
            y = YCENTRE/3 - 9;
            if x >= XCENTRE-7 {
                x = XCENTRE-7;
                self.start_tick = Instant::now();
                self.is_looping = true;
            }
        } else {
            let tsin = (self.start_tick.elapsed().as_millis() as f32 / 600f32).sin();
            let tcos = (self.start_tick.elapsed().as_millis() as f32 / 600f32).cos();
            let xoff = tsin * (3f32*XCENTRE as f32 / 4f32);
            let yoff = tcos * (YCENTRE as f32 / 2f32);
            x = ((XCENTRE-7) as f32 + xoff) as i32;
            y = ((YCENTRE-21) as f32 - yoff) as i32;
        }

        self.sprite.draw(manager, x, y);
    }
}

pub struct MainMenu {
    start_tick: Instant,
    logo_moving: bool,
    logo_y: i32,
    sound_channel: Channel,
    gems: Vec<SpinnyGem>
}

impl MainMenu {
    pub(crate) fn new(sound_channel: Channel) -> Self {
        Self {
            start_tick: Instant::now(),
            logo_moving: true,
            logo_y: -LOGO_SIZE_Y*2,
            sound_channel,
            gems: Vec::new()
        }
    }

    pub(crate) fn update(&mut self, input: &Input, manager: &AssetManager) -> bool {
        let mut ret = false;

        if self.logo_moving {
            self.logo_y = -LOGO_SIZE_Y*2 + (self.start_tick.elapsed().as_millis() as f32 / 40f32) as i32;
            if (self.logo_y >= YCENTRE - LOGO_SIZE_Y - 18) || input.is_pressed(Keycode::Return) {
                self.logo_moving = false;
            }
        } else {
            self.logo_y = YCENTRE - LOGO_SIZE_Y - 18;
            if self.start_tick.elapsed().as_millis() > 620 {
                self.start_tick = Instant::now();
                let len = self.gems.len() as u32;
                if len < 6 {
                    self.gems.push(SpinnyGem::new(len+1));
                }
            }

            if input.is_pressed(Keycode::Return) {
                manager.stop_sound(self.sound_channel);
                ret = true;
            }

            manager.draw_text("a game by bytemaniak", 6, 1);
            manager.draw_text("press enter to play", 10, 100);
        }

        manager.draw_texture(
            "logo",
            31, self.logo_y, (LOGO_SIZE_X*2) as u32, (LOGO_SIZE_Y*2) as u32,
            0, 0, LOGO_SIZE_X as u32, LOGO_SIZE_Y as u32
        );
        for tex in self.gems.iter_mut() {
            tex.update(manager);
        }

        ret
    }
}