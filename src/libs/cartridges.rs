use sha1::{Digest, Sha1};

use crate::libs::rom_database::search_in_rom_database;

use super::memory::Mapper;

pub enum CartType {
    NORMAL = 0,
    UNKNOWN,
    KONAMI4,
    KONAMI5,
    ASCII8KB,
    ASCII16KB,
    RTYPE,
}

pub fn get_cart_type(data: &[u8]) -> CartType {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hash = format!("{:016x}", result);
    log::info!("Hash: {}", hash);
    if let Ok(str) = search_in_rom_database(&hash) {
        match str.as_str() {
            "NORMAL" => CartType::NORMAL,
            "Konami" => CartType::KONAMI4,
            "KonamiSCC" => CartType::KONAMI5,
            "ASCII8" => CartType::ASCII8KB,
            _ => {
                log::info!("Rom {} not supported\n", str);
                unimplemented!()
            }
        }
    } else {
        log::error!("Rom hash {} not supported\n", hash);
        unreachable!()
    }
}

#[derive(Clone)]
pub struct MapperKonami4 {
    contents: Vec<u8>,
    sels: [usize; 4],
}

impl Default for MapperKonami4 {
    fn default() -> Self {
        Self::new()
    }
}

impl MapperKonami4 {
    pub fn new() -> Self {
        Self {
            contents: vec![0_u8],
            sels: [0; 4],
        }
    }
    pub fn init(&mut self, data: &[u8]) {
        self.contents = data.to_vec();
        self.sels = [0, 1, 2, 3];
    }
}

impl Mapper for MapperKonami4 {
    fn read_byte(&self, mut address: u16) -> u8 {
        address -= 0x4000;
        let place = address / 0x2000;
        let real_mem = &self.contents[self.sels[place as usize] * 0x2000_usize..];
        let delta = address - 0x2000 * place;
        real_mem[delta as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        let address = address - 0x4000;
        let place = address / 0x2000;
        if place == 0 {
            return;
        }
        self.sels[place as usize] = value as usize;
    }
}

#[derive(Clone)]
pub struct MapperKonami5 {
    contents: Vec<u8>,
    num_banks: isize,
    sels: [usize; 4],
    scc: [u8; 0x800],
}

impl Default for MapperKonami5 {
    fn default() -> Self {
        Self::new()
    }
}

impl MapperKonami5 {
    pub fn new() -> Self {
        Self {
            contents: vec![0_u8],
            num_banks: 0,
            sels: [0; 4],
            scc: [0; 0x800],
        }
    }
    pub fn init(&mut self, data: &[u8]) {
        self.contents = data.to_vec();
        self.num_banks = (data.len() / 8192) as isize;
        self.sels = [0, 1, 2, 3];
    }
}

impl Mapper for MapperKonami5 {
    fn read_byte(&self, mut address: u16) -> u8 {
        if (self.sels[2] & 0x3f == 0x3f) && (0x9800..=0x9fff).contains(&address) {
            // SCC Area
            return self.scc[address as usize - 0x9800];
        }
        address -= 0x4000;
        let place = address / 0x2000;
        let real_mem = &self.contents[self.sels[place as usize] * 0x2000_usize..];
        let delta = address - 0x2000 * place;
        real_mem[delta as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        if (self.sels[2] & 0x3f == 0x3f) && (0x9800..=0x9fff).contains(&address) {
            // SCC AREA
            unimplemented!();
            // scc_write(address - 0x9800, value);
            // self.scc[address-0x9800] = value
            // return;
        }

        if (0x5000..=0x57ff).contains(&address) {
            self.sels[0] = value.into();
            return;
        }

        if (0x7000..=0x77ff).contains(&address) {
            self.sels[1] = value.into();
            return;
        }

        if (0x9000..=0x97ff).contains(&address) {
            self.sels[2] = value.into();
            return;
        }

        if (0xb000..=0xb7ff).contains(&address) {
            self.sels[3] = value.into();
            return;
        }

        // TODO: Fix for SCC...
        let address = address - 0x4000;
        let place = address / 0x2000;
        if place == 0 {
            return;
        }
        self.sels[place as usize] = value as usize;
    }
}

#[derive(Clone)]
pub struct MapperASCII8 {
    contents: Vec<u8>,
    num_banks: isize,
    sels: [usize; 4],
}

impl Default for MapperASCII8 {
    fn default() -> Self {
        Self::new()
    }
}

impl MapperASCII8 {
    pub fn new() -> Self {
        Self {
            contents: vec![0_u8],
            num_banks: 0,
            sels: [0; 4],
        }
    }
    pub fn init(&mut self, data: &[u8]) {
        self.contents = data.to_vec();
        self.num_banks = (data.len() / 8192) as isize;
        self.sels = [0, 1, 2, 3];
    }
}

impl Mapper for MapperASCII8 {
    fn read_byte(&self, mut address: u16) -> u8 {
        address -= 0x4000;
        let place = address / 0x2000;
        let real_mem = &self.contents[self.sels[place as usize] * 0x2000_usize..];
        let delta = address - 0x2000 * place;
        real_mem[delta as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        if (0x6000..=0x67ff).contains(&address) {
            self.sels[0] = (value % self.num_banks as u8).into();
            return;
        }

        if (0x6800..=0x6fff).contains(&address) {
            self.sels[1] = (value % self.num_banks as u8).into();
            return;
        }

        if (0x7000..=0x77ff).contains(&address) {
            self.sels[2] = (value % self.num_banks as u8).into();
            return;
        }

        if (0x7800..=0x7fff).contains(&address) {
            self.sels[3] = (value % self.num_banks as u8).into();
        }
    }
}
