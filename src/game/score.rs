use std::{panic::AssertUnwindSafe, fmt::Binary};

use sdl2::render::WindowCanvas;

use crate::engine::assets::AssetManager;

pub(crate) struct Score {
    score: u32,
    level: u32
}

const SCOREX: i32 = 100;
const SCOREY: i32 = 132;
const LEVELX: i32 = 6;
const LEVELY: i32 = 132;

impl Score {
    pub fn new() -> Self {
        Self {score: 0, level: 0}
    }

    pub fn add_score(&mut self, combo: u32) {
        self.score += combo * self.level*5;
    }

    pub fn draw(&self, manager: &AssetManager,) {
        let level_string = format!("{:03}", self.level);
        let score_string = format!("{:07}", self.score);

        manager.draw_text("level", LEVELX, LEVELY);
        manager.draw_text(&level_string, LEVELX+48, LEVELY);
        manager.draw_text(&score_string, SCOREX, SCOREY);
    }

    pub fn increase_level(&mut self) {
        self.level += 1;
    }

    pub fn reset(&mut self) {
        self.level = 1;
        self.score = 0;
    }
}