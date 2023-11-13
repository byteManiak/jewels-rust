use std::collections::HashSet;

use sdl2::{keyboard::Keycode, mouse::MouseButton};

pub struct Input {
    new_keys: HashSet<Keycode>,
    old_keys: HashSet<Keycode>,
    new_mouse_buttons: HashSet<MouseButton>,
    old_mouse_buttons: HashSet<MouseButton>,
    mouse_coords: (i32, i32)
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            new_keys: HashSet::new(), old_keys: HashSet::new(),
            new_mouse_buttons: HashSet::new(), old_mouse_buttons: HashSet::new(),
            mouse_coords: (0, 0)
        }
    }

    pub(crate) fn update(&mut self, keys: HashSet<Keycode>, mouse_buttons: HashSet<MouseButton>) {
        self.old_keys = self.new_keys.clone();
        self.new_keys = keys;

        self.old_mouse_buttons = self.new_mouse_buttons.clone();
        self.new_mouse_buttons = mouse_buttons;
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
        self.new_mouse_buttons.contains(&button) && !self.old_mouse_buttons.contains(&button)
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.old_mouse_buttons.contains(&button) && !self.new_mouse_buttons.contains(&button)
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        self.new_mouse_buttons.contains(&button) && self.old_mouse_buttons.contains(&button)
    }

    pub fn is_in_bounds(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let xmin = i32::min(x1, x2);
        let xmax = i32::max(x1, x2);
        let ymin = i32::min(y1, y2);
        let ymax = i32::max(y1, y2);

        x >= xmin && y >= ymin && x <= xmax && y <= ymax
    }
}
