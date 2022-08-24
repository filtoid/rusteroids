use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::cpal::CpalBackend,
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use std::collections::HashMap;

pub struct SoundManager {
    pub sound_manager: AudioManager::<CpalBackend>,
    pub sounds: HashMap::<String, StaticSoundData>
}

impl SoundManager {
    pub fn new() -> Self {
        Self {
            sound_manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("Failed to load Kira Audio Engine"),
            sounds: HashMap::new()
        }
    }

    pub fn play_sound(&mut self, filename: String) {
        if self.sounds.contains_key(&filename){
            if let Some(x) = self.sounds.get_mut(&filename) {
                self.sound_manager.play(x.clone()).expect("Failed to play sounds");
            }
        }else{
            println!("Sound doesn't exist");
        }
    }

    pub fn load_sound(&mut self, filename: &String) {
        self.sounds.entry( (&filename).to_string() ).or_insert(
            StaticSoundData::from_file(
                filename, StaticSoundSettings::default()
            ).expect("Failed to load sound")
        );
    }
}