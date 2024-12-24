use super::sound::FREQUENCY;

pub struct ToneGenerator {
    amp: f32,
    freq: f32,
    // count: usize,
    active: bool,
    w_form: [i16; 32],
    index: f32,
}

const SQ_WAVE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255,
];
impl Default for ToneGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ToneGenerator {
    pub fn new() -> Self {
        let mut sd = Self {
            amp: 0.0,
            freq: 0.0,
            // count: 0,
            active: false,
            w_form: [0; 32],
            index: 0.0,
        };
        sd.update_waveform(&SQ_WAVE);
        sd
    }

    pub fn update_waveform(&mut self, data: &[u8]) {
        // Set waveform of tone generator
        // TODO: implement
        for (i, item) in data.iter().enumerate() {
            self.w_form[i] = *item as i16 - 127;
        }
        // for i in 0..data.len() {
        //     self.w_form[i] = (data[i]) as i16 - 127;
        // }
    }
    pub fn set_volume(&mut self, volume: f32) {
        self.amp = volume / 2.0;
    }

    pub fn get_volume(&mut self) -> f32 {
        self.amp
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }

    pub fn get_frequency(&mut self) -> f32 {
        self.freq
    }

    pub fn activate(&mut self, par: bool) {
        self.active = par;
    }

    pub fn feed_samples(&mut self, data: &mut [i16]) {
        if !self.active || self.freq == 0.0 || self.amp == 0.0 {
            return;
        }

        if self.freq > (FREQUENCY as f32 / 2.0) {
            return;
        }
        let n_samples = (FREQUENCY as f32) / (self.freq);
        let delta = 32.0 / n_samples;
        for item in data.iter_mut() {
            *item += ((self.w_form[self.index as usize] as f32) * self.amp) as i16;
            self.index += delta;
            while self.index >= 32.0 {
                self.index -= 32.0;
            }
        }
    }
}
