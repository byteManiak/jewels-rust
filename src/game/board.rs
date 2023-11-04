use std::time::Instant;

use rand::Rng;
use sdl2::{keyboard::Keycode, mouse::MouseButton};

use crate::engine::{input::Input, assets::AssetManager};

use super::{gem::Gem, progressgem::ProgressGem, bar::Bar, pause::{PauseMenu, PauseReturn}, score::Score};

pub(crate) const BASEX: i32 = 30;
pub(crate) const BASEY: i32 = 1;
pub(crate) const BWIDTH: i32 = 16*8;

#[derive(PartialEq)]
enum SwapState {
    NoSwap,
    SwapFirst,
    SwapBack
}

pub struct Board {
    gameover: bool,
    has_match: bool,
    is_paused: bool,
    is_selecting: bool,
    is_animating: bool,
    short_wait: bool,
    swap_state: SwapState,
    mouse_x: i32,
    mouse_y: i32,
    x_cursor: i32,
    y_cursor: i32,
    x1swap: i32,
    y1swap: i32,
    x2swap: i32,
    y2swap: i32,
    gems: Vec<Vec<Gem>>,
    wait_tick: Instant,
    combo: i8,
    progress_gems: Vec<ProgressGem>,
    bar: Bar,
    pause_menu: PauseMenu,
    score: Score
}

const LEFT: i32 = 0;
const RIGHT: i32 = 1;
const UP: i32 = 2;
const DOWN: i32 = 3;

const NO_GEM: u8 = 255;

impl Board {
    pub(crate) fn new() -> Self {
        let default_gem = Gem::new(NO_GEM, 0, 0, 0);
        Self {
            gameover: false, has_match: false, is_paused: false, is_selecting: false,
            is_animating: false, short_wait: false, swap_state: SwapState::NoSwap,
            mouse_x: 0, mouse_y: 0,
            x_cursor: 4, y_cursor: 4,
            x1swap: 0, y1swap: 0, x2swap: 0, y2swap: 0,
            gems: vec![vec![default_gem; 8]; 8],
            wait_tick: Instant::now(), combo: 0,
            progress_gems: Vec::new(), bar: Bar::new(),
            pause_menu: PauseMenu::new(), score: Score::new()
        }
    }

    pub(super) fn new_game(&mut self) {
        self.gen_board();
        self.score.reset();
        self.bar.reset();
        self.gameover = false;
    }

    fn gen_partial_match(&mut self, x: i32, y: i32, gem_type: u8) {
        if !self.is_slot_available(x, y) {
            return self.gen_partial_match(
                rand::thread_rng().gen_range(0..8),
                rand::thread_rng().gen_range(0..8),
                gem_type);
        }

        let mut dir = rand::thread_rng().gen_range(0..3);
        if dir == DOWN && y >= 6 {
            dir = UP;
        } else if dir == UP && y <= 1 {
            dir = DOWN;
        } else if dir == RIGHT && x >= 6 {
            dir = LEFT;
        } else if dir == LEFT && x <= 1 {
            dir = RIGHT;
        }

        let mut x1 = 0;
        let mut y1 = 0;
        let mut x2 = 0;
        let mut y2 = 0;

        match dir {
            LEFT | RIGHT => {
                x1 = x+1; x2 = x+2;
                if dir == LEFT { x1 -= 2; x2 -= 4; }

                let r = rand::thread_rng().gen_range(0..6);
                match r {
                    0 => { y1 = y+1; y2 = y; }
                    1 => { y1 = y-1; y2 = y; }
                    2 => { y1 = y; y2 = y-1; }
                    3 => { y1 = y; y2 = y+1; }
                    4 => { y1 = y-1; y2 = y-1; }
                    5 => { y1 = y+1; y2 = y+1; }
                    _ => {}
                }
            }

            UP | DOWN => {
                y1 = y+1; y2 = y+2;
                if dir == UP { y1 -= 2; y2 -= 4; }

                let r = rand::thread_rng().gen_range(0..6);
                match r {
                    0 => { x1 = x+1; x2 = x; }
                    1 => { x1 = x-1; x2 = x; }
                    2 => { x1 = x; x2 = x-1; }
                    3 => { x1 = x; x2 = x+1; }
                    4 => { x1 = x-1; x2 = x-1; }
                    5 => { x1 = x+1; x2 = x+1; }
                    _ => {}
                }
            }
            _ => {}
        }

        if self.is_slot_available(x1, y1) && self.is_slot_available(x2, y2) {
            self.gems[x as usize][y as usize] = Gem::new(gem_type, x, y, 0);
            self.gems[x1 as usize][y1 as usize] = Gem::new(gem_type, x1, y1, 0);
            self.gems[x2 as usize][y2 as usize] = Gem::new(gem_type, x2, y2, 0);
        } else {
            return self.gen_partial_match(
                rand::thread_rng().gen_range(0..8),
                rand::thread_rng().gen_range(0..8),
                gem_type);
        }
    }

