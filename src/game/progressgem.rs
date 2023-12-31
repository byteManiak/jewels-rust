use std::{time::Instant, cmp::Ordering};

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

        match self.x.cmp(&self.xdest) {
            Ordering::Less => {
                self.x += (dx * self.start_tick.elapsed().as_millis() as f32 / 2000f32) as i32;
                if self.x > self.xdest {
                    self.x = self.xdest;
                }
            },
            Ordering::Greater => {
                self.x -= (dx * self.start_tick.elapsed().as_millis() as f32 / 2000f32) as i32;
                if self.x < self.xdest {
                    self.x = self.xdest;
                }
            },
            _ => {}
        }

        match self.y.cmp(&self.ydest) {
            Ordering::Less => {
                self.y += (dy * self.start_tick.elapsed().as_millis() as f32 / 2000f32) as i32;
                if self.y > self.ydest {
                    self.y = self.ydest;
                }
            },
            Ordering::Greater => {
                self.y -= (dy * self.start_tick.elapsed().as_millis() as f32 / 2000f32) as i32;
                if self.y < self.ydest {
                    self.y = self.ydest;
                }
            },
            _ => {}
        }

        if i32::abs(self.x - self.xdest) < 4 && i32::abs(self.y - self.ydest) < 4 {
            self.reached = true;
        }

        manager.draw_texture(&self.texture, self.x, self.y, 15, 15, 0, 0, 15, 15);
    }
}