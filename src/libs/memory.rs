use std::io::Read;
use std::{cell::RefCell, fs::File, rc::Rc};

use crate::libs::cartridges::MapperKonami4;

use super::cartridges::{get_cart_type, CartType, MapperASCII8, MapperKonami5};
use super::ppi::PPI;

pub struct NullMapper {}
impl NullMapper {
    pub fn new() -> Self {
        Self {}
    }
}

impl Mapper for NullMapper {
    fn read_byte(&self, _address: u16) -> u8 {
        0
    }

    fn write_byte(&mut self, _address: u16, _value: u8) {}
}

pub trait Mapper {
    fn is_void(&self) -> bool {
        true
    }
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}

// TODO: Secondary mapper (0xFFFF)
pub type MemoryAccessor = Rc<RefCell<Memory>>;

pub struct Memory {
    contents: Vec<u8>, //[u8; 4 * 4 * 0x4000],
    can_write: [bool; 4 * 4],
    slot_mapper: isize,
    pub(crate) ppi: Rc<RefCell<PPI>>,
    mapper: Rc<RefCell<dyn Mapper>>,
}

impl Memory {
    pub fn new(ppi: Rc<RefCell<PPI>>) -> Self {
        Self {
            contents: vec![0; 4 * 4 * 0x4000],
            can_write: [true; 4 * 4],
            slot_mapper: -1,
            ppi,
            mapper: Rc::new(RefCell::new(NullMapper::new())),
        }
    }
    pub fn save_state(&self) -> Memory {
        let mut m = Memory::new(self.ppi.clone());
        m.contents = self.contents.clone();
        m.can_write = self.can_write;
        m.slot_mapper = self.slot_mapper;
        // m.mapper = self.mapper;
        m.mapper = self.mapper.clone();
        m
    }
    pub fn restore_state(&mut self, m: Memory) {
        self.contents = m.contents.clone();
        self.can_write = m.can_write;
        self.slot_mapper = m.slot_mapper;
        self.mapper = m.mapper.clone();
        self.ppi = m.ppi;
    }
    pub fn load_bios_basic(&mut self, fname: &str) {
        let mut f = File::open(fname).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        self.load(&buffer, 0, 0);
        if buffer.len() > 0x4000 {
            // Load BASIC, if present
            self.load(&buffer[0x4000..], 1, 0);
        }
    }

    pub fn load_rom(&mut self, fname: &str, slot: usize, mapper_type: &str) {
        let mut f = File::open(fname).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        match get_cart_type(&buffer) {
            CartType::KONAMI4 => {
                log::info!("Loading ROM {} to slot 1 as type KONAMI4", fname);
                let mut mapper_konami4 = MapperKonami4::new();
                mapper_konami4.init(&buffer);
                self.set_mapper(Rc::new(RefCell::new(mapper_konami4)), slot);
                return;
            }
            CartType::KONAMI5 => {
                log::info!("Loading ROM {} to slot 1 as type KONAMI5", fname);
                let mut mapper_konami5 = MapperKonami5::new();
                mapper_konami5.init(&buffer);
                self.set_mapper(Rc::new(RefCell::new(mapper_konami5)), slot);
                return;
            }
            CartType::ASCII8KB => {
                log::info!("Loading ROM {} to slot 1 as type ASCII8KB", fname);
                let mut mapper_ascii8 = MapperASCII8::new();
                mapper_ascii8.init(&buffer);
                self.set_mapper(Rc::new(RefCell::new(mapper_ascii8)), slot);
                return;
            }
            CartType::NORMAL => {
                log::info!("Cartridge is type NORMAL");
            }
            _ => {
                unimplemented!()
            }
        }
        log::info!("Trying to load as a standard cartridge...");

        if !mapper_type.is_empty() && mapper_type == "KONAMI4" {
            let mut mapper_konami4 = MapperKonami4::new();
            mapper_konami4.init(&buffer);
            self.set_mapper(Rc::new(RefCell::new(mapper_konami4)), slot);
            return;
        }
        let num_of_pages = buffer.len() / 0x4000;
        match num_of_pages {
            1 => {
                // Load ROM to page 1, slot 1
                // TODO: mirrored????
                log::info!("Loading ROM {} to slot 1 (16KB)", fname);
                self.load(&buffer, 1, slot);
            }
            2 => {
                // Load ROM to slot 1. Mirrored pg1&pg2 <=> pg3&pg4
                log::info!("Loading ROM {} to slot 1 (32KB)", fname);
                self.load(&buffer, 0, slot);
                self.load(&buffer, 1, slot);
                self.load(&buffer[0x4000..], 2, slot);
                self.load(&buffer[0x4000..], 3, slot);
            }
            4 => {
                log::info!("Loading ROM {} to slot 1 (64KB)", fname);
                self.load(&buffer, 0, slot);
                self.load(&buffer[0x4000..], 1, slot);
                self.load(&buffer[0x8000..], 2, slot);
                self.load(&buffer[0xC000..], 3, slot);
            }
            _ => {
                log::error!("ROM size not supported")
            }
        }
    }

