use std::collections::HashSet;

use sdl2::keyboard::Keycode;

pub struct Input {
    prev_keys_state: HashSet<Keycode>,
    new_keys: HashSet<Keycode>,
    old_keys: HashSet<Keycode>
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {prev_keys_state: HashSet::new(), new_keys: HashSet::new(), old_keys: HashSet::new()}
    }

    pub(crate) fn update(&mut self, keys: &HashSet<Keycode>) {
        self.new_keys = keys - &self.prev_keys_state;
        self.old_keys = &self.prev_keys_state - keys;

        self.prev_keys_state = keys.clone();
    }

    pub fn is_pressed(&self, key: Keycode) -> bool {
        self.new_keys.contains(&key) && !self.old_keys.contains(&key)
    }

    pub fn is_released(&self, key: Keycode) -> bool {
        self.old_keys.contains(&key) && !self.new_keys.contains(&key)
    }

    pub fn is_key_down(&self, key: Keycode) -> bool {
        self.new_keys.contains(&key) && self.old_keys.contains(&key)
    }
}