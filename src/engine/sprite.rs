use std::time::Instant;

use super::assets::AssetManager;

#[derive(Clone)]
pub struct Sprite {
    tile_w: u32,
    tile_h: u32,
    num_tiles: u32,
    current_tile: u32,
    texture: String,
    start_tick: Instant,
    tick_speed: u32
}

impl Sprite {
    pub fn new(name: String, tile_w: u32, tile_h: u32, num_tiles: u32, tick_speed: u32) -> Self {
        Self {tile_w, tile_h, num_tiles, current_tile: 0, texture: name.to_string(), start_tick: Instant::now(), tick_speed}
    }

    pub fn draw(&mut self, manager: &AssetManager, x: i32, y: i32) {
        if self.start_tick.elapsed().as_millis() > self.tick_speed as u128 {
            self.start_tick = Instant::now();
            self.current_tile = (self.current_tile+1) % self.num_tiles;
        }

        manager.draw_texture(
            &self.texture,
            x, y, self.tile_w, self.tile_h,
            (self.tile_w*self.current_tile) as i32, 0, self.tile_w, self.tile_h
        );
    }

    pub fn draw_tile(&self, manager: &AssetManager, x: i32, y: i32, tile: i32) {
        if tile == -1 {
            manager.draw_texture(
                &self.texture,
                x, y, self.tile_w, self.tile_h,
                (self.current_tile*self.tile_w) as i32, 0, self.tile_w, self.tile_h
            );
        } else {
            manager.draw_texture(
                &self.texture,
                x, y, self.tile_w, self.tile_h,
                tile*(self.tile_w as i32), 0, self.tile_w, self.tile_h
            );
        }
    }

    pub fn set_sprite(&mut self, name: &str) {
        self.texture = name.to_string();
    }
}