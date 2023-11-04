use std::time::Instant;



use crate::engine::{assets::AssetManager, sprite::Sprite};

use super::board::{BASEX, BASEY};

#[derive(Clone)]
pub(crate) struct Gem {
    pub gem_type: u8,
    pub is_moving: bool,
    pub is_matched: bool,
    xdest: i32,
    ydest: i32,
    x: f32,
    y: f32,
    sprite: Sprite,
    start_tick: Instant
}

impl Gem {
    pub fn new(gem_type: u8, x: i32, y: i32, yo: i32) -> Self {
        let xpos = BASEX+1+x*16;
        Self {
            gem_type, is_moving: false, is_matched: false,
            xdest: xpos, x: xpos as f32,
            ydest: BASEY+1+y*16, y: (BASEY-16-yo*16)as f32,
            sprite: Sprite::new(format!("gem{:?}", gem_type), 15, 15, 6, 100),
            start_tick: Instant::now()
        }
    }

    pub fn empty(&mut self) {
        self.gem_type = 255;
    }

    pub fn draw(&mut self, is_selected: bool, manager: &AssetManager) {
        let dx = f32::abs(self.x - self.xdest as f32) + 1f32;
        let dy = f32::abs(self.y - self.ydest as f32) + 1f32;

        let tick = self.start_tick.elapsed().as_millis() as f32 / 120f32;

        if (self.x as i32) < self.xdest {
            self.x += dx * tick;
            if (self.x as i32) > self.xdest {
                self.x = self.xdest as f32;
            }
        } else if (self.x as i32) > self.xdest {
            self.x -= dx * tick;
            if (self.x as i32) < self.xdest {
                self.x = self.xdest as f32;
            }
        }

        if (self.y as i32) < self.ydest {
            self.y += dy * tick;
            if (self.y as i32) > self.ydest {
                self.y = self.ydest as f32;
            }
        } else if (self.y as i32) > self.ydest {
            self.y -= dy * tick;
            if (self.y as i32) < self.ydest {
                self.y = self.ydest as f32;
            }
        }

        if self.x as i32 == self.xdest && self.y as i32 == self.ydest {
            self.is_moving = false;
        }

        self.start_tick = Instant::now();

        if is_selected {
            self.sprite.draw(manager, self.x as i32, self.y as i32);
        } else {
            self.sprite.draw_tile(manager, self.x as i32, self.y as i32, 0);
        }
    }

    pub fn set_next_type(&mut self) {
        self.gem_type += 1;
        if self.gem_type > 6 {
            self.gem_type = 1;
        }

        self.sprite.set_sprite(format!("gem{:?}", self.gem_type).as_str());
    }

    pub fn set_coords(&mut self, x: i32, y: i32) {
        self.is_moving = true;
        self.xdest = BASEX+1+x*16;
        self.ydest = BASEY+1+y*16;
    }
}