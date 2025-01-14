/*

Copyright (c) 2010 Andrea Fazzi

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

/*
The z80 package implements a Zilog Z80 emulator.
*/
use serde::{Deserialize, Serialize};

use crate::libs::{
    memory::{Memory, MemoryData},
    ports::Ports,
    ppi::PPIData,
};

use super::z80_tables::{
    Z80Tables, HALF_CARRY_ADD_TABLE, HALF_CARRY_SUB_TABLE, OVERFLOW_ADD_TABLE, OVERFLOW_SUB_TABLE,
};

// The flags
pub(crate) const FLAG_C: u8 = 0x01;
pub(crate) const FLAG_N: u8 = 0x02;
pub(crate) const FLAG_P: u8 = 0x04;
pub(crate) const FLAG_V: u8 = FLAG_P;
pub(crate) const FLAG_3: u8 = 0x08;
pub(crate) const FLAG_H: u8 = 0x10;
pub(crate) const FLAG_5: u8 = 0x20;
pub(crate) const FLAG_Z: u8 = 0x40;
pub(crate) const FLAG_S: u8 = 0x80;

pub(crate) const SHIFT_0X_CB: u16 = 256;
pub(crate) const SHIFT_0X_ED: u16 = 512;
pub(crate) const SHIFT_0X_DD: u16 = 768;
pub(crate) const SHIFT_0X_DDCB: u16 = 1024;
pub(crate) const SHIFT_0X_FDCB: u16 = 1024;
pub(crate) const SHIFT_0X_FD: u16 = 1280;

/// returns (high, low)
pub(crate) fn split_word(word: u16) -> (u8, u8) {
    ((word >> 8) as u8, (word & 0xff) as u8)
}

pub(crate) fn join_bytes(h: u8, l: u8) -> u16 {
    (l as u16) | ((h as u16) << 8)
}

pub(crate) fn tern_op_b(cond: bool, ret1: u8, ret2: u8) -> u8 {
    if cond {
        return ret1;
    }
    ret2
}

pub(crate) fn sign_extend(v: u8) -> i16 {
    i16::from(v as i8)
}

pub struct Register16 {
    high: u8,
    low: u8,
}

impl Register16 {
    pub fn new(high: u8, low: u8) -> Self {
        Self { high, low }
    }
    /// return high,low
    pub fn result(&self) -> (u8, u8) {
        (self.high, self.low)
    }
    pub fn inc(&mut self) {
        let temp = self.get() + 1;
        self.high = (temp >> 8) as u8;
        self.low = (temp & 0xff) as u8;
    }

    pub fn dec(&mut self) {
        let temp = self.get() - 1;
        self.high = (temp >> 8) as u8;
        self.low = (temp & 0xff) as u8;
    }

    pub fn set(&mut self, value: u16) {
        // *r.high, *r.low = splitWord(value)
        (self.high, self.low) = split_word(value);
    }

