use std::collections::HashSet;

use sdl2::{keyboard::Keycode, mouse::MouseButton};

pub struct Input {
    prev_keys_state: HashSet<Keycode>,
    new_keys: HashSet<Keycode>,
    old_keys: HashSet<Keycode>,
    mouse_buttons: HashSet<MouseButton>,
    mouse_coords: (i32, i32)
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {prev_keys_state: HashSet::new(), new_keys: HashSet::new(), old_keys: HashSet::new(), mouse_buttons: HashSet::new(), mouse_coords: (0, 0)}
    }

    pub(crate) fn update(&mut self, keys: &HashSet<Keycode>, mouse_buttons: &HashSet<MouseButton>) {
        self.new_keys = keys - &self.prev_keys_state;
        self.old_keys = &self.prev_keys_state - keys;

        self.prev_keys_state = keys.clone();

        self.mouse_buttons = mouse_buttons.clone();
    }

    pub(crate) fn update_mouse_coords(&mut self, mouse_coords: (i32, i32)) {
        self.mouse_coords = mouse_coords;
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

    pub fn get_mouse_coords(&self) -> (i32, i32) {
        self.mouse_coords
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }

    pub fn is_in_bounds(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let xmin = i32::min(x1, x2);
        let xmax = i32::max(x1, x2);
        let ymin = i32::min(y1, y2);
        let ymax = i32::max(y1, y2);

        x >= xmin && y >= ymin && x <= xmax && y <= ymax
    }
}