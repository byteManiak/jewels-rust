use std::time::Instant;

use crate::engine::assets::AssetManager;

pub(crate) struct Bar {
    pub start_level: bool,
    pub maxgems: i32,
    pub gemcount: i32,
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

    pub fn draw(&self, manager: &AssetManager) {
        let t = 79 - (self.start_tick.elapsed().as_millis() as u32/300) % 80;
        let p = 1f32 - self.gemcount as f32 / self.maxgems as f32;

        manager.draw_texture("bar", BARX+1, BARY+1+80-t as i32, 20, t,    0, 0,        20, t);
        manager.draw_texture("bar", BARX+1, BARY+1,             20, 80-t, 0, t as i32, 20, 80-t);
        manager.draw_rectangle(BARX+1, BARY+1, 20, (80f32*p) as u32, 2, true);
        manager.draw_texture("barholder", BARX, BARY, 22, 82, 0, 0, 0, 0);

        manager.draw_texture("bardesc", BARX-2, BARY+83, 26, 36, 0, 0, 0, 0);
    }
}