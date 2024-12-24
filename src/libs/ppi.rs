use crate::libs::key_matrix::key_matrix;

pub struct PPI {
    slots: u8,
    reg_c: u8,
    pub pg_slots: [isize; 4],
}

impl Default for PPI {
    fn default() -> Self {
        Self::new()
    }
}

impl PPI {
    pub fn new() -> Self {
        Self {
            slots: 0,
            reg_c: 0,
            pg_slots: [0; 4],
        }
    }
    pub fn refresh_slots_values(&mut self) {
        self.pg_slots[0] = (self.slots & 0x03).into();
        self.pg_slots[1] = ((self.slots & 0x0C) >> 2).into();
        self.pg_slots[2] = ((self.slots & 0x30) >> 4).into();
        self.pg_slots[3] = ((self.slots & 0xC0) >> 6).into();
    }
    pub fn write_port(&mut self, ad: u8, val: u8) {
        match ad {
            0xab => {
                if val & 0x80 != 0 {
                    // log.Println("PPI initialization");
                    log::info!("PPI initialization");
                } else {
                    let bit_n = (val & 0x0f) >> 1;
                    if (val & 0x01) != 0 {
                        self.reg_c |= 0x01 << bit_n;
                    } else {
                        self.reg_c &= !(0x01 << bit_n);
                    }
                }
            }
            0xa8 => {
                self.slots = val;
                self.refresh_slots_values();
            }
            0xaa => {
                self.reg_c = val;
            }
            _ => {
                log::error!("PPI: not implemented: out({:02x},{:02x})", ad, val);
                unimplemented!()
            }
        }
    }
    pub fn read_port(&self, ad: u8) -> u8 {
        match ad {
            0xa8 => {
                // log::info!("Get slots: {:02x}", self.slots);
                self.slots
            }
            0xaa => {
                self.reg_c
            }
            0xa9 => {
                key_matrix((self.reg_c & 0x0f).into())
            }
            _ => {
                log::error!("PPI: not implemented: in({:02x})", ad);
                unimplemented!()
            }
        }
    }
}
