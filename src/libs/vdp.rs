use std::{cell::RefCell, rc::Rc};

use super::graphics::{GraphicsDriver, GraphicsType};

pub(crate) const SCREEN0: u8 = 0;
pub(crate) const SCREEN1: u8 = 1;
pub(crate) const SCREEN2: u8 = 2;
pub(crate) const SCREEN3: u8 = 3;

#[derive(Clone)]
pub struct Vdp {
    screen_enabled: bool,
    screen_mode: u8,
    value_read: u8,
    write_state: bool,
    pub enabled_interrupts: bool,
    registers: [u8; 10],
    write_to_vram: bool,
    vram: Vec<u8>, //[u8; 0x10000]),
    pointer_vram: u16,
    status_reg: u8,
    magnif_num: u8,
    graphics: Rc<RefCell<dyn GraphicsDriver>>,
}

impl Vdp {
    pub fn new(graphics_type: GraphicsType, quality: bool) -> Self {
        let graphics = graphics_type.create(quality);
        Self {
            screen_enabled: false,
            screen_mode: 0,
            value_read: 0,
            write_state: false,
            enabled_interrupts: false,
            registers: [0; 10],
            write_to_vram: false,
            vram: vec![0; 0x10000],
            pointer_vram: 0,
            status_reg: 0,
            magnif_num: 0,
            graphics,
        }
    }

    pub fn init_graphics(&mut self) {
        self.graphics.borrow_mut().init();
    }

    pub fn save_state(&self) -> Vdp {
        self.clone()
    }

    pub fn restore_state(&mut self, vdp2: &Vdp) {
        self.screen_enabled = vdp2.screen_enabled;
        self.screen_mode = vdp2.screen_mode;
        self.value_read = vdp2.value_read;
        self.write_state = vdp2.write_state;
        self.enabled_interrupts = vdp2.enabled_interrupts;
        self.registers = vdp2.registers;
        self.write_to_vram = vdp2.write_to_vram;
        self.vram = vdp2.vram.clone();
        self.pointer_vram = vdp2.pointer_vram;
        self.status_reg = vdp2.status_reg;
        self.magnif_num = vdp2.magnif_num;
    }

    pub fn set_frame_flag(&mut self) {
        self.status_reg |= 0x80;
    }

    pub fn update_registers(&mut self) {
        self.screen_enabled = self.registers[1] & 0x40 != 0;
        self.enabled_interrupts = self.registers[1] & 0x20 != 0;
        let m1 = self.registers[1] & 0x10 != 0;
        let m2 = self.registers[1] & 0x08 != 0;
        let m3 = self.registers[0] & 0x02 != 0;
        let m4 = self.registers[0] & 0x04 != 0;
        let m5 = self.registers[0] & 0x08 != 0;
        let scm = self.screen_mode;

        if !m4 && !m5 {
            if !m1 && !m2 && !m3 {
                self.screen_mode = SCREEN1;
            }
            if m1 && !m2 && !m3 {
                self.screen_mode = SCREEN0;
            }
            if !m1 && m2 && !m3 {
                self.screen_mode = SCREEN3;
            }
            if !m1 && !m2 && m3 {
                self.screen_mode = SCREEN2;
            }
        }
        if scm != self.screen_mode {
            log::info!("Change screen mode: {}", self.screen_mode);
            self.graphics
                .borrow_mut()
                .set_logical_resolution(self.screen_mode);
        }
    }

    pub fn write_port(&mut self, ad: u8, mut val: u8) {
        // log::info!("VDP: Out({:02x}, {:02x})", ad, val);
        match ad {
            0x99 => {
                if !self.write_state {
                    self.value_read = val;
                    self.write_state = true;
                } else {
                    self.write_state = false;
                    // Bit 7 must be 1 for write
                    if val & 0x80 != 0 {
                        let regn = val - 128;
                        self.registers[regn as usize] = self.value_read;
                        self.update_registers();
                    } else {
                        self.write_to_vram = val & 0x40 != 0;
                        val &= 0xBF;
                        self.pointer_vram = 0;
                        self.pointer_vram |= self.value_read as u16;
                        self.pointer_vram |= (val as u16) << 8;
                    }
                }
            }
            0x98 => {
                // Writing to VRAM
                // log::info!("Writing to VRAM: {:04x} -> {:02x}", self.pointer_vram, val);
                self.vram[self.pointer_vram as usize] = val;
                self.pointer_vram += 1;
            }
            _ => {
                log::error!("Not implemented: VDP: Out({:02x}, {:02x})", ad, val);
                unimplemented!()
            }
        }
    }

    pub fn read_port(&mut self, ad: u8) -> u8 {
        match ad {
            0x98 => {
                // Reading from VRAM
                //log.Printf("Reading from VRAM: %04x", vdp.pointerVRAM)
                let r = self.vram[self.pointer_vram as usize];
                self.pointer_vram += 1;
                r
            }
            0x99 => {
                // Reading status register
                // TODO: look at it carefully....
                let r = self.status_reg;
                self.status_reg &= 0x7F; // Clear frame flag
                r
            }
            _ => {
                log::error!("Not implemented: VDP: In({:02x})", ad);
                0
            }
        }
    }

