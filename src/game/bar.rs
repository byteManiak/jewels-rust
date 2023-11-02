use std::time::{Duration, Instant};

use sdl2::{render::WindowCanvas, rect::Rect};

use crate::engine::assets::AssetManager;

pub(crate) struct Bar {
    pub start_level: bool,
    maxgems: i32,
    gemcount: i32,
    start_tick: Instant
}

const BARX: i32 = 5;
const BARY: i32 = 4;

impl Bar {
    pub fn new() -> Self {
        Self { start_level: false, maxgems: 60, gemcount: 0, start_tick: Instant::now() }
    }

    pub fn add_progress(&mut self) {
        self.gemcount += 1;
        if self.gemcount >= self.maxgems {
            self.maxgems = (self.maxgems as f32 * 1.5f32) as i32;
            self.gemcount = 0;
            self.start_level = true;
        }
    }

    pub fn reset(&mut self) {
        self.maxgems = 60;
        self.gemcount = 0;
    }

    pub fn draw(&self, manager: &AssetManager, renderer: &mut WindowCanvas) {
        let t = 79 - (self.start_tick.elapsed().as_millis() as u32/300) % 80;
        let p = 1f32 - self.gemcount as f32 / self.maxgems as f32;

        manager.draw_texture(renderer, "bar", BARX+1, BARY+1+80-t as i32, 20, t,    0, 0,        20, t);
        manager.draw_texture(renderer, "bar", BARX+1, BARY+1,             20, 80-t, 0, t as i32, 20, 80-t);
        renderer.fill_rect(Rect::new(BARX+1, BARY+1, 20, (80 as f32*p) as u32));
        manager.draw_texture(renderer, "barholder", BARX, BARY, 22, 82, 0, 0, 0, 0);

        manager.draw_texture(renderer, "bardesc", BARX-2, BARY+83, 26, 36, 0, 0, 0, 0);
    }
}