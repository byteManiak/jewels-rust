use std::collections::HashMap;

use sdl2::mixer::{Chunk, Channel};

pub struct SoundManager {
    sounds: HashMap<String, Chunk>
}

impl SoundManager {
    pub(crate) fn new() -> Self {
        Self {sounds: HashMap::new()}
    }
    pub(crate) fn load_sound(&mut self, path: &str, name: &str) -> Result<(), String> {
        let sound = Chunk::from_file(path)?;
        self.sounds.insert(name.to_string(), sound);

        Ok(())
    }

    pub fn play_sound(&self, name: &str) -> Result<Channel, String> {
        let name_str = name.to_string();

        if !self.sounds.contains_key(&name_str) {
            return Err(format!("Could not play audio '{name}'"));
        }

        let sound = self.sounds.get(&name_str).unwrap(); 
        let channel = sdl2::mixer::Channel::all().play(sound, 0)?;

        Ok(channel)
    }

    pub fn stop_sound(&self, channel: Channel) {
        sdl2::mixer::Channel::halt(channel);
    }
}