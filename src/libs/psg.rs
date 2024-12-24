/*

    Example in msx basic:

    sound 0,105          // Set tone A frequency
    sound 1,0
    sound 7, &B10111110  // enable tone generator A
    sound 8, &B00010000  // Set amplitude for channel A (envelope)

    sound 11, 0          // Frequency for envelope
    sound 12, 20

    sound 13, &B00001000 // Select envelope "1000"


*/

use std::{cell::RefCell, rc::Rc};

use super::{sound::SoundDriver, tone_generator::ToneGenerator};

pub struct PSG {
    sound: Rc<RefCell<dyn SoundDriver>>,
    registers: [u8; 16],
    reg_next: u8,
    // bytes_cass: Vec<u8>,
    sound_tones: [ToneGenerator; 3],
}

impl PSG {
    pub fn new(sound: Rc<RefCell<dyn SoundDriver>>) -> Self {
        for i in 0..3 {
            let _ = sound.borrow_mut().add_channel(i);
        }
        Self {
            sound,
            registers: [0; 16],
            reg_next: 0,
            // bytes_cass: vec![0_u8],
            sound_tones: [
                ToneGenerator::new(),
                ToneGenerator::new(),
                ToneGenerator::new(),
            ],
        }
    }
    // pub fn feed_samples(&mut self, data: &mut [i16]) {
    //     for i in 0..3 {
    //         self.sound_tones[i].feed_samples(data);
    //     }
    // }

    pub fn write_port(&mut self, ad: u8, val: u8) {
        match ad {
            0xa0 => {
                // Register write port
                self.reg_next = val;
            }
            0xa1 => {
                // Write value to port
                self.registers[self.reg_next as usize] = val;
                if self.reg_next < 14 {
                    for i in 0..3 {
                        self.do_tones(i)
                    }

                    // TODO: sound_doNoises()
                }
            }
            _ => {
                log::error!("Sound, not implemented: out({:02x},{:02x})", ad, val);
            }
        }
    }

    pub fn read_port(&self, ad: u8) -> u8 {
        if ad == 0xa2 {
            // Read value from port
            if self.reg_next == 0x0e {
                // joystick triggers i cassete input
                let bit_cass = self.cassete_get_next_bit() << 7;
                // For now we set it to 1 (no joystick movement)
                return 0x3f | bit_cass;
            }
            if self.reg_next == 0x0f {
                // PSG port 15 (joystick select)
                // TODO: improve
                return 0;
            }
            return self.registers[self.reg_next as usize];
        }

        log::error!("Sound, not implemented: in({:02x})", ad);
        0
    }

    pub fn cassete_get_next_bit(&self) -> u8 {
        // log.Println("NextByte")
        0
    }

    // TODO: envelopes
    pub fn do_tones(&mut self, chn: usize) {
        let freq =
            (((self.registers[chn * 2 + 1] & 0x0f) as u16) << 8) | (self.registers[chn * 2]) as u16;
        let envelope_enabled = (self.registers[8 + chn] & 0x10) != 0;
        if freq > 0 {
            let real_freq = 111861_f32 / (freq as f32);
            if envelope_enabled {
                // envFreq := (uint16(psg.registers[12]) << 8) | uint16(psg.registers[11])
                // envShape := psg.registers[13] & 0x0F
                // sound_tones[chn].setEnvelope(envFreq, envShape)
            } else {
                let volume = (self.registers[8 + chn] & 0x0F) as f32;
                self.sound_tones[chn].set_volume(volume);
                self.sound_tones[chn].set_frequency(real_freq);
            }
        }
        let is_active = self.registers[7] & (0x01 << chn) == 0;
        self.sound_tones[chn].activate(is_active);
        if is_active {
            if freq > 0 {
                self.feed_samples(chn);
                let _ = self.sound.borrow_mut().play(chn);
            }
        } else {
            // unreachable!();
            let _ = self.sound.borrow_mut().pause(chn);
        }
    }
    fn feed_samples(&mut self, chn: usize) {
        let freq = self.sound_tones[chn].get_frequency();
        let _ = self
            .sound
            .borrow_mut()
            .feed_samples(chn, freq, self.sound_tones[chn].get_volume());
    }
}

// func psg_loadCassette(fileName string) {
// 	var err error
// 	psg_bytesCass, err = ioutil.ReadFile(fileName)
// 	if err != nil {
// 		log.Println(err)
// 		psg_bytesCass = nil
// 	}
// 	log.Println("PSG: Loaded cassete:", fileName)
// }

// func sound_doNoises() {
// 	// if (self.registers[7] & 0x38) == 0x38 {
// 	// 	// sound_noise.activate(false)
// 	// } else {
// 	// 	freq := int(self.registers[6] & 0x1F)
// 	//
// 	// 	if freq > 0 {
// 	// 		realFreq := float32(111861) / float32(freq)
// 	//
// 	// 		var vol float32 = 0
// 	// 		if (self.registers[7] & 0x20) == 0 {
// 	// 			v := float32(self.registers[8] & 0x0F)
// 	// 			if v > vol {
// 	// 				vol = v
// 	// 			}
// 	// 		}
// 	//
// 	// 		if (self.registers[7] & 0x10) == 0 {
// 	// 			v := float32(self.registers[9] & 0x0F)
// 	// 			if v > vol {
// 	// 				vol = v
// 	// 			}
// 	// 		}
// 	//
// 	// 		if (self.registers[7] & 0x04) == 0 {
// 	// 			v := float32(self.registers[10] & 0x0F)
// 	// 			if v > vol {
// 	// 				vol = v
// 	// 			}
// 	// 		}
// 	//
// 	// 		// sound_noise.setParameters(realFreq, vol)
// 	// 		// sound_noise.activate(true)
// 	// 	}
// 	// }
// }
