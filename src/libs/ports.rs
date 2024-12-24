use std::{cell::RefCell, rc::Rc};

use super::{ppi::PPI, psg::PSG, vdp::Vdp};

pub struct Ports {
    vdp: Rc<RefCell<Vdp>>,
    ppi: Rc<RefCell<PPI>>,
    psg: PSG,
}

impl Ports {
    pub fn new(vdp: Rc<RefCell<Vdp>>, ppi: Rc<RefCell<PPI>>, psg: PSG) -> Self {
        Self { vdp, ppi, psg }
    }

    pub fn read_port(&self, address: u16) -> u8 {
        let ad = (address & 0xFF) as u8;
        match ad {
            0xa8..=0xab => self.ppi.borrow().read_port(ad),
            0xa0..=0xa2 => self.psg.read_port(ad),
            0x98..=0x9b => self.vdp.borrow_mut().read_port(ad),
            _ => {
                log::error!("ReadPort: {:02x}\n", ad);
                0
            }
        }
    }

    pub fn write_port(&mut self, address: u16, b: u8) {
        let ad = (address & 0xFF) as u8;
        match ad {
            0xa8..=0xab => self.ppi.borrow_mut().write_port(ad, b),
            0xa0..=0xa2 => self.psg.write_port(ad, b),
            0x90..=0x91 => {
                // Printer. Do nothing
            }
            0x98..=0x9b => self.vdp.borrow_mut().write_port(ad, b),
            0x00..=0x01 => {
                // MIDI / Sensor Kid
            }
            _ => {
                log::info!("WritePort: {:02x} -> {:02x}", ad, b);
            }
        }
    }
}
