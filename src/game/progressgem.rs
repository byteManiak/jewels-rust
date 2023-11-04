use std::time::Instant;

use sdl2::render::WindowCanvas;

use crate::engine::assets::AssetManager;

pub(crate) struct ProgressGem {
    pub(crate) reached: bool,
    start_tick: Instant,
    texture: String,
    xdest: i32,
    ydest: i32,
    x: i32,
    y: i32
}

impl ProgressGem {
    pub(crate) fn new(gem_type: u8, x: i32, y: i32) -> Self {
        Self {reached: false, start_tick: Instant::now(), texture: format!("gem{:?}", gem_type), xdest: 8, ydest: 4, x, y}
    }

    pub fn draw(&mut self, manager: &AssetManager) {
        let dx = f32::abs(self.x as f32 - self.xdest as f32) + 1f32;
        let dy = f32::abs(self.y as f32 - self.ydest as f32) - 1f32;

        if self.x < self.xdest {
            self.x += (dx * self.start_tick.elapsed().as_millis() as f32 / 1000f32) as i32;
            if self.x > self.xdest {
                self.x = self.xdest;
            }
        } else if self.x > self.xdest {
            self.x -= (dx * self.start_tick.elapsed().as_millis() as f32 / 1000f32) as i32;
            if self.x < self.xdest {
                self.x = self.xdest;
            }
        }

        if self.y < self.ydest {
            self.y += (dy * self.start_tick.elapsed().as_millis() as f32 / 1000f32) as i32;
            if self.y > self.ydest {
                self.y = self.ydest;
            }
        } else if self.y > self.ydest {
            self.y -= (dy * self.start_tick.elapsed().as_millis() as f32 / 1000f32) as i32;
            if self.y < self.ydest {
                self.y = self.ydest;
            }
        }

        if i32::abs(self.x - self.xdest) < 4 && i32::abs(self.y - self.ydest) < 4 {
            self.reached = true;
        }

        manager.draw_texture(&self.texture, self.x, self.y, 15, 15, 0, 0, 15, 15);
    }
}