    pub fn get(&mut self) -> u16 {
        // return joinBytes(self.high, self.low)
        join_bytes(self.high, self.low)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Z80Data {
    pub(crate) A: u8,
    pub(crate) F: u8,
    pub(crate) B: u8,
    pub(crate) C: u8,
    pub(crate) D: u8,
    pub(crate) E: u8,
    pub(crate) H: u8,
    pub(crate) L: u8,
    pub(crate) A_: u8,
    pub(crate) F_: u8,
    pub(crate) B_: u8,
    pub(crate) C_: u8,
    pub(crate) D_: u8,
    pub(crate) E_: u8,
    pub(crate) H_: u8,
    pub(crate) L_: u8,
    pub(crate) IXH: u8,
    pub(crate) IXL: u8,
    pub(crate) IYH: u8,
    pub(crate) IYL: u8,
    pub(crate) I: u8,
    pub(crate) IFF1: u8,
    pub(crate) IFF2: u8,
    pub(crate) IM: u8, // interrupt mode

    // The highest bit (bit 7) of the R register
    pub(crate) R7: u8,

    // The low 7 bits of the R register. 16 bits long so it can
    // also act as an RZX instruction counter.
    pub(crate) R: u16,

    pub(crate) sp: u16,
    pc: u16,

    // bc: register16,
    // bc_: register16,
    // hl: register16,
    // hl_: register16,
    // af: register16,
    // de: register16,
    // de_: register16,
    // ix: register16,
    // iy: register16,
    pub(crate) event_next_event: isize,

    // Number of t_states since the beginning of the last frame.
    // The value of this variable is usually smaller than TStatesPerFrame,
    // but in some unlikely circumstances it may be >= than that.
    pub(crate) t_states: isize,

    pub(crate) halted: bool,

    // Needed when executing opcodes prefixed by 0xCB
    pub(crate) temp_addr: u16,

    pub(crate) interrupts_enabled_at: isize,

    pub(crate) rzx_instructions_offset: isize,

    // Clock Cycles
    pub(crate) cycles: u64,
    pub(crate) debug: bool,
}
impl Z80Data {
    pub fn new() -> Self {
        Self {
            A: 0,
            F: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            A_: 0,
            F_: 0,
            B_: 0,
            C_: 0,
            D_: 0,
            E_: 0,
            H_: 0,
            L_: 0,
            IXH: 0,
            IXL: 0,
            IYH: 0,
            IYL: 0,
            I: 0,
            IFF1: 0,
            IFF2: 0,
            IM: 0,

            R7: 0,

            R: 0,

            sp: 0,
            pc: 0,

            event_next_event: 0,

            t_states: 0,

            halted: false,

            temp_addr: 0,

            interrupts_enabled_at: 0,

            rzx_instructions_offset: 0,

            cycles: 0,

            debug: false,
        }
    }
    pub fn reset(&mut self) {
        (
            self.A, self.F, self.B, self.C, self.D, self.E, self.H, self.L,
        ) = (0, 0, 0, 0, 0, 0, 0, 0);
        (
            self.A_, self.F_, self.B_, self.C_, self.D_, self.E_, self.H_, self.L_,
        ) = (0, 0, 0, 0, 0, 0, 0, 0);
        (self.IXH, self.IXL, self.IYH, self.IYL) = (0, 0, 0, 0);

        (
            self.sp, self.I, self.R, self.R7, self.pc, self.IFF1, self.IFF2, self.IM,
        ) = (0, 0, 0, 0, 0, 0, 0, 0);

        self.t_states = 0;

        self.halted = false;
        self.interrupts_enabled_at = 0;
    }
}
impl Default for Z80Data {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(non_snake_case)]
pub struct Z80 {
    pub(crate) data: Z80Data,

    pub(crate) tables: Z80Tables,
    pub(crate) memory: Memory,
    ports: Ports,
}

#[allow(non_snake_case)]
impl Z80 {
    pub fn new(memory: Memory, ports: Ports) -> Self {
        Self {
            data: Z80Data::default(),
            memory,
            ports,
            tables: Z80Tables::new(),
        }
    }

    // Reset resets the Z80.
    pub fn reset(&mut self) {
        self.data.reset();
    }
    pub fn reset_cycles(&mut self) {
        self.data.cycles = 0;
    }
    pub fn get_cycles(&self) -> u64 {
        self.data.cycles
    }
    pub fn is_halted(&self) -> bool {
        self.data.halted
    }
    // Interrupt process a Z80 maskable interrupt
    pub fn interrupt(&mut self) {
        if self.data.IFF1 != 0 {
            if self.data.halted {
                self.data.pc += 1;
                self.data.halted = false;
            }

            self.data.t_states += 7;

            self.data.R = (self.data.R + 1) & 0x7f;
            (self.data.IFF1, self.data.IFF2) = (0, 0);

            // push PC
            {
                let (pch, pcl) = split_word(self.data.pc);

                self.data.sp -= 1;
                self.memory.write_byte(self.data.sp, pch);
                self.data.sp -= 1;
                self.memory.write_byte(self.data.sp, pcl);
            }

            match self.data.IM {
                0 | 1 => {
                    self.data.pc = 0x0038;
                }
                2 => {
                    let mut int_temp: u16 = ((self.data.I as u16) << 8) | 0xff;
                    let pcl = self.memory.read_byte(int_temp);
                    int_temp += 1;
                    let pch = self.memory.read_byte(int_temp);
                    self.data.pc = join_bytes(pch, pcl);
                }
                _ => {
                    panic!("Unknown interrupt mode");
                }
            }
        }
    }
    // Process a Z80 non-maskable interrupt.
    pub fn non_maskable_interrupt(&mut self) {
        if self.data.halted {
            self.data.pc += 1;
            self.data.halted = false;
        }

        self.data.t_states += 7;

        self.data.R = (self.data.R + 1) & 0x7f;
        (self.data.IFF1, self.data.IFF2) = (0, 0);

        // push PC
        {
            let (pch, pcl) = split_word(self.data.pc);

            self.data.sp -= 1;
            self.memory.write_byte(self.data.sp, pch);
            self.data.sp -= 1;
            self.memory.write_byte(self.data.sp, pcl);
        }

        self.data.pc = 0x0066;
    }
    pub fn jp(&mut self) {
        let mut jp_temp: u16 = self.data.pc;
        let pcl = self.memory.read_byte(jp_temp);
        jp_temp += 1;
        let pch = self.memory.read_byte(jp_temp);
        self.data.pc = join_bytes(pch, pcl);
    }

    pub fn dec(&mut self, value: &mut u8) {
        self.data.F = self.data.F & FLAG_C | tern_op_b((*value & 0x0f) != 0, 0, FLAG_H) | FLAG_N;
        *value = (*value).wrapping_sub(1);
        self.data.F |=
            tern_op_b(*value == 0x7f, FLAG_V, 0) | self.tables.sz53_table[*value as usize];
    }

    pub fn inc(&mut self, value: &mut u8) {
        *value = (*value).wrapping_add(1);
        self.data.F = (self.data.F & FLAG_C)
            | tern_op_b(*value == 0x80, FLAG_V, 0)
            | tern_op_b((*value & 0x0f) != 0, 0, FLAG_H)
            | self.tables.sz53_table[*value as usize];
    }
    pub fn jr(&mut self) {
        let jr_temp: i16 = sign_extend(self.memory.read_byte(self.data.pc));
        self.memory.contend_read_no_mreq_loop(self.data.pc, 1, 5);
        self.data.pc = self.data.pc.wrapping_add(jr_temp as u16);
    }

    pub fn ld16nnrr(&mut self, reg_l: u8, reg_h: u8) {
        let mut ld_temp: u16;

        ld_temp = self.memory.read_byte(self.data.pc) as u16;
        self.data.pc += 1;
        ld_temp |= (self.memory.read_byte(self.data.pc) as u16) << 8;
        self.data.pc += 1;
        self.memory.write_byte(ld_temp, reg_l);
        ld_temp += 1;
        self.memory.write_byte(ld_temp, reg_h);
    }

    // pub fn ld16rrnn(&mut self, reg_l: &mut u8, reg_h: &mut u8) {
    //     let mut ld_temp: u16;

    //     ld_temp = self.memory.ReadByte(self.data.pc) as u16;
    //     self.data.pc += 1;
    //     ld_temp |= (self.memory.ReadByte(self.data.pc) as u16) << 8;
    //     self.data.pc += 1;
    //     *reg_l = self.memory.ReadByte(ld_temp);
    //     ld_temp += 1;
    //     *reg_h = self.memory.ReadByte(ld_temp);
    // }

    /// return low, high
    pub fn ld16rrnn_ex(&mut self) -> (u8, u8) {
        let mut ld_temp: u16;

        ld_temp = self.memory.read_byte(self.data.pc) as u16;
        self.data.pc += 1;
        ld_temp |= (self.memory.read_byte(self.data.pc) as u16) << 8;
        self.data.pc += 1;
        let reg_l = self.memory.read_byte(ld_temp);
        ld_temp += 1;
        let reg_h = self.memory.read_byte(ld_temp);
        (reg_l, reg_h)
    }

    pub fn sub(&mut self, value: u8) {
        let sub_temp: u16 = (self.data.A as u16).wrapping_sub(value as u16);
        let lookup: u8 =
            ((self.data.A & 0x88) >> 3) | ((value & 0x88) >> 2) | (((sub_temp & 0x88) >> 1) as u8);
        self.data.A = sub_temp as u8;
        self.data.F = tern_op_b(sub_temp & 0x100 != 0, FLAG_C, 0)
            | FLAG_N
            | HALF_CARRY_SUB_TABLE[(lookup & 0x07) as usize]
            | OVERFLOW_SUB_TABLE[(lookup >> 4) as usize]
            | self.tables.sz53_table[self.data.A as usize];
    }

    pub fn and(&mut self, value: u8) {
        self.data.A &= value;
        self.data.F = FLAG_H | self.tables.sz53p_table[self.data.A as usize];
    }

    pub fn adc(&mut self, value: u8) {
        let adc_temp: u16 =
            (self.data.A as u16) + (value as u16) + (((self.data.F) & FLAG_C) as u16);
        let lookup: u8 = (((self.data.A as u16) & 0x88) >> 3
            | ((value as u16) & 0x88) >> 2
            | (adc_temp & 0x88) >> 1) as u8;

        self.data.A = adc_temp as u8;

        self.data.F = tern_op_b((adc_temp & 0x100) != 0, FLAG_C, 0)
            | HALF_CARRY_ADD_TABLE[(lookup & 0x07) as usize]
            | OVERFLOW_ADD_TABLE[(lookup >> 4) as usize]
            | self.tables.sz53_table[self.data.A as usize];
    }

    pub fn adc16(&mut self, value: u16) {
        let add16_temp: usize =
            (self.HL() as usize) + (value as usize) + ((self.data.F & FLAG_C) as usize);
        let lookup: u8 = ((((self.HL()) & 0x8800) >> 11) as usize
            | (((value) & 0x8800) >> 10) as usize
            | (add16_temp & 0x8800) >> 9) as u8;

        self.SetHL(add16_temp as u16);

        self.data.F = tern_op_b((add16_temp & 0x10000) != 0, FLAG_C, 0)
            | OVERFLOW_ADD_TABLE[(lookup >> 4) as usize]
            | self.data.H & (FLAG_3 | FLAG_5 | FLAG_S)
            | HALF_CARRY_ADD_TABLE[(lookup & 0x07) as usize]
            | tern_op_b(self.HL() != 0, 0, FLAG_Z);
    }

    pub fn add16(&mut self, value1: &mut Register16, value2: u16) {
        let add16_temp: usize = (value1.get() as usize) + (value2 as usize);
        let lookup: u8 = ((value1.get() & 0x0800) >> 11
            | (value2 & 0x0800) >> 10
            | ((add16_temp as u16) & 0x0800) >> 9) as u8;

        value1.set(add16_temp as u16);

        self.data.F = self.data.F & (FLAG_V | FLAG_Z | FLAG_S)
            | tern_op_b((add16_temp & 0x10000) != 0, FLAG_C, 0)
            | ((add16_temp >> 8) as u8) & (FLAG_3 | FLAG_5)
            | HALF_CARRY_ADD_TABLE[lookup as usize];
    }

    pub fn add(&mut self, value: u8) {
        let add_temp: usize = (self.data.A as usize) + (value as usize);
        let lookup: u8 =
            ((self.data.A & 0x88) >> 3) | ((value & 0x88) >> 2) | (((add_temp & 0x88) >> 1) as u8);
        self.data.A = add_temp as u8;
        self.data.F = tern_op_b(add_temp & 0x100 != 0, FLAG_C, 0)
            | HALF_CARRY_ADD_TABLE[(lookup & 0x07) as usize]
            | OVERFLOW_ADD_TABLE[(lookup >> 4) as usize]
            | self.tables.sz53_table[self.data.A as usize];
    }

    pub fn or(&mut self, value: u8) {
        self.data.A |= value;
        self.data.F = self.tables.sz53p_table[self.data.A as usize];
    }

    /// return reg_l, reg_h
    pub fn pop16(&mut self) -> (u8, u8) {
        let reg_l = self.memory.read_byte(self.data.sp);
        self.data.sp += 1;
        let reg_h = self.memory.read_byte(self.data.sp);
        self.data.sp += 1;
        (reg_l, reg_h)
    }

    pub fn push16(&mut self, reg_l: u8, reg_h: u8) {
        // self.data.sp -= 1;
        self.data.sp = self.data.sp.wrapping_sub(1);
        self.memory.write_byte(self.data.sp, reg_h);
        // self.data.sp -= 1;
        self.data.sp = self.data.sp.wrapping_sub(1);
        self.memory.write_byte(self.data.sp, reg_l);
    }

    pub fn ret(&mut self) {
        let (pcl, pch) = self.pop16();
        // let old_pc = self.data.pc;
        self.data.pc = join_bytes(pch, pcl);
        // println!("z80:ret 0x{:04x} -> 0x{:04x}", old_pc, self.data.pc);
    }

    pub fn rl(&mut self, mut value: u8) -> u8 {
        let rl_temp = value;
        value = (value << 1) | (self.data.F & FLAG_C);
        self.data.F = (rl_temp >> 7) | self.tables.sz53p_table[value as usize];
        value
    }

    pub fn rlc(&mut self, mut value: u8) -> u8 {
        value = value.rotate_left(1);
        self.data.F = (value & FLAG_C) | self.tables.sz53p_table[value as usize];
        value
    }

    pub fn rr(&mut self, mut value: u8) -> u8 {
        let rr_temp = value;
        value = (value >> 1) | (self.data.F << 7);
        self.data.F = (rr_temp & FLAG_C) | self.tables.sz53p_table[value as usize];
        value
    }

    pub fn rrc(&mut self, mut value: u8) -> u8 {
        self.data.F = value & FLAG_C;
        value = value.rotate_right(1);
        self.data.F |= self.tables.sz53p_table[value as usize];
        value
    }

    pub fn rst(&mut self, value: u8) {
        let (pch, pcl) = split_word(self.data.pc);
        self.push16(pcl, pch);
        self.data.pc = value as u16;
    }

    pub fn sbc(&mut self, value: u8) {
        let sbc_temp: u16 = (self.data.A as u16)
            .wrapping_sub(value as u16)
            .wrapping_sub((self.data.F & FLAG_C) as u16);
        let lookup: u8 =
            ((self.data.A & 0x88) >> 3) | ((value & 0x88) >> 2) | ((sbc_temp & 0x88) >> 1) as u8;
        self.data.A = sbc_temp as u8;
        self.data.F = tern_op_b((sbc_temp & 0x100) != 0, FLAG_C, 0)
            | FLAG_N
            | HALF_CARRY_SUB_TABLE[lookup as usize & 0x07]
            | OVERFLOW_SUB_TABLE[(lookup >> 4) as usize]
            | self.tables.sz53_table[self.data.A as usize];
    }

    pub fn sbc16(&mut self, value: u16) {
        let sub16_temp: usize = (self.HL() as usize)
            .wrapping_sub(value as usize)
            .wrapping_sub((self.data.F & FLAG_C) as usize);
        let lookup: u8 = ((self.HL() & 0x8800) >> 11
            | (value & 0x8800) >> 10
            | ((sub16_temp as u16) & 0x8800) >> 9)
            .try_into()
            .unwrap();

        self.SetHL(sub16_temp as u16);

        self.data.F = tern_op_b((sub16_temp & 0x10000) != 0, FLAG_C, 0)
            | FLAG_N
            | OVERFLOW_SUB_TABLE[lookup as usize >> 4]
            | self.data.H & (FLAG_3 | FLAG_5 | FLAG_S)
            | HALF_CARRY_SUB_TABLE[(lookup & 0x07) as usize]
            | tern_op_b(self.HL() != 0, 0, FLAG_Z);
    }

    pub fn sla(&mut self, mut value: u8) -> u8 {
        self.data.F = value >> 7;
        value <<= 1;
        self.data.F |= self.tables.sz53p_table[value as usize];
        value
    }

    pub fn sll(&mut self, mut value: u8) -> u8 {
        self.data.F = value >> 7;
        value = (value << 1) | 0x01;
        self.data.F |= self.tables.sz53p_table[value as usize];
        value
    }

    pub fn sra(&mut self, mut value: u8) -> u8 {
        self.data.F = value & FLAG_C;
        value = (value & 0x80) | (value >> 1);
        self.data.F |= self.tables.sz53p_table[value as usize];
        value
    }

    pub fn srl(&mut self, mut value: u8) -> u8 {
        self.data.F = value & FLAG_C;
        value >>= 1;
        self.data.F |= self.tables.sz53p_table[value as usize];
        value
    }

    pub fn xor(&mut self, value: u8) {
        self.data.A ^= value;
        self.data.F = self.tables.sz53p_table[self.data.A as usize];
    }

    pub fn bit(&mut self, bit: u8, value: u8) {
        self.data.F = self.data.F & FLAG_C | FLAG_H | value & (FLAG_3 | FLAG_5);
        if value & (0x01 << bit) == 0 {
            self.data.F |= FLAG_P | FLAG_Z;
        }
        if bit == 7 && (value & 0x80) != 0 {
            self.data.F |= FLAG_S;
        }
    }

    pub fn biti(&mut self, bit: u8, value: u8, address: u16) {
        self.data.F = self.data.F & FLAG_C | FLAG_H | (address >> 8) as u8 & (FLAG_3 | FLAG_5);
        if value & (0x01 << bit) == 0 {
            self.data.F |= FLAG_P | FLAG_Z;
        }
        if (bit == 7) && (value & 0x80) != 0 {
            self.data.F |= FLAG_S;
        }
    }

    pub fn jp_hl(&mut self) {
        let jmp_tgt = self.HL();
        if self.has_hook(jmp_tgt) {
            // println!("jp_iHL:0x{:04x}", jmp_tgt);
            let skip = self.call_hook(jmp_tgt);
            assert!(skip);
            self.ret();
        } else {
            self.SetPC(jmp_tgt);
        }
    }

    pub fn call(&mut self) {
        let call_temp_l: u8 = self.memory.read_byte(self.data.pc);
        self.data.pc += 1;
        let call_temp_h: u8 = self.memory.read_byte(self.data.pc);
        let new_pc = join_bytes(call_temp_h, call_temp_l);
        self.memory.contend_read_no_mreq(self.data.pc, 1);
        self.data.pc += 1;
        let (pch, pcl) = split_word(self.data.pc);
        self.push16(pcl, pch);
        let old_pc = self.data.pc;
        // if new_pc >= 0x4000 && new_pc <= 0xc000 {
        //     log::info!("z80:call 0x{:04x}=>0x{:04x}", old_pc, new_pc);
        // }
        if self.has_hook(new_pc) {
            let die_after_unknown_caller = true;
            let skip = self.call_hook(new_pc);
            if !self.is_known_caller(old_pc) {
                let debug_call = true;
                if debug_call {
                    log::info!("z80:call UNKNOWN 0x{:04x}=>0x{:04x}", old_pc, new_pc);
                    self.peek_stack(5);
                }
                assert!(!die_after_unknown_caller);
            }
            if skip {
                self.pop16();
                self.data.pc = old_pc;
                return;
            } else {
                log::info!("z80:call ignored 0x{:04x}=>0x{:04x}", old_pc, new_pc);
                panic!();
            }
        }

        self.data.pc = new_pc;
        // println!("z80:call 0x{:04x} -> 0x{:04x}", old_pc, self.data.pc);
    }

    pub fn cp(&mut self, value: u8) {
        let cp_temp: u16 = (self.data.A as u16).wrapping_sub(value as u16);
        let lookup: u8 =
            (self.data.A & 0x88) >> 3 | (value & 0x88) >> 2 | ((cp_temp & 0x88) >> 1) as u8;
        self.data.F = tern_op_b(
            (cp_temp & 0x100) != 0,
            FLAG_C,
            tern_op_b(cp_temp != 0, 0, FLAG_Z),
        ) | FLAG_N
            | HALF_CARRY_SUB_TABLE[(lookup & 0x07) as usize]
            | OVERFLOW_SUB_TABLE[(lookup >> 4) as usize]
            | value & (FLAG_3 | FLAG_5)
            | (cp_temp as u8 & FLAG_S);
    }

    // pub fn in(&mut self, reg: &mut u8, port :u16) {
    // pub fn in_u8(&mut self, reg: &mut u8, port: u16) {
    //     *reg = self.readPort(port);
    //     self.data.F = (self.data.F & FLAG_C) | self.tables.sz53pTable[*reg as usize];
    // }

    pub fn in_u8_ex(&mut self, port: u16) -> u8 {
        let reg = self.read_port(port);
        self.data.F = self.data.F & FLAG_C | self.tables.sz53p_table[reg as usize];
        reg
    }

    pub fn read_port(&mut self, address: u16) -> u8 {
        self.ports.read_port(address)
    }

    pub fn write_port(&mut self, address: u16, b: u8) {
        self.ports.write_port(address, b);
    }

    // The following functions can not be generated as they need special treatments

    // PC returns the program counter.
    pub fn PC(&self) -> u16 {
        self.data.pc
    }

    // SetPC sets the program counter.
    pub fn SetPC(&mut self, value: u16) {
        self.data.pc = value;
    }

    // IncPC increments the program counter.
    pub fn IncPC(&mut self, value: u16) {
        self.data.pc += value;
    }

    // IncPC decrements the program counter.
    pub fn DecPC(&mut self, value: u16) {
        self.data.pc -= value;
    }

    // SP returns the SP register.
    pub fn SP(&self) -> u16 {
        self.data.sp
    }

    // SetSP sets the SP register.
    pub fn SetSP(&mut self, value: u16) {
        self.data.sp = value;
    }

    // IncSP increments the SP register.
    pub fn IncSP(&mut self) {
        self.data.sp += 1;
    }

    // DecSP decrements the SP register.
    pub fn DecSP(&mut self) {
        self.data.sp -= 1;
    }

    // IR returns the IR register.
    pub fn IR(&mut self) -> u16 {
        let mut ir: u16 = 0;
        ir |= (self.data.I as u16) << 8;
        ir |= self.data.R7 as u16 & 0x80 | self.data.R & 0x7f;
        ir
    }

    pub fn slt_trap(&mut self, _address: i16, _level: u8) -> isize {
        // Dummy implementation
        0
    }

    // Execute a single instruction at the program counter.
    pub fn do_opcode(&mut self) {
        self.memory.contend_read(self.data.pc, 4);
        let opcode = self.memory.read_byte_internal(self.data.pc);
        self.data.R = (self.data.R + 1) & 0x7f;
        self.data.pc += 1;
        // self.data.cycles += get_timings[opcode](z80);
        self.data.cycles += self.get_timings(opcode as u16);

        // OpcodesMap[opcode](z80)
        if self.data.debug {
            self.disassemble_map(opcode as u16);
        }
        match opcode {
            0xcb => {
                self.opcode_cb();
            }
            0xdd => {
                self.opcode_dd();
            }
            0xed => {
                self.opcode_ed();
            }
            0xfd => {
                self.opcode_fd();
            }
            _ => {
                self.execute_opcode(opcode as u16);
            }
        }
    }

    fn opcode_cb(&mut self) {
        self.memory.contend_read(self.data.pc, 4);
        let opcode2: u8 = self.memory.read_byte_internal(self.data.pc);
        self.data.pc += 1;
        self.data.R += 1;
        self.execute_opcode(SHIFT_0X_CB + opcode2 as u16);
        self.data.cycles += self.get_timings(SHIFT_0X_CB + opcode2 as u16);
    }

    fn opcode_dd(&mut self) {
        self.memory.contend_read(self.data.pc, 4);
        let opcode2: u8 = self.memory.read_byte_internal(self.data.pc);
        self.data.pc += 1;
        self.data.R += 1;

        match opcode2 {
            0xcb => {
                self.memory.contend_read(self.data.pc, 3);
                self.data.temp_addr =
                    self.IX() + sign_extend(self.memory.read_byte_internal(self.data.pc)) as u16;
                self.data.pc += 1;
                self.memory.contend_read(self.data.pc, 3);
                let opcode3: u8 = self.memory.read_byte_internal(self.data.pc);
                self.memory.contend_read_no_mreq_loop(self.data.pc, 1, 2);
                self.data.pc += 1;
                self.execute_opcode(SHIFT_0X_DDCB + (opcode3 as u16));
                self.data.cycles += self.get_timings(SHIFT_0X_DDCB + opcode3 as u16);
            }
            _ => {
                if self.execute_opcode(SHIFT_0X_DD + (opcode2 as u16)) {
                    self.data.cycles += self.get_timings(SHIFT_0X_DD + (opcode2 as u16));
                } else {
                    /* Instruction did not involve H or L */
                    self.execute_opcode(opcode2 as u16);
                    self.data.cycles += self.get_timings(opcode2 as u16);
                }
            }
        }
    }

    fn opcode_ed(&mut self) {
        self.memory.contend_read(self.data.pc, 4);
        let opcode2: u8 = self.memory.read_byte_internal(self.data.pc);
        self.data.pc += 1;
        self.data.R += 1;

        self.data.cycles += self.get_timings(SHIFT_0X_ED + opcode2 as u16);
        if !self.execute_opcode(SHIFT_0X_ED + opcode2 as u16) {
            self.invalid_opcode();
        }
    }

    fn opcode_fd(&mut self) {
        self.memory.contend_read(self.data.pc, 4);
        let opcode2: u8 = self.memory.read_byte_internal(self.data.pc);
        self.data.pc += 1;
        self.data.R += 1;

        match opcode2 {
            0xcb => {
                self.memory.contend_read(self.data.pc, 3);
                self.data.temp_addr =
                    self.IY() + sign_extend(self.memory.read_byte_internal(self.data.pc)) as u16;
                self.data.pc += 1;
                self.memory.contend_read(self.data.pc, 3);
                let opcode3: u8 = self.memory.read_byte_internal(self.data.pc);
                self.memory.contend_read_no_mreq_loop(self.data.pc, 1, 2);
                self.data.pc += 1;

                self.execute_opcode(SHIFT_0X_FDCB + (opcode3 as u16));
                self.data.cycles += self.get_timings(SHIFT_0X_FDCB + (opcode3 as u16));
            }

            _ => {
                if self.execute_opcode(SHIFT_0X_FD + opcode2 as u16) {
                    // f(z80)
                    self.data.cycles += self.get_timings(SHIFT_0X_FD + opcode2 as u16);
                } else {
                    /* Instruction did not involve H or L */
                    self.execute_opcode(opcode2 as u16);
                    self.data.cycles += self.get_timings(opcode2 as u16);
                }
            }
        }
    }

    fn invalid_opcode(&mut self) {
        panic!("invalid opcode");
    }

    pub fn get_data(&self) -> Z80Data {
        self.data.clone()
    }
    pub fn set_data(&mut self, data: Z80Data) {
        self.data = data;
    }
    pub fn get_memory_data(&self) -> MemoryData {
        self.memory.get_data()
    }
    pub fn set_memory_data(&mut self, data: MemoryData) {
        self.memory.set_data(data);
    }
    pub fn get_ppi_data(&self) -> PPIData {
        self.memory.get_ppi_data()
    }
    pub fn set_ppi_data(&mut self, data: PPIData) {
        self.memory.set_ppi_data(data);
    }
}
