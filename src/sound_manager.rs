use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::cpal::CpalBackend,
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings, StaticSoundHandle, PlaybackState},
    tween::Tween,
    LoopBehavior,
};
use std::collections::HashMap;

pub struct SoundManager {
    pub sound_manager: AudioManager::<CpalBackend>,
    pub sounds: HashMap::<String, StaticSoundData>,
    pub loop_sounds: HashMap::<String, StaticSoundHandle>
}

impl SoundManager {
    pub fn new() -> Self {
        Self {
            sound_manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("Failed to load Kira Audio Engine"),
            sounds: HashMap::new(),
            loop_sounds: HashMap::new(),
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

    pub fn start_sound(&mut self, filename: &String) {
        match self.sounds.get_mut(filename) {
            Some(x) => {
                match self.sound_manager.play(x.clone()) {
                    Ok(res) => {
                        self.loop_sounds.entry( filename.to_string() )
                                        .or_insert(res);
                    },
                    Err(e) => {
                        println!("{}", e);
                    } 
                }
            },
            None => {
                println!("Error when playing music");
            }
        }
    }

    pub fn stop_sound(&mut self, filename: &String) {
        if self.loop_sounds.contains_key(&filename.to_string()){
            if let Some(x) = self.loop_sounds.get_mut(&filename.to_string()) {
                if x.state() == PlaybackState::Playing {
                    // Pausing sound
                    x.pause(Tween::default()).expect("Failed to pause sound");
                }
            }
        }
    }

    pub fn resume_sound(&mut self,filename: &String) {
        if self.loop_sounds.contains_key(&filename.to_string()) {
            if let Some(x) = self.loop_sounds.get_mut(&filename.to_string()) {
                if x.state() == PlaybackState::Paused  {
                    // Resuming sound
                    x.resume(Tween::default()).expect("Failed to resume sound");
                } else if x.state() == PlaybackState::Stopped {
                    // Restarting sound after it has stopped
                    x.resume(Tween::default()).expect("Failed to restart sound");
                } 
            }
        }else{
            // Starting the sound for the first time
            self.start_sound(filename);
        }
    }

    pub fn load_sound(&mut self, filename: &String, looped: bool) {
        let mut sss = StaticSoundSettings::default();
        if looped {
            sss.loop_behavior = Some(LoopBehavior{start_position:0.0});
        }
        
        self.sounds.entry( (&filename).to_string() ).or_insert(
            StaticSoundData::from_file(
                filename, 
                sss
            ).expect("Failed to load sound")
        );
    }
}