    pub fn graphics_render(&mut self) {
        self.graphics.borrow_mut().render();
    }
    pub fn update_buffer(&mut self) {
        if !self.screen_enabled {
            return;
        }
        let name_table_addr = (self.registers[2] as usize) << 10;
        let pat_table_addr = (self.registers[4] as usize) << 11;
        let color_table_addr = (self.registers[3] as usize) << 6;

        match self.screen_mode {
            SCREEN0 => {
                // Render SCREEN0 (40x24)
                let color1 = (self.registers[7] & 0xF0) >> 4;
                let color2 = self.registers[7] & 0x0F;
                for y in 0..24 {
                    for x in 0..40 {
                        let name_table = &self.vram[name_table_addr..];
                        let pt = name_table[(x + y * 40) as usize] as u16 * 8;
                        self.draw_patterns_s0(x * 8, y * 8, pt, pat_table_addr, color1, color2);
                    }
                }
            }
            SCREEN1 => {
                // Render SCREEN1 (32x24)
                for y in 0..24 {
                    for x in 0..32 {
                        let name_table = &self.vram[name_table_addr..];
                        let pat = name_table[(x + y * 32) as usize];
                        let color_table = &self.vram[color_table_addr..];
                        let color = color_table[(pat / 8) as usize];
                        self.draw_patterns_s1(x * 8, y * 8, pat as u16 * 8, pat_table_addr, color);
                    }
                }
                self.draw_sprites();
            }
            SCREEN2 => {
                // Render SCREEN2
                // Pattern table: 0000H to 17FFH
                let pat_table_addr = ((self.registers[4] & 0x04) as usize) << 11;
                let color_table_addr = ((self.registers[3] & 0x80) as usize) << 6;
                for y in 0..24 {
                    for x in 0..32 {
                        let name_table = &self.vram[name_table_addr..];
                        let pat = name_table[(x + y * 32) as usize];
                        self.draw_patterns_s2(
                            x * 8,
                            y * 8,
                            pat as u16 * 8,
                            pat_table_addr,
                            color_table_addr,
                        );
                    }
                }
                self.draw_sprites();
            }
            SCREEN3 => {
                // Render SCREEN3
                log::error!("Drawing in screen3 not implemented yet");
            }
            _ => {
                panic!("RenderScreen: impossible mode");
            }
        }
    }

    pub fn draw_patterns_s0(
        &mut self,
        x: u16,
        y: u16,
        pt: u16,
        pat_table_addr: usize,
        color1: u8,
        color2: u8,
    ) {
        let pat_table = &self.vram[pat_table_addr..];
        for i in 0..8 {
            let b = pat_table[(i + pt) as usize];
            let mut xx = 0;
            let mut mask = 0x80;
            while mask > 0 {
                if mask & b != 0 {
                    self.graphics.borrow_mut().draw_pixel(
                        (x + xx).into(),
                        (y + i).into(),
                        color1.into(),
                    );
                } else {
                    self.graphics.borrow_mut().draw_pixel(
                        (x + xx).into(),
                        (y + i).into(),
                        color2.into(),
                    );
                }
                xx += 1;
                mask >>= 1;
            }
        }
    }

    fn draw_patterns_s1(&mut self, x: u16, y: u16, pt: u16, pat_table_addr: usize, color: u8) {
        let color1 = (color & 0xF0) as usize >> 4;
        let color2 = (color & 0x0F) as usize;
        // let mask: u8 = 0;
        let pat_table = &self.vram[pat_table_addr..];
        for i in 0..8 {
            let b = pat_table[(i + pt) as usize];
            let mut xx = 0;
            let mut mask = 0x80;
            while mask > 0 {
                if mask & b != 0 {
                    self.graphics
                        .borrow_mut()
                        .draw_pixel((x + xx).into(), (y + i).into(), color1);
                } else {
                    self.graphics
                        .borrow_mut()
                        .draw_pixel((x + xx).into(), (y + i).into(), color2);
                }
                xx += 1;
                mask >>= 1;
            }
        }
    }