    fn is_slot_available(&self, x: i32, y: i32) -> bool {
        if x < 0 || x > 7 || y < 0 || y > 7 {
            return false;
        }

        self.gems[x as usize][y as usize].gem_type == NO_GEM
    }

    fn avoid_matches(&mut self) {
        self.find_match(true);

        for i in 1..=6 {
            for j in 0..8 {
                if self.gems[i][j].gem_type == self.gems[i-1][j].gem_type &&
                   self.gems[i][j].gem_type == self.gems[i+1][j].gem_type {
                    self.gems[i][j].set_next_type();
                   }
            }
        }

        for j in 1..=6 {
            for i in 0..8 {
                if self.gems[i][j].gem_type == self.gems[i][j-1].gem_type &&
                   self.gems[i][j].gem_type == self.gems[i][j+1].gem_type {
                    self.gems[i][j].set_next_type();
                   }
            }
        }
    }

    fn gen_board(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                self.gems[i][j].empty();
            }
        }

        for i in 0..6 {
            self.gen_partial_match(
                rand::thread_rng().gen_range(0..8),
                rand::thread_rng().gen_range(0..8),
                i+1);
        }

        for i in 0..8 {
            for j in 0..8 {
                if self.gems[i][j].gem_type == NO_GEM {
                    self.gems[i][j] = Gem::new(rand::thread_rng().gen_range(1..6), i as i32, j as i32, 0);
                }
            }
        }

        self.avoid_matches();
        while self.has_match {
            self.avoid_matches();
        }
    }

    fn is_mouse_at_coords(&self, x: i32, y: i32) -> bool {
        Input::is_in_bounds(self.mouse_x, self.mouse_y, BASEX+x*16, BASEY+y*16, BASEX+(x+1)*16, BASEY+(y+1)*16)
    }

    fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, move_cursor: bool) {
        let temp_gem = self.gems[x1 as usize][y1 as usize].clone();
        self.gems[x1 as usize][y1 as usize] = self.gems[x2 as usize][y2 as usize].clone();
        self.gems[x2 as usize][y2 as usize] = temp_gem;

        if move_cursor {
            self.x_cursor = x2;
            self.y_cursor = y2;
        }

        if self.gems[x1 as usize][y1 as usize].gem_type != NO_GEM {
            self.gems[x1 as usize][y1 as usize].set_coords(x1, y1);
        }
        if self.gems[x2 as usize][y2 as usize].gem_type != NO_GEM {
            self.gems[x2 as usize][y2 as usize].set_coords(x2, y2);
        }

        self.x1swap = x2; self.y1swap = y2;
        self.x2swap = x1; self.y2swap = y1;

        if self.swap_state == SwapState::NoSwap {
            self.swap_state = SwapState::SwapFirst;
        } else if self.swap_state == SwapState::SwapFirst {
            self.swap_state = SwapState::SwapBack;
        }
    }

    pub(super) fn update(&mut self, input: &Input, manager: &mut AssetManager) -> bool {
        (self.mouse_x, self.mouse_y) = input.get_mouse_coords();

        if self.gameover {
            manager.pause_music();
            if input.is_pressed(Keycode::Return) {
                self.new_game();
                manager.resume_music();
            }
        } else if self.is_paused {
            if input.is_pressed(Keycode::Escape) {
                self.is_paused = false;
            }
        } else {
            if input.is_pressed(Keycode::Escape) {
                self.is_paused = true;
            } else {
                self.is_selecting = Input::is_in_bounds(self.mouse_x, self.mouse_y, BASEX, BASEY, BASEX+BWIDTH, BASEY+BWIDTH) && input.is_button_pressed(MouseButton::Left);

                if !self.is_selecting {
                    self.x_cursor = (self.mouse_x - BASEX) / 16;
                    self.y_cursor = (self.mouse_y - BASEY) / 16;
                } else if !self.is_animating && !self.short_wait && self.swap_state == SwapState::NoSwap {
                    if self.is_mouse_at_coords(self.x_cursor-1, self.y_cursor) && self.x_cursor > 0 {
                        self.swap(self.x_cursor, self.y_cursor, self.x_cursor-1, self.y_cursor, true);
                    } else if self.is_mouse_at_coords(self.x_cursor+1, self.y_cursor) && self.x_cursor < 7 {
                        self.swap(self.x_cursor, self.y_cursor, self.x_cursor+1, self.y_cursor, true);
                    } else if self.is_mouse_at_coords(self.x_cursor, self.y_cursor-1) && self.y_cursor > 0 {
                        self.swap(self.x_cursor, self.y_cursor, self.x_cursor, self.y_cursor-1, true);
                    } else if self.is_mouse_at_coords(self.x_cursor, self.y_cursor+1) && self.y_cursor < 7 {
                        self.swap(self.x_cursor, self.y_cursor, self.x_cursor, self.y_cursor+1, true);
                    }
                }

                self.x_cursor = self.x_cursor.clamp(0, 7);
                self.y_cursor = self.y_cursor.clamp(0, 7);
            }
        }

        self.is_animating = false;
        
        for i in 0..8 {
            for j in 0..8 {
                let gem = &mut self.gems[i as usize][j as usize];

                manager.draw_rectangle(BASEX+i*16, BASEY+j*16, 17, 17, 1, false);

                if self.x_cursor == i && self.y_cursor == j {
                    let color = if self.is_selecting {3} else {1};
                    manager.draw_rectangle(BASEX+i*16+1, BASEY+j*16+1, 15, 15, color, true);

                    if gem.gem_type != NO_GEM {
                        gem.draw(true, manager);
                        if gem.is_moving {
                            self.is_animating = true;
                        }
                    }
                } else {
                    if gem.gem_type != NO_GEM {
                        gem.draw(false, manager);
                        if gem.is_moving {
                            self.is_animating = true;
                        }
                    }
                }
            }
        }

        self.find_match(false);

        if !self.is_animating {
            if self.short_wait {
                if self.wait_tick.elapsed().as_millis() > 900 || !self.has_match {
                    self.short_wait = false;
                }

                for i in 0..8 {
                    for j in 0..8 {
                        let gem = &mut self.gems[i as usize][j as usize];
                        if gem.is_matched {
                            manager.draw_rectangle(BASEX+i*16+1, BASEY+j*16+1, 15, 15, 3, true);
                            gem.draw(true, manager);
                        }
                    }
                }
            } else {
                if self.swap_state == SwapState::SwapFirst {
                    if self.has_match {
                        self.swap_state = SwapState::NoSwap;
                    } else {
                        self.swap(self.x1swap, self.y1swap, self.x2swap, self.y2swap, true);
                    }
                } else if self.swap_state == SwapState::SwapBack {
                    self.swap_state = SwapState::NoSwap;
                }

                if self.has_match {
                    self.combo += 1;
                    let c = if self.combo > 7 {7} else {self.combo};
                    let _ =  manager.play_sound(format!("combo{:?}", c).as_str());
                    self.sweep_matches(&manager);
                } else {
                    self.combo = -1;
                }
            }
        }

        self.score.draw(manager);
        self.bar.draw(manager);

        for gem in &mut self.progress_gems {
            gem.draw(manager);
            if gem.reached {
                self.bar.add_progress();
                if self.bar.start_level {
                    self.bar.start_level = false;
                    self.score.increase_level();
                    manager.play_sound("levelup");
                    manager.set_next_palette();
                }
            }
        }
        self.progress_gems.retain(|gem|{!gem.reached});

        if self.gameover {
            manager.draw_rectangle(0, 63, 160, 1, 1, true);
            manager.draw_rectangle(0, 64, 160, 25, 2, true);
            manager.draw_rectangle(0, 88, 160, 1, 1, true);
            manager.draw_text("game over", 48, 67);
            manager.draw_text("press enter to reset", 1, 75);
        }

        if self.is_paused {
            let ret = self.pause_menu.update(manager, input);
            if ret == PauseReturn::NewGame {
                self.new_game();
                self.is_paused = false;
            } else if ret == PauseReturn::Quit {
                return true;
            }
        }

        false
    }

    fn find_match(&mut self, init_stage: bool) {
        self.has_match = false;

        for i in 0..6 {
            for j in 0..8 {
                if self.gems[i][j].gem_type == self.gems[i+1][j].gem_type &&
                   self.gems[i][j].gem_type == self.gems[i+2][j].gem_type {
                    if !init_stage {
                        self.gems[i][j].is_matched = true;
                        self.gems[i+1][j].is_matched = true;
                        self.gems[i+2][j].is_matched = true;
                    }
                    self.has_match = true;
                }
            }
        }

        for i in 0..8 {
            for j in 0..6 {
                if self.gems[i][j].gem_type == self.gems[i][j+1].gem_type &&
                   self.gems[i][j].gem_type == self.gems[i][j+2].gem_type {
                    if !init_stage {
                        self.gems[i][j].is_matched = true;
                        self.gems[i][j+1].is_matched = true;
                        self.gems[i][j+2].is_matched = true;
                    }
                    self.has_match = true;
                }
            }
        }
    }

    fn sweep_matches(&mut self, manager: &AssetManager) {
        for i in 0..8 {
            let mut gems_matched = 0;
            for j in 0..8 {
                let gem = &mut self.gems[i as usize][j as usize];

                if gem.is_matched {
                    self.progress_gems.push(ProgressGem::new(gem.gem_type, BASEX+i*16, BASEY+j*16));

                    gem.gem_type = NO_GEM;

                    for k in (1..=j).rev() {
                        self.swap(i, k, i, k-1, false);
                    }

                    self.gems[i as usize][0] = Gem::new(rand::thread_rng().gen_range(1..=6), i, 0, gems_matched);
                    gems_matched += 1;

                    self.short_wait = true;
                    self.wait_tick = Instant::now();

                    self.score.add_score(self.combo as u32+1);
                }
            }
        }

        self.check_gameover();
        if self.gameover {
            let _ = manager.play_sound("gameover");
        }
    }

    fn check_gameover(&mut self) {
        self.gameover = true;

        for i in 0..7 {
            let mut occurences = [0; 6];
            for j in 0..8 {
                for k in 1..=6 {
                    if self.gems[i][j].gem_type == k || self.gems[i+1][j].gem_type == k {
                        occurences[k as usize -1] += 1;
                        if occurences[k as usize -1] >= 3 {
                            self.gameover = false;
                            return;
                        }
                    } else {
                        occurences[k as usize -1] = 0;
                    }
                }
            }
        }

        for j in 0..7 {
            let mut occurences = [0; 6];
            for i in 0..8 {
                for k in 1..=6 {
                    if self.gems[i][j].gem_type == k || self.gems[i][j+1].gem_type == k {
                        occurences[k as usize -1] += 1;
                        if occurences[k as usize -1] >= 3 {
                            self.gameover = false;
                            return;
                        }
                    } else {
                        occurences[k as usize -1] = 0;
                    }
                }
            }
        }

        for i in 0..5 {
            for j in 0..8 {
                let gem = &self.gems[i][j];
                let gem2 = &self.gems[i+1][j];
                let gem3 = &self.gems[i+2][j];
                let gem4 = &self.gems[i+3][j];

                if gem.gem_type == gem4.gem_type &&
                    (gem.gem_type == gem2.gem_type || gem.gem_type == gem3.gem_type) {
                        self.gameover = false;
                        return;
                }
            }
        }

        for j in 0..5 {
            for i in 0..8 {
                let gem = &self.gems[i][j];
                let gem2 = &self.gems[i][j+1];
                let gem3 = &self.gems[i][j+2];
                let gem4 = &self.gems[i][j+3];

                if gem.gem_type == gem4.gem_type &&
                    (gem.gem_type == gem2.gem_type || gem.gem_type == gem3.gem_type) {
                        self.gameover = false;
                        return;
                }
            }
        }
    }
}
