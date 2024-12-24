pub const FREQUENCY: i32 = 22050;
// pub const FREQUENCY: i32 = 44100;

use std::collections::HashMap;

use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    AudioSubsystem,
};

pub trait SoundDriver {
    fn add_channel(&mut self, channel: usize) -> Result<String, String>;
    // pub fn feed_samples(&mut self, channel: usize, wave: &[i16]) -> Result<String, String> {
    fn feed_samples(&mut self, channel: usize, freq: f32, volume: f32) -> Result<String, String>;
    fn pause(&mut self, channel: usize) -> Result<String, String>;
    fn play(&mut self, channel: usize) -> Result<String, String>;
}

pub struct SoundNull {}

impl Default for SoundNull {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundNull {
    pub fn new() -> Self {
        Self {}
    }
}
impl SoundDriver for SoundNull {
    fn add_channel(&mut self, _channel: usize) -> Result<String, String> {
        Ok("null sound driver".to_string())
    }
    fn feed_samples(
        &mut self,
        _channel: usize,
        _freq: f32,
        _volume: f32,
    ) -> Result<String, String> {
        Ok("null sound driver".to_string())
    }
    fn pause(&mut self, _channel: usize) -> Result<String, String> {
        Ok("null sound driver".to_string())
    }
    fn play(&mut self, _channel: usize) -> Result<String, String> {
        Ok("null sound driver".to_string())
    }
}

#[allow(dead_code)]
enum Tone {
    Square,
    Sin,
}

struct Oscillator {
    current_step: f32,
    step_size: f32,
    volume: f32,
    mode: Tone,
}
impl Oscillator {
    pub fn new(rate: f32, volume: f32, mode: Tone) -> Self {
        Self {
            current_step: 0.0,
            volume,
            step_size: 2.0 * std::f32::consts::PI / rate,
            mode,
        }
    }
    pub fn next(&mut self) -> f32 {
        self.current_step += self.step_size;
        match self.mode {
            Tone::Square => {
                if f32::sin(self.current_step) > 0.0 {
                    self.volume
                } else {
                    -self.volume
                }
            }
            Tone::Sin => f32::sin(self.current_step) * self.volume,
        }
    }
    pub fn modify(&mut self, rate: f32, volume: f32) {
        self.step_size = 2.0 * std::f32::consts::PI / rate;
        self.volume = volume;
    }
}

impl AudioCallback for Oscillator {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = self.next();
        }
    }
}

pub struct Sound {
    audio_subsystem: AudioSubsystem,
    // devices: HashMap<usize, AudioQueue<i16>>,
    devices: HashMap<usize, AudioDevice<Oscillator>>,
}

impl Default for Sound {
    fn default() -> Self {
        Self::new()
    }
}

impl Sound {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();
        Self {
            audio_subsystem,
            devices: HashMap::new(),
        }
    }
}
impl SoundDriver for Sound {
    fn add_channel(&mut self, channel: usize) -> Result<String, String> {
        // log::info!("add_channel channel:{}", channel);
        if self.devices.contains_key(&channel) {
            return Err(format!("cannot add same channel {}", channel));
        }
        let desired_spec = AudioSpecDesired {
            freq: Some(FREQUENCY),
            channels: Some(1), // mono
            samples: Some(512),
        };

        let device = self
            .audio_subsystem
            .open_playback(None, &desired_spec, |_spec| {
                // initialize the audio callback
                Oscillator::new(1.0, 0.0, Tone::Sin)
            })?;
        self.devices.insert(channel, device);
        Ok("channel added".to_string())
    }
    // pub fn feed_samples(&mut self, channel: usize, wave: &[i16]) -> Result<String, String> {
    fn feed_samples(&mut self, channel: usize, freq: f32, volume: f32) -> Result<String, String> {
        let audio_queue = self
            .devices
            .get_mut(&channel)
            .ok_or(format!("no such channel {}", channel))?;

        {
            audio_queue
                .lock()
                .modify(FREQUENCY as f32 / freq, volume / 15.0);
        }

        Ok("sound queue".to_string())
    }
    fn pause(&mut self, channel: usize) -> Result<String, String> {
        let audio_queue = self
            .devices
            .get(&channel)
            .ok_or(format!("no such channel {}", channel))?;
        audio_queue.pause();
        Ok("sound pause".to_string())
    }
    fn play(&mut self, channel: usize) -> Result<String, String> {
        let audio_queue = self
            .devices
            .get_mut(&channel)
            .ok_or(format!("no such channel {}", channel))?;

        // Start playback
        audio_queue.resume();

        Ok("sound play".to_string())
    }
}