    fn draw_patterns_s2(
        &mut self,
        x: u16,
        y: u16,
        pt: u16,
        pat_table_addr: usize,
        color_table_addr: usize,
    ) {
        let mut b: u8;
        let mut color: u8;
        let pat_table = &self.vram[pat_table_addr..];
        let color_table = &self.vram[color_table_addr..];
        for i in 0..8 {
            if y < 64 {
                let idx = (i + pt) as usize;
                b = pat_table[idx];
                color = color_table[idx];
            } else if y < 128 {
                let idx = (i + pt + 2048) as usize;
                b = pat_table[idx];
                color = color_table[idx];
            } else {
                let idx = (i + pt + 2048 * 2) as usize;
                b = pat_table[idx];
                color = color_table[idx];
            }
            let color1 = (color & 0xF0) >> 4;
            let color2 = color & 0x0F;
            let mut xx = 0;
            let mut mask = 0x80;
            while mask > 0 {
                if mask & b != 0 {
                    self.graphics.borrow_mut().draw_pixel(
                        (x + xx).into(),
                        (y + i).into(),
                        color1.into(),
                    );
                } else {
                    self.graphics.borrow_mut().draw_pixel(
                        (x + xx).into(),
                        (y + i).into(),
                        color2.into(),
                    );
                }
                xx += 1;
                mask >>= 1;
            }
        }
    }

    fn draw_sprites(&mut self) {
        // Sprite name table: 1B00H to 1B7FH
        // Sprite pattern table: 3800H to 3FFFH
        let spr_table_addr = (self.registers[5] as usize) << 7;
        let spr_pat_table_addr = (self.registers[6] as usize) << 11;
        let magnif = (self.registers[1] & 0x01) != 0;
        let spr16x16 = (self.registers[1] & 0x02) != 0;
        self.magnif_num = 0;
        let mut i = 0;
        let mut j = 0;
        while i < 32 {
            let ypos = self.vram[spr_table_addr + j] as u16;
            if ypos == 0xd0 {
                // Ignore all sprites
                return;
            }
            let mut xpos = self.vram[spr_table_addr + j + 1] as u16;
            let pattern = self.vram[spr_table_addr + j + 2];
            let ec = (self.vram[spr_table_addr + j + 3] & 0x80) != 0;
            if ec {
                xpos -= 32
            }
            let color = self.vram[spr_table_addr + j + 3] & 0x0F;
            if !spr16x16 {
                let pattern_addr = spr_pat_table_addr + (pattern as usize) * 8;
                // self.draw_spr(magnif, xpos, ypos, pattern_addr, ec, color);
                self.draw_spr(magnif, xpos, ypos, pattern_addr, color);
            } else {
                let pattern_addr = spr_pat_table_addr + ((pattern >> 2) as usize) * 8 * 4;
                // self.draw_spr(magnif, xpos, ypos, pattern_addr, ec, color);
                // self.draw_spr(magnif, xpos, ypos + 8, pattern_addr + 8, ec, color);
                // self.draw_spr(magnif, xpos + 8, ypos, pattern_addr + 16, ec, color);
                // self.draw_spr(magnif, xpos + 8, ypos + 8, pattern_addr + 24, ec, color);
                self.draw_spr(magnif, xpos, ypos, pattern_addr, color);
                self.draw_spr(magnif, xpos, ypos + 8, pattern_addr + 8, color);
                self.draw_spr(magnif, xpos + 8, ypos, pattern_addr + 16, color);
                self.draw_spr(magnif, xpos + 8, ypos + 8, pattern_addr + 24, color);
            }

            i += 1;
            j += 4;
        }
    }

    fn draw_spr(
        &mut self,
        magnif: bool,
        xpos: u16,
        ypos: u16,
        pattern_addr: usize,
        // ec: bool,
        color: u8,
    ) {
        if ypos > 191 {
            return;
        }

        let pattern = &self.vram[pattern_addr..];
        for y in 0..8 {
            let b = pattern[y as usize];
            let mut x = 0;
            let mut mask = 0x80;
            while mask > 0 {
                if magnif && x == 0 && y == 0 {
                    if self.magnif_num == 4 {
                        self.magnif_num = 1;
                    } else {
                        self.magnif_num += 1;
                    }
                }
                if mask & b != 0 {
                    if magnif {
                        let mut x = x * 2;
                        let mut y = y * 2;
                        if self.magnif_num == 2 || self.magnif_num == 4 {
                            y += 8;
                        }
                        if self.magnif_num == 3 || self.magnif_num == 4 {
                            x += 8;
                        }
                        self.graphics.borrow_mut().draw_pixel(
                            (xpos + x).into(),
                            (ypos + y).into(),
                            color.into(),
                        );
                        self.graphics.borrow_mut().draw_pixel(
                            (xpos + x + 1).into(),
                            (ypos + y).into(),
                            color.into(),
                        );
                        self.graphics.borrow_mut().draw_pixel(
                            (xpos + x).into(),
                            (ypos + y + 1).into(),
                            color.into(),
                        );
                        self.graphics.borrow_mut().draw_pixel(
                            (xpos + x + 1).into(),
                            (ypos + y + 1).into(),
                            color.into(),
                        );
                    } else {
                        self.graphics.borrow_mut().draw_pixel(
                            (xpos + x).into(),
                            (ypos + y).into(),
                            color.into(),
                        );
                    }
                }
                x += 1;

                mask >>= 1;
            }
        }
    }
}
