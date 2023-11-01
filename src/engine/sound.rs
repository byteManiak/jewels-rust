use std::collections::HashMap;

use sdl2::mixer::{Chunk, Channel, Music};

pub struct SoundManager<'a> {
    sounds: HashMap<String, Chunk>,
    music: Option<Music<'a>>
}

impl<'a> SoundManager<'a> {
    pub(crate) fn new() -> Self {
        Self {sounds: HashMap::new(), music: None}
    }

    pub(crate) fn load_sound(&mut self, path: &str, name: &str) -> Result<(), String> {
        let sound = Chunk::from_file(path)?;
        self.sounds.insert(name.to_string(), sound);

        Ok(())
    }

    pub(crate) fn play_sound(&self, name: &str) -> Result<Channel, String> {
        let name_str = name.to_string();

        if !self.sounds.contains_key(&name_str) {
            return Err(format!("Could not play audio '{name}'"));
        }

        let sound = self.sounds.get(&name_str).unwrap(); 
        let channel = sdl2::mixer::Channel::all().play(sound, 0)?;

        Ok(channel)
    }

    pub(crate) fn stop_sound(&self, channel: Channel) {
        sdl2::mixer::Channel::halt(channel);
    }

    pub(crate) fn load_music(&mut self, path: &str) {
        let music = Music::from_file(path);
        if let Ok(mus) = music {
            self.music = Some(mus);
        }
    }

    pub(crate) fn play_music(&self) {
        if let Some(music) = &self.music {
            let _ = music.play(-1);
        }
    }
}