    // Loads 16k (one page)
    pub fn load(&mut self, data: &[u8], page: usize, slot: usize) {
        let base_addr = (page * 4 + slot) * 0x4000;
        self.contents[base_addr..(0x4000 + base_addr)].copy_from_slice(&data[..0x4000]);
        self.can_write[page * 4 + slot] = false;
    }

    pub fn set_mapper(&mut self, mapper: Rc<RefCell<dyn Mapper>>, slot: usize) {
        log::info!("Loading MegaROM in slot {}", slot);
        self.mapper = mapper;
        self.slot_mapper = slot as isize;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.read_byte_internal(address)
    }

    // ReadByteInternal reads a byte from address without taking
    // into account contention.
    pub fn read_byte_internal(&self, address: u16) -> u8 {
        let page = (address / 0x4000) as usize;
        let slot = self.ppi.borrow().pg_slots[page];

        if !self.mapper.borrow().is_void() && self.slot_mapper == slot && (page == 1 || page == 2) {
            return self.mapper.borrow().read_byte(address);
        }

        let delta = (address as usize) - page * 0x4000;
        // return self.contents[page][slot as usize][delta];
        self.contents[(page * 4 + slot as usize) * 0x4000 + delta]
    }

    // WriteByte writes a byte at address taking into account
    // contention.
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.write_byte_internal(address, value)
    }

    // WriteByteInternal writes a byte at address without taking
    // into account contention.
    fn write_byte_internal(&mut self, address: u16, value: u8) {
        let page = (address / 0x4000) as usize;
        let slot = self.ppi.borrow().pg_slots[page];

        if !self.mapper.borrow().is_void() && self.slot_mapper == slot && (page == 1 || page == 2) {
            self.mapper.borrow_mut().write_byte(address, value);
            return;
        }

        if self.can_write[page * 4 + slot as usize] {
            let delta = (address as usize) - page * 0x4000;
            // return self.contents[page][slot as usize][delta];
            self.contents[(page * 4 + slot as usize) * 0x4000 + delta] = value;
        }
    }

    pub fn contend_read(&mut self, _address: u16, _time: isize) {
        //panic("ContendRead not implemented")
    }

    pub fn contend_read_no_mreq(&mut self, _address: u16, _time: isize) {
        //panic("ContendReadNoMreq not implemented")
    }

    pub fn contend_read_no_mreq_loop(&mut self, _address: u16, _time: isize, _count: usize) {
        //panic("ContendReadNoMreq_loop not implemented")
    }

    pub fn contend_write_no_mreq(&mut self, _address: u16, _time: isize) {
        //panic("ContendWriteNoMreq not implemented")
    }

    pub fn contend_write_no_mreq_loop(&mut self, _address: u16, _time: isize, _count: usize) {
        //panic("ContendWriteNoMreq_loop not implemented")
    }
}
