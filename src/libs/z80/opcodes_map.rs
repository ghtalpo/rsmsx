// /*

// Copyright (c) 2010 Andrea Fazzi

// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
// 8
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use super::{
    z80_base::{
        join_bytes, sign_extend, split_word, tern_op_b, Register16, FLAG_3, FLAG_5, FLAG_C, FLAG_H,
        FLAG_N, FLAG_P, FLAG_S, FLAG_V, FLAG_Z, SHIFT_0X_CB, SHIFT_0X_DD, SHIFT_0X_DDCB,
        SHIFT_0X_ED, SHIFT_0X_FD, Z80,
    },
    z80_tables::HALF_CARRY_SUB_TABLE,
};

macro_rules! fn_instr_ld_r16_nnnn {
    ($fn:tt, $r:ident, $fs:ident) => {
        pub(crate) fn $fn(&mut self) {
            let address = self.PC();
            let b1 = self.memory.read_byte(address);
            self.IncPC(1);
            let address = self.PC();
            let b2 = self.memory.read_byte(address);
            self.IncPC(1);
            self.$fs(join_bytes(b2, b1));
        }
    };
}

macro_rules! fn_instr_ld_i_nnnn_r16 {
    ($fn:tt, $rl:ident, $rh:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.ld16nnrr(self.data.$rl, self.data.$rh);
        }
    };
}

macro_rules! fn_instr_ld_i_r16_r8 {
    ($fn:tt, $r16:ident, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.memory.write_byte(self.$r16(), self.data.$r8);
        }
    };
}

macro_rules! fn_instr_add_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.add(self.data.$r8);
        }
    };
}

macro_rules! fn_instr_adc_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.adc(self.data.$r8);
        }
    };
}

macro_rules! fn_instr_sub_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.sub(self.data.$r8);
        }
    };
}

macro_rules! fn_instr_sbc_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.sbc(self.data.$r8);
        }
    };
}

macro_rules! fn_instr_and_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.and(self.data.$r8);
        }
    };
}

macro_rules! fn_instr_xor_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.xor(self.data.$r8);
        }
    };
}

macro_rules! fn_instr_or_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.or(self.data.$r8);
        }
    };
}

macro_rules! fn_instr_cp_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.cp(self.data.$r8);
        }
    };
}

macro_rules! fn_instr_pop_r16 {
    ($fn:tt, $rl:ident, $rh:ident) => {
        pub(crate) fn $fn(&mut self) {
            (self.data.$rl, self.data.$rh) = self.pop16();
        }
    };
}

macro_rules! fn_instr_push_r16 {
    ($fn:tt, $rl:ident, $rh:ident) => {
        pub(crate) fn $fn(&mut self) {
            let _address = self.IR();
            self.memory.contend_read_no_mreq(_address, 1);
            self.push16(self.data.$rl, self.data.$rh);
        }
    };
}

macro_rules! fn_instr_ld_a_r16 {
    ($fn:tt, $r16:ident) => {
        pub(crate) fn $fn(&mut self) {
            let address = self.$r16();
            self.data.A = self.memory.read_byte(address);
        }
    };
}

macro_rules! fn_instr_ld_a_r8 {
    ($fn:tt, $r8:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.A = self.data.$r8;
        }
    };
}

macro_rules! fn_instr_op_16 {
    ($fn:tt, $op:ident) => {
        pub(crate) fn $fn(&mut self) {
            let _address = self.IR();
            self.memory.contend_read_no_mreq_loop(_address, 1, 2);
            self.$op();
        }
    };
}

macro_rules! fn_instr_op_8 {
    ($fn:tt, $op:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.$op();
        }
    };
}

macro_rules! fn_instr_ld_r8_nn {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            let address = self.PC();
            self.data.$r = self.memory.read_byte(address);
            self.IncPC(1);
        }
    };
}

macro_rules! fn_instr_add_hl_r16 {
    ($fn:tt, $r16:ident) => {
        pub(crate) fn $fn(&mut self) {
            let _address = self.IR();
            self.memory.contend_read_no_mreq_loop(_address, 1, 7);
            let mut hl = Register16::new(self.data.H, self.data.L);
            let value2 = self.$r16();
            self.add16(&mut hl, value2);
            (self.data.H, self.data.L) = hl.result();
        }
    };
}

macro_rules! fn_instr_ld_hl_i_nnnn {
    ($fn:tt, $rl:ident, $rh:ident) => {
        pub(crate) fn $fn(&mut self) {
            (self.data.$rl, self.data.$rh) = self.ld16rrnn_ex();
        }
    };
}

macro_rules! fn_instr_ld_i_reg_p_dd_r8 {
    ($fn:tt, $ri:ident, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            let address = self.PC();
            let offset = self.memory.read_byte(address);
            let _address = self.PC();
            self.memory.contend_read_no_mreq_loop(_address, 1, 5);
            self.IncPC(1);
            self.memory
                .write_byte(self.$ri() + (sign_extend(offset) as u16), self.data.$r)
        }
    };
}

macro_rules! fn_instr_op_a_i_reg_p_dd {
    ($fn:tt, $r:ident, $op:ident) => {
        pub(crate) fn $fn(&mut self) {
            let address = self.PC();
            let offset: u8 = self.memory.read_byte(address);
            let _address = self.PC();
            self.memory.contend_read_no_mreq_loop(_address, 1, 5);
            self.IncPC(1);
            let byte_temp: u8 = self
                .memory
                .read_byte(self.$r() + (sign_extend(offset) as u16));
            self.$op(byte_temp)
        }
    };
}

macro_rules! fn_instr_dd_op_a_i_reg_p_dd {
    ($fn:tt, $op:ident) => {
        fn_instr_op_a_i_reg_p_dd!($fn, IX, $op);
    };
}

macro_rules! fn_instr_fd_op_a_i_reg_p_dd {
    ($fn:tt, $op:ident) => {
        fn_instr_op_a_i_reg_p_dd!($fn, IY, $op);
    };
}

macro_rules! fn_instr_op_i_reg_p_dd {
    ($fn:tt, $ri:ident, $op:ident) => {
        pub(crate) fn $fn(&mut self) {
            let address = self.PC();
            let offset: u8 = self.memory.read_byte(address);
            let _address = self.PC();
            self.memory.contend_read_no_mreq_loop(_address, 1, 5);
            self.IncPC(1);
            let word_temp: u16 = self.$ri() + sign_extend(offset) as u16;
            let mut byte_temp: u8 = self.memory.read_byte(word_temp);
            self.memory.contend_read_no_mreq(word_temp, 1);
            self.$op(&mut byte_temp);
            self.memory.write_byte(word_temp, byte_temp)
        }
    };
}

macro_rules! fn_instr_dd_op_i_reg_p_dd {
    ($fn:tt, $op:ident) => {
        fn_instr_op_i_reg_p_dd!($fn, IX, $op);
    };
}

macro_rules! fn_instr_fd_op_i_reg_p_dd {
    ($fn:tt, $op:ident) => {
        fn_instr_op_i_reg_p_dd!($fn, IY, $op);
    };
}

macro_rules! fn_instr_ld_r_i_reg_p_dd {
    ($fn:tt, $r:ident, $ri:ident) => {
        pub(crate) fn $fn(&mut self) {
            let address = self.PC();
            let offset: u8 = self.memory.read_byte(address);
            let _address = self.PC();
            self.memory.contend_read_no_mreq_loop(_address, 1, 5);
            self.IncPC(1);
            self.data.$r = self
                .memory
                .read_byte(self.$ri() + (sign_extend(offset) as u16));
        }
    };
}

macro_rules! fn_instr_dd_ld_r_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ld_r_i_reg_p_dd!($fn, $r, IX);
    };
}

macro_rules! fn_instr_fd_ld_r_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ld_r_i_reg_p_dd!($fn, $r, IY);
    };
}

macro_rules! fn_instr_ddcb_op_i_reg_p_dd {
    ($fn:tt, $op:ident) => {
        pub(crate) fn $fn(&mut self) {
            let mut byte_temp: u8 = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            byte_temp = self.$op(byte_temp);
            self.memory.write_byte(self.data.temp_addr, byte_temp);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_rlc_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.data.$r = self.rlc(self.data.$r);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_rrc_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.data.$r = self.rrc(self.data.$r);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_rl_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.data.$r = self.rl(self.data.$r);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_rr_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.data.$r = self.rr(self.data.$r);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_sla_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.data.$r = self.sla(self.data.$r);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_sra_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.data.$r = self.sra(self.data.$r);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_sll_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.data.$r = self.sll(self.data.$r);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_srl_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.data.$r = self.srl(self.data.$r);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_bit_n_i_reg_p_dd {
    ($fn:tt, $r:expr) => {
        pub(crate) fn $fn(&mut self) {
            let byte_temp = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.biti($r, byte_temp, self.data.temp_addr);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_n_i_reg_p_dd {
    ($fn:tt, $r:ident, $mask:expr) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr) & $mask;
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_res_n_i_reg_p_dd {
    ($fn:tt, $mask:expr) => {
        pub(crate) fn $fn(&mut self) {
            let byte_temp: u8 = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.memory
                .write_byte(self.data.temp_addr, byte_temp & $mask);
        }
    };
}

macro_rules! fn_instr_ddcb_set_n_i_reg_p_dd {
    ($fn:tt, $mask:expr) => {
        pub(crate) fn $fn(&mut self) {
            let byte_temp: u8 = self.memory.read_byte(self.data.temp_addr);
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.memory
                .write_byte(self.data.temp_addr, byte_temp | $mask);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_0_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_res_n_i_reg_p_dd!($fn, $r, 0xfe);
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_1_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_res_n_i_reg_p_dd!($fn, $r, 0xfd);
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_2_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_res_n_i_reg_p_dd!($fn, $r, 0xfb);
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_3_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_res_n_i_reg_p_dd!($fn, $r, 0xf7);
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_4_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_res_n_i_reg_p_dd!($fn, $r, 0xef);
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_5_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_res_n_i_reg_p_dd!($fn, $r, 0xdf);
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_6_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_res_n_i_reg_p_dd!($fn, $r, 0xbf);
    };
}

macro_rules! fn_instr_ddcb_ld_r_res_7_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_res_n_i_reg_p_dd!($fn, $r, 0x7f);
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_n_i_reg_p_dd {
    ($fn:tt, $r:ident, $mask:expr) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.memory.read_byte(self.data.temp_addr) | $mask;
            self.memory.contend_read_no_mreq(self.data.temp_addr, 1);
            self.memory.write_byte(self.data.temp_addr, self.data.$r);
        }
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_0_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_set_n_i_reg_p_dd!($fn, $r, 0x01);
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_1_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_set_n_i_reg_p_dd!($fn, $r, 0x02);
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_2_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_set_n_i_reg_p_dd!($fn, $r, 0x04);
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_3_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_set_n_i_reg_p_dd!($fn, $r, 0x08);
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_4_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_set_n_i_reg_p_dd!($fn, $r, 0x10);
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_5_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_set_n_i_reg_p_dd!($fn, $r, 0x20);
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_6_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_set_n_i_reg_p_dd!($fn, $r, 0x40);
    };
}

macro_rules! fn_instr_ddcb_ld_r_set_7_i_reg_p_dd {
    ($fn:tt, $r:ident) => {
        fn_instr_ddcb_ld_r_set_n_i_reg_p_dd!($fn, $r, 0x80);
    };
}

// */
#[allow(non_snake_case)]
impl Z80 {
    pub fn execute_opcode(&mut self, opcode: u16) -> bool {
        match opcode {
            0 => {
                /* NOP */
                self.instr__NOP();
                true
            }
            1 => {
                /* LD BC,nnnn */
                self.instr__LD_BC_NNNN();
                true
            }
            0x02 => {
                /* LD (BC),A */
                self.instr__LD_iBC_A();
                true
            }

            0x03 => {
                /* INC BC */
                self.instr__INC_BC();
                true
            }

            0x04 => {
                /* INC B */
                self.instr__INC_B();
                true
            }

            0x05 => {
                /* DEC B */
                self.instr__DEC_B();
                true
            }

            0x06 => {
                /* LD B,nn */
                self.instr__LD_B_NN();
                true
            }

            0x07 => {
                /* RLCA */
                self.instr__RLCA();
                true
            }

            0x08 => {
                /* EX AF,AF' */
                self.instr__EX_AF_AF();
                true
            }

            0x09 => {
                /* ADD HL,BC */
                self.instr__ADD_HL_BC();
                true
            }

            0x0a => {
                /* LD A,(BC) */
                self.instr__LD_A_iBC();
                true
            }

            0x0b => {
                /* DEC BC */
                self.instr__DEC_BC();
                true
            }

            0x0c => {
                /* INC C */
                self.instr__INC_C();
                true
            }

            0x0d => {
                /* DEC C */
                self.instr__DEC_C();
                true
            }

            0x0e => {
                /* LD C,nn */
                self.instr__LD_C_NN();
                true
            }

            0x0f => {
                /* RRCA */
                self.instr__RRCA();
                true
            }

            0x10 => {
                /* DJNZ offset */
                self.instr__DJNZ_OFFSET();
                true
            }

            0x11 => {
                /* LD DE,nnnn */
                self.instr__LD_DE_NNNN();
                true
            }

            0x12 => {
                /* LD (DE),A */
                self.instr__LD_iDE_A();
                true
            }

            0x13 => {
                /* INC DE */
                self.instr__INC_DE();
                true
            }

            0x14 => {
                /* INC D */
                self.instr__INC_D();
                true
            }

            0x15 => {
                /* DEC D */
                self.instr__DEC_D();
                true
            }

            0x16 => {
                /* LD D,nn */
                self.instr__LD_D_NN();
                true
            }

            0x17 => {
                /* RLA */
                self.instr__RLA();
                true
            }

            0x18 => {
                /* JR offset */
                self.instr__JR_OFFSET();
                true
            }

            0x19 => {
                /* ADD HL,DE */
                self.instr__ADD_HL_DE();
                true
            }

            0x1a => {
                /* LD A,(DE) */
                self.instr__LD_A_iDE();
                true
            }

            0x1b => {
                /* DEC DE */
                self.instr__DEC_DE();
                true
            }

            0x1c => {
                /* INC E */
                self.instr__INC_E();
                true
            }

            0x1d => {
                /* DEC E */
                self.instr__DEC_E();
                true
            }

            0x1e => {
                /* LD E,nn */
                self.instr__LD_E_NN();
                true
            }

            0x1f => {
                /* RRA */
                self.instr__RRA();
                true
            }

            0x20 => {
                /* JR NZ,offset */
                self.instr__JR_NZ_OFFSET();
                true
            }

            0x21 => {
                /* LD HL,nnnn */
                self.instr__LD_HL_NNNN();
                true
            }

            0x22 => {
                /* LD (nnnn),HL */
                self.instr__LD_iNNNN_HL();
                true
            }

            0x23 => {
                /* INC HL */
                self.instr__INC_HL();
                true
            }

            0x24 => {
                /* INC H */
                self.instr__INC_H();
                true
            }

            0x25 => {
                /* DEC H */
                self.instr__DEC_H();
                true
            }

            0x26 => {
                /* LD H,nn */
                self.instr__LD_H_NN();
                true
            }

            0x27 => {
                /* DAA */
                self.instr__DAA();
                true
            }

            0x28 => {
                /* JR Z,offset */
                self.instr__JR_Z_OFFSET();
                true
            }

            0x29 => {
                /* ADD HL,HL */
                self.instr__ADD_HL_HL();
                true
            }

            0x2a => {
                /* LD HL,(nnnn) */
                self.instr__LD_HL_iNNNN();
                true
            }

            0x2b => {
                /* DEC HL */
                self.instr__DEC_HL();
                true
            }

            0x2c => {
                /* INC L */
                self.instr__INC_L();
                true
            }

            0x2d => {
                /* DEC L */
                self.instr__DEC_L();
                true
            }

            0x2e => {
                /* LD L,nn */
                self.instr__LD_L_NN();
                true
            }

            0x2f => {
                /* CPL */
                self.instr__CPL();
                true
            }

            0x30 => {
                /* JR NC,offset */
                self.instr__JR_NC_OFFSET();
                true
            }

            0x31 => {
                /* LD SP,nnnn */
                self.instr__LD_SP_NNNN();
                true
            }

            0x32 => {
                /* LD (nnnn),A */
                self.instr__LD_iNNNN_A();
                true
            }

            0x33 => {
                /* INC SP */
                self.instr__INC_SP();
                true
            }

            0x34 => {
                /* INC (HL) */
                self.instr__INC_iHL();
                true
            }

            0x35 => {
                /* DEC (HL) */
                self.instr__DEC_iHL();
                true
            }

            0x36 => {
                /* LD (HL),nn */
                self.instr__LD_iHL_NN();
                true
            }

            0x37 => {
                /* SCF */
                self.instr__SCF();
                true
            }

            0x38 => {
                /* JR C,offset */
                self.instr__JR_C_OFFSET();
                true
            }

            0x39 => {
                /* ADD HL,SP */
                self.instr__ADD_HL_SP();
                true
            }

            0x3a => {
                /* LD A,(nnnn) */
                self.instr__LD_A_iNNNN();
                true
            }

            0x3b => {
                /* DEC SP */
                self.instr__DEC_SP();
                true
            }

            0x3c => {
                /* INC A */
                self.instr__INC_A();
                true
            }

            0x3d => {
                /* DEC A */
                self.instr__DEC_A();
                true
            }

            0x3e => {
                /* LD A,nn */
                self.instr__LD_A_NN();
                true
            }

            0x3f => {
                /* CCF */
                self.instr__CCF();
                true
            }

            0x40 => {
                /* LD B,B */
                self.instr__LD_B_B();
                true
            }

            0x41 => {
                /* LD B,C */
                self.instr__LD_B_C();
                true
            }

            0x42 => {
                /* LD B,D */
                self.instr__LD_B_D();
                true
            }

            0x43 => {
                /* LD B,E */
                self.instr__LD_B_E();
                true
            }

            0x44 => {
                /* LD B,H */
                self.instr__LD_B_H();
                true
            }

            0x45 => {
                /* LD B,L */
                self.instr__LD_B_L();
                true
            }

            0x46 => {
                /* LD B,(HL) */
                self.instr__LD_B_iHL();
                true
            }

            0x47 => {
                /* LD B,A */
                self.instr__LD_B_A();
                true
            }

            0x48 => {
                /* LD C,B */
                self.instr__LD_C_B();
                true
            }

            0x49 => {
                /* LD C,C */
                self.instr__LD_C_C();
                true
            }

            0x4a => {
                /* LD C,D */
                self.instr__LD_C_D();
                true
            }

            0x4b => {
                /* LD C,E */
                self.instr__LD_C_E();
                true
            }

            0x4c => {
                /* LD C,H */
                self.instr__LD_C_H();
                true
            }

            0x4d => {
                /* LD C,L */
                self.instr__LD_C_L();
                true
            }

            0x4e => {
                /* LD C,(HL) */
                self.instr__LD_C_iHL();
                true
            }

            0x4f => {
                /* LD C,A */
                self.instr__LD_C_A();
                true
            }

            0x50 => {
                /* LD D,B */
                self.instr__LD_D_B();
                true
            }

            0x51 => {
                /* LD D,C */
                self.instr__LD_D_C();
                true
            }

            0x52 => {
                /* LD D,D */
                self.instr__LD_D_D();
                true
            }

            0x53 => {
                /* LD D,E */
                self.instr__LD_D_E();
                true
            }

            0x54 => {
                /* LD D,H */
                self.instr__LD_D_H();
                true
            }

            0x55 => {
                /* LD D,L */
                self.instr__LD_D_L();
                true
            }

            0x56 => {
                /* LD D,(HL) */
                self.instr__LD_D_iHL();
                true
            }

            0x57 => {
                /* LD D,A */
                self.instr__LD_D_A();
                true
            }

            0x58 => {
                /* LD E,B */
                self.instr__LD_E_B();
                true
            }

            0x59 => {
                /* LD E,C */
                self.instr__LD_E_C();
                true
            }

            0x5a => {
                /* LD E,D */
                self.instr__LD_E_D();
                true
            }

            0x5b => {
                /* LD E,E */
                self.instr__LD_E_E();
                true
            }

            0x5c => {
                /* LD E,H */
                self.instr__LD_E_H();
                true
            }

            0x5d => {
                /* LD E,L */
                self.instr__LD_E_L();
                true
            }

            0x5e => {
                /* LD E,(HL) */
                self.instr__LD_E_iHL();
                true
            }

            0x5f => {
                /* LD E,A */
                self.instr__LD_E_A();
                true
            }

            0x60 => {
                /* LD H,B */
                self.instr__LD_H_B();
                true
            }

            0x61 => {
                /* LD H,C */
                self.instr__LD_H_C();
                true
            }

            0x62 => {
                /* LD H,D */
                self.instr__LD_H_D();
                true
            }

            0x63 => {
                /* LD H,E */
                self.instr__LD_H_E();
                true
            }

            0x64 => {
                /* LD H,H */
                self.instr__LD_H_H();
                true
            }

            0x65 => {
                /* LD H,L */
                self.instr__LD_H_L();
                true
            }

            0x66 => {
                /* LD H,(HL) */
                self.instr__LD_H_iHL();
                true
            }

            0x67 => {
                /* LD H,A */
                self.instr__LD_H_A();
                true
            }

            0x68 => {
                /* LD L,B */
                self.instr__LD_L_B();
                true
            }

            0x69 => {
                /* LD L,C */
                self.instr__LD_L_C();
                true
            }

            0x6a => {
                /* LD L,D */
                self.instr__LD_L_D();
                true
            }

            0x6b => {
                /* LD L,E */
                self.instr__LD_L_E();
                true
            }

            0x6c => {
                /* LD L,H */
                self.instr__LD_L_H();
                true
            }

            0x6d => {
                /* LD L,L */
                self.instr__LD_L_L();
                true
            }

            0x6e => {
                /* LD L,(HL) */
                self.instr__LD_L_iHL();
                true
            }

            0x6f => {
                /* LD L,A */
                self.instr__LD_L_A();
                true
            }

            0x70 => {
                /* LD (HL),B */
                self.instr__LD_iHL_B();
                true
            }

            0x71 => {
                /* LD (HL),C */
                self.instr__LD_iHL_C();
                true
            }

            0x72 => {
                /* LD (HL),D */
                self.instr__LD_iHL_D();
                true
            }

            0x73 => {
                /* LD (HL),E */
                self.instr__LD_iHL_E();
                true
            }

            0x74 => {
                /* LD (HL),H */
                self.instr__LD_iHL_H();
                true
            }

            0x75 => {
                /* LD (HL),L */
                self.instr__LD_iHL_L();
                true
            }

            0x76 => {
                /* HALT */
                self.instr__HALT();
                true
            }

            0x77 => {
                /* LD (HL),A */
                self.instr__LD_iHL_A();
                true
            }

            0x78 => {
                /* LD A,B */
                self.instr__LD_A_B();
                true
            }

            0x79 => {
                /* LD A,C */
                self.instr__LD_A_C();
                true
            }

            0x7a => {
                /* LD A,D */
                self.instr__LD_A_D();
                true
            }

            0x7b => {
                /* LD A,E */
                self.instr__LD_A_E();
                true
            }

            0x7c => {
                /* LD A,H */
                self.instr__LD_A_H();
                true
            }

            0x7d => {
                /* LD A,L */
                self.instr__LD_A_L();
                true
            }

            0x7e => {
                /* LD A,(HL) */
                self.instr__LD_A_iHL();
                true
            }

            0x7f => {
                /* LD A,A */
                self.instr__LD_A_A();
                true
            }

            0x80 => {
                /* ADD A,B */
                self.instr__ADD_A_B();
                true
            }

            0x81 => {
                /* ADD A,C */
                self.instr__ADD_A_C();
                true
            }

            0x82 => {
                /* ADD A,D */
                self.instr__ADD_A_D();
                true
            }

            0x83 => {
                /* ADD A,E */
                self.instr__ADD_A_E();
                true
            }

            0x84 => {
                /* ADD A,H */
                self.instr__ADD_A_H();
                true
            }

            0x85 => {
                /* ADD A,L */
                self.instr__ADD_A_L();
                true
            }

            0x86 => {
                /* ADD A,(HL) */
                self.instr__ADD_A_iHL();
                true
            }

            0x87 => {
                /* ADD A,A */
                self.instr__ADD_A_A();
                true
            }

            0x88 => {
                /* ADC A,B */
                self.instr__ADC_A_B();
                true
            }

            0x89 => {
                /* ADC A,C */
                self.instr__ADC_A_C();
                true
            }

            0x8a => {
                /* ADC A,D */
                self.instr__ADC_A_D();
                true
            }

            0x8b => {
                /* ADC A,E */
                self.instr__ADC_A_E();
                true
            }

            0x8c => {
                /* ADC A,H */
                self.instr__ADC_A_H();
                true
            }

            0x8d => {
                /* ADC A,L */
                self.instr__ADC_A_L();
                true
            }

            0x8e => {
                /* ADC A,(HL) */
                self.instr__ADC_A_iHL();
                true
            }

            0x8f => {
                /* ADC A,A */
                self.instr__ADC_A_A();
                true
            }

            0x90 => {
                /* SUB A,B */
                self.instr__SUB_A_B();
                true
            }

            0x91 => {
                /* SUB A,C */
                self.instr__SUB_A_C();
                true
            }

            0x92 => {
                /* SUB A,D */
                self.instr__SUB_A_D();
                true
            }

            0x93 => {
                /* SUB A,E */
                self.instr__SUB_A_E();
                true
            }

            0x94 => {
                /* SUB A,H */
                self.instr__SUB_A_H();
                true
            }

            0x95 => {
                /* SUB A,L */
                self.instr__SUB_A_L();
                true
            }

            0x96 => {
                /* SUB A,(HL) */
                self.instr__SUB_A_iHL();
                true
            }

            0x97 => {
                /* SUB A,A */
                self.instr__SUB_A_A();
                true
            }

            0x98 => {
                /* SBC A,B */
                self.instr__SBC_A_B();
                true
            }

            0x99 => {
                /* SBC A,C */
                self.instr__SBC_A_C();
                true
            }

            0x9a => {
                /* SBC A,D */
                self.instr__SBC_A_D();
                true
            }

            0x9b => {
                /* SBC A,E */
                self.instr__SBC_A_E();
                true
            }

            0x9c => {
                /* SBC A,H */
                self.instr__SBC_A_H();
                true
            }

            0x9d => {
                /* SBC A,L */
                self.instr__SBC_A_L();
                true
            }

            0x9e => {
                /* SBC A,(HL) */
                self.instr__SBC_A_iHL();
                true
            }

            0x9f => {
                /* SBC A,A */
                self.instr__SBC_A_A();
                true
            }

            0xa0 => {
                /* AND A,B */
                self.instr__AND_A_B();
                true
            }

            0xa1 => {
                /* AND A,C */
                self.instr__AND_A_C();
                true
            }

            0xa2 => {
                /* AND A,D */
                self.instr__AND_A_D();
                true
            }

            0xa3 => {
                /* AND A,E */
                self.instr__AND_A_E();
                true
            }

            0xa4 => {
                /* AND A,H */
                self.instr__AND_A_H();
                true
            }

            0xa5 => {
                /* AND A,L */
                self.instr__AND_A_L();
                true
            }

            0xa6 => {
                /* AND A,(HL) */
                self.instr__AND_A_iHL();
                true
            }

            0xa7 => {
                /* AND A,A */
                self.instr__AND_A_A();
                true
            }

            0xa8 => {
                /* XOR A,B */
                self.instr__XOR_A_B();
                true
            }

            0xa9 => {
                /* XOR A,C */
                self.instr__XOR_A_C();
                true
            }

            0xaa => {
                /* XOR A,D */
                self.instr__XOR_A_D();
                true
            }

            0xab => {
                /* XOR A,E */
                self.instr__XOR_A_E();
                true
            }

            0xac => {
                /* XOR A,H */
                self.instr__XOR_A_H();
                true
            }

            0xad => {
                /* XOR A,L */
                self.instr__XOR_A_L();
                true
            }

            0xae => {
                /* XOR A,(HL) */
                self.instr__XOR_A_iHL();
                true
            }

            0xaf => {
                /* XOR A,A */
                self.instr__XOR_A_A();
                true
            }

            0xb0 => {
                /* OR A,B */
                self.instr__OR_A_B();
                true
            }

            0xb1 => {
                /* OR A,C */
                self.instr__OR_A_C();
                true
            }

            0xb2 => {
                /* OR A,D */
                self.instr__OR_A_D();
                true
            }

            0xb3 => {
                /* OR A,E */
                self.instr__OR_A_E();
                true
            }

            0xb4 => {
                /* OR A,H */
                self.instr__OR_A_H();
                true
            }

            0xb5 => {
                /* OR A,L */
                self.instr__OR_A_L();
                true
            }

            0xb6 => {
                /* OR A,(HL) */
                self.instr__OR_A_iHL();
                true
            }

            0xb7 => {
                /* OR A,A */
                self.instr__OR_A_A();
                true
            }

            0xb8 => {
                /* CP B */
                self.instr__CP_B();
                true
            }

            0xb9 => {
                /* CP C */
                self.instr__CP_C();
                true
            }

            0xba => {
                /* CP D */
                self.instr__CP_D();
                true
            }

            0xbb => {
                /* CP E */
                self.instr__CP_E();
                true
            }

            0xbc => {
                /* CP H */
                self.instr__CP_H();
                true
            }

            0xbd => {
                /* CP L */
                self.instr__CP_L();
                true
            }

            0xbe => {
                /* CP (HL) */
                self.instr__CP_iHL();
                true
            }

            0xbf => {
                /* CP A */
                self.instr__CP_A();
                true
            }

            0xc0 => {
                /* RET NZ */
                self.instr__RET_NZ();
                true
            }

            0xc1 => {
                /* POP BC */
                self.instr__POP_BC();
                true
            }

            0xc2 => {
                /* JP NZ,nnnn */
                self.instr__JP_NZ_NNNN();
                true
            }

            0xc3 => {
                /* JP nnnn */
                self.instr__JP_NNNN();
                true
            }

            0xc4 => {
                /* CALL NZ,nnnn */
                self.instr__CALL_NZ_NNNN();
                true
            }

            0xc5 => {
                /* PUSH BC */
                self.instr__PUSH_BC();
                true
            }

            0xc6 => {
                /* ADD A,nn */
                self.instr__ADD_A_NN();
                true
            }

            0xc7 => {
                /* RST 00 */
                self.instr__RST_00();
                true
            }

            0xc8 => {
                /* RET Z */
                self.instr__RET_Z();
                true
            }

            0xc9 => {
                /* RET */
                self.instr__RET();
                true
            }

            0xca => {
                /* JP Z,nnnn */
                self.instr__JP_Z_NNNN();
                true
            }

            0xcb => {
                /* shift CB */
                self.instr__SHIFT_CB();
                true
            }

            0xcc => {
                /* CALL Z,nnnn */
                self.instr__CALL_Z_NNNN();
                true
            }

            0xcd => {
                /* CALL nnnn */
                self.instr__CALL_NNNN();
                true
            }

            0xce => {
                /* ADC A,nn */
                self.instr__ADC_A_NN();
                true
            }

            0xcf => {
                /* RST 8 */
                self.instr__RST_8();
                true
            }

            0xd0 => {
                /* RET NC */
                self.instr__RET_NC();
                true
            }

            0xd1 => {
                /* POP DE */
                self.instr__POP_DE();
                true
            }

            0xd2 => {
                /* JP NC,nnnn */
                self.instr__JP_NC_NNNN();
                true
            }

            0xd3 => {
                /* OUT (nn),A */
                self.instr__OUT_iNN_A();
                true
            }

            0xd4 => {
                /* CALL NC,nnnn */
                self.instr__CALL_NC_NNNN();
                true
            }

            0xd5 => {
                /* PUSH DE */
                self.instr__PUSH_DE();
                true
            }

            0xd6 => {
                /* SUB nn */
                self.instr__SUB_NN();
                true
            }

            0xd7 => {
                /* RST 10 */
                self.instr__RST_10();
                true
            }

            0xd8 => {
                /* RET C */
                self.instr__RET_C();
                true
            }

            0xd9 => {
                /* EXX */
                self.instr__EXX();
                true
            }

            0xda => {
                /* JP C,nnnn */
                self.instr__JP_C_NNNN();
                true
            }

            0xdb => {
                /* IN A,(nn) */
                self.instr__IN_A_iNN();
                true
            }

            0xdc => {
                /* CALL C,nnnn */
                self.instr__CALL_C_NNNN();
                true
            }

            0xdd => {
                /* shift DD */
                self.instr__SHIFT_DD();
                true
            }

            0xde => {
                /* SBC A,nn */
                self.instr__SBC_A_NN();
                true
            }

            0xdf => {
                /* RST 18 */
                self.instr__RST_18();
                true
            }

            0xe0 => {
                /* RET PO */
                self.instr__RET_PO();
                true
            }

            0xe1 => {
                /* POP HL */
                self.instr__POP_HL();
                true
            }

            0xe2 => {
                /* JP PO,nnnn */
                self.instr__JP_PO_NNNN();
                true
            }

            0xe3 => {
                /* EX (SP),HL */
                self.instr__EX_iSP_HL();
                true
            }

            0xe4 => {
                /* CALL PO,nnnn */
                self.instr__CALL_PO_NNNN();
                true
            }

            0xe5 => {
                /* PUSH HL */
                self.instr__PUSH_HL();
                true
            }

            0xe6 => {
                /* AND nn */
                self.instr__AND_NN();
                true
            }

            0xe7 => {
                /* RST 20 */
                self.instr__RST_20();
                true
            }

            0xe8 => {
                /* RET PE */
                self.instr__RET_PE();
                true
            }

            0xe9 => {
                /* JP HL */
                self.instr__JP_HL();
                true
            }

            0xea => {
                /* JP PE,nnnn */
                self.instr__JP_PE_NNNN();
                true
            }

            0xeb => {
                /* EX DE,HL */
                self.instr__EX_DE_HL();
                true
            }

            0xec => {
                /* CALL PE,nnnn */
                self.instr__CALL_PE_NNNN();
                true
            }

            0xed => {
                /* shift ED */
                self.instr__SHIFT_ED();
                true
            }

            0xee => {
                /* XOR A,nn */
                self.instr__XOR_A_NN();
                true
            }

            0xef => {
                /* RST 28 */
                self.instr__RST_28();
                true
            }

            0xf0 => {
                /* RET P */
                self.instr__RET_P();
                true
            }

            0xf1 => {
                /* POP AF */
                self.instr__POP_AF();
                true
            }

            0xf2 => {
                /* JP P,nnnn */
                self.instr__JP_P_NNNN();
                true
            }

            0xf3 => {
                /* DI */
                self.instr__DI();
                true
            }

            0xf4 => {
                /* CALL P,nnnn */
                self.instr__CALL_P_NNNN();
                true
            }

            0xf5 => {
                /* PUSH AF */
                self.instr__PUSH_AF();
                true
            }

            0xf6 => {
                /* OR nn */
                self.instr__OR_NN();
                true
            }

            0xf7 => {
                /* RST 30 */
                self.instr__RST_30();
                true
            }

            0xf8 => {
                /* RET M */
                self.instr__RET_M();
                true
            }

            0xf9 => {
                /* LD SP,HL */
                self.instr__LD_SP_HL();
                true
            }

            0xfa => {
                /* JP M,nnnn */
                self.instr__JP_M_NNNN();
                true
            }

            0xfb => {
                /* EI */
                self.instr__EI();
                true
            }

            0xfc => {
                /* CALL M,nnnn */
                self.instr__CALL_M_NNNN();
                true
            }

            0xfd => {
                /* shift FD */
                self.instr__SHIFT_FD();
                true
            }

            0xfe => {
                /* CP nn */
                self.instr__CP_NN();
                true
            }

            0xff => {
                /* RST 38 */
                self.instr__RST_38();
                true
            }

            val if val == SHIFT_0X_CB => {
                /* RLC B */
                self.instrCB__RLC_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x01 => {
                /* RLC C */
                self.instrCB__RLC_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x02 => {
                /* RLC D */
                self.instrCB__RLC_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x03 => {
                /* RLC E */
                self.instrCB__RLC_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x04 => {
                /* RLC H */
                self.instrCB__RLC_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x05 => {
                /* RLC L */
                self.instrCB__RLC_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x06 => {
                /* RLC (HL) */
                self.instrCB__RLC_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x07 => {
                /* RLC A */
                self.instrCB__RLC_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x08 => {
                /* RRC B */
                self.instrCB__RRC_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x09 => {
                /* RRC C */
                self.instrCB__RRC_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x0a => {
                /* RRC D */
                self.instrCB__RRC_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x0b => {
                /* RRC E */
                self.instrCB__RRC_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x0c => {
                /* RRC H */
                self.instrCB__RRC_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x0d => {
                /* RRC L */
                self.instrCB__RRC_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x0e => {
                /* RRC (HL) */
                self.instrCB__RRC_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x0f => {
                /* RRC A */
                self.instrCB__RRC_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x10 => {
                /* RL B */
                self.instrCB__RL_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x11 => {
                /* RL C */
                self.instrCB__RL_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x12 => {
                /* RL D */
                self.instrCB__RL_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x13 => {
                /* RL E */
                self.instrCB__RL_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x14 => {
                /* RL H */
                self.instrCB__RL_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x15 => {
                /* RL L */
                self.instrCB__RL_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x16 => {
                /* RL (HL) */
                self.instrCB__RL_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x17 => {
                /* RL A */
                self.instrCB__RL_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x18 => {
                /* RR B */
                self.instrCB__RR_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x19 => {
                /* RR C */
                self.instrCB__RR_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x1a => {
                /* RR D */
                self.instrCB__RR_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x1b => {
                /* RR E */
                self.instrCB__RR_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x1c => {
                /* RR H */
                self.instrCB__RR_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x1d => {
                /* RR L */
                self.instrCB__RR_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x1e => {
                /* RR (HL) */
                self.instrCB__RR_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x1f => {
                /* RR A */
                self.instrCB__RR_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x20 => {
                /* SLA B */
                self.instrCB__SLA_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x21 => {
                /* SLA C */
                self.instrCB__SLA_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x22 => {
                /* SLA D */
                self.instrCB__SLA_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x23 => {
                /* SLA E */
                self.instrCB__SLA_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x24 => {
                /* SLA H */
                self.instrCB__SLA_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x25 => {
                /* SLA L */
                self.instrCB__SLA_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x26 => {
                /* SLA (HL) */
                self.instrCB__SLA_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x27 => {
                /* SLA A */
                self.instrCB__SLA_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x28 => {
                /* SRA B */
                self.instrCB__SRA_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x29 => {
                /* SRA C */
                self.instrCB__SRA_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x2a => {
                /* SRA D */
                self.instrCB__SRA_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x2b => {
                /* SRA E */
                self.instrCB__SRA_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x2c => {
                /* SRA H */
                self.instrCB__SRA_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x2d => {
                /* SRA L */
                self.instrCB__SRA_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x2e => {
                /* SRA (HL) */
                self.instrCB__SRA_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x2f => {
                /* SRA A */
                self.instrCB__SRA_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x30 => {
                /* SLL B */
                self.instrCB__SLL_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x31 => {
                /* SLL C */
                self.instrCB__SLL_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x32 => {
                /* SLL D */
                self.instrCB__SLL_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x33 => {
                /* SLL E */
                self.instrCB__SLL_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x34 => {
                /* SLL H */
                self.instrCB__SLL_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x35 => {
                /* SLL L */
                self.instrCB__SLL_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x36 => {
                /* SLL (HL) */
                self.instrCB__SLL_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x37 => {
                /* SLL A */
                self.instrCB__SLL_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x38 => {
                /* SRL B */
                self.instrCB__SRL_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x39 => {
                /* SRL C */
                self.instrCB__SRL_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x3a => {
                /* SRL D */
                self.instrCB__SRL_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x3b => {
                /* SRL E */
                self.instrCB__SRL_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x3c => {
                /* SRL H */
                self.instrCB__SRL_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x3d => {
                /* SRL L */
                self.instrCB__SRL_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x3e => {
                /* SRL (HL) */
                self.instrCB__SRL_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x3f => {
                /* SRL A */
                self.instrCB__SRL_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x40 => {
                /* BIT 0,B */
                self.instrCB__BIT_0_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x41 => {
                /* BIT 0,C */
                self.instrCB__BIT_0_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x42 => {
                /* BIT 0,D */
                self.instrCB__BIT_0_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x43 => {
                /* BIT 0,E */
                self.instrCB__BIT_0_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x44 => {
                /* BIT 0,H */
                self.instrCB__BIT_0_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x45 => {
                /* BIT 0,L */
                self.instrCB__BIT_0_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x46 => {
                /* BIT 0,(HL) */
                self.instrCB__BIT_0_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x47 => {
                /* BIT 0,A */
                self.instrCB__BIT_0_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x48 => {
                /* BIT 1,B */
                self.instrCB__BIT_1_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x49 => {
                /* BIT 1,C */
                self.instrCB__BIT_1_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x4a => {
                /* BIT 1,D */
                self.instrCB__BIT_1_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x4b => {
                /* BIT 1,E */
                self.instrCB__BIT_1_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x4c => {
                /* BIT 1,H */
                self.instrCB__BIT_1_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x4d => {
                /* BIT 1,L */
                self.instrCB__BIT_1_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x4e => {
                /* BIT 1,(HL) */
                self.instrCB__BIT_1_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x4f => {
                /* BIT 1,A */
                self.instrCB__BIT_1_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x50 => {
                /* BIT 2,B */
                self.instrCB__BIT_2_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x51 => {
                /* BIT 2,C */
                self.instrCB__BIT_2_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x52 => {
                /* BIT 2,D */
                self.instrCB__BIT_2_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x53 => {
                /* BIT 2,E */
                self.instrCB__BIT_2_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x54 => {
                /* BIT 2,H */
                self.instrCB__BIT_2_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x55 => {
                /* BIT 2,L */
                self.instrCB__BIT_2_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x56 => {
                /* BIT 2,(HL) */
                self.instrCB__BIT_2_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x57 => {
                /* BIT 2,A */
                self.instrCB__BIT_2_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x58 => {
                /* BIT 3,B */
                self.instrCB__BIT_3_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x59 => {
                /* BIT 3,C */
                self.instrCB__BIT_3_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x5a => {
                /* BIT 3,D */
                self.instrCB__BIT_3_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x5b => {
                /* BIT 3,E */
                self.instrCB__BIT_3_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x5c => {
                /* BIT 3,H */
                self.instrCB__BIT_3_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x5d => {
                /* BIT 3,L */
                self.instrCB__BIT_3_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x5e => {
                /* BIT 3,(HL) */
                self.instrCB__BIT_3_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x5f => {
                /* BIT 3,A */
                self.instrCB__BIT_3_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x60 => {
                /* BIT 4,B */
                self.instrCB__BIT_4_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x61 => {
                /* BIT 4,C */
                self.instrCB__BIT_4_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x62 => {
                /* BIT 4,D */
                self.instrCB__BIT_4_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x63 => {
                /* BIT 4,E */
                self.instrCB__BIT_4_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x64 => {
                /* BIT 4,H */
                self.instrCB__BIT_4_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x65 => {
                /* BIT 4,L */
                self.instrCB__BIT_4_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x66 => {
                /* BIT 4,(HL) */
                self.instrCB__BIT_4_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x67 => {
                /* BIT 4,A */
                self.instrCB__BIT_4_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x68 => {
                /* BIT 5,B */
                self.instrCB__BIT_5_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x69 => {
                /* BIT 5,C */
                self.instrCB__BIT_5_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x6a => {
                /* BIT 5,D */
                self.instrCB__BIT_5_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x6b => {
                /* BIT 5,E */
                self.instrCB__BIT_5_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x6c => {
                /* BIT 5,H */
                self.instrCB__BIT_5_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x6d => {
                /* BIT 5,L */
                self.instrCB__BIT_5_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x6e => {
                /* BIT 5,(HL) */
                self.instrCB__BIT_5_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x6f => {
                /* BIT 5,A */
                self.instrCB__BIT_5_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x70 => {
                /* BIT 6,B */
                self.instrCB__BIT_6_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x71 => {
                /* BIT 6,C */
                self.instrCB__BIT_6_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x72 => {
                /* BIT 6,D */
                self.instrCB__BIT_6_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x73 => {
                /* BIT 6,E */
                self.instrCB__BIT_6_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x74 => {
                /* BIT 6,H */
                self.instrCB__BIT_6_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x75 => {
                /* BIT 6,L */
                self.instrCB__BIT_6_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x76 => {
                /* BIT 6,(HL) */
                self.instrCB__BIT_6_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x77 => {
                /* BIT 6,A */
                self.instrCB__BIT_6_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x78 => {
                /* BIT 7,B */
                self.instrCB__BIT_7_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x79 => {
                /* BIT 7,C */
                self.instrCB__BIT_7_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x7a => {
                /* BIT 7,D */
                self.instrCB__BIT_7_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x7b => {
                /* BIT 7,E */
                self.instrCB__BIT_7_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x7c => {
                /* BIT 7,H */
                self.instrCB__BIT_7_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x7d => {
                /* BIT 7,L */
                self.instrCB__BIT_7_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x7e => {
                /* BIT 7,(HL) */
                self.instrCB__BIT_7_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x7f => {
                /* BIT 7,A */
                self.instrCB__BIT_7_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x80 => {
                /* RES 0,B */
                self.instrCB__RES_0_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x81 => {
                /* RES 0,C */
                self.instrCB__RES_0_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x82 => {
                /* RES 0,D */
                self.instrCB__RES_0_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x83 => {
                /* RES 0,E */
                self.instrCB__RES_0_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x84 => {
                /* RES 0,H */
                self.instrCB__RES_0_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x85 => {
                /* RES 0,L */
                self.instrCB__RES_0_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x86 => {
                /* RES 0,(HL) */
                self.instrCB__RES_0_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x87 => {
                /* RES 0,A */
                self.instrCB__RES_0_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x88 => {
                /* RES 1,B */
                self.instrCB__RES_1_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x89 => {
                /* RES 1,C */
                self.instrCB__RES_1_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x8a => {
                /* RES 1,D */
                self.instrCB__RES_1_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x8b => {
                /* RES 1,E */
                self.instrCB__RES_1_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x8c => {
                /* RES 1,H */
                self.instrCB__RES_1_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x8d => {
                /* RES 1,L */
                self.instrCB__RES_1_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x8e => {
                /* RES 1,(HL) */
                self.instrCB__RES_1_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x8f => {
                /* RES 1,A */
                self.instrCB__RES_1_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x90 => {
                /* RES 2,B */
                self.instrCB__RES_2_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x91 => {
                /* RES 2,C */
                self.instrCB__RES_2_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x92 => {
                /* RES 2,D */
                self.instrCB__RES_2_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x93 => {
                /* RES 2,E */
                self.instrCB__RES_2_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x94 => {
                /* RES 2,H */
                self.instrCB__RES_2_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x95 => {
                /* RES 2,L */
                self.instrCB__RES_2_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x96 => {
                /* RES 2,(HL) */
                self.instrCB__RES_2_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x97 => {
                /* RES 2,A */
                self.instrCB__RES_2_A();
                true
            }

            val if val == SHIFT_0X_CB + 0x98 => {
                /* RES 3,B */
                self.instrCB__RES_3_B();
                true
            }

            val if val == SHIFT_0X_CB + 0x99 => {
                /* RES 3,C */
                self.instrCB__RES_3_C();
                true
            }

            val if val == SHIFT_0X_CB + 0x9a => {
                /* RES 3,D */
                self.instrCB__RES_3_D();
                true
            }

            val if val == SHIFT_0X_CB + 0x9b => {
                /* RES 3,E */
                self.instrCB__RES_3_E();
                true
            }

            val if val == SHIFT_0X_CB + 0x9c => {
                /* RES 3,H */
                self.instrCB__RES_3_H();
                true
            }

            val if val == SHIFT_0X_CB + 0x9d => {
                /* RES 3,L */
                self.instrCB__RES_3_L();
                true
            }

            val if val == SHIFT_0X_CB + 0x9e => {
                /* RES 3,(HL) */
                self.instrCB__RES_3_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0x9f => {
                /* RES 3,A */
                self.instrCB__RES_3_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xa0 => {
                /* RES 4,B */
                self.instrCB__RES_4_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xa1 => {
                /* RES 4,C */
                self.instrCB__RES_4_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xa2 => {
                /* RES 4,D */
                self.instrCB__RES_4_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xa3 => {
                /* RES 4,E */
                self.instrCB__RES_4_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xa4 => {
                /* RES 4,H */
                self.instrCB__RES_4_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xa5 => {
                /* RES 4,L */
                self.instrCB__RES_4_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xa6 => {
                /* RES 4,(HL) */
                self.instrCB__RES_4_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xa7 => {
                /* RES 4,A */
                self.instrCB__RES_4_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xa8 => {
                /* RES 5,B */
                self.instrCB__RES_5_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xa9 => {
                /* RES 5,C */
                self.instrCB__RES_5_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xaa => {
                /* RES 5,D */
                self.instrCB__RES_5_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xab => {
                /* RES 5,E */
                self.instrCB__RES_5_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xac => {
                /* RES 5,H */
                self.instrCB__RES_5_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xad => {
                /* RES 5,L */
                self.instrCB__RES_5_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xae => {
                /* RES 5,(HL) */
                self.instrCB__RES_5_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xaf => {
                /* RES 5,A */
                self.instrCB__RES_5_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xb0 => {
                /* RES 6,B */
                self.instrCB__RES_6_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xb1 => {
                /* RES 6,C */
                self.instrCB__RES_6_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xb2 => {
                /* RES 6,D */
                self.instrCB__RES_6_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xb3 => {
                /* RES 6,E */
                self.instrCB__RES_6_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xb4 => {
                /* RES 6,H */
                self.instrCB__RES_6_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xb5 => {
                /* RES 6,L */
                self.instrCB__RES_6_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xb6 => {
                /* RES 6,(HL) */
                self.instrCB__RES_6_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xb7 => {
                /* RES 6,A */
                self.instrCB__RES_6_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xb8 => {
                /* RES 7,B */
                self.instrCB__RES_7_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xb9 => {
                /* RES 7,C */
                self.instrCB__RES_7_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xba => {
                /* RES 7,D */
                self.instrCB__RES_7_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xbb => {
                /* RES 7,E */
                self.instrCB__RES_7_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xbc => {
                /* RES 7,H */
                self.instrCB__RES_7_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xbd => {
                /* RES 7,L */
                self.instrCB__RES_7_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xbe => {
                /* RES 7,(HL) */
                self.instrCB__RES_7_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xbf => {
                /* RES 7,A */
                self.instrCB__RES_7_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xc0 => {
                /* SET 0,B */
                self.instrCB__SET_0_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xc1 => {
                /* SET 0,C */
                self.instrCB__SET_0_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xc2 => {
                /* SET 0,D */
                self.instrCB__SET_0_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xc3 => {
                /* SET 0,E */
                self.instrCB__SET_0_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xc4 => {
                /* SET 0,H */
                self.instrCB__SET_0_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xc5 => {
                /* SET 0,L */
                self.instrCB__SET_0_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xc6 => {
                /* SET 0,(HL) */
                self.instrCB__SET_0_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xc7 => {
                /* SET 0,A */
                self.instrCB__SET_0_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xc8 => {
                /* SET 1,B */
                self.instrCB__SET_1_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xc9 => {
                /* SET 1,C */
                self.instrCB__SET_1_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xca => {
                /* SET 1,D */
                self.instrCB__SET_1_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xcb => {
                /* SET 1,E */
                self.instrCB__SET_1_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xcc => {
                /* SET 1,H */
                self.instrCB__SET_1_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xcd => {
                /* SET 1,L */
                self.instrCB__SET_1_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xce => {
                /* SET 1,(HL) */
                self.instrCB__SET_1_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xcf => {
                /* SET 1,A */
                self.instrCB__SET_1_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xd0 => {
                /* SET 2,B */
                self.instrCB__SET_2_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xd1 => {
                /* SET 2,C */
                self.instrCB__SET_2_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xd2 => {
                /* SET 2,D */
                self.instrCB__SET_2_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xd3 => {
                /* SET 2,E */
                self.instrCB__SET_2_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xd4 => {
                /* SET 2,H */
                self.instrCB__SET_2_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xd5 => {
                /* SET 2,L */
                self.instrCB__SET_2_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xd6 => {
                /* SET 2,(HL) */
                self.instrCB__SET_2_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xd7 => {
                /* SET 2,A */
                self.instrCB__SET_2_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xd8 => {
                /* SET 3,B */
                self.instrCB__SET_3_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xd9 => {
                /* SET 3,C */
                self.instrCB__SET_3_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xda => {
                /* SET 3,D */
                self.instrCB__SET_3_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xdb => {
                /* SET 3,E */
                self.instrCB__SET_3_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xdc => {
                /* SET 3,H */
                self.instrCB__SET_3_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xdd => {
                /* SET 3,L */
                self.instrCB__SET_3_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xde => {
                /* SET 3,(HL) */
                self.instrCB__SET_3_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xdf => {
                /* SET 3,A */
                self.instrCB__SET_3_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xe0 => {
                /* SET 4,B */
                self.instrCB__SET_4_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xe1 => {
                /* SET 4,C */
                self.instrCB__SET_4_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xe2 => {
                /* SET 4,D */
                self.instrCB__SET_4_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xe3 => {
                /* SET 4,E */
                self.instrCB__SET_4_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xe4 => {
                /* SET 4,H */
                self.instrCB__SET_4_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xe5 => {
                /* SET 4,L */
                self.instrCB__SET_4_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xe6 => {
                /* SET 4,(HL) */
                self.instrCB__SET_4_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xe7 => {
                /* SET 4,A */
                self.instrCB__SET_4_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xe8 => {
                /* SET 5,B */
                self.instrCB__SET_5_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xe9 => {
                /* SET 5,C */
                self.instrCB__SET_5_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xea => {
                /* SET 5,D */
                self.instrCB__SET_5_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xeb => {
                /* SET 5,E */
                self.instrCB__SET_5_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xec => {
                /* SET 5,H */
                self.instrCB__SET_5_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xed => {
                /* SET 5,L */
                self.instrCB__SET_5_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xee => {
                /* SET 5,(HL) */
                self.instrCB__SET_5_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xef => {
                /* SET 5,A */
                self.instrCB__SET_5_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xf0 => {
                /* SET 6,B */
                self.instrCB__SET_6_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xf1 => {
                /* SET 6,C */
                self.instrCB__SET_6_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xf2 => {
                /* SET 6,D */
                self.instrCB__SET_6_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xf3 => {
                /* SET 6,E */
                self.instrCB__SET_6_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xf4 => {
                /* SET 6,H */
                self.instrCB__SET_6_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xf5 => {
                /* SET 6,L */
                self.instrCB__SET_6_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xf6 => {
                /* SET 6,(HL) */
                self.instrCB__SET_6_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xf7 => {
                /* SET 6,A */
                self.instrCB__SET_6_A();
                true
            }

            val if val == SHIFT_0X_CB + 0xf8 => {
                /* SET 7,B */
                self.instrCB__SET_7_B();
                true
            }

            val if val == SHIFT_0X_CB + 0xf9 => {
                /* SET 7,C */
                self.instrCB__SET_7_C();
                true
            }

            val if val == SHIFT_0X_CB + 0xfa => {
                /* SET 7,D */
                self.instrCB__SET_7_D();
                true
            }

            val if val == SHIFT_0X_CB + 0xfb => {
                /* SET 7,E */
                self.instrCB__SET_7_E();
                true
            }

            val if val == SHIFT_0X_CB + 0xfc => {
                /* SET 7,H */
                self.instrCB__SET_7_H();
                true
            }

            val if val == SHIFT_0X_CB + 0xfd => {
                /* SET 7,L */
                self.instrCB__SET_7_L();
                true
            }

            val if val == SHIFT_0X_CB + 0xfe => {
                /* SET 7,(HL) */
                self.instrCB__SET_7_iHL();
                true
            }

            val if val == SHIFT_0X_CB + 0xff => {
                /* SET 7,A */
                self.instrCB__SET_7_A();
                true
            }

            val if val == SHIFT_0X_ED + 0x40 => {
                /* IN B,(C) */
                self.instrED__IN_B_iC();
                true
            }

            val if val == SHIFT_0X_ED + 0x41 => {
                /* OUT (C),B */
                self.instrED__OUT_iC_B();
                true
            }

            val if val == SHIFT_0X_ED + 0x42 => {
                /* SBC HL,BC */
                self.instrED__SBC_HL_BC();
                true
            }

            val if val == SHIFT_0X_ED + 0x43 => {
                /* LD (nnnn),BC */
                self.instrED__LD_iNNNN_BC();
                true
            }

            val if val == SHIFT_0X_ED + 0x7c => {
                /* NEG */
                self.instrED__NEG();
                true
            }

            val if val == SHIFT_0X_ED + 0x44 => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();return true;
                self.instrED__NEG();
                true
            }

            val if val == SHIFT_0X_ED + 0x4c => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();return true;
                self.instrED__NEG();
                true
            }

            val if val == SHIFT_0X_ED + 0x54 => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();return true;
                self.instrED__NEG();
                true
            }

            val if val == SHIFT_0X_ED + 0x5c => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();return true;
                self.instrED__NEG();
                true
            }

            val if val == SHIFT_0X_ED + 0x64 => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();return true;
                self.instrED__NEG();
                true
            }

            val if val == SHIFT_0X_ED + 0x6c => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();return true;
                self.instrED__NEG();
                true
            }

            val if val == SHIFT_0X_ED + 0x74 => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();return true;
                self.instrED__NEG();
                true
            }

            val if val == SHIFT_0X_ED + 0x7d => {
                /* RETN */
                self.instrED__RETN();
                true
            }

            val if val == SHIFT_0X_ED + 0x45 => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();return true;
                self.instrED__RETN();
                true
            }

            val if val == SHIFT_0X_ED + 0x4d => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();return true;
                self.instrED__RETN();
                true
            }

            val if val == SHIFT_0X_ED + 0x55 => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();return true;
                self.instrED__RETN();
                true
            }

            val if val == SHIFT_0X_ED + 0x5d => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();return true;
                self.instrED__RETN();
                true
            }

            val if val == SHIFT_0X_ED + 0x65 => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();return true;
                self.instrED__RETN();
                true
            }

            val if val == SHIFT_0X_ED + 0x6d => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();return true;
                self.instrED__RETN();
                true
            }

            val if val == SHIFT_0X_ED + 0x75 => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();return true;
                self.instrED__RETN();
                true
            }

            val if val == SHIFT_0X_ED + 0x6e => {
                /* IM 0 */
                self.instrED__IM_0();
                true
            }

            val if val == SHIFT_0X_ED + 0x46 => {
                /* IM 0 */
                // self.OpcodesMap[SHIFT_0xED + 0x6e]();return true;
                self.instrED__IM_0();
                true
            }

            val if val == SHIFT_0X_ED + 0x4e => {
                /* IM 0 */
                // self.OpcodesMap[SHIFT_0xED + 0x6e]();return true;
                self.instrED__IM_0();
                true
            }

            val if val == SHIFT_0X_ED + 0x66 => {
                /* IM 0 */
                // self.OpcodesMap[SHIFT_0xED + 0x6e]();return true;
                self.instrED__IM_0();
                true
            }

            val if val == SHIFT_0X_ED + 0x47 => {
                /* LD I,A */
                self.instrED__LD_I_A();
                true
            }

            val if val == SHIFT_0X_ED + 0x48 => {
                /* IN C,(C) */
                self.instrED__IN_C_iC();
                true
            }

            val if val == SHIFT_0X_ED + 0x49 => {
                /* OUT (C),C */
                self.instrED__OUT_iC_C();
                true
            }

            val if val == SHIFT_0X_ED + 0x4a => {
                /* ADC HL,BC */
                self.instrED__ADC_HL_BC();
                true
            }

            val if val == SHIFT_0X_ED + 0x4b => {
                /* LD BC,(nnnn) */
                self.instrED__LD_BC_iNNNN();
                true
            }

            val if val == SHIFT_0X_ED + 0x4f => {
                /* LD R,A */
                self.instrED__LD_R_A();
                true
            }

            val if val == SHIFT_0X_ED + 0x50 => {
                /* IN D,(C) */
                self.instrED__IN_D_iC();
                true
            }

            val if val == SHIFT_0X_ED + 0x51 => {
                /* OUT (C),D */
                self.instrED__OUT_iC_D();
                true
            }

            val if val == SHIFT_0X_ED + 0x52 => {
                /* SBC HL,DE */
                self.instrED__SBC_HL_DE();
                true
            }

            val if val == SHIFT_0X_ED + 0x53 => {
                /* LD (nnnn),DE */
                self.instrED__LD_iNNNN_DE();
                true
            }

            val if val == SHIFT_0X_ED + 0x76 => {
                /* IM 1 */
                self.instrED__IM_1();
                true
            }

            val if val == SHIFT_0X_ED + 0x56 => {
                /* IM 1 */
                // self.OpcodesMap[SHIFT_0xED + 0x76]();return true;
                self.instrED__IM_1();
                true
            }

            val if val == SHIFT_0X_ED + 0x57 => {
                /* LD A,I */
                self.instrED__LD_A_I();
                true
            }

            val if val == SHIFT_0X_ED + 0x58 => {
                /* IN E,(C) */
                self.instrED__IN_E_iC();
                true
            }

            val if val == SHIFT_0X_ED + 0x59 => {
                /* OUT (C),E */
                self.instrED__OUT_iC_E();
                true
            }

            val if val == SHIFT_0X_ED + 0x5a => {
                /* ADC HL,DE */
                self.instrED__ADC_HL_DE();
                true
            }

            val if val == SHIFT_0X_ED + 0x5b => {
                /* LD DE,(nnnn) */
                self.instrED__LD_DE_iNNNN();
                true
            }

            val if val == SHIFT_0X_ED + 0x7e => {
                /* IM 2 */
                self.instrED__IM_2();
                true
            }

            val if val == SHIFT_0X_ED + 0x5e => {
                /* IM 2 */
                // self.OpcodesMap[SHIFT_0xED + 0x7e]();return true;
                self.instrED__IM_2();
                true
            }

            val if val == SHIFT_0X_ED + 0x5f => {
                /* LD A,R */
                self.instrED__LD_A_R();
                true
            }

            val if val == SHIFT_0X_ED + 0x60 => {
                /* IN H,(C) */
                self.instrED__IN_H_iC();
                true
            }

            val if val == SHIFT_0X_ED + 0x61 => {
                /* OUT (C),H */
                self.instrED__OUT_iC_H();
                true
            }

            val if val == SHIFT_0X_ED + 0x62 => {
                /* SBC HL,HL */
                self.instrED__SBC_HL_HL();
                true
            }

            val if val == SHIFT_0X_ED + 0x63 => {
                /* LD (nnnn),HL */
                self.instrED__LD_iNNNN_HL();
                true
            }

            val if val == SHIFT_0X_ED + 0x67 => {
                /* RRD */
                self.instrED__RRD();
                true
            }

            val if val == SHIFT_0X_ED + 0x68 => {
                /* IN L,(C) */
                self.instrED__IN_L_iC();
                true
            }

            val if val == SHIFT_0X_ED + 0x69 => {
                /* OUT (C),L */
                self.instrED__OUT_iC_L();
                true
            }

            val if val == SHIFT_0X_ED + 0x6a => {
                /* ADC HL,HL */
                self.instrED__ADC_HL_HL();
                true
            }

            val if val == SHIFT_0X_ED + 0x6b => {
                /* LD HL,(nnnn) */
                self.instrED__LD_HL_iNNNN();
                true
            }

            val if val == SHIFT_0X_ED + 0x6f => {
                /* RLD */
                self.instrED__RLD();
                true
            }

            val if val == SHIFT_0X_ED + 0x70 => {
                /* IN F,(C) */
                self.instrED__IN_F_iC();
                true
            }

            val if val == SHIFT_0X_ED + 0x71 => {
                /* OUT (C),0 */
                self.instrED__OUT_iC_0();
                true
            }

            val if val == SHIFT_0X_ED + 0x72 => {
                /* SBC HL,SP */
                self.instrED__SBC_HL_SP();
                true
            }

            val if val == SHIFT_0X_ED + 0x73 => {
                /* LD (nnnn),SP */
                self.instrED__LD_iNNNN_SP();
                true
            }

            val if val == SHIFT_0X_ED + 0x78 => {
                /* IN A,(C) */
                self.instrED__IN_A_iC();
                true
            }

            val if val == SHIFT_0X_ED + 0x79 => {
                /* OUT (C),A */
                self.instrED__OUT_iC_A();
                true
            }

            val if val == SHIFT_0X_ED + 0x7a => {
                /* ADC HL,SP */
                self.instrED__ADC_HL_SP();
                true
            }

            val if val == SHIFT_0X_ED + 0x7b => {
                /* LD SP,(nnnn) */
                self.instrED__LD_SP_iNNNN();
                true
            }

            val if val == SHIFT_0X_ED + 0xa0 => {
                /* LDI */
                self.instrED__LDI();
                true
            }

            val if val == SHIFT_0X_ED + 0xa1 => {
                /* CPI */
                self.instrED__CPI();
                true
            }

            val if val == SHIFT_0X_ED + 0xa2 => {
                /* INI */
                self.instrED__INI();
                true
            }

            val if val == SHIFT_0X_ED + 0xa3 => {
                /* OUTI */
                self.instrED__OUTI();
                true
            }

            val if val == SHIFT_0X_ED + 0xa8 => {
                /* LDD */
                self.instrED__LDD();
                true
            }

            val if val == SHIFT_0X_ED + 0xa9 => {
                /* CPD */
                self.instrED__CPD();
                true
            }

            val if val == SHIFT_0X_ED + 0xaa => {
                /* IND */
                self.instrED__IND();
                true
            }

            val if val == SHIFT_0X_ED + 0xab => {
                /* OUTD */
                self.instrED__OUTD();
                true
            }

            val if val == SHIFT_0X_ED + 0xb0 => {
                /* LDIR */
                self.instrED__LDIR();
                true
            }

            val if val == SHIFT_0X_ED + 0xb1 => {
                /* CPIR */
                self.instrED__CPIR();
                true
            }

            val if val == SHIFT_0X_ED + 0xb2 => {
                /* INIR */
                self.instrED__INIR();
                true
            }

            val if val == SHIFT_0X_ED + 0xb3 => {
                /* OTIR */
                self.instrED__OTIR();
                true
            }

            val if val == SHIFT_0X_ED + 0xb8 => {
                /* LDDR */
                self.instrED__LDDR();
                true
            }

            val if val == SHIFT_0X_ED + 0xb9 => {
                /* CPDR */
                self.instrED__CPDR();
                true
            }

            val if val == SHIFT_0X_ED + 0xba => {
                /* INDR */
                self.instrED__INDR();
                true
            }

            val if val == SHIFT_0X_ED + 0xbb => {
                /* OTDR */
                self.instrED__OTDR();
                true
            }

            val if val == SHIFT_0X_ED + 0xfb => {
                /* slttrap */
                self.instrED__SLTTRAP();
                true
            }

            val if val == SHIFT_0X_DD + 0x09 => {
                /* ADD REGISTER,BC */
                self.instrDD__ADD_REG_BC();
                true
            }

            val if val == SHIFT_0X_DD + 0x19 => {
                /* ADD REGISTER,DE */
                self.instrDD__ADD_REG_DE();
                true
            }

            val if val == SHIFT_0X_DD + 0x21 => {
                /* LD REGISTER,nnnn */
                self.instrDD__LD_REG_NNNN();
                true
            }

            val if val == SHIFT_0X_DD + 0x22 => {
                /* LD (nnnn),REGISTER */
                self.instrDD__LD_iNNNN_REG();
                true
            }

            val if val == SHIFT_0X_DD + 0x23 => {
                /* INC REGISTER */
                self.instrDD__INC_REG();
                true
            }

            val if val == SHIFT_0X_DD + 0x24 => {
                /* INC REGISTERH */
                self.instrDD__INC_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x25 => {
                /* DEC REGISTERH */
                self.instrDD__DEC_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x26 => {
                /* LD REGISTERH,nn */
                self.instrDD__LD_REGH_NN();
                true
            }

            val if val == SHIFT_0X_DD + 0x29 => {
                /* ADD REGISTER,REGISTER */
                self.instrDD__ADD_REG_REG();
                true
            }

            val if val == SHIFT_0X_DD + 0x2a => {
                /* LD REGISTER,(nnnn) */
                self.instrDD__LD_REG_iNNNN();
                true
            }

            val if val == SHIFT_0X_DD + 0x2b => {
                /* DEC REGISTER */
                self.instrDD__DEC_REG();
                true
            }

            val if val == SHIFT_0X_DD + 0x2c => {
                /* INC REGISTERL */
                self.instrDD__INC_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x2d => {
                /* DEC REGISTERL */
                self.instrDD__DEC_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x2e => {
                /* LD REGISTERL,nn */
                self.instrDD__LD_REGL_NN();
                true
            }

            val if val == SHIFT_0X_DD + 0x34 => {
                /* INC (REGISTER+dd) */
                self.instrDD__INC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x35 => {
                /* DEC (REGISTER+dd) */
                self.instrDD__DEC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x36 => {
                /* LD (REGISTER+dd),nn */
                self.instrDD__LD_iREGpDD_NN();
                true
            }

            val if val == SHIFT_0X_DD + 0x39 => {
                /* ADD REGISTER,SP */
                self.instrDD__ADD_REG_SP();
                true
            }

            val if val == SHIFT_0X_DD + 0x44 => {
                /* LD B,REGISTERH */
                self.instrDD__LD_B_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x45 => {
                /* LD B,REGISTERL */
                self.instrDD__LD_B_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x46 => {
                /* LD B,(REGISTER+dd) */
                self.instrDD__LD_B_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x4c => {
                /* LD C,REGISTERH */
                self.instrDD__LD_C_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x4d => {
                /* LD C,REGISTERL */
                self.instrDD__LD_C_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x4e => {
                /* LD C,(REGISTER+dd) */
                self.instrDD__LD_C_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x54 => {
                /* LD D,REGISTERH */
                self.instrDD__LD_D_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x55 => {
                /* LD D,REGISTERL */
                self.instrDD__LD_D_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x56 => {
                /* LD D,(REGISTER+dd) */
                self.instrDD__LD_D_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x5c => {
                /* LD E,REGISTERH */
                self.instrDD__LD_E_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x5d => {
                /* LD E,REGISTERL */
                self.instrDD__LD_E_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x5e => {
                /* LD E,(REGISTER+dd) */
                self.instrDD__LD_E_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x60 => {
                /* LD REGISTERH,B */
                self.instrDD__LD_REGH_B();
                true
            }

            val if val == SHIFT_0X_DD + 0x61 => {
                /* LD REGISTERH,C */
                self.instrDD__LD_REGH_C();
                true
            }

            val if val == SHIFT_0X_DD + 0x62 => {
                /* LD REGISTERH,D */
                self.instrDD__LD_REGH_D();
                true
            }

            val if val == SHIFT_0X_DD + 0x63 => {
                /* LD REGISTERH,E */
                self.instrDD__LD_REGH_E();
                true
            }

            val if val == SHIFT_0X_DD + 0x64 => {
                /* LD REGISTERH,REGISTERH */
                self.instrDD__LD_REGH_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x65 => {
                /* LD REGISTERH,REGISTERL */
                self.instrDD__LD_REGH_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x66 => {
                /* LD H,(REGISTER+dd) */
                self.instrDD__LD_H_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x67 => {
                /* LD REGISTERH,A */
                self.instrDD__LD_REGH_A();
                true
            }

            val if val == SHIFT_0X_DD + 0x68 => {
                /* LD REGISTERL,B */
                self.instrDD__LD_REGL_B();
                true
            }

            val if val == SHIFT_0X_DD + 0x69 => {
                /* LD REGISTERL,C */
                self.instrDD__LD_REGL_C();
                true
            }

            val if val == SHIFT_0X_DD + 0x6a => {
                /* LD REGISTERL,D */
                self.instrDD__LD_REGL_D();
                true
            }

            val if val == SHIFT_0X_DD + 0x6b => {
                /* LD REGISTERL,E */
                self.instrDD__LD_REGL_E();
                true
            }

            val if val == SHIFT_0X_DD + 0x6c => {
                /* LD REGISTERL,REGISTERH */
                self.instrDD__LD_REGL_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x6d => {
                /* LD REGISTERL,REGISTERL */
                self.instrDD__LD_REGL_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x6e => {
                /* LD L,(REGISTER+dd) */
                self.instrDD__LD_L_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x6f => {
                /* LD REGISTERL,A */
                self.instrDD__LD_REGL_A();
                true
            }

            val if val == SHIFT_0X_DD + 0x70 => {
                /* LD (REGISTER+dd),B */
                self.instrDD__LD_iREGpDD_B();
                true
            }

            val if val == SHIFT_0X_DD + 0x71 => {
                /* LD (REGISTER+dd),C */
                self.instrDD__LD_iREGpDD_C();
                true
            }

            val if val == SHIFT_0X_DD + 0x72 => {
                /* LD (REGISTER+dd),D */
                self.instrDD__LD_iREGpDD_D();
                true
            }

            val if val == SHIFT_0X_DD + 0x73 => {
                /* LD (REGISTER+dd),E */
                self.instrDD__LD_iREGpDD_E();
                true
            }

            val if val == SHIFT_0X_DD + 0x74 => {
                /* LD (REGISTER+dd),H */
                self.instrDD__LD_iREGpDD_H();
                true
            }

            val if val == SHIFT_0X_DD + 0x75 => {
                /* LD (REGISTER+dd),L */
                self.instrDD__LD_iREGpDD_L();
                true
            }

            val if val == SHIFT_0X_DD + 0x77 => {
                /* LD (REGISTER+dd),A */
                self.instrDD__LD_iREGpDD_A();
                true
            }

            val if val == SHIFT_0X_DD + 0x7c => {
                /* LD A,REGISTERH */
                self.instrDD__LD_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x7d => {
                /* LD A,REGISTERL */
                self.instrDD__LD_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x7e => {
                /* LD A,(REGISTER+dd) */
                self.instrDD__LD_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x84 => {
                /* ADD A,REGISTERH */
                self.instrDD__ADD_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x85 => {
                /* ADD A,REGISTERL */
                self.instrDD__ADD_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x86 => {
                /* ADD A,(REGISTER+dd) */
                self.instrDD__ADD_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x8c => {
                /* ADC A,REGISTERH */
                self.instrDD__ADC_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x8d => {
                /* ADC A,REGISTERL */
                self.instrDD__ADC_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x8e => {
                /* ADC A,(REGISTER+dd) */
                self.instrDD__ADC_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x94 => {
                /* SUB A,REGISTERH */
                self.instrDD__SUB_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x95 => {
                /* SUB A,REGISTERL */
                self.instrDD__SUB_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x96 => {
                /* SUB A,(REGISTER+dd) */
                self.instrDD__SUB_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0x9c => {
                /* SBC A,REGISTERH */
                self.instrDD__SBC_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0x9d => {
                /* SBC A,REGISTERL */
                self.instrDD__SBC_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0x9e => {
                /* SBC A,(REGISTER+dd) */
                self.instrDD__SBC_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0xa4 => {
                /* AND A,REGISTERH */
                self.instrDD__AND_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0xa5 => {
                /* AND A,REGISTERL */
                self.instrDD__AND_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0xa6 => {
                /* AND A,(REGISTER+dd) */
                self.instrDD__AND_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0xac => {
                /* XOR A,REGISTERH */
                self.instrDD__XOR_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0xad => {
                /* XOR A,REGISTERL */
                self.instrDD__XOR_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0xae => {
                /* XOR A,(REGISTER+dd) */
                self.instrDD__XOR_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0xb4 => {
                /* OR A,REGISTERH */
                self.instrDD__OR_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0xb5 => {
                /* OR A,REGISTERL */
                self.instrDD__OR_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0xb6 => {
                /* OR A,(REGISTER+dd) */
                self.instrDD__OR_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0xbc => {
                /* CP A,REGISTERH */
                self.instrDD__CP_A_REGH();
                true
            }

            val if val == SHIFT_0X_DD + 0xbd => {
                /* CP A,REGISTERL */
                self.instrDD__CP_A_REGL();
                true
            }

            val if val == SHIFT_0X_DD + 0xbe => {
                /* CP A,(REGISTER+dd) */
                self.instrDD__CP_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DD + 0xcb => {
                /* shift DDFDCB */
                self.instrDD__SHIFT_DDFDCB();
                true
            }

            val if val == SHIFT_0X_DD + 0xe1 => {
                /* POP REGISTER */
                self.instrDD__POP_REG();
                true
            }

            val if val == SHIFT_0X_DD + 0xe3 => {
                /* EX (SP),REGISTER */
                self.instrDD__EX_iSP_REG();
                true
            }

            val if val == SHIFT_0X_DD + 0xe5 => {
                /* PUSH REGISTER */
                self.instrDD__PUSH_REG();
                true
            }

            val if val == SHIFT_0X_DD + 0xe9 => {
                /* JP REGISTER */
                self.instrDD__JP_REG();
                true
            }

            val if val == SHIFT_0X_DD + 0xf9 => {
                /* LD SP,REGISTER */
                self.instrDD__LD_SP_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0x09 => {
                /* ADD REGISTER,BC */
                self.instrFD__ADD_REG_BC();
                true
            }

            val if val == SHIFT_0X_FD + 0x19 => {
                /* ADD REGISTER,DE */
                self.instrFD__ADD_REG_DE();
                true
            }

            val if val == SHIFT_0X_FD + 0x21 => {
                /* LD REGISTER,nnnn */
                self.instrFD__LD_REG_NNNN();
                true
            }

            val if val == SHIFT_0X_FD + 0x22 => {
                /* LD (nnnn),REGISTER */
                self.instrFD__LD_iNNNN_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0x23 => {
                /* INC REGISTER */
                self.instrFD__INC_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0x24 => {
                /* INC REGISTERH */
                self.instrFD__INC_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x25 => {
                /* DEC REGISTERH */
                self.instrFD__DEC_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x26 => {
                /* LD REGISTERH,nn */
                self.instrFD__LD_REGH_NN();
                true
            }

            val if val == SHIFT_0X_FD + 0x29 => {
                /* ADD REGISTER,REGISTER */
                self.instrFD__ADD_REG_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0x2a => {
                /* LD REGISTER,(nnnn) */
                self.instrFD__LD_REG_iNNNN();
                true
            }

            val if val == SHIFT_0X_FD + 0x2b => {
                /* DEC REGISTER */
                self.instrFD__DEC_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0x2c => {
                /* INC REGISTERL */
                self.instrFD__INC_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x2d => {
                /* DEC REGISTERL */
                self.instrFD__DEC_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x2e => {
                /* LD REGISTERL,nn */
                self.instrFD__LD_REGL_NN();
                true
            }

            val if val == SHIFT_0X_FD + 0x34 => {
                /* INC (REGISTER+dd) */
                self.instrFD__INC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x35 => {
                /* DEC (REGISTER+dd) */
                self.instrFD__DEC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x36 => {
                /* LD (REGISTER+dd),nn */
                self.instrFD__LD_iREGpDD_NN();
                true
            }

            val if val == SHIFT_0X_FD + 0x39 => {
                /* ADD REGISTER,SP */
                self.instrFD__ADD_REG_SP();
                true
            }

            val if val == SHIFT_0X_FD + 0x44 => {
                /* LD B,REGISTERH */
                self.instrFD__LD_B_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x45 => {
                /* LD B,REGISTERL */
                self.instrFD__LD_B_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x46 => {
                /* LD B,(REGISTER+dd) */
                self.instrFD__LD_B_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x4c => {
                /* LD C,REGISTERH */
                self.instrFD__LD_C_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x4d => {
                /* LD C,REGISTERL */
                self.instrFD__LD_C_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x4e => {
                /* LD C,(REGISTER+dd) */
                self.instrFD__LD_C_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x54 => {
                /* LD D,REGISTERH */
                self.instrFD__LD_D_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x55 => {
                /* LD D,REGISTERL */
                self.instrFD__LD_D_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x56 => {
                /* LD D,(REGISTER+dd) */
                self.instrFD__LD_D_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x5c => {
                /* LD E,REGISTERH */
                self.instrFD__LD_E_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x5d => {
                /* LD E,REGISTERL */
                self.instrFD__LD_E_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x5e => {
                /* LD E,(REGISTER+dd) */
                self.instrFD__LD_E_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x60 => {
                /* LD REGISTERH,B */
                self.instrFD__LD_REGH_B();
                true
            }

            val if val == SHIFT_0X_FD + 0x61 => {
                /* LD REGISTERH,C */
                self.instrFD__LD_REGH_C();
                true
            }

            val if val == SHIFT_0X_FD + 0x62 => {
                /* LD REGISTERH,D */
                self.instrFD__LD_REGH_D();
                true
            }

            val if val == SHIFT_0X_FD + 0x63 => {
                /* LD REGISTERH,E */
                self.instrFD__LD_REGH_E();
                true
            }

            val if val == SHIFT_0X_FD + 0x64 => {
                /* LD REGISTERH,REGISTERH */
                self.instrFD__LD_REGH_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x65 => {
                /* LD REGISTERH,REGISTERL */
                self.instrFD__LD_REGH_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x66 => {
                /* LD H,(REGISTER+dd) */
                self.instrFD__LD_H_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x67 => {
                /* LD REGISTERH,A */
                self.instrFD__LD_REGH_A();
                true
            }

            val if val == SHIFT_0X_FD + 0x68 => {
                /* LD REGISTERL,B */
                self.instrFD__LD_REGL_B();
                true
            }

            val if val == SHIFT_0X_FD + 0x69 => {
                /* LD REGISTERL,C */
                self.instrFD__LD_REGL_C();
                true
            }

            val if val == SHIFT_0X_FD + 0x6a => {
                /* LD REGISTERL,D */
                self.instrFD__LD_REGL_D();
                true
            }

            val if val == SHIFT_0X_FD + 0x6b => {
                /* LD REGISTERL,E */
                self.instrFD__LD_REGL_E();
                true
            }

            val if val == SHIFT_0X_FD + 0x6c => {
                /* LD REGISTERL,REGISTERH */
                self.instrFD__LD_REGL_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x6d => {
                /* LD REGISTERL,REGISTERL */
                self.instrFD__LD_REGL_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x6e => {
                /* LD L,(REGISTER+dd) */
                self.instrFD__LD_L_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x6f => {
                /* LD REGISTERL,A */
                self.instrFD__LD_REGL_A();
                true
            }

            val if val == SHIFT_0X_FD + 0x70 => {
                /* LD (REGISTER+dd),B */
                self.instrFD__LD_iREGpDD_B();
                true
            }

            val if val == SHIFT_0X_FD + 0x71 => {
                /* LD (REGISTER+dd),C */
                self.instrFD__LD_iREGpDD_C();
                true
            }

            val if val == SHIFT_0X_FD + 0x72 => {
                /* LD (REGISTER+dd),D */
                self.instrFD__LD_iREGpDD_D();
                true
            }

            val if val == SHIFT_0X_FD + 0x73 => {
                /* LD (REGISTER+dd),E */
                self.instrFD__LD_iREGpDD_E();
                true
            }

            val if val == SHIFT_0X_FD + 0x74 => {
                /* LD (REGISTER+dd),H */
                self.instrFD__LD_iREGpDD_H();
                true
            }

            val if val == SHIFT_0X_FD + 0x75 => {
                /* LD (REGISTER+dd),L */
                self.instrFD__LD_iREGpDD_L();
                true
            }

            val if val == SHIFT_0X_FD + 0x77 => {
                /* LD (REGISTER+dd),A */
                self.instrFD__LD_iREGpDD_A();
                true
            }

            val if val == SHIFT_0X_FD + 0x7c => {
                /* LD A,REGISTERH */
                self.instrFD__LD_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x7d => {
                /* LD A,REGISTERL */
                self.instrFD__LD_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x7e => {
                /* LD A,(REGISTER+dd) */
                self.instrFD__LD_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x84 => {
                /* ADD A,REGISTERH */
                self.instrFD__ADD_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x85 => {
                /* ADD A,REGISTERL */
                self.instrFD__ADD_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x86 => {
                /* ADD A,(REGISTER+dd) */
                self.instrFD__ADD_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x8c => {
                /* ADC A,REGISTERH */
                self.instrFD__ADC_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x8d => {
                /* ADC A,REGISTERL */
                self.instrFD__ADC_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x8e => {
                /* ADC A,(REGISTER+dd) */
                self.instrFD__ADC_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x94 => {
                /* SUB A,REGISTERH */
                self.instrFD__SUB_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x95 => {
                /* SUB A,REGISTERL */
                self.instrFD__SUB_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x96 => {
                /* SUB A,(REGISTER+dd) */
                self.instrFD__SUB_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0x9c => {
                /* SBC A,REGISTERH */
                self.instrFD__SBC_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0x9d => {
                /* SBC A,REGISTERL */
                self.instrFD__SBC_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0x9e => {
                /* SBC A,(REGISTER+dd) */
                self.instrFD__SBC_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0xa4 => {
                /* AND A,REGISTERH */
                self.instrFD__AND_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0xa5 => {
                /* AND A,REGISTERL */
                self.instrFD__AND_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0xa6 => {
                /* AND A,(REGISTER+dd) */
                self.instrFD__AND_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0xac => {
                /* XOR A,REGISTERH */
                self.instrFD__XOR_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0xad => {
                /* XOR A,REGISTERL */
                self.instrFD__XOR_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0xae => {
                /* XOR A,(REGISTER+dd) */
                self.instrFD__XOR_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0xb4 => {
                /* OR A,REGISTERH */
                self.instrFD__OR_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0xb5 => {
                /* OR A,REGISTERL */
                self.instrFD__OR_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0xb6 => {
                /* OR A,(REGISTER+dd) */
                self.instrFD__OR_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0xbc => {
                /* CP A,REGISTERH */
                self.instrFD__CP_A_REGH();
                true
            }

            val if val == SHIFT_0X_FD + 0xbd => {
                /* CP A,REGISTERL */
                self.instrFD__CP_A_REGL();
                true
            }

            val if val == SHIFT_0X_FD + 0xbe => {
                /* CP A,(REGISTER+dd) */
                self.instrFD__CP_A_iREGpDD();
                true
            }

            val if val == SHIFT_0X_FD + 0xcb => {
                /* shift DDFDCB */
                self.instrFD__SHIFT_DDFDCB();
                true
            }

            val if val == SHIFT_0X_FD + 0xe1 => {
                /* POP REGISTER */
                self.instrFD__POP_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0xe3 => {
                /* EX (SP),REGISTER */
                self.instrFD__EX_iSP_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0xe5 => {
                /* PUSH REGISTER */
                self.instrFD__PUSH_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0xe9 => {
                /* JP REGISTER */
                self.instrFD__JP_REG();
                true
            }

            val if val == SHIFT_0X_FD + 0xf9 => {
                /* LD SP,REGISTER */
                self.instrFD__LD_SP_REG();
                true
            }

            val if val == SHIFT_0X_DDCB => {
                /* LD B,RLC (REGISTER+dd) */
                self.instrDDCB__LD_B_RLC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x01 => {
                /* LD C,RLC (REGISTER+dd) */
                self.instrDDCB__LD_C_RLC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x02 => {
                /* LD D,RLC (REGISTER+dd) */
                self.instrDDCB__LD_D_RLC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x03 => {
                /* LD E,RLC (REGISTER+dd) */
                self.instrDDCB__LD_E_RLC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x04 => {
                /* LD H,RLC (REGISTER+dd) */
                self.instrDDCB__LD_H_RLC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x05 => {
                /* LD L,RLC (REGISTER+dd) */
                self.instrDDCB__LD_L_RLC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x06 => {
                /* RLC (REGISTER+dd) */
                self.instrDDCB__RLC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x07 => {
                /* LD A,RLC (REGISTER+dd) */
                self.instrDDCB__LD_A_RLC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x08 => {
                /* LD B,RRC (REGISTER+dd) */
                self.instrDDCB__LD_B_RRC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x09 => {
                /* LD C,RRC (REGISTER+dd) */
                self.instrDDCB__LD_C_RRC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x0a => {
                /* LD D,RRC (REGISTER+dd) */
                self.instrDDCB__LD_D_RRC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x0b => {
                /* LD E,RRC (REGISTER+dd) */
                self.instrDDCB__LD_E_RRC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x0c => {
                /* LD H,RRC (REGISTER+dd) */
                self.instrDDCB__LD_H_RRC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x0d => {
                /* LD L,RRC (REGISTER+dd) */
                self.instrDDCB__LD_L_RRC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x0e => {
                /* RRC (REGISTER+dd) */
                self.instrDDCB__RRC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x0f => {
                /* LD A,RRC (REGISTER+dd) */
                self.instrDDCB__LD_A_RRC_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x10 => {
                /* LD B,RL (REGISTER+dd) */
                self.instrDDCB__LD_B_RL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x11 => {
                /* LD C,RL (REGISTER+dd) */
                self.instrDDCB__LD_C_RL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x12 => {
                /* LD D,RL (REGISTER+dd) */
                self.instrDDCB__LD_D_RL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x13 => {
                /* LD E,RL (REGISTER+dd) */
                self.instrDDCB__LD_E_RL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x14 => {
                /* LD H,RL (REGISTER+dd) */
                self.instrDDCB__LD_H_RL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x15 => {
                /* LD L,RL (REGISTER+dd) */
                self.instrDDCB__LD_L_RL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x16 => {
                /* RL (REGISTER+dd) */
                self.instrDDCB__RL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x17 => {
                /* LD A,RL (REGISTER+dd) */
                self.instrDDCB__LD_A_RL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x18 => {
                /* LD B,RR (REGISTER+dd) */
                self.instrDDCB__LD_B_RR_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x19 => {
                /* LD C,RR (REGISTER+dd) */
                self.instrDDCB__LD_C_RR_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x1a => {
                /* LD D,RR (REGISTER+dd) */
                self.instrDDCB__LD_D_RR_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x1b => {
                /* LD E,RR (REGISTER+dd) */
                self.instrDDCB__LD_E_RR_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x1c => {
                /* LD H,RR (REGISTER+dd) */
                self.instrDDCB__LD_H_RR_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x1d => {
                /* LD L,RR (REGISTER+dd) */
                self.instrDDCB__LD_L_RR_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x1e => {
                /* RR (REGISTER+dd) */
                self.instrDDCB__RR_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x1f => {
                /* LD A,RR (REGISTER+dd) */
                self.instrDDCB__LD_A_RR_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x20 => {
                /* LD B,SLA (REGISTER+dd) */
                self.instrDDCB__LD_B_SLA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x21 => {
                /* LD C,SLA (REGISTER+dd) */
                self.instrDDCB__LD_C_SLA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x22 => {
                /* LD D,SLA (REGISTER+dd) */
                self.instrDDCB__LD_D_SLA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x23 => {
                /* LD E,SLA (REGISTER+dd) */
                self.instrDDCB__LD_E_SLA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x24 => {
                /* LD H,SLA (REGISTER+dd) */
                self.instrDDCB__LD_H_SLA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x25 => {
                /* LD L,SLA (REGISTER+dd) */
                self.instrDDCB__LD_L_SLA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x26 => {
                /* SLA (REGISTER+dd) */
                self.instrDDCB__SLA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x27 => {
                /* LD A,SLA (REGISTER+dd) */
                self.instrDDCB__LD_A_SLA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x28 => {
                /* LD B,SRA (REGISTER+dd) */
                self.instrDDCB__LD_B_SRA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x29 => {
                /* LD C,SRA (REGISTER+dd) */
                self.instrDDCB__LD_C_SRA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x2a => {
                /* LD D,SRA (REGISTER+dd) */
                self.instrDDCB__LD_D_SRA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x2b => {
                /* LD E,SRA (REGISTER+dd) */
                self.instrDDCB__LD_E_SRA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x2c => {
                /* LD H,SRA (REGISTER+dd) */
                self.instrDDCB__LD_H_SRA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x2d => {
                /* LD L,SRA (REGISTER+dd) */
                self.instrDDCB__LD_L_SRA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x2e => {
                /* SRA (REGISTER+dd) */
                self.instrDDCB__SRA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x2f => {
                /* LD A,SRA (REGISTER+dd) */
                self.instrDDCB__LD_A_SRA_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x30 => {
                /* LD B,SLL (REGISTER+dd) */
                self.instrDDCB__LD_B_SLL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x31 => {
                /* LD C,SLL (REGISTER+dd) */
                self.instrDDCB__LD_C_SLL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x32 => {
                /* LD D,SLL (REGISTER+dd) */
                self.instrDDCB__LD_D_SLL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x33 => {
                /* LD E,SLL (REGISTER+dd) */
                self.instrDDCB__LD_E_SLL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x34 => {
                /* LD H,SLL (REGISTER+dd) */
                self.instrDDCB__LD_H_SLL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x35 => {
                /* LD L,SLL (REGISTER+dd) */
                self.instrDDCB__LD_L_SLL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x36 => {
                /* SLL (REGISTER+dd) */
                self.instrDDCB__SLL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x37 => {
                /* LD A,SLL (REGISTER+dd) */
                self.instrDDCB__LD_A_SLL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x38 => {
                /* LD B,SRL (REGISTER+dd) */
                self.instrDDCB__LD_B_SRL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x39 => {
                /* LD C,SRL (REGISTER+dd) */
                self.instrDDCB__LD_C_SRL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x3a => {
                /* LD D,SRL (REGISTER+dd) */
                self.instrDDCB__LD_D_SRL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x3b => {
                /* LD E,SRL (REGISTER+dd) */
                self.instrDDCB__LD_E_SRL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x3c => {
                /* LD H,SRL (REGISTER+dd) */
                self.instrDDCB__LD_H_SRL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x3d => {
                /* LD L,SRL (REGISTER+dd) */
                self.instrDDCB__LD_L_SRL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x3e => {
                /* SRL (REGISTER+dd) */
                self.instrDDCB__SRL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x3f => {
                /* LD A,SRL (REGISTER+dd) */
                self.instrDDCB__LD_A_SRL_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x47 => {
                /* BIT 0,(REGISTER+dd) */
                self.instrDDCB__BIT_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x40 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();return true;
                self.instrDDCB__BIT_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x41 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();return true;
                self.instrDDCB__BIT_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x42 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();return true;
                self.instrDDCB__BIT_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x43 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();return true;
                self.instrDDCB__BIT_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x44 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();return true;
                self.instrDDCB__BIT_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x45 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();return true;
                self.instrDDCB__BIT_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x46 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();return true;
                self.instrDDCB__BIT_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x4f => {
                /* BIT 1,(REGISTER+dd) */
                self.instrDDCB__BIT_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x48 => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();return true;
                self.instrDDCB__BIT_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x49 => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();return true;
                self.instrDDCB__BIT_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x4a => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();return true;
                self.instrDDCB__BIT_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x4b => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();return true;
                self.instrDDCB__BIT_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x4c => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();return true;
                self.instrDDCB__BIT_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x4d => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();return true;
                self.instrDDCB__BIT_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x4e => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();return true;
                self.instrDDCB__BIT_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x57 => {
                /* BIT 2,(REGISTER+dd) */
                self.instrDDCB__BIT_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x50 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();return true;
                self.instrDDCB__BIT_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x51 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();return true;
                self.instrDDCB__BIT_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x52 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();return true;
                self.instrDDCB__BIT_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x53 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();return true;
                self.instrDDCB__BIT_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x54 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();return true;
                self.instrDDCB__BIT_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x55 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();return true;
                self.instrDDCB__BIT_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x56 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();return true;
                self.instrDDCB__BIT_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x5f => {
                /* BIT 3,(REGISTER+dd) */
                self.instrDDCB__BIT_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x58 => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();return true;
                self.instrDDCB__BIT_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x59 => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();return true;
                self.instrDDCB__BIT_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x5a => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();return true;
                self.instrDDCB__BIT_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x5b => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();return true;
                self.instrDDCB__BIT_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x5c => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();return true;
                self.instrDDCB__BIT_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x5d => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();return true;
                self.instrDDCB__BIT_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x5e => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();return true;
                self.instrDDCB__BIT_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x67 => {
                /* BIT 4,(REGISTER+dd) */
                self.instrDDCB__BIT_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x60 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();return true;
                self.instrDDCB__BIT_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x61 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();return true;
                self.instrDDCB__BIT_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x62 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();return true;
                self.instrDDCB__BIT_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x63 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();return true;
                self.instrDDCB__BIT_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x64 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();return true;
                self.instrDDCB__BIT_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x65 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();return true;
                self.instrDDCB__BIT_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x66 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();return true;
                self.instrDDCB__BIT_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x6f => {
                /* BIT 5,(REGISTER+dd) */
                self.instrDDCB__BIT_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x68 => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();return true;
                self.instrDDCB__BIT_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x69 => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();return true;
                self.instrDDCB__BIT_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x6a => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();return true;
                self.instrDDCB__BIT_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x6b => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();return true;
                self.instrDDCB__BIT_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x6c => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();return true;
                self.instrDDCB__BIT_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x6d => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();return true;
                self.instrDDCB__BIT_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x6e => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();return true;
                self.instrDDCB__BIT_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x77 => {
                /* BIT 6,(REGISTER+dd) */
                self.instrDDCB__BIT_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x70 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();return true;
                self.instrDDCB__BIT_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x71 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();return true;
                self.instrDDCB__BIT_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x72 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();return true;
                self.instrDDCB__BIT_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x73 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();return true;
                self.instrDDCB__BIT_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x74 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();return true;
                self.instrDDCB__BIT_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x75 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();return true;
                self.instrDDCB__BIT_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x76 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();return true;
                self.instrDDCB__BIT_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x7f => {
                /* BIT 7,(REGISTER+dd) */
                self.instrDDCB__BIT_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x78 => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();return true;
                self.instrDDCB__BIT_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x79 => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();return true;
                self.instrDDCB__BIT_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x7a => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();return true;
                self.instrDDCB__BIT_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x7b => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();return true;
                self.instrDDCB__BIT_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x7c => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();return true;
                self.instrDDCB__BIT_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x7d => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();return true;
                self.instrDDCB__BIT_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x7e => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();return true;
                self.instrDDCB__BIT_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x80 => {
                /* LD B,RES 0,(REGISTER+dd) */
                self.instrDDCB__LD_B_RES_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x81 => {
                /* LD C,RES 0,(REGISTER+dd) */
                self.instrDDCB__LD_C_RES_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x82 => {
                /* LD D,RES 0,(REGISTER+dd) */
                self.instrDDCB__LD_D_RES_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x83 => {
                /* LD E,RES 0,(REGISTER+dd) */
                self.instrDDCB__LD_E_RES_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x84 => {
                /* LD H,RES 0,(REGISTER+dd) */
                self.instrDDCB__LD_H_RES_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x85 => {
                /* LD L,RES 0,(REGISTER+dd) */
                self.instrDDCB__LD_L_RES_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x86 => {
                /* RES 0,(REGISTER+dd) */
                self.instrDDCB__RES_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x87 => {
                /* LD A,RES 0,(REGISTER+dd) */
                self.instrDDCB__LD_A_RES_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x88 => {
                /* LD B,RES 1,(REGISTER+dd) */
                self.instrDDCB__LD_B_RES_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x89 => {
                /* LD C,RES 1,(REGISTER+dd) */
                self.instrDDCB__LD_C_RES_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x8a => {
                /* LD D,RES 1,(REGISTER+dd) */
                self.instrDDCB__LD_D_RES_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x8b => {
                /* LD E,RES 1,(REGISTER+dd) */
                self.instrDDCB__LD_E_RES_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x8c => {
                /* LD H,RES 1,(REGISTER+dd) */
                self.instrDDCB__LD_H_RES_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x8d => {
                /* LD L,RES 1,(REGISTER+dd) */
                self.instrDDCB__LD_L_RES_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x8e => {
                /* RES 1,(REGISTER+dd) */
                self.instrDDCB__RES_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x8f => {
                /* LD A,RES 1,(REGISTER+dd) */
                self.instrDDCB__LD_A_RES_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x90 => {
                /* LD B,RES 2,(REGISTER+dd) */
                self.instrDDCB__LD_B_RES_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x91 => {
                /* LD C,RES 2,(REGISTER+dd) */
                self.instrDDCB__LD_C_RES_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x92 => {
                /* LD D,RES 2,(REGISTER+dd) */
                self.instrDDCB__LD_D_RES_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x93 => {
                /* LD E,RES 2,(REGISTER+dd) */
                self.instrDDCB__LD_E_RES_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x94 => {
                /* LD H,RES 2,(REGISTER+dd) */
                self.instrDDCB__LD_H_RES_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x95 => {
                /* LD L,RES 2,(REGISTER+dd) */
                self.instrDDCB__LD_L_RES_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x96 => {
                /* RES 2,(REGISTER+dd) */
                self.instrDDCB__RES_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x97 => {
                /* LD A,RES 2,(REGISTER+dd) */
                self.instrDDCB__LD_A_RES_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x98 => {
                /* LD B,RES 3,(REGISTER+dd) */
                self.instrDDCB__LD_B_RES_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x99 => {
                /* LD C,RES 3,(REGISTER+dd) */
                self.instrDDCB__LD_C_RES_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x9a => {
                /* LD D,RES 3,(REGISTER+dd) */
                self.instrDDCB__LD_D_RES_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x9b => {
                /* LD E,RES 3,(REGISTER+dd) */
                self.instrDDCB__LD_E_RES_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x9c => {
                /* LD H,RES 3,(REGISTER+dd) */
                self.instrDDCB__LD_H_RES_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x9d => {
                /* LD L,RES 3,(REGISTER+dd) */
                self.instrDDCB__LD_L_RES_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x9e => {
                /* RES 3,(REGISTER+dd) */
                self.instrDDCB__RES_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0x9f => {
                /* LD A,RES 3,(REGISTER+dd) */
                self.instrDDCB__LD_A_RES_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa0 => {
                /* LD B,RES 4,(REGISTER+dd) */
                self.instrDDCB__LD_B_RES_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa1 => {
                /* LD C,RES 4,(REGISTER+dd) */
                self.instrDDCB__LD_C_RES_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa2 => {
                /* LD D,RES 4,(REGISTER+dd) */
                self.instrDDCB__LD_D_RES_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa3 => {
                /* LD E,RES 4,(REGISTER+dd) */
                self.instrDDCB__LD_E_RES_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa4 => {
                /* LD H,RES 4,(REGISTER+dd) */
                self.instrDDCB__LD_H_RES_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa5 => {
                /* LD L,RES 4,(REGISTER+dd) */
                self.instrDDCB__LD_L_RES_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa6 => {
                /* RES 4,(REGISTER+dd) */
                self.instrDDCB__RES_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa7 => {
                /* LD A,RES 4,(REGISTER+dd) */
                self.instrDDCB__LD_A_RES_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa8 => {
                /* LD B,RES 5,(REGISTER+dd) */
                self.instrDDCB__LD_B_RES_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xa9 => {
                /* LD C,RES 5,(REGISTER+dd) */
                self.instrDDCB__LD_C_RES_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xaa => {
                /* LD D,RES 5,(REGISTER+dd) */
                self.instrDDCB__LD_D_RES_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xab => {
                /* LD E,RES 5,(REGISTER+dd) */
                self.instrDDCB__LD_E_RES_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xac => {
                /* LD H,RES 5,(REGISTER+dd) */
                self.instrDDCB__LD_H_RES_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xad => {
                /* LD L,RES 5,(REGISTER+dd) */
                self.instrDDCB__LD_L_RES_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xae => {
                /* RES 5,(REGISTER+dd) */
                self.instrDDCB__RES_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xaf => {
                /* LD A,RES 5,(REGISTER+dd) */
                self.instrDDCB__LD_A_RES_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb0 => {
                /* LD B,RES 6,(REGISTER+dd) */
                self.instrDDCB__LD_B_RES_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb1 => {
                /* LD C,RES 6,(REGISTER+dd) */
                self.instrDDCB__LD_C_RES_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb2 => {
                /* LD D,RES 6,(REGISTER+dd) */
                self.instrDDCB__LD_D_RES_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb3 => {
                /* LD E,RES 6,(REGISTER+dd) */
                self.instrDDCB__LD_E_RES_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb4 => {
                /* LD H,RES 6,(REGISTER+dd) */
                self.instrDDCB__LD_H_RES_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb5 => {
                /* LD L,RES 6,(REGISTER+dd) */
                self.instrDDCB__LD_L_RES_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb6 => {
                /* RES 6,(REGISTER+dd) */
                self.instrDDCB__RES_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb7 => {
                /* LD A,RES 6,(REGISTER+dd) */
                self.instrDDCB__LD_A_RES_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb8 => {
                /* LD B,RES 7,(REGISTER+dd) */
                self.instrDDCB__LD_B_RES_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xb9 => {
                /* LD C,RES 7,(REGISTER+dd) */
                self.instrDDCB__LD_C_RES_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xba => {
                /* LD D,RES 7,(REGISTER+dd) */
                self.instrDDCB__LD_D_RES_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xbb => {
                /* LD E,RES 7,(REGISTER+dd) */
                self.instrDDCB__LD_E_RES_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xbc => {
                /* LD H,RES 7,(REGISTER+dd) */
                self.instrDDCB__LD_H_RES_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xbd => {
                /* LD L,RES 7,(REGISTER+dd) */
                self.instrDDCB__LD_L_RES_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xbe => {
                /* RES 7,(REGISTER+dd) */
                self.instrDDCB__RES_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xbf => {
                /* LD A,RES 7,(REGISTER+dd) */
                self.instrDDCB__LD_A_RES_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc0 => {
                /* LD B,SET 0,(REGISTER+dd) */
                self.instrDDCB__LD_B_SET_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc1 => {
                /* LD C,SET 0,(REGISTER+dd) */
                self.instrDDCB__LD_C_SET_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc2 => {
                /* LD D,SET 0,(REGISTER+dd) */
                self.instrDDCB__LD_D_SET_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc3 => {
                /* LD E,SET 0,(REGISTER+dd) */
                self.instrDDCB__LD_E_SET_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc4 => {
                /* LD H,SET 0,(REGISTER+dd) */
                self.instrDDCB__LD_H_SET_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc5 => {
                /* LD L,SET 0,(REGISTER+dd) */
                self.instrDDCB__LD_L_SET_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc6 => {
                /* SET 0,(REGISTER+dd) */
                self.instrDDCB__SET_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc7 => {
                /* LD A,SET 0,(REGISTER+dd) */
                self.instrDDCB__LD_A_SET_0_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc8 => {
                /* LD B,SET 1,(REGISTER+dd) */
                self.instrDDCB__LD_B_SET_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xc9 => {
                /* LD C,SET 1,(REGISTER+dd) */
                self.instrDDCB__LD_C_SET_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xca => {
                /* LD D,SET 1,(REGISTER+dd) */
                self.instrDDCB__LD_D_SET_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xcb => {
                /* LD E,SET 1,(REGISTER+dd) */
                self.instrDDCB__LD_E_SET_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xcc => {
                /* LD H,SET 1,(REGISTER+dd) */
                self.instrDDCB__LD_H_SET_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xcd => {
                /* LD L,SET 1,(REGISTER+dd) */
                self.instrDDCB__LD_L_SET_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xce => {
                /* SET 1,(REGISTER+dd) */
                self.instrDDCB__SET_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xcf => {
                /* LD A,SET 1,(REGISTER+dd) */
                self.instrDDCB__LD_A_SET_1_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd0 => {
                /* LD B,SET 2,(REGISTER+dd) */
                self.instrDDCB__LD_B_SET_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd1 => {
                /* LD C,SET 2,(REGISTER+dd) */
                self.instrDDCB__LD_C_SET_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd2 => {
                /* LD D,SET 2,(REGISTER+dd) */
                self.instrDDCB__LD_D_SET_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd3 => {
                /* LD E,SET 2,(REGISTER+dd) */
                self.instrDDCB__LD_E_SET_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd4 => {
                /* LD H,SET 2,(REGISTER+dd) */
                self.instrDDCB__LD_H_SET_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd5 => {
                /* LD L,SET 2,(REGISTER+dd) */
                self.instrDDCB__LD_L_SET_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd6 => {
                /* SET 2,(REGISTER+dd) */
                self.instrDDCB__SET_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd7 => {
                /* LD A,SET 2,(REGISTER+dd) */
                self.instrDDCB__LD_A_SET_2_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd8 => {
                /* LD B,SET 3,(REGISTER+dd) */
                self.instrDDCB__LD_B_SET_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xd9 => {
                /* LD C,SET 3,(REGISTER+dd) */
                self.instrDDCB__LD_C_SET_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xda => {
                /* LD D,SET 3,(REGISTER+dd) */
                self.instrDDCB__LD_D_SET_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xdb => {
                /* LD E,SET 3,(REGISTER+dd) */
                self.instrDDCB__LD_E_SET_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xdc => {
                /* LD H,SET 3,(REGISTER+dd) */
                self.instrDDCB__LD_H_SET_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xdd => {
                /* LD L,SET 3,(REGISTER+dd) */
                self.instrDDCB__LD_L_SET_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xde => {
                /* SET 3,(REGISTER+dd) */
                self.instrDDCB__SET_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xdf => {
                /* LD A,SET 3,(REGISTER+dd) */
                self.instrDDCB__LD_A_SET_3_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe0 => {
                /* LD B,SET 4,(REGISTER+dd) */
                self.instrDDCB__LD_B_SET_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe1 => {
                /* LD C,SET 4,(REGISTER+dd) */
                self.instrDDCB__LD_C_SET_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe2 => {
                /* LD D,SET 4,(REGISTER+dd) */
                self.instrDDCB__LD_D_SET_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe3 => {
                /* LD E,SET 4,(REGISTER+dd) */
                self.instrDDCB__LD_E_SET_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe4 => {
                /* LD H,SET 4,(REGISTER+dd) */
                self.instrDDCB__LD_H_SET_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe5 => {
                /* LD L,SET 4,(REGISTER+dd) */
                self.instrDDCB__LD_L_SET_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe6 => {
                /* SET 4,(REGISTER+dd) */
                self.instrDDCB__SET_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe7 => {
                /* LD A,SET 4,(REGISTER+dd) */
                self.instrDDCB__LD_A_SET_4_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe8 => {
                /* LD B,SET 5,(REGISTER+dd) */
                self.instrDDCB__LD_B_SET_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xe9 => {
                /* LD C,SET 5,(REGISTER+dd) */
                self.instrDDCB__LD_C_SET_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xea => {
                /* LD D,SET 5,(REGISTER+dd) */
                self.instrDDCB__LD_D_SET_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xeb => {
                /* LD E,SET 5,(REGISTER+dd) */
                self.instrDDCB__LD_E_SET_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xec => {
                /* LD H,SET 5,(REGISTER+dd) */
                self.instrDDCB__LD_H_SET_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xed => {
                /* LD L,SET 5,(REGISTER+dd) */
                self.instrDDCB__LD_L_SET_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xee => {
                /* SET 5,(REGISTER+dd) */
                self.instrDDCB__SET_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xef => {
                /* LD A,SET 5,(REGISTER+dd) */
                self.instrDDCB__LD_A_SET_5_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf0 => {
                /* LD B,SET 6,(REGISTER+dd) */
                self.instrDDCB__LD_B_SET_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf1 => {
                /* LD C,SET 6,(REGISTER+dd) */
                self.instrDDCB__LD_C_SET_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf2 => {
                /* LD D,SET 6,(REGISTER+dd) */
                self.instrDDCB__LD_D_SET_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf3 => {
                /* LD E,SET 6,(REGISTER+dd) */
                self.instrDDCB__LD_E_SET_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf4 => {
                /* LD H,SET 6,(REGISTER+dd) */
                self.instrDDCB__LD_H_SET_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf5 => {
                /* LD L,SET 6,(REGISTER+dd) */
                self.instrDDCB__LD_L_SET_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf6 => {
                /* SET 6,(REGISTER+dd) */
                self.instrDDCB__SET_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf7 => {
                /* LD A,SET 6,(REGISTER+dd) */
                self.instrDDCB__LD_A_SET_6_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf8 => {
                /* LD B,SET 7,(REGISTER+dd) */
                self.instrDDCB__LD_B_SET_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xf9 => {
                /* LD C,SET 7,(REGISTER+dd) */
                self.instrDDCB__LD_C_SET_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xfa => {
                /* LD D,SET 7,(REGISTER+dd) */
                self.instrDDCB__LD_D_SET_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xfb => {
                /* LD E,SET 7,(REGISTER+dd) */
                self.instrDDCB__LD_E_SET_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xfc => {
                /* LD H,SET 7,(REGISTER+dd) */
                self.instrDDCB__LD_H_SET_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xfd => {
                /* LD L,SET 7,(REGISTER+dd) */
                self.instrDDCB__LD_L_SET_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xfe => {
                /* SET 7,(REGISTER+dd) */
                self.instrDDCB__SET_7_iREGpDD();
                true
            }

            val if val == SHIFT_0X_DDCB + 0xff => {
                /* LD A,SET 7,(REGISTER+dd) */
                self.instrDDCB__LD_A_SET_7_iREGpDD();
                true
            }
            _ => false,
        }
    }
    /* NOP */
    pub(crate) fn instr__NOP(&mut self) {}
    /* LD BC,nnnn */
    fn_instr_ld_r16_nnnn!(instr__LD_BC_NNNN, BC, SetBC);

    /* LD (BC),A */
    pub(crate) fn instr__LD_iBC_A(&mut self) {
        let address = self.BC();
        self.memory.write_byte(address, self.data.A);
    }

    /* INC BC */
    fn_instr_op_16!(instr__INC_BC, IncBC);

    /* INC B */
    fn_instr_op_8!(instr__INC_B, incB);

    /* DEC B */
    fn_instr_op_8!(instr__DEC_B, decB);

    /* LD B,nn */
    fn_instr_ld_r8_nn!(instr__LD_B_NN, B);

    /* RLCA */
    pub(crate) fn instr__RLCA(&mut self) {
        self.data.A = self.data.A.rotate_left(1);
        self.data.F =
            (self.data.F & (FLAG_P | FLAG_Z | FLAG_S)) | (self.data.A & (FLAG_C | FLAG_3 | FLAG_5));
    }

    /* EX AF,AF' */
    pub(crate) fn instr__EX_AF_AF(&mut self) {
        let old_a: u8 = self.data.A;
        let old_f: u8 = self.data.F;
        self.data.A = self.data.A_;
        self.data.F = self.data.F_;
        self.data.A_ = old_a;
        self.data.F_ = old_f;
    }

    /* ADD HL,BC */
    fn_instr_add_hl_r16!(instr__ADD_HL_BC, BC);

    /* LD A,(BC) */
    fn_instr_ld_a_r16!(instr__LD_A_iBC, BC);

    /* DEC BC */
    fn_instr_op_16!(instr__DEC_BC, DecBC);

    /* INC C */
    fn_instr_op_8!(instr__INC_C, incC);

    /* DEC C */
    fn_instr_op_8!(instr__DEC_C, decC);

    /* LD C,nn */
    fn_instr_ld_r8_nn!(instr__LD_C_NN, C);

    /* RRCA */
    pub(crate) fn instr__RRCA(&mut self) {
        self.data.F = (self.data.F & (FLAG_P | FLAG_Z | FLAG_S)) | (self.data.A & FLAG_C);
        self.data.A = self.data.A.rotate_right(1);
        self.data.F |= self.data.A & (FLAG_3 | FLAG_5);
    }

    /* DJNZ offset */
    pub(crate) fn instr__DJNZ_OFFSET(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.data.B = self.data.B.wrapping_sub(1);
        if self.data.B != 0 {
            self.jr();
            self.data.cycles += 14;
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            self.data.cycles += 9;
        }
        self.IncPC(1);
    }

    /* LD DE,nnnn */
    fn_instr_ld_r16_nnnn!(instr__LD_DE_NNNN, DE, SetDE);

    /* LD (DE),A */
    fn_instr_ld_i_r16_r8!(instr__LD_iDE_A, DE, A);

    /* INC DE */
    fn_instr_op_16!(instr__INC_DE, IncDE);

    /* INC D */
    fn_instr_op_8!(instr__INC_D, incD);

    /* DEC D */
    fn_instr_op_8!(instr__DEC_D, decD);

    /* LD D,nn */
    fn_instr_ld_r8_nn!(instr__LD_D_NN, D);

    /* RLA */
    pub(crate) fn instr__RLA(&mut self) {
        let byte_temp: u8 = self.data.A;
        self.data.A = (self.data.A << 1) | self.data.F & FLAG_C;
        self.data.F = (self.data.F & (FLAG_P | FLAG_Z | FLAG_S))
            | (self.data.A & (FLAG_3 | FLAG_5))
            | (byte_temp >> 7);
    }

    /* JR offset */
    pub(crate) fn instr__JR_OFFSET(&mut self) {
        self.jr();
        self.IncPC(1);
    }

    /* ADD HL,DE */
    fn_instr_add_hl_r16!(instr__ADD_HL_DE, DE);

    /* LD A,(DE) */
    fn_instr_ld_a_r16!(instr__LD_A_iDE, DE);

    /* DEC DE */
    fn_instr_op_16!(instr__DEC_DE, DecDE);

    /* INC E */
    fn_instr_op_8!(instr__INC_E, incE);

    /* DEC E */
    fn_instr_op_8!(instr__DEC_E, decE);

    /* LD E,nn */
    fn_instr_ld_r8_nn!(instr__LD_E_NN, E);

    /* RRA */
    pub(crate) fn instr__RRA(&mut self) {
        let byte_temp: u8 = self.data.A;
        self.data.A = (self.data.A >> 1) | (self.data.F << 7);
        self.data.F = self.data.F & (FLAG_P | FLAG_Z | FLAG_S)
            | self.data.A & (FLAG_3 | FLAG_5)
            | byte_temp & FLAG_C;
    }

    /* JR NZ,offset */
    pub(crate) fn instr__JR_NZ_OFFSET(&mut self) {
        if (self.data.F & FLAG_Z) == 0 {
            self.jr();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
        }
        self.IncPC(1);
    }

    /* LD HL,nnnn */
    fn_instr_ld_r16_nnnn!(instr__LD_HL_NNNN, HL, SetHL);

    /* LD (nnnn),HL */
    fn_instr_ld_i_nnnn_r16!(instr__LD_iNNNN_HL, L, H);

    /* INC HL */
    fn_instr_op_16!(instr__INC_HL, IncHL);

    /* INC H */
    fn_instr_op_8!(instr__INC_H, incH);

    /* DEC H */
    fn_instr_op_8!(instr__DEC_H, decH);

    /* LD H,nn */
    fn_instr_ld_r8_nn!(instr__LD_H_NN, H);

    /* DAA */
    pub(crate) fn instr__DAA(&mut self) {
        let mut add: u8 = 0;
        let mut carry: u8 = self.data.F & FLAG_C;
        if ((self.data.F & FLAG_H) != 0) || ((self.data.A & 0x0f) > 9) {
            add = 6;
        }
        if (carry != 0) || (self.data.A > 0x99) {
            add |= 0x60;
        }
        if self.data.A > 0x99 {
            carry = FLAG_C;
        }
        if (self.data.F & FLAG_N) != 0 {
            self.sub(add);
        } else {
            self.add(add);
        }
        let temp: u8 = self.data.F & !(FLAG_C | FLAG_P)
            | carry
            | self.tables.parity_table[self.data.A as usize];
        self.data.F = temp;
    }

    /* JR Z,offset */
    pub(crate) fn instr__JR_Z_OFFSET(&mut self) {
        if (self.data.F & FLAG_Z) != 0 {
            self.jr();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
        }
        self.IncPC(1);
    }

    /* ADD HL,HL */
    fn_instr_add_hl_r16!(instr__ADD_HL_HL, HL);

    /* LD HL,(nnnn) */
    fn_instr_ld_hl_i_nnnn!(instr__LD_HL_iNNNN, L, H);

    /* DEC HL */
    fn_instr_op_16!(instr__DEC_HL, DecHL);

    /* INC L */
    fn_instr_op_8!(instr__INC_L, incL);

    /* DEC L */
    fn_instr_op_8!(instr__DEC_L, decL);

    /* LD L,nn */
    fn_instr_ld_r8_nn!(instr__LD_L_NN, L);

    /* CPL */
    pub(crate) fn instr__CPL(&mut self) {
        self.data.A ^= 0xff;
        self.data.F = self.data.F & (FLAG_C | FLAG_P | FLAG_Z | FLAG_S)
            | self.data.A & (FLAG_3 | FLAG_5)
            | (FLAG_N | FLAG_H);
    }

    /* JR NC,offset */
    pub(crate) fn instr__JR_NC_OFFSET(&mut self) {
        if self.data.F & FLAG_C == 0 {
            self.jr();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
        }
        self.IncPC(1);
    }

    /* LD SP,nnnn */
    fn_instr_ld_r16_nnnn!(instr__LD_SP_NNNN, SP, SetSP);

    /* LD (nnnn),A */
    pub(crate) fn instr__LD_iNNNN_A(&mut self) {
        let address = self.PC();
        let mut word_temp: u16 = (self.memory.read_byte(address)) as u16;
        self.IncPC(1);
        let address = self.PC();
        word_temp |= (self.memory.read_byte(address) as u16) << 8;
        self.IncPC(1);
        self.memory.write_byte(word_temp, self.data.A);
    }

    /* INC SP */
    fn_instr_op_16!(instr__INC_SP, IncSP);

    /* INC (HL) */
    pub(crate) fn instr__INC_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.inc(&mut byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* DEC (HL) */
    pub(crate) fn instr__DEC_iHL(&mut self) {
        let address = self.HL();
        let mut byte_temp: u8 = self.memory.read_byte(address);
        let _address = self.HL();
        self.memory.contend_read_no_mreq(_address, 1);
        self.dec(&mut byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* LD (HL),nn */
    pub(crate) fn instr__LD_iHL_NN(&mut self) {
        let address = self.PC();
        let value = self.memory.read_byte(address);
        let address = self.HL();
        self.memory.write_byte(address, value);
        self.IncPC(1);
    }

    /* SCF */
    pub(crate) fn instr__SCF(&mut self) {
        self.data.F =
            (self.data.F & (FLAG_P | FLAG_Z | FLAG_S)) | (self.data.A & (FLAG_3 | FLAG_5)) | FLAG_C;
    }

    /* JR C,offset */
    pub(crate) fn instr__JR_C_OFFSET(&mut self) {
        if self.data.F & FLAG_C != 0 {
            self.jr();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
        }
        self.IncPC(1);
    }

    /* ADD HL,SP */
    fn_instr_add_hl_r16!(instr__ADD_HL_SP, SP);

    /* LD A,(nnnn) */
    pub(crate) fn instr__LD_A_iNNNN(&mut self) {
        let address = self.PC();
        let mut word_temp: u16 = self.memory.read_byte(address) as u16;
        self.IncPC(1);
        let address = self.PC();
        word_temp |= (self.memory.read_byte(address) as u16) << 8;
        self.IncPC(1);
        self.data.A = self.memory.read_byte(word_temp);
    }

    /* DEC SP */
    fn_instr_op_16!(instr__DEC_SP, DecSP);

    /* INC A */
    fn_instr_op_8!(instr__INC_A, incA);

    /* DEC A */
    fn_instr_op_8!(instr__DEC_A, decA);

    /* LD A,nn */
    fn_instr_ld_r8_nn!(instr__LD_A_NN, A);

    /* CCF */
    pub(crate) fn instr__CCF(&mut self) {
        self.data.F = self.data.F & (FLAG_P | FLAG_Z | FLAG_S)
            | tern_op_b(self.data.F & FLAG_C != 0, FLAG_H, FLAG_C)
            | self.data.A & (FLAG_3 | FLAG_5);
    }

    /* LD B,B */
    pub(crate) fn instr__LD_B_B(&mut self) {}

    /* LD B,C */
    pub(crate) fn instr__LD_B_C(&mut self) {
        self.data.B = self.data.C;
    }

    /* LD B,D */
    pub(crate) fn instr__LD_B_D(&mut self) {
        self.data.B = self.data.D;
    }

    /* LD B,E */
    pub(crate) fn instr__LD_B_E(&mut self) {
        self.data.B = self.data.E;
    }

    /* LD B,H */
    pub(crate) fn instr__LD_B_H(&mut self) {
        self.data.B = self.data.H;
    }

    /* LD B,L */
    pub(crate) fn instr__LD_B_L(&mut self) {
        self.data.B = self.data.L;
    }

    /* LD B,(HL) */
    pub(crate) fn instr__LD_B_iHL(&mut self) {
        self.data.B = self.memory.read_byte(self.HL());
    }

    /* LD B,A */
    pub(crate) fn instr__LD_B_A(&mut self) {
        self.data.B = self.data.A;
    }

    /* LD C,B */
    pub(crate) fn instr__LD_C_B(&mut self) {
        self.data.C = self.data.B;
    }

    /* LD C,C */
    pub(crate) fn instr__LD_C_C(&mut self) {}

    /* LD C,D */
    pub(crate) fn instr__LD_C_D(&mut self) {
        self.data.C = self.data.D;
    }

    /* LD C,E */
    pub(crate) fn instr__LD_C_E(&mut self) {
        self.data.C = self.data.E;
    }

    /* LD C,H */
    pub(crate) fn instr__LD_C_H(&mut self) {
        self.data.C = self.data.H;
    }

    /* LD C,L */
    pub(crate) fn instr__LD_C_L(&mut self) {
        self.data.C = self.data.L;
    }

    /* LD C,(HL) */
    pub(crate) fn instr__LD_C_iHL(&mut self) {
        self.data.C = self.memory.read_byte(self.HL());
    }

    /* LD C,A */
    pub(crate) fn instr__LD_C_A(&mut self) {
        self.data.C = self.data.A;
    }

    /* LD D,B */
    pub(crate) fn instr__LD_D_B(&mut self) {
        self.data.D = self.data.B;
    }

    /* LD D,C */
    pub(crate) fn instr__LD_D_C(&mut self) {
        self.data.D = self.data.C;
    }

    /* LD D,D */
    pub(crate) fn instr__LD_D_D(&mut self) {}

    /* LD D,E */
    pub(crate) fn instr__LD_D_E(&mut self) {
        self.data.D = self.data.E;
    }

    /* LD D,H */
    pub(crate) fn instr__LD_D_H(&mut self) {
        self.data.D = self.data.H;
    }

    /* LD D,L */
    pub(crate) fn instr__LD_D_L(&mut self) {
        self.data.D = self.data.L;
    }

    /* LD D,(HL) */
    pub(crate) fn instr__LD_D_iHL(&mut self) {
        self.data.D = self.memory.read_byte(self.HL());
    }

    /* LD D,A */
    pub(crate) fn instr__LD_D_A(&mut self) {
        self.data.D = self.data.A;
    }

    /* LD E,B */
    pub(crate) fn instr__LD_E_B(&mut self) {
        self.data.E = self.data.B;
    }

    /* LD E,C */
    pub(crate) fn instr__LD_E_C(&mut self) {
        self.data.E = self.data.C;
    }

    /* LD E,D */
    pub(crate) fn instr__LD_E_D(&mut self) {
        self.data.E = self.data.D;
    }

    /* LD E,E */
    pub(crate) fn instr__LD_E_E(&mut self) {}

    /* LD E,H */
    pub(crate) fn instr__LD_E_H(&mut self) {
        self.data.E = self.data.H;
    }

    /* LD E,L */
    pub(crate) fn instr__LD_E_L(&mut self) {
        self.data.E = self.data.L;
    }

    /* LD E,(HL) */
    pub(crate) fn instr__LD_E_iHL(&mut self) {
        self.data.E = self.memory.read_byte(self.HL());
    }

    /* LD E,A */
    pub(crate) fn instr__LD_E_A(&mut self) {
        self.data.E = self.data.A;
    }

    /* LD H,B */
    pub(crate) fn instr__LD_H_B(&mut self) {
        self.data.H = self.data.B;
    }

    /* LD H,C */
    pub(crate) fn instr__LD_H_C(&mut self) {
        self.data.H = self.data.C;
    }

    /* LD H,D */
    pub(crate) fn instr__LD_H_D(&mut self) {
        self.data.H = self.data.D;
    }

    /* LD H,E */
    pub(crate) fn instr__LD_H_E(&mut self) {
        self.data.H = self.data.E;
    }

    /* LD H,H */
    pub(crate) fn instr__LD_H_H(&mut self) {}

    /* LD H,L */
    pub(crate) fn instr__LD_H_L(&mut self) {
        self.data.H = self.data.L;
    }

    /* LD H,(HL) */
    pub(crate) fn instr__LD_H_iHL(&mut self) {
        self.data.H = self.memory.read_byte(self.HL());
    }

    /* LD H,A */
    pub(crate) fn instr__LD_H_A(&mut self) {
        self.data.H = self.data.A;
    }

    /* LD L,B */
    pub(crate) fn instr__LD_L_B(&mut self) {
        self.data.L = self.data.B;
    }

    /* LD L,C */
    pub(crate) fn instr__LD_L_C(&mut self) {
        self.data.L = self.data.C;
    }

    /* LD L,D */
    pub(crate) fn instr__LD_L_D(&mut self) {
        self.data.L = self.data.D;
    }

    /* LD L,E */
    pub(crate) fn instr__LD_L_E(&mut self) {
        self.data.L = self.data.E;
    }

    /* LD L,H */
    pub(crate) fn instr__LD_L_H(&mut self) {
        self.data.L = self.data.H;
    }

    /* LD L,L */
    pub(crate) fn instr__LD_L_L(&mut self) {}

    /* LD L,(HL) */
    pub(crate) fn instr__LD_L_iHL(&mut self) {
        self.data.L = self.memory.read_byte(self.HL());
    }

    /* LD L,A */
    pub(crate) fn instr__LD_L_A(&mut self) {
        self.data.L = self.data.A;
    }

    /* LD (HL),B */
    fn_instr_ld_i_r16_r8!(instr__LD_iHL_B, HL, B);

    /* LD (HL),C */
    fn_instr_ld_i_r16_r8!(instr__LD_iHL_C, HL, C);

    /* LD (HL),D */
    fn_instr_ld_i_r16_r8!(instr__LD_iHL_D, HL, D);

    /* LD (HL),E */
    fn_instr_ld_i_r16_r8!(instr__LD_iHL_E, HL, E);

    /* LD (HL),H */
    fn_instr_ld_i_r16_r8!(instr__LD_iHL_H, HL, H);

    /* LD (HL),L */
    fn_instr_ld_i_r16_r8!(instr__LD_iHL_L, HL, L);

    /* HALT */
    pub(crate) fn instr__HALT(&mut self) {
        self.data.halted = true;
        self.DecPC(1);
    }

    /* LD (HL),A */
    fn_instr_ld_i_r16_r8!(instr__LD_iHL_A, HL, A);

    /* LD A,B */
    fn_instr_ld_a_r8!(instr__LD_A_B, B);

    /* LD A,C */
    fn_instr_ld_a_r8!(instr__LD_A_C, C);

    /* LD A,D */
    fn_instr_ld_a_r8!(instr__LD_A_D, D);

    /* LD A,E */
    fn_instr_ld_a_r8!(instr__LD_A_E, E);

    /* LD A,H */
    fn_instr_ld_a_r8!(instr__LD_A_H, H);

    /* LD A,L */
    fn_instr_ld_a_r8!(instr__LD_A_L, L);

    /* LD A,(HL) */
    fn_instr_ld_a_r16!(instr__LD_A_iHL, HL);

    /* LD A,A */
    pub(crate) fn instr__LD_A_A(&mut self) {}

    /* ADD A,B */
    fn_instr_add_r8!(instr__ADD_A_B, B);

    /* ADD A,C */
    fn_instr_add_r8!(instr__ADD_A_C, C);

    /* ADD A,D */
    fn_instr_add_r8!(instr__ADD_A_D, D);

    /* ADD A,E */
    fn_instr_add_r8!(instr__ADD_A_E, E);

    /* ADD A,H */
    fn_instr_add_r8!(instr__ADD_A_H, H);

    /* ADD A,L */
    fn_instr_add_r8!(instr__ADD_A_L, L);

    /* ADD A,(HL) */
    pub(crate) fn instr__ADD_A_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());

        self.add(byte_temp);
    }

    /* ADD A,A */
    fn_instr_add_r8!(instr__ADD_A_A, A);

    /* ADC A,B */
    fn_instr_adc_r8!(instr__ADC_A_B, B);

    /* ADC A,C */
    fn_instr_adc_r8!(instr__ADC_A_C, C);

    /* ADC A,D */
    fn_instr_adc_r8!(instr__ADC_A_D, D);

    /* ADC A,E */
    fn_instr_adc_r8!(instr__ADC_A_E, E);

    /* ADC A,H */
    fn_instr_adc_r8!(instr__ADC_A_H, H);

    /* ADC A,L */
    fn_instr_adc_r8!(instr__ADC_A_L, L);

    /* ADC A,(HL) */
    pub(crate) fn instr__ADC_A_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());

        self.adc(byte_temp);
    }

    /* ADC A,A */
    fn_instr_adc_r8!(instr__ADC_A_A, A);

    /* SUB A,B */
    fn_instr_sub_r8!(instr__SUB_A_B, B);

    /* SUB A,C */
    fn_instr_sub_r8!(instr__SUB_A_C, C);

    /* SUB A,D */
    fn_instr_sub_r8!(instr__SUB_A_D, D);

    /* SUB A,E */
    fn_instr_sub_r8!(instr__SUB_A_E, E);

    /* SUB A,H */
    fn_instr_sub_r8!(instr__SUB_A_H, H);

    /* SUB A,L */
    fn_instr_sub_r8!(instr__SUB_A_L, L);

    /* SUB A,(HL) */
    pub(crate) fn instr__SUB_A_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());

        self.sub(byte_temp);
    }

    /* SUB A,A */
    fn_instr_sub_r8!(instr__SUB_A_A, A);

    /* SBC A,B */
    fn_instr_sbc_r8!(instr__SBC_A_B, B);

    /* SBC A,C */
    fn_instr_sbc_r8!(instr__SBC_A_C, C);

    /* SBC A,D */
    fn_instr_sbc_r8!(instr__SBC_A_D, D);

    /* SBC A,E */
    fn_instr_sbc_r8!(instr__SBC_A_E, E);

    /* SBC A,H */
    fn_instr_sbc_r8!(instr__SBC_A_H, H);

    /* SBC A,L */
    fn_instr_sbc_r8!(instr__SBC_A_L, L);

    /* SBC A,(HL) */
    pub(crate) fn instr__SBC_A_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());

        self.sbc(byte_temp);
    }

    /* SBC A,A */
    fn_instr_sbc_r8!(instr__SBC_A_A, A);

    /* AND A,B */
    fn_instr_and_r8!(instr__AND_A_B, B);

    /* AND A,C */
    fn_instr_and_r8!(instr__AND_A_C, C);

    /* AND A,D */
    fn_instr_and_r8!(instr__AND_A_D, D);

    /* AND A,E */
    fn_instr_and_r8!(instr__AND_A_E, E);

    /* AND A,H */
    fn_instr_and_r8!(instr__AND_A_H, H);

    /* AND A,L */
    fn_instr_and_r8!(instr__AND_A_L, L);

    /* AND A,(HL) */
    pub(crate) fn instr__AND_A_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());

        self.and(byte_temp);
    }

    /* AND A,A */
    fn_instr_and_r8!(instr__AND_A_A, A);

    /* XOR A,B */
    fn_instr_xor_r8!(instr__XOR_A_B, B);

    /* XOR A,C */
    fn_instr_xor_r8!(instr__XOR_A_C, C);

    /* XOR A,D */
    fn_instr_xor_r8!(instr__XOR_A_D, D);

    /* XOR A,E */
    fn_instr_xor_r8!(instr__XOR_A_E, E);

    /* XOR A,H */
    fn_instr_xor_r8!(instr__XOR_A_H, H);

    /* XOR A,L */
    fn_instr_xor_r8!(instr__XOR_A_L, L);

    /* XOR A,(HL) */
    pub(crate) fn instr__XOR_A_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());

        self.xor(byte_temp);
    }

    /* XOR A,A */
    fn_instr_xor_r8!(instr__XOR_A_A, A);

    /* OR A,B */
    fn_instr_or_r8!(instr__OR_A_B, B);

    /* OR A,C */
    fn_instr_or_r8!(instr__OR_A_C, C);

    /* OR A,D */
    fn_instr_or_r8!(instr__OR_A_D, D);

    /* OR A,E */
    fn_instr_or_r8!(instr__OR_A_E, E);

    /* OR A,H */
    fn_instr_or_r8!(instr__OR_A_H, H);

    /* OR A,L */
    fn_instr_or_r8!(instr__OR_A_L, L);

    /* OR A,(HL) */
    pub(crate) fn instr__OR_A_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());

        self.or(byte_temp);
    }

    /* OR A,A */
    fn_instr_or_r8!(instr__OR_A_A, A);

    /* CP B */
    fn_instr_cp_r8!(instr__CP_B, B);

    /* CP C */
    fn_instr_cp_r8!(instr__CP_C, C);

    /* CP D */
    fn_instr_cp_r8!(instr__CP_D, D);

    /* CP E */
    fn_instr_cp_r8!(instr__CP_E, E);

    /* CP H */
    fn_instr_cp_r8!(instr__CP_H, H);

    /* CP L */
    fn_instr_cp_r8!(instr__CP_L, L);

    /* CP (HL) */
    pub(crate) fn instr__CP_iHL(&mut self) {
        let address = self.HL();
        let byte_temp: u8 = self.memory.read_byte(address);

        self.cp(byte_temp);
    }

    /* CP A */
    fn_instr_cp_r8!(instr__CP_A, A);

    /* RET NZ */
    pub(crate) fn instr__RET_NZ(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        if (self.data.F & FLAG_Z) == 0 {
            self.ret();
        }
    }

    /* POP BC */
    fn_instr_pop_r16!(instr__POP_BC, C, B);

    /* JP NZ,nnnn */
    pub(crate) fn instr__JP_NZ_NNNN(&mut self) {
        if (self.data.F & FLAG_Z) == 0 {
            self.jp();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* JP nnnn */
    pub(crate) fn instr__JP_NNNN(&mut self) {
        self.jp();
    }

    /* CALL NZ,nnnn */
    pub(crate) fn instr__CALL_NZ_NNNN(&mut self) {
        if (self.data.F & FLAG_Z) == 0 {
            self.call();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* PUSH BC */
    fn_instr_push_r16!(instr__PUSH_BC, C, B);

    /* ADD A,nn */
    pub(crate) fn instr__ADD_A_NN(&mut self) {
        let address = self.PC();
        let byte_temp: u8 = self.memory.read_byte(address);
        self.IncPC(1);
        self.add(byte_temp);
    }

    /* RST 00 */
    pub(crate) fn instr__RST_00(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.rst(0x00);
    }

    /* RET Z */
    pub(crate) fn instr__RET_Z(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        if (self.data.F & FLAG_Z) != 0 {
            self.ret();
        }
    }

    /* RET */
    pub(crate) fn instr__RET(&mut self) {
        self.ret();
    }

    /* JP Z,nnnn */
    pub(crate) fn instr__JP_Z_NNNN(&mut self) {
        if (self.data.F & FLAG_Z) != 0 {
            self.jp();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* shift CB */
    pub(crate) fn instr__SHIFT_CB(&mut self) {}

    /* CALL Z,nnnn */
    pub(crate) fn instr__CALL_Z_NNNN(&mut self) {
        if (self.data.F & FLAG_Z) != 0 {
            self.call();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* CALL nnnn */
    pub(crate) fn instr__CALL_NNNN(&mut self) {
        self.call();
    }

    /* ADC A,nn */
    pub(crate) fn instr__ADC_A_NN(&mut self) {
        let address = self.PC();
        let byte_temp: u8 = self.memory.read_byte(address);
        self.IncPC(1);
        self.adc(byte_temp);
    }

    /* RST 8 */
    pub(crate) fn instr__RST_8(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.rst(0x8);
    }

    /* RET NC */
    pub(crate) fn instr__RET_NC(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        if self.data.F & FLAG_C == 0 {
            self.ret();
        }
    }

    /* POP DE */
    fn_instr_pop_r16!(instr__POP_DE, E, D);

    /* JP NC,nnnn */
    pub(crate) fn instr__JP_NC_NNNN(&mut self) {
        if self.data.F & FLAG_C == 0 {
            self.jp();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* OUT (nn),A */
    pub(crate) fn instr__OUT_iNN_A(&mut self) {
        let address = self.PC();
        let out_temp: u16 = (self.memory.read_byte(address) as u16) + ((self.data.A as u16) << 8);
        self.IncPC(1);
        self.write_port(out_temp, self.data.A);
    }

    /* CALL NC,nnnn */
    pub(crate) fn instr__CALL_NC_NNNN(&mut self) {
        if self.data.F & FLAG_C == 0 {
            self.call();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* PUSH DE */
    fn_instr_push_r16!(instr__PUSH_DE, E, D);

    /* SUB nn */
    pub(crate) fn instr__SUB_NN(&mut self) {
        let address = self.PC();
        let byte_temp: u8 = self.memory.read_byte(address);
        self.IncPC(1);
        self.sub(byte_temp);
    }

    /* RST 10 */
    pub(crate) fn instr__RST_10(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.rst(0x10);
    }

    /* RET C */
    pub(crate) fn instr__RET_C(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        if self.data.F & FLAG_C != 0 {
            self.ret();
        }
    }

    /* EXX */
    pub(crate) fn instr__EXX(&mut self) {
        let word_temp: u16 = self.BC();
        self.SetBC(self.BC_());
        self.SetBC_(word_temp);

        let word_temp = self.DE();
        self.SetDE(self.DE_());
        self.SetDE_(word_temp);

        let word_temp = self.HL();
        self.SetHL(self.HL_());
        self.SetHL_(word_temp);
    }

    /* JP C,nnnn */
    pub(crate) fn instr__JP_C_NNNN(&mut self) {
        if self.data.F & FLAG_C != 0 {
            self.jp();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* IN A,(nn) */
    pub(crate) fn instr__IN_A_iNN(&mut self) {
        let address = self.PC();
        let in_temp: u16 = (self.memory.read_byte(address) as u16) + ((self.data.A as u16) << 8);
        self.IncPC(1);
        self.data.A = self.read_port(in_temp);
    }

    /* CALL C,nnnn */
    pub(crate) fn instr__CALL_C_NNNN(&mut self) {
        if self.data.F & FLAG_C != 0 {
            self.call();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* shift DD */
    pub(crate) fn instr__SHIFT_DD(&mut self) {}

    /* SBC A,nn */
    pub(crate) fn instr__SBC_A_NN(&mut self) {
        let address = self.PC();
        let byte_temp: u8 = self.memory.read_byte(address);
        self.IncPC(1);
        self.sbc(byte_temp);
    }

    /* RST 18 */
    pub(crate) fn instr__RST_18(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.rst(0x18);
    }

    /* RET PO */
    pub(crate) fn instr__RET_PO(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        if (self.data.F & FLAG_P) == 0 {
            self.ret();
        }
    }

    /* POP HL */
    fn_instr_pop_r16!(instr__POP_HL, L, H);

    /* JP PO,nnnn */
    pub(crate) fn instr__JP_PO_NNNN(&mut self) {
        if (self.data.F & FLAG_P) == 0 {
            self.jp();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* EX (SP),HL */
    pub(crate) fn instr__EX_iSP_HL(&mut self) {
        let address = self.SP();
        let byte_temp_l = self.memory.read_byte(address);
        let sp = self.SP();
        let byte_temp_h = self.memory.read_byte(sp + 1);
        let sp = self.SP();
        self.memory.contend_read_no_mreq(sp + 1, 1);
        let sp = self.SP();
        self.memory.write_byte(sp + 1, self.data.H);
        let address = self.SP();
        self.memory.write_byte(address, self.data.L);
        let _address = self.SP();
        self.memory.contend_write_no_mreq_loop(_address, 1, 2);
        self.data.L = byte_temp_l;
        self.data.H = byte_temp_h;
    }

    /* CALL PO,nnnn */
    pub(crate) fn instr__CALL_PO_NNNN(&mut self) {
        if (self.data.F & FLAG_P) == 0 {
            self.call();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* PUSH HL */
    fn_instr_push_r16!(instr__PUSH_HL, L, H);

    /* AND nn */
    pub(crate) fn instr__AND_NN(&mut self) {
        let address = self.PC();
        let byte_temp: u8 = self.memory.read_byte(address);
        self.IncPC(1);
        self.and(byte_temp);
    }

    /* RST 20 */
    pub(crate) fn instr__RST_20(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.rst(0x20);
    }

    /* RET PE */
    pub(crate) fn instr__RET_PE(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        if (self.data.F & FLAG_P) != 0 {
            self.ret();
        }
    }

    /* JP HL */
    pub(crate) fn instr__JP_HL(&mut self) {
        // self.SetPC(self.HL());
        self.jp_hl();
        /* NB: NOT INDIRECT! */
    }

    /* JP PE,nnnn */
    pub(crate) fn instr__JP_PE_NNNN(&mut self) {
        if (self.data.F & FLAG_P) != 0 {
            self.jp();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* EX DE,HL */
    pub(crate) fn instr__EX_DE_HL(&mut self) {
        let word_temp: u16 = self.DE();
        self.SetDE(self.HL());
        self.SetHL(word_temp);
    }

    /* CALL PE,nnnn */
    pub(crate) fn instr__CALL_PE_NNNN(&mut self) {
        if (self.data.F & FLAG_P) != 0 {
            self.call();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* shift ED */
    pub(crate) fn instr__SHIFT_ED(&mut self) {}

    /* XOR A,nn */
    pub(crate) fn instr__XOR_A_NN(&mut self) {
        let address = self.PC();
        let byte_temp: u8 = self.memory.read_byte(address);
        self.IncPC(1);
        self.xor(byte_temp);
    }

    /* RST 28 */
    pub(crate) fn instr__RST_28(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.rst(0x28);
    }

    /* RET P */
    pub(crate) fn instr__RET_P(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        if (self.data.F & FLAG_S) == 0 {
            self.ret();
        }
    }

    /* POP AF */
    fn_instr_pop_r16!(instr__POP_AF, F, A);

    /* JP P,nnnn */
    pub(crate) fn instr__JP_P_NNNN(&mut self) {
        if (self.data.F & FLAG_S) == 0 {
            self.jp();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* DI */
    pub(crate) fn instr__DI(&mut self) {
        (self.data.IFF1, self.data.IFF2) = (0, 0);
    }

    /* CALL P,nnnn */
    pub(crate) fn instr__CALL_P_NNNN(&mut self) {
        if (self.data.F & FLAG_S) == 0 {
            self.call();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* PUSH AF */
    fn_instr_push_r16!(instr__PUSH_AF, F, A);

    /* OR nn */
    pub(crate) fn instr__OR_NN(&mut self) {
        let address = self.PC();
        let byte_temp: u8 = self.memory.read_byte(address);
        self.IncPC(1);
        self.or(byte_temp);
    }

    /* RST 30 */
    pub(crate) fn instr__RST_30(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.rst(0x30);
    }

    /* RET M */
    pub(crate) fn instr__RET_M(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        if (self.data.F & FLAG_S) != 0 {
            self.ret();
        }
    }

    /* LD SP,HL */
    pub(crate) fn instr__LD_SP_HL(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.SetSP(self.HL());
    }

    /* JP M,nnnn */
    pub(crate) fn instr__JP_M_NNNN(&mut self) {
        if (self.data.F & FLAG_S) != 0 {
            self.jp();
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* EI */
    pub(crate) fn instr__EI(&mut self) {
        /* Interrupts are not accepted immediately after an EI, but are
        accepted after the next instruction */
        (self.data.IFF1, self.data.IFF2) = (1, 1);
        self.data.interrupts_enabled_at = self.data.t_states;
        // eventAdd(self.Tstates + 1, z80InterruptEvent)
    }

    /* CALL M,nnnn */
    pub(crate) fn instr__CALL_M_NNNN(&mut self) {
        if (self.data.F & FLAG_S) != 0 {
            self.call()
        } else {
            let _address = self.PC();
            self.memory.contend_read(_address, 3);
            let pc = self.PC();
            self.memory.contend_read(pc + 1, 3);
            self.IncPC(2);
        }
    }

    /* shift FD */
    pub(crate) fn instr__SHIFT_FD(&mut self) {}

    /* CP nn */
    pub(crate) fn instr__CP_NN(&mut self) {
        let address = self.PC();
        let byte_temp: u8 = self.memory.read_byte(address);
        self.IncPC(1);
        self.cp(byte_temp);
    }

    /* RST 38 */
    pub(crate) fn instr__RST_38(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.rst(0x38);
    }

    /* RLC B */
    pub(crate) fn instrCB__RLC_B(&mut self) {
        self.data.B = self.rlc(self.data.B);
    }

    /* RLC C */
    pub(crate) fn instrCB__RLC_C(&mut self) {
        self.data.C = self.rlc(self.data.C);
    }

    /* RLC D */
    pub(crate) fn instrCB__RLC_D(&mut self) {
        self.data.D = self.rlc(self.data.D);
    }

    /* RLC E */
    pub(crate) fn instrCB__RLC_E(&mut self) {
        self.data.E = self.rlc(self.data.E);
    }

    /* RLC H */
    pub(crate) fn instrCB__RLC_H(&mut self) {
        self.data.H = self.rlc(self.data.H);
    }

    /* RLC L */
    pub(crate) fn instrCB__RLC_L(&mut self) {
        self.data.L = self.rlc(self.data.L);
    }

    /* RLC (HL) */
    pub(crate) fn instrCB__RLC_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        byte_temp = self.rlc(byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* RLC A */
    pub(crate) fn instrCB__RLC_A(&mut self) {
        self.data.A = self.rlc(self.data.A);
    }

    /* RRC B */
    pub(crate) fn instrCB__RRC_B(&mut self) {
        self.data.B = self.rrc(self.data.B);
    }

    /* RRC C */
    pub(crate) fn instrCB__RRC_C(&mut self) {
        self.data.C = self.rrc(self.data.C);
    }

    /* RRC D */
    pub(crate) fn instrCB__RRC_D(&mut self) {
        self.data.D = self.rrc(self.data.D);
    }

    /* RRC E */
    pub(crate) fn instrCB__RRC_E(&mut self) {
        self.data.E = self.rrc(self.data.E);
    }

    /* RRC H */
    pub(crate) fn instrCB__RRC_H(&mut self) {
        self.data.H = self.rrc(self.data.H);
    }

    /* RRC L */
    pub(crate) fn instrCB__RRC_L(&mut self) {
        self.data.L = self.rrc(self.data.L);
    }

    /* RRC (HL) */
    pub(crate) fn instrCB__RRC_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        byte_temp = self.rrc(byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* RRC A */
    pub(crate) fn instrCB__RRC_A(&mut self) {
        self.data.A = self.rrc(self.data.A);
    }

    /* RL B */
    pub(crate) fn instrCB__RL_B(&mut self) {
        self.data.B = self.rl(self.data.B);
    }

    /* RL C */
    pub(crate) fn instrCB__RL_C(&mut self) {
        self.data.C = self.rl(self.data.C);
    }

    /* RL D */
    pub(crate) fn instrCB__RL_D(&mut self) {
        self.data.D = self.rl(self.data.D);
    }

    /* RL E */
    pub(crate) fn instrCB__RL_E(&mut self) {
        self.data.E = self.rl(self.data.E);
    }

    /* RL H */
    pub(crate) fn instrCB__RL_H(&mut self) {
        self.data.H = self.rl(self.data.H);
    }

    /* RL L */
    pub(crate) fn instrCB__RL_L(&mut self) {
        self.data.L = self.rl(self.data.L);
    }

    /* RL (HL) */
    pub(crate) fn instrCB__RL_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        byte_temp = self.rl(byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* RL A */
    pub(crate) fn instrCB__RL_A(&mut self) {
        self.data.A = self.rl(self.data.A);
    }

    /* RR B */
    pub(crate) fn instrCB__RR_B(&mut self) {
        self.data.B = self.rr(self.data.B);
    }

    /* RR C */
    pub(crate) fn instrCB__RR_C(&mut self) {
        self.data.C = self.rr(self.data.C);
    }

    /* RR D */
    pub(crate) fn instrCB__RR_D(&mut self) {
        self.data.D = self.rr(self.data.D);
    }

    /* RR E */
    pub(crate) fn instrCB__RR_E(&mut self) {
        self.data.E = self.rr(self.data.E);
    }

    /* RR H */
    pub(crate) fn instrCB__RR_H(&mut self) {
        self.data.H = self.rr(self.data.H);
    }

    /* RR L */
    pub(crate) fn instrCB__RR_L(&mut self) {
        self.data.L = self.rr(self.data.L);
    }

    /* RR (HL) */
    pub(crate) fn instrCB__RR_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        byte_temp = self.rr(byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* RR A */
    pub(crate) fn instrCB__RR_A(&mut self) {
        self.data.A = self.rr(self.data.A);
    }

    /* SLA B */
    pub(crate) fn instrCB__SLA_B(&mut self) {
        self.data.B = self.sla(self.data.B);
    }

    /* SLA C */
    pub(crate) fn instrCB__SLA_C(&mut self) {
        self.data.C = self.sla(self.data.C);
    }

    /* SLA D */
    pub(crate) fn instrCB__SLA_D(&mut self) {
        self.data.D = self.sla(self.data.D);
    }

    /* SLA E */
    pub(crate) fn instrCB__SLA_E(&mut self) {
        self.data.E = self.sla(self.data.E);
    }

    /* SLA H */
    pub(crate) fn instrCB__SLA_H(&mut self) {
        self.data.H = self.sla(self.data.H);
    }

    /* SLA L */
    pub(crate) fn instrCB__SLA_L(&mut self) {
        self.data.L = self.sla(self.data.L);
    }

    /* SLA (HL) */
    pub(crate) fn instrCB__SLA_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        byte_temp = self.sla(byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* SLA A */
    pub(crate) fn instrCB__SLA_A(&mut self) {
        self.data.A = self.sla(self.data.A);
    }

    /* SRA B */
    pub(crate) fn instrCB__SRA_B(&mut self) {
        self.data.B = self.sra(self.data.B);
    }

    /* SRA C */
    pub(crate) fn instrCB__SRA_C(&mut self) {
        self.data.C = self.sra(self.data.C);
    }

    /* SRA D */
    pub(crate) fn instrCB__SRA_D(&mut self) {
        self.data.D = self.sra(self.data.D);
    }

    /* SRA E */
    pub(crate) fn instrCB__SRA_E(&mut self) {
        self.data.E = self.sra(self.data.E);
    }

    /* SRA H */
    pub(crate) fn instrCB__SRA_H(&mut self) {
        self.data.H = self.sra(self.data.H);
    }

    /* SRA L */
    pub(crate) fn instrCB__SRA_L(&mut self) {
        self.data.L = self.sra(self.data.L);
    }

    /* SRA (HL) */
    pub(crate) fn instrCB__SRA_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        byte_temp = self.sra(byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* SRA A */
    pub(crate) fn instrCB__SRA_A(&mut self) {
        self.data.A = self.sra(self.data.A);
    }

    /* SLL B */
    pub(crate) fn instrCB__SLL_B(&mut self) {
        self.data.B = self.sll(self.data.B);
    }

    /* SLL C */
    pub(crate) fn instrCB__SLL_C(&mut self) {
        self.data.C = self.sll(self.data.C);
    }

    /* SLL D */
    pub(crate) fn instrCB__SLL_D(&mut self) {
        self.data.D = self.sll(self.data.D);
    }

    /* SLL E */
    pub(crate) fn instrCB__SLL_E(&mut self) {
        self.data.E = self.sll(self.data.E);
    }

    /* SLL H */
    pub(crate) fn instrCB__SLL_H(&mut self) {
        self.data.H = self.sll(self.data.H);
    }

    /* SLL L */
    pub(crate) fn instrCB__SLL_L(&mut self) {
        self.data.L = self.sll(self.data.L);
    }

    /* SLL (HL) */
    pub(crate) fn instrCB__SLL_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        byte_temp = self.sll(byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* SLL A */
    pub(crate) fn instrCB__SLL_A(&mut self) {
        self.data.A = self.sll(self.data.A);
    }

    /* SRL B */
    pub(crate) fn instrCB__SRL_B(&mut self) {
        self.data.B = self.srl(self.data.B);
    }

    /* SRL C */
    pub(crate) fn instrCB__SRL_C(&mut self) {
        self.data.C = self.srl(self.data.C);
    }

    /* SRL D */
    pub(crate) fn instrCB__SRL_D(&mut self) {
        self.data.D = self.srl(self.data.D);
    }

    /* SRL E */
    pub(crate) fn instrCB__SRL_E(&mut self) {
        self.data.E = self.srl(self.data.E);
    }

    /* SRL H */
    pub(crate) fn instrCB__SRL_H(&mut self) {
        self.data.H = self.srl(self.data.H);
    }

    /* SRL L */
    pub(crate) fn instrCB__SRL_L(&mut self) {
        self.data.L = self.srl(self.data.L);
    }

    /* SRL (HL) */
    pub(crate) fn instrCB__SRL_iHL(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        byte_temp = self.srl(byte_temp);
        self.memory.write_byte(self.HL(), byte_temp);
    }

    /* SRL A */
    pub(crate) fn instrCB__SRL_A(&mut self) {
        self.data.A = self.srl(self.data.A);
    }

    /* BIT 0,B */
    pub(crate) fn instrCB__BIT_0_B(&mut self) {
        self.bit(0, self.data.B);
    }

    /* BIT 0,C */
    pub(crate) fn instrCB__BIT_0_C(&mut self) {
        self.bit(0, self.data.C);
    }

    /* BIT 0,D */
    pub(crate) fn instrCB__BIT_0_D(&mut self) {
        self.bit(0, self.data.D);
    }

    /* BIT 0,E */
    pub(crate) fn instrCB__BIT_0_E(&mut self) {
        self.bit(0, self.data.E);
    }

    /* BIT 0,H */
    pub(crate) fn instrCB__BIT_0_H(&mut self) {
        self.bit(0, self.data.H);
    }

    /* BIT 0,L */
    pub(crate) fn instrCB__BIT_0_L(&mut self) {
        self.bit(0, self.data.L);
    }

    /* BIT 0,(HL) */
    pub(crate) fn instrCB__BIT_0_iHL(&mut self) {
        let byte_temp = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.bit(0, byte_temp)
    }

    /* BIT 0,A */
    pub(crate) fn instrCB__BIT_0_A(&mut self) {
        self.bit(0, self.data.A)
    }

    /* BIT 1,B */
    pub(crate) fn instrCB__BIT_1_B(&mut self) {
        self.bit(1, self.data.B)
    }

    /* BIT 1,C */
    pub(crate) fn instrCB__BIT_1_C(&mut self) {
        self.bit(1, self.data.C)
    }

    /* BIT 1,D */
    pub(crate) fn instrCB__BIT_1_D(&mut self) {
        self.bit(1, self.data.D)
    }

    /* BIT 1,E */
    pub(crate) fn instrCB__BIT_1_E(&mut self) {
        self.bit(1, self.data.E)
    }

    /* BIT 1,H */
    pub(crate) fn instrCB__BIT_1_H(&mut self) {
        self.bit(1, self.data.H)
    }

    /* BIT 1,L */
    pub(crate) fn instrCB__BIT_1_L(&mut self) {
        self.bit(1, self.data.L)
    }

    /* BIT 1,(HL) */
    pub(crate) fn instrCB__BIT_1_iHL(&mut self) {
        let byte_temp = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.bit(1, byte_temp);
    }

    /* BIT 1,A */
    pub(crate) fn instrCB__BIT_1_A(&mut self) {
        self.bit(1, self.data.A);
    }

    /* BIT 2,B */
    pub(crate) fn instrCB__BIT_2_B(&mut self) {
        self.bit(2, self.data.B);
    }

    /* BIT 2,C */
    pub(crate) fn instrCB__BIT_2_C(&mut self) {
        self.bit(2, self.data.C);
    }

    /* BIT 2,D */
    pub(crate) fn instrCB__BIT_2_D(&mut self) {
        self.bit(2, self.data.D);
    }

    /* BIT 2,E */
    pub(crate) fn instrCB__BIT_2_E(&mut self) {
        self.bit(2, self.data.E);
    }

    /* BIT 2,H */
    pub(crate) fn instrCB__BIT_2_H(&mut self) {
        self.bit(2, self.data.H);
    }

    /* BIT 2,L */
    pub(crate) fn instrCB__BIT_2_L(&mut self) {
        self.bit(2, self.data.L);
    }

    /* BIT 2,(HL) */
    pub(crate) fn instrCB__BIT_2_iHL(&mut self) {
        let byte_temp = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.bit(2, byte_temp);
    }

    /* BIT 2,A */
    pub(crate) fn instrCB__BIT_2_A(&mut self) {
        self.bit(2, self.data.A);
    }

    /* BIT 3,B */
    pub(crate) fn instrCB__BIT_3_B(&mut self) {
        self.bit(3, self.data.B);
    }

    /* BIT 3,C */
    pub(crate) fn instrCB__BIT_3_C(&mut self) {
        self.bit(3, self.data.C);
    }

    /* BIT 3,D */
    pub(crate) fn instrCB__BIT_3_D(&mut self) {
        self.bit(3, self.data.D)
    }

    /* BIT 3,E */
    pub(crate) fn instrCB__BIT_3_E(&mut self) {
        self.bit(3, self.data.E)
    }

    /* BIT 3,H */
    pub(crate) fn instrCB__BIT_3_H(&mut self) {
        self.bit(3, self.data.H)
    }

    /* BIT 3,L */
    pub(crate) fn instrCB__BIT_3_L(&mut self) {
        self.bit(3, self.data.L)
    }

    /* BIT 3,(HL) */
    pub(crate) fn instrCB__BIT_3_iHL(&mut self) {
        let byte_temp = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.bit(3, byte_temp);
    }

    /* BIT 3,A */
    pub(crate) fn instrCB__BIT_3_A(&mut self) {
        self.bit(3, self.data.A);
    }

    /* BIT 4,B */
    pub(crate) fn instrCB__BIT_4_B(&mut self) {
        self.bit(4, self.data.B);
    }

    /* BIT 4,C */
    pub(crate) fn instrCB__BIT_4_C(&mut self) {
        self.bit(4, self.data.C);
    }

    /* BIT 4,D */
    pub(crate) fn instrCB__BIT_4_D(&mut self) {
        self.bit(4, self.data.D);
    }

    /* BIT 4,E */
    pub(crate) fn instrCB__BIT_4_E(&mut self) {
        self.bit(4, self.data.E);
    }

    /* BIT 4,H */
    pub(crate) fn instrCB__BIT_4_H(&mut self) {
        self.bit(4, self.data.H);
    }

    /* BIT 4,L */
    pub(crate) fn instrCB__BIT_4_L(&mut self) {
        self.bit(4, self.data.L);
    }

    /* BIT 4,(HL) */
    pub(crate) fn instrCB__BIT_4_iHL(&mut self) {
        let byte_temp = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.bit(4, byte_temp);
    }

    /* BIT 4,A */
    pub(crate) fn instrCB__BIT_4_A(&mut self) {
        self.bit(4, self.data.A);
    }

    /* BIT 5,B */
    pub(crate) fn instrCB__BIT_5_B(&mut self) {
        self.bit(5, self.data.B);
    }

    /* BIT 5,C */
    pub(crate) fn instrCB__BIT_5_C(&mut self) {
        self.bit(5, self.data.C);
    }

    /* BIT 5,D */
    pub(crate) fn instrCB__BIT_5_D(&mut self) {
        self.bit(5, self.data.D);
    }

    /* BIT 5,E */
    pub(crate) fn instrCB__BIT_5_E(&mut self) {
        self.bit(5, self.data.E);
    }

    /* BIT 5,H */
    pub(crate) fn instrCB__BIT_5_H(&mut self) {
        self.bit(5, self.data.H);
    }

    /* BIT 5,L */
    pub(crate) fn instrCB__BIT_5_L(&mut self) {
        self.bit(5, self.data.L);
    }

    /* BIT 5,(HL) */
    pub(crate) fn instrCB__BIT_5_iHL(&mut self) {
        let byte_temp = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.bit(5, byte_temp);
    }

    /* BIT 5,A */
    pub(crate) fn instrCB__BIT_5_A(&mut self) {
        self.bit(5, self.data.A);
    }

    /* BIT 6,B */
    pub(crate) fn instrCB__BIT_6_B(&mut self) {
        self.bit(6, self.data.B);
    }

    /* BIT 6,C */
    pub(crate) fn instrCB__BIT_6_C(&mut self) {
        self.bit(6, self.data.C);
    }

    /* BIT 6,D */
    pub(crate) fn instrCB__BIT_6_D(&mut self) {
        self.bit(6, self.data.D);
    }

    /* BIT 6,E */
    pub(crate) fn instrCB__BIT_6_E(&mut self) {
        self.bit(6, self.data.E);
    }

    /* BIT 6,H */
    pub(crate) fn instrCB__BIT_6_H(&mut self) {
        self.bit(6, self.data.H)
    }

    /* BIT 6,L */
    pub(crate) fn instrCB__BIT_6_L(&mut self) {
        self.bit(6, self.data.L)
    }

    /* BIT 6,(HL) */
    pub(crate) fn instrCB__BIT_6_iHL(&mut self) {
        let byte_temp = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.bit(6, byte_temp);
    }

    /* BIT 6,A */
    pub(crate) fn instrCB__BIT_6_A(&mut self) {
        self.bit(6, self.data.A);
    }

    /* BIT 7,B */
    pub(crate) fn instrCB__BIT_7_B(&mut self) {
        self.bit(7, self.data.B);
    }

    /* BIT 7,C */
    pub(crate) fn instrCB__BIT_7_C(&mut self) {
        self.bit(7, self.data.C);
    }

    /* BIT 7,D */
    pub(crate) fn instrCB__BIT_7_D(&mut self) {
        self.bit(7, self.data.D);
    }

    /* BIT 7,E */
    pub(crate) fn instrCB__BIT_7_E(&mut self) {
        self.bit(7, self.data.E);
    }

    /* BIT 7,H */
    pub(crate) fn instrCB__BIT_7_H(&mut self) {
        self.bit(7, self.data.H);
    }

    /* BIT 7,L */
    pub(crate) fn instrCB__BIT_7_L(&mut self) {
        self.bit(7, self.data.L);
    }

    /* BIT 7,(HL) */
    pub(crate) fn instrCB__BIT_7_iHL(&mut self) {
        let byte_temp = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.bit(7, byte_temp);
    }

    /* BIT 7,A */
    pub(crate) fn instrCB__BIT_7_A(&mut self) {
        self.bit(7, self.data.A);
    }

    /* RES 0,B */
    pub(crate) fn instrCB__RES_0_B(&mut self) {
        self.data.B &= 0xfe;
    }

    /* RES 0,C */
    pub(crate) fn instrCB__RES_0_C(&mut self) {
        self.data.C &= 0xfe;
    }

    /* RES 0,D */
    pub(crate) fn instrCB__RES_0_D(&mut self) {
        self.data.D &= 0xfe;
    }

    /* RES 0,E */
    pub(crate) fn instrCB__RES_0_E(&mut self) {
        self.data.E &= 0xfe;
    }

    /* RES 0,H */
    pub(crate) fn instrCB__RES_0_H(&mut self) {
        self.data.H &= 0xfe;
    }

    /* RES 0,L */
    pub(crate) fn instrCB__RES_0_L(&mut self) {
        self.data.L &= 0xfe;
    }

    /* RES 0,(HL) */
    pub(crate) fn instrCB__RES_0_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp & 0xfe);
    }

    /* RES 0,A */
    pub(crate) fn instrCB__RES_0_A(&mut self) {
        self.data.A &= 0xfe;
    }

    /* RES 1,B */
    pub(crate) fn instrCB__RES_1_B(&mut self) {
        self.data.B &= 0xfd;
    }

    /* RES 1,C */
    pub(crate) fn instrCB__RES_1_C(&mut self) {
        self.data.C &= 0xfd;
    }

    /* RES 1,D */
    pub(crate) fn instrCB__RES_1_D(&mut self) {
        self.data.D &= 0xfd;
    }

    /* RES 1,E */
    pub(crate) fn instrCB__RES_1_E(&mut self) {
        self.data.E &= 0xfd;
    }

    /* RES 1,H */
    pub(crate) fn instrCB__RES_1_H(&mut self) {
        self.data.H &= 0xfd;
    }

    /* RES 1,L */
    pub(crate) fn instrCB__RES_1_L(&mut self) {
        self.data.L &= 0xfd;
    }

    /* RES 1,(HL) */
    pub(crate) fn instrCB__RES_1_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp & 0xfd);
    }

    /* RES 1,A */
    pub(crate) fn instrCB__RES_1_A(&mut self) {
        self.data.A &= 0xfd;
    }

    /* RES 2,B */
    pub(crate) fn instrCB__RES_2_B(&mut self) {
        self.data.B &= 0xfb;
    }

    /* RES 2,C */
    pub(crate) fn instrCB__RES_2_C(&mut self) {
        self.data.C &= 0xfb;
    }

    /* RES 2,D */
    pub(crate) fn instrCB__RES_2_D(&mut self) {
        self.data.D &= 0xfb;
    }

    /* RES 2,E */
    pub(crate) fn instrCB__RES_2_E(&mut self) {
        self.data.E &= 0xfb;
    }

    /* RES 2,H */
    pub(crate) fn instrCB__RES_2_H(&mut self) {
        self.data.H &= 0xfb;
    }

    /* RES 2,L */
    pub(crate) fn instrCB__RES_2_L(&mut self) {
        self.data.L &= 0xfb;
    }

    /* RES 2,(HL) */
    pub(crate) fn instrCB__RES_2_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp & 0xfb)
    }

    /* RES 2,A */
    pub(crate) fn instrCB__RES_2_A(&mut self) {
        self.data.A &= 0xfb
    }

    /* RES 3,B */
    pub(crate) fn instrCB__RES_3_B(&mut self) {
        self.data.B &= 0xf7
    }

    /* RES 3,C */
    pub(crate) fn instrCB__RES_3_C(&mut self) {
        self.data.C &= 0xf7
    }

    /* RES 3,D */
    pub(crate) fn instrCB__RES_3_D(&mut self) {
        self.data.D &= 0xf7
    }

    /* RES 3,E */
    pub(crate) fn instrCB__RES_3_E(&mut self) {
        self.data.E &= 0xf7
    }

    /* RES 3,H */
    pub(crate) fn instrCB__RES_3_H(&mut self) {
        self.data.H &= 0xf7
    }

    /* RES 3,L */
    pub(crate) fn instrCB__RES_3_L(&mut self) {
        self.data.L &= 0xf7
    }

    /* RES 3,(HL) */
    pub(crate) fn instrCB__RES_3_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp & 0xf7);
    }

    /* RES 3,A */
    pub(crate) fn instrCB__RES_3_A(&mut self) {
        self.data.A &= 0xf7;
    }

    /* RES 4,B */
    pub(crate) fn instrCB__RES_4_B(&mut self) {
        self.data.B &= 0xef;
    }

    /* RES 4,C */
    pub(crate) fn instrCB__RES_4_C(&mut self) {
        self.data.C &= 0xef;
    }

    /* RES 4,D */
    pub(crate) fn instrCB__RES_4_D(&mut self) {
        self.data.D &= 0xef;
    }

    /* RES 4,E */
    pub(crate) fn instrCB__RES_4_E(&mut self) {
        self.data.E &= 0xef;
    }

    /* RES 4,H */
    pub(crate) fn instrCB__RES_4_H(&mut self) {
        self.data.H &= 0xef;
    }

    /* RES 4,L */
    pub(crate) fn instrCB__RES_4_L(&mut self) {
        self.data.L &= 0xef;
    }

    /* RES 4,(HL) */
    pub(crate) fn instrCB__RES_4_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp & 0xef);
    }

    /* RES 4,A */
    pub(crate) fn instrCB__RES_4_A(&mut self) {
        self.data.A &= 0xef;
    }

    /* RES 5,B */
    pub(crate) fn instrCB__RES_5_B(&mut self) {
        self.data.B &= 0xdf;
    }

    /* RES 5,C */
    pub(crate) fn instrCB__RES_5_C(&mut self) {
        self.data.C &= 0xdf;
    }

    /* RES 5,D */
    pub(crate) fn instrCB__RES_5_D(&mut self) {
        self.data.D &= 0xdf;
    }

    /* RES 5,E */
    pub(crate) fn instrCB__RES_5_E(&mut self) {
        self.data.E &= 0xdf;
    }

    /* RES 5,H */
    pub(crate) fn instrCB__RES_5_H(&mut self) {
        self.data.H &= 0xdf;
    }

    /* RES 5,L */
    pub(crate) fn instrCB__RES_5_L(&mut self) {
        self.data.L &= 0xdf;
    }

    /* RES 5,(HL) */
    pub(crate) fn instrCB__RES_5_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp & 0xdf);
    }

    /* RES 5,A */
    pub(crate) fn instrCB__RES_5_A(&mut self) {
        self.data.A &= 0xdf;
    }

    /* RES 6,B */
    pub(crate) fn instrCB__RES_6_B(&mut self) {
        self.data.B &= 0xbf
    }

    /* RES 6,C */
    pub(crate) fn instrCB__RES_6_C(&mut self) {
        self.data.C &= 0xbf
    }

    /* RES 6,D */
    pub(crate) fn instrCB__RES_6_D(&mut self) {
        self.data.D &= 0xbf
    }

    /* RES 6,E */
    pub(crate) fn instrCB__RES_6_E(&mut self) {
        self.data.E &= 0xbf
    }

    /* RES 6,H */
    pub(crate) fn instrCB__RES_6_H(&mut self) {
        self.data.H &= 0xbf
    }

    /* RES 6,L */
    pub(crate) fn instrCB__RES_6_L(&mut self) {
        self.data.L &= 0xbf
    }

    /* RES 6,(HL) */
    pub(crate) fn instrCB__RES_6_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp & 0xbf);
    }

    /* RES 6,A */
    pub(crate) fn instrCB__RES_6_A(&mut self) {
        self.data.A &= 0xbf;
    }

    /* RES 7,B */
    pub(crate) fn instrCB__RES_7_B(&mut self) {
        self.data.B &= 0x7f;
    }

    /* RES 7,C */
    pub(crate) fn instrCB__RES_7_C(&mut self) {
        self.data.C &= 0x7f;
    }

    /* RES 7,D */
    pub(crate) fn instrCB__RES_7_D(&mut self) {
        self.data.D &= 0x7f;
    }

    /* RES 7,E */
    pub(crate) fn instrCB__RES_7_E(&mut self) {
        self.data.E &= 0x7f;
    }

    /* RES 7,H */
    pub(crate) fn instrCB__RES_7_H(&mut self) {
        self.data.H &= 0x7f;
    }

    /* RES 7,L */
    pub(crate) fn instrCB__RES_7_L(&mut self) {
        self.data.L &= 0x7f;
    }

    /* RES 7,(HL) */
    pub(crate) fn instrCB__RES_7_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp & 0x7f);
    }

    /* RES 7,A */
    pub(crate) fn instrCB__RES_7_A(&mut self) {
        self.data.A &= 0x7f;
    }

    /* SET 0,B */
    pub(crate) fn instrCB__SET_0_B(&mut self) {
        self.data.B |= 0x01;
    }

    /* SET 0,C */
    pub(crate) fn instrCB__SET_0_C(&mut self) {
        self.data.C |= 0x01;
    }

    /* SET 0,D */
    pub(crate) fn instrCB__SET_0_D(&mut self) {
        self.data.D |= 0x01;
    }

    /* SET 0,E */
    pub(crate) fn instrCB__SET_0_E(&mut self) {
        self.data.E |= 0x01;
    }

    /* SET 0,H */
    pub(crate) fn instrCB__SET_0_H(&mut self) {
        self.data.H |= 0x01;
    }

    /* SET 0,L */
    pub(crate) fn instrCB__SET_0_L(&mut self) {
        self.data.L |= 0x01;
    }

    /* SET 0,(HL) */
    pub(crate) fn instrCB__SET_0_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp | 0x01);
    }

    /* SET 0,A */
    pub(crate) fn instrCB__SET_0_A(&mut self) {
        self.data.A |= 0x01;
    }

    /* SET 1,B */
    pub(crate) fn instrCB__SET_1_B(&mut self) {
        self.data.B |= 0x02;
    }

    /* SET 1,C */
    pub(crate) fn instrCB__SET_1_C(&mut self) {
        self.data.C |= 0x02;
    }

    /* SET 1,D */
    pub(crate) fn instrCB__SET_1_D(&mut self) {
        self.data.D |= 0x02;
    }

    /* SET 1,E */
    pub(crate) fn instrCB__SET_1_E(&mut self) {
        self.data.E |= 0x02;
    }

    /* SET 1,H */
    pub(crate) fn instrCB__SET_1_H(&mut self) {
        self.data.H |= 0x02;
    }

    /* SET 1,L */
    pub(crate) fn instrCB__SET_1_L(&mut self) {
        self.data.L |= 0x02;
    }

    /* SET 1,(HL) */
    pub(crate) fn instrCB__SET_1_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp | 0x02)
    }

    /* SET 1,A */
    pub(crate) fn instrCB__SET_1_A(&mut self) {
        self.data.A |= 0x02;
    }

    /* SET 2,B */
    pub(crate) fn instrCB__SET_2_B(&mut self) {
        self.data.B |= 0x04;
    }

    /* SET 2,C */
    pub(crate) fn instrCB__SET_2_C(&mut self) {
        self.data.C |= 0x04;
    }

    /* SET 2,D */
    pub(crate) fn instrCB__SET_2_D(&mut self) {
        self.data.D |= 0x04;
    }

    /* SET 2,E */
    pub(crate) fn instrCB__SET_2_E(&mut self) {
        self.data.E |= 0x04;
    }

    /* SET 2,H */
    pub(crate) fn instrCB__SET_2_H(&mut self) {
        self.data.H |= 0x04;
    }

    /* SET 2,L */
    pub(crate) fn instrCB__SET_2_L(&mut self) {
        self.data.L |= 0x04;
    }

    /* SET 2,(HL) */
    pub(crate) fn instrCB__SET_2_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp | 0x04)
    }

    /* SET 2,A */
    pub(crate) fn instrCB__SET_2_A(&mut self) {
        self.data.A |= 0x04;
    }

    /* SET 3,B */
    pub(crate) fn instrCB__SET_3_B(&mut self) {
        self.data.B |= 0x08;
    }

    /* SET 3,C */
    pub(crate) fn instrCB__SET_3_C(&mut self) {
        self.data.C |= 0x08;
    }

    /* SET 3,D */
    pub(crate) fn instrCB__SET_3_D(&mut self) {
        self.data.D |= 0x08;
    }

    /* SET 3,E */
    pub(crate) fn instrCB__SET_3_E(&mut self) {
        self.data.E |= 0x08;
    }

    /* SET 3,H */
    pub(crate) fn instrCB__SET_3_H(&mut self) {
        self.data.H |= 0x08;
    }

    /* SET 3,L */
    pub(crate) fn instrCB__SET_3_L(&mut self) {
        self.data.L |= 0x08;
    }

    /* SET 3,(HL) */
    pub(crate) fn instrCB__SET_3_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp | 0x08)
    }

    /* SET 3,A */
    pub(crate) fn instrCB__SET_3_A(&mut self) {
        self.data.A |= 0x08;
    }

    /* SET 4,B */
    pub(crate) fn instrCB__SET_4_B(&mut self) {
        self.data.B |= 0x10;
    }

    /* SET 4,C */
    pub(crate) fn instrCB__SET_4_C(&mut self) {
        self.data.C |= 0x10;
    }

    /* SET 4,D */
    pub(crate) fn instrCB__SET_4_D(&mut self) {
        self.data.D |= 0x10;
    }

    /* SET 4,E */
    pub(crate) fn instrCB__SET_4_E(&mut self) {
        self.data.E |= 0x10;
    }

    /* SET 4,H */
    pub(crate) fn instrCB__SET_4_H(&mut self) {
        self.data.H |= 0x10;
    }

    /* SET 4,L */
    pub(crate) fn instrCB__SET_4_L(&mut self) {
        self.data.L |= 0x10;
    }

    /* SET 4,(HL) */
    pub(crate) fn instrCB__SET_4_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp | 0x10)
    }

    /* SET 4,A */
    pub(crate) fn instrCB__SET_4_A(&mut self) {
        self.data.A |= 0x10;
    }

    /* SET 5,B */
    pub(crate) fn instrCB__SET_5_B(&mut self) {
        self.data.B |= 0x20;
    }

    /* SET 5,C */
    pub(crate) fn instrCB__SET_5_C(&mut self) {
        self.data.C |= 0x20;
    }

    /* SET 5,D */
    pub(crate) fn instrCB__SET_5_D(&mut self) {
        self.data.D |= 0x20;
    }

    /* SET 5,E */
    pub(crate) fn instrCB__SET_5_E(&mut self) {
        self.data.E |= 0x20;
    }

    /* SET 5,H */
    pub(crate) fn instrCB__SET_5_H(&mut self) {
        self.data.H |= 0x20;
    }

    /* SET 5,L */
    pub(crate) fn instrCB__SET_5_L(&mut self) {
        self.data.L |= 0x20;
    }

    /* SET 5,(HL) */
    pub(crate) fn instrCB__SET_5_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp | 0x20)
    }

    /* SET 5,A */
    pub(crate) fn instrCB__SET_5_A(&mut self) {
        self.data.A |= 0x20;
    }

    /* SET 6,B */
    pub(crate) fn instrCB__SET_6_B(&mut self) {
        self.data.B |= 0x40;
    }

    /* SET 6,C */
    pub(crate) fn instrCB__SET_6_C(&mut self) {
        self.data.C |= 0x40;
    }

    /* SET 6,D */
    pub(crate) fn instrCB__SET_6_D(&mut self) {
        self.data.D |= 0x40;
    }

    /* SET 6,E */
    pub(crate) fn instrCB__SET_6_E(&mut self) {
        self.data.E |= 0x40;
    }

    /* SET 6,H */
    pub(crate) fn instrCB__SET_6_H(&mut self) {
        self.data.H |= 0x40;
    }

    /* SET 6,L */
    pub(crate) fn instrCB__SET_6_L(&mut self) {
        self.data.L |= 0x40;
    }

    /* SET 6,(HL) */
    pub(crate) fn instrCB__SET_6_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp | 0x40)
    }

    /* SET 6,A */
    pub(crate) fn instrCB__SET_6_A(&mut self) {
        self.data.A |= 0x40;
    }

    /* SET 7,B */
    pub(crate) fn instrCB__SET_7_B(&mut self) {
        self.data.B |= 0x80;
    }

    /* SET 7,C */
    pub(crate) fn instrCB__SET_7_C(&mut self) {
        self.data.C |= 0x80;
    }

    /* SET 7,D */
    pub(crate) fn instrCB__SET_7_D(&mut self) {
        self.data.D |= 0x80;
    }

    /* SET 7,E */
    pub(crate) fn instrCB__SET_7_E(&mut self) {
        self.data.E |= 0x80;
    }

    /* SET 7,H */
    pub(crate) fn instrCB__SET_7_H(&mut self) {
        self.data.H |= 0x80;
    }

    /* SET 7,L */
    pub(crate) fn instrCB__SET_7_L(&mut self) {
        self.data.L |= 0x80;
    }

    /* SET 7,(HL) */
    pub(crate) fn instrCB__SET_7_iHL(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq(self.HL(), 1);
        self.memory.write_byte(self.HL(), byte_temp | 0x80)
    }

    /* SET 7,A */
    pub(crate) fn instrCB__SET_7_A(&mut self) {
        self.data.A |= 0x80;
    }

    /* IN B,(C) */
    pub(crate) fn instrED__IN_B_iC(&mut self) {
        // self.in(&self.data.B, self.BC())
        let bc = self.BC();
        self.data.B = self.in_u8_ex(bc);
    }

    /* OUT (C),B */
    pub(crate) fn instrED__OUT_iC_B(&mut self) {
        self.write_port(self.BC(), self.data.B);
    }

    /* SBC HL,BC */
    pub(crate) fn instrED__SBC_HL_BC(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        self.sbc16(self.BC());
    }

    /* LD (nnnn),BC */
    fn_instr_ld_i_nnnn_r16!(instrED__LD_iNNNN_BC, C, B);

    /* NEG */
    pub(crate) fn instrED__NEG(&mut self) {
        let byte_temp = self.data.A;
        self.data.A = 0;
        self.sub(byte_temp);
    }

    /* RETN */
    pub(crate) fn instrED__RETN(&mut self) {
        self.data.IFF1 = self.data.IFF2;
        self.ret();
    }

    /* IM 0 */
    pub(crate) fn instrED__IM_0(&mut self) {
        self.data.IM = 0;
    }

    /* LD I,A */
    pub(crate) fn instrED__LD_I_A(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.data.I = self.data.A;
    }

    /* IN C,(C) */
    pub(crate) fn instrED__IN_C_iC(&mut self) {
        self.data.C = self.in_u8_ex(self.BC());
    }

    /* OUT (C),C */
    pub(crate) fn instrED__OUT_iC_C(&mut self) {
        self.write_port(self.BC(), self.data.C);
    }

    /* ADC HL,BC */
    pub(crate) fn instrED__ADC_HL_BC(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        self.adc16(self.BC());
    }

    /* LD BC,(nnnn) */
    fn_instr_ld_hl_i_nnnn!(instrED__LD_BC_iNNNN, C, B);

    /* LD R,A */
    pub(crate) fn instrED__LD_R_A(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        /* Keep the RZX instruction counter right */
        self.data.rzx_instructions_offset += (self.data.R as isize) - (self.data.A as isize);
        (self.data.R, self.data.R7) = ((self.data.A as u16), self.data.A);
    }

    /* IN D,(C) */
    pub(crate) fn instrED__IN_D_iC(&mut self) {
        self.data.D = self.in_u8_ex(self.BC());
    }

    /* OUT (C),D */
    pub(crate) fn instrED__OUT_iC_D(&mut self) {
        self.write_port(self.BC(), self.data.D);
    }

    /* SBC HL,DE */
    pub(crate) fn instrED__SBC_HL_DE(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        self.sbc16(self.DE());
    }

    /* LD (nnnn),DE */
    fn_instr_ld_i_nnnn_r16!(instrED__LD_iNNNN_DE, E, D);

    /* IM 1 */
    pub(crate) fn instrED__IM_1(&mut self) {
        self.data.IM = 1;
    }

    /* LD A,I */
    pub(crate) fn instrED__LD_A_I(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.data.A = self.data.I;
        self.data.F = self.data.F & FLAG_C
            | self.tables.sz53_table[self.data.A as usize]
            | tern_op_b(self.data.IFF2 != 0, FLAG_V, 0);
    }

    /* IN E,(C) */
    pub(crate) fn instrED__IN_E_iC(&mut self) {
        let port = self.BC();
        self.data.E = self.in_u8_ex(port);
    }

    /* OUT (C),E */
    pub(crate) fn instrED__OUT_iC_E(&mut self) {
        self.write_port(self.BC(), self.data.E);
    }

    /* ADC HL,DE */
    pub(crate) fn instrED__ADC_HL_DE(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        self.adc16(self.DE());
    }

    /* LD DE,(nnnn) */
    fn_instr_ld_hl_i_nnnn!(instrED__LD_DE_iNNNN, E, D);

    /* IM 2 */
    pub(crate) fn instrED__IM_2(&mut self) {
        self.data.IM = 2;
    }

    /* LD A,R */
    pub(crate) fn instrED__LD_A_R(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        self.data.A = (self.data.R & 0x7f) as u8 | (self.data.R7 & 0x80);
        self.data.F = self.data.F & FLAG_C
            | self.tables.sz53_table[self.data.A as usize]
            | tern_op_b(self.data.IFF2 != 0, FLAG_V, 0);
    }

    /* IN H,(C) */
    pub(crate) fn instrED__IN_H_iC(&mut self) {
        self.data.H = self.in_u8_ex(self.BC());
    }

    /* OUT (C),H */
    pub(crate) fn instrED__OUT_iC_H(&mut self) {
        self.write_port(self.BC(), self.data.H);
    }

    /* SBC HL,HL */
    pub(crate) fn instrED__SBC_HL_HL(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        self.sbc16(self.HL())
    }

    /* LD (nnnn),HL */
    fn_instr_ld_i_nnnn_r16!(instrED__LD_iNNNN_HL, L, H);

    /* RRD */
    pub(crate) fn instrED__RRD(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq_loop(self.HL(), 1, 4);
        self.memory
            .write_byte(self.HL(), (self.data.A << 4) | (byte_temp >> 4));
        self.data.A = self.data.A & 0xf0 | byte_temp & 0x0f;
        self.data.F = self.data.F & FLAG_C | self.tables.sz53p_table[self.data.A as usize];
    }

    /* IN L,(C) */
    pub(crate) fn instrED__IN_L_iC(&mut self) {
        self.data.L = self.in_u8_ex(self.BC());
    }

    /* OUT (C),L */
    pub(crate) fn instrED__OUT_iC_L(&mut self) {
        self.write_port(self.BC(), self.data.L);
    }

    /* ADC HL,HL */
    pub(crate) fn instrED__ADC_HL_HL(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        self.adc16(self.HL());
    }

    /* LD HL,(nnnn) */
    fn_instr_ld_hl_i_nnnn!(instrED__LD_HL_iNNNN, L, H);

    /* RLD */
    pub(crate) fn instrED__RLD(&mut self) {
        let byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.contend_read_no_mreq_loop(self.HL(), 1, 4);
        self.memory
            .write_byte(self.HL(), (byte_temp << 4) | (self.data.A & 0x0f));
        self.data.A = (self.data.A & 0xf0) | (byte_temp >> 4);
        self.data.F = self.data.F & FLAG_C | self.tables.sz53p_table[self.data.A as usize];
    }

    /* IN F,(C) */
    pub(crate) fn instrED__IN_F_iC(&mut self) {
        let _byte_temp: u8 = self.in_u8_ex(self.BC());
    }

    /* OUT (C),0 */
    pub(crate) fn instrED__OUT_iC_0(&mut self) {
        self.write_port(self.BC(), 0);
    }

    /* SBC HL,SP */
    pub(crate) fn instrED__SBC_HL_SP(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        let value = self.SP();
        self.sbc16(value);
    }

    /* LD (nnnn),SP */
    pub(crate) fn instrED__LD_iNNNN_SP(&mut self) {
        let (sph, spl) = split_word(self.data.sp);
        self.ld16nnrr(spl, sph);
        // break
    }

    /* IN A,(C) */
    pub(crate) fn instrED__IN_A_iC(&mut self) {
        self.data.A = self.in_u8_ex(self.BC());
    }

    /* OUT (C),A */
    pub(crate) fn instrED__OUT_iC_A(&mut self) {
        self.write_port(self.BC(), self.data.A);
    }

    /* ADC HL,SP */
    pub(crate) fn instrED__ADC_HL_SP(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        let value = self.SP();
        self.adc16(value);
    }

    /* LD SP,(nnnn) */
    pub(crate) fn instrED__LD_SP_iNNNN(&mut self) {
        // let (sph, spl) = splitWord(self.SP());
        let (spl, sph) = self.ld16rrnn_ex();
        self.SetSP(join_bytes(sph, spl));
        // break
    }

    /* LDI */
    pub(crate) fn instrED__LDI(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.DecBC();
        self.memory.write_byte(self.DE(), byte_temp);
        self.memory.contend_write_no_mreq_loop(self.DE(), 1, 2);
        self.IncDE();
        self.IncHL();
        byte_temp += self.data.A;
        self.data.F = self.data.F & (FLAG_C | FLAG_Z | FLAG_S)
            | tern_op_b(self.BC() != 0, FLAG_V, 0)
            | byte_temp & FLAG_3
            | tern_op_b((byte_temp & 0x02) != 0, FLAG_5, 0);
    }

    /* CPI */
    pub(crate) fn instrED__CPI(&mut self) {
        let value: u8 = self.memory.read_byte(self.HL());
        let mut byte_temp: u8 = self.data.A - value;
        let lookup: u8 =
            ((self.data.A & 0x08) >> 3) | ((value & 0x08) >> 2) | ((byte_temp & 0x08) >> 1);
        self.memory.contend_read_no_mreq_loop(self.HL(), 1, 5);
        self.IncHL();
        self.DecBC();
        self.data.F = self.data.F & FLAG_C
            | tern_op_b(self.BC() != 0, FLAG_V | FLAG_N, FLAG_N)
            | HALF_CARRY_SUB_TABLE[lookup as usize]
            | tern_op_b(byte_temp != 0, 0, FLAG_Z)
            | byte_temp & FLAG_S;
        if (self.data.F & FLAG_H) != 0 {
            byte_temp -= 1;
        }
        self.data.F |= (byte_temp & FLAG_3) | tern_op_b((byte_temp & 0x02) != 0, FLAG_5, 0);
    }

    /* INI */
    pub(crate) fn instrED__INI(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        let in_i_temp: u8 = self.read_port(self.BC());
        self.memory.write_byte(self.HL(), in_i_temp);

        self.data.B = self.data.B.wrapping_sub(1);
        self.IncHL();
        let in_i_temp2: u8 = in_i_temp.wrapping_add(self.data.C).wrapping_add(1);
        self.data.F = tern_op_b((in_i_temp & 0x80) != 0, FLAG_N, 0)
            | tern_op_b(in_i_temp2 < in_i_temp, FLAG_H | FLAG_C, 0)
            | tern_op_b(
                self.tables.parity_table[((in_i_temp2 & 0x07) ^ self.data.B) as usize] != 0,
                FLAG_P,
                0,
            )
            | self.tables.sz53_table[self.data.B as usize];
    }

    /* OUTI */
    pub(crate) fn instrED__OUTI(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        let out_i_temp: u8 = self.memory.read_byte(self.HL());
        self.data.B = self.data.B.wrapping_sub(1); /* This does happen first, despite what the specs say */
        self.write_port(self.BC(), out_i_temp);

        self.IncHL();
        let out_i_temp2: u8 = out_i_temp.wrapping_add(self.data.L); // + self.data.L;
        self.data.F = tern_op_b((out_i_temp & 0x80) != 0, FLAG_N, 0)
            | tern_op_b(out_i_temp2 < out_i_temp, FLAG_H | FLAG_C, 0)
            | tern_op_b(
                self.tables.parity_table[((out_i_temp2 & 0x07) ^ self.data.B) as usize] != 0,
                FLAG_P,
                0,
            )
            | self.tables.sz53_table[self.data.B as usize];
    }

    /* LDD */
    pub(crate) fn instrED__LDD(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.DecBC();
        self.memory.write_byte(self.DE(), byte_temp);
        self.memory.contend_write_no_mreq_loop(self.DE(), 1, 2);
        self.DecDE();
        self.DecHL();
        byte_temp += self.data.A;
        self.data.F = self.data.F & (FLAG_C | FLAG_Z | FLAG_S)
            | tern_op_b(self.BC() != 0, FLAG_V, 0)
            | byte_temp & FLAG_3
            | tern_op_b((byte_temp & 0x02) != 0, FLAG_5, 0);
    }

    /* CPD */
    pub(crate) fn instrED__CPD(&mut self) {
        let value: u8 = self.memory.read_byte(self.HL());
        let mut byte_temp: u8 = self.data.A - value;
        let lookup: u8 =
            ((self.data.A & 0x08) >> 3) | ((value & 0x08) >> 2) | ((byte_temp & 0x08) >> 1);
        self.memory.contend_read_no_mreq_loop(self.HL(), 1, 5);
        self.DecHL();
        self.DecBC();
        self.data.F = self.data.F & FLAG_C
            | tern_op_b(self.BC() != 0, FLAG_V | FLAG_N, FLAG_N)
            | HALF_CARRY_SUB_TABLE[lookup as usize]
            | tern_op_b(byte_temp != 0, 0, FLAG_Z)
            | byte_temp & FLAG_S;
        if (self.data.F & FLAG_H) != 0 {
            byte_temp -= 1;
        }
        self.data.F |= (byte_temp & FLAG_3) | tern_op_b((byte_temp & 0x02) != 0, FLAG_5, 0)
    }

    /* IND */
    pub(crate) fn instrED__IND(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        let in_i_temp: u8 = self.read_port(self.BC());
        self.memory.write_byte(self.HL(), in_i_temp);

        self.data.B = self.data.B.wrapping_sub(1);
        self.DecHL();
        let in_i_temp2: u8 = in_i_temp + self.data.C - 1;
        self.data.F = tern_op_b((in_i_temp & 0x80) != 0, FLAG_N, 0)
            | tern_op_b(in_i_temp2 < in_i_temp, FLAG_H | FLAG_C, 0)
            | tern_op_b(
                self.tables.parity_table[((in_i_temp2 & 0x07) ^ self.data.B) as usize] != 0,
                FLAG_P,
                0,
            )
            | self.tables.sz53_table[self.data.B as usize]
    }

    /* OUTD */
    pub(crate) fn instrED__OUTD(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        let out_i_temp: u8 = self.memory.read_byte(self.HL());
        self.data.B = self.data.B.wrapping_sub(1); /* This does happen first, despite what the specs say */
        self.write_port(self.BC(), out_i_temp);

        self.DecHL();
        let out_i_temp2: u8 = out_i_temp + self.data.L;
        self.data.F = tern_op_b((out_i_temp & 0x80) != 0, FLAG_N, 0)
            | tern_op_b(out_i_temp2 < out_i_temp, FLAG_H | FLAG_C, 0)
            | tern_op_b(
                self.tables.parity_table[((out_i_temp2 & 0x07) ^ self.data.B) as usize] != 0,
                FLAG_P,
                0,
            )
            | self.tables.sz53_table[self.data.B as usize];
    }

    /* LDIR */
    pub(crate) fn instrED__LDIR(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.write_byte(self.DE(), byte_temp);
        self.memory.contend_write_no_mreq_loop(self.DE(), 1, 2);
        self.DecBC();
        byte_temp = byte_temp.wrapping_add(self.data.A);
        self.data.F = self.data.F & (FLAG_C | FLAG_Z | FLAG_S)
            | tern_op_b(self.BC() != 0, FLAG_V, 0)
            | byte_temp & FLAG_3
            | tern_op_b(byte_temp & 0x02 != 0, FLAG_5, 0);
        if self.BC() != 0 {
            self.memory.contend_write_no_mreq_loop(self.DE(), 1, 5);
            self.DecPC(2);
            self.data.cycles += 23;
        } else {
            self.data.cycles += 18;
        }
        self.IncHL();
        self.IncDE();
    }

    /* CPIR */
    pub(crate) fn instrED__CPIR(&mut self) {
        let value: u8 = self.memory.read_byte(self.HL());
        let mut byte_temp: u8 = self.data.A - value;
        let lookup: u8 =
            ((self.data.A & 0x08) >> 3) | ((value & 0x08) >> 2) | ((byte_temp & 0x08) >> 1);
        self.memory.contend_read_no_mreq_loop(self.HL(), 1, 5);
        self.DecBC();
        self.data.F = self.data.F & FLAG_C
            | tern_op_b(self.BC() != 0, FLAG_V | FLAG_N, FLAG_N)
            | HALF_CARRY_SUB_TABLE[lookup as usize]
            | tern_op_b(byte_temp != 0, 0, FLAG_Z)
            | byte_temp & FLAG_S;
        if self.data.F & FLAG_H != 0 {
            byte_temp -= 1;
        }
        self.data.F |= (byte_temp & FLAG_3) | tern_op_b((byte_temp & 0x02) != 0, FLAG_5, 0);
        if (self.data.F & (FLAG_V | FLAG_Z)) == FLAG_V {
            self.memory.contend_read_no_mreq_loop(self.HL(), 1, 5);
            self.DecPC(2);
            self.data.cycles += 18;
        } else {
            self.data.cycles += 23;
        }
        self.IncHL();
    }

    /* INIR */
    pub(crate) fn instrED__INIR(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        let in_i_temp: u8 = self.read_port(self.BC());
        self.memory.write_byte(self.HL(), in_i_temp);

        self.data.B = self.data.B.wrapping_sub(1);
        let in_i_temp2: u8 = in_i_temp + self.data.C + 1;
        self.data.F = tern_op_b(in_i_temp & 0x80 != 0, FLAG_N, 0)
            | tern_op_b(in_i_temp2 < in_i_temp, FLAG_H | FLAG_C, 0)
            | tern_op_b(
                self.tables.parity_table[((in_i_temp2 & 0x07) ^ self.data.B) as usize] != 0,
                FLAG_P,
                0,
            )
            | self.tables.sz53_table[self.data.B as usize];

        if self.data.B != 0 {
            self.memory.contend_write_no_mreq_loop(self.HL(), 1, 5);
            self.DecPC(2);
            self.data.cycles += 23;
        } else {
            self.data.cycles += 18;
        }
        self.IncHL();
    }

    /* OTIR */
    pub(crate) fn instrED__OTIR(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        let out_i_temp: u8 = self.memory.read_byte(self.HL());
        self.data.B = self.data.B.wrapping_sub(1); /* This does happen first, despite what the specs say */
        self.write_port(self.BC(), out_i_temp);

        self.IncHL();
        let out_i_temp2: u8 = out_i_temp.wrapping_add(self.data.L);

        self.data.F = tern_op_b((out_i_temp & 0x80) != 0, FLAG_N, 0)
            | tern_op_b(out_i_temp2 < out_i_temp, FLAG_H | FLAG_C, 0)
            | tern_op_b(
                self.tables.parity_table[((out_i_temp2 & 0x07) ^ self.data.B) as usize] != 0,
                FLAG_P,
                0,
            )
            | self.tables.sz53_table[self.data.B as usize];

        if self.data.B != 0 {
            self.memory.contend_read_no_mreq_loop(self.BC(), 1, 5);
            self.DecPC(2);
            self.data.cycles += 23;
        } else {
            self.data.cycles += 18;
        }
    }

    /* LDDR */
    pub(crate) fn instrED__LDDR(&mut self) {
        let mut byte_temp: u8 = self.memory.read_byte(self.HL());
        self.memory.write_byte(self.DE(), byte_temp);
        self.memory.contend_write_no_mreq_loop(self.DE(), 1, 2);
        self.DecBC();
        byte_temp = byte_temp.wrapping_add(self.data.A);
        self.data.F = self.data.F & (FLAG_C | FLAG_Z | FLAG_S)
            | tern_op_b(self.BC() != 0, FLAG_V, 0)
            | byte_temp & FLAG_3
            | tern_op_b(byte_temp & 0x02 != 0, FLAG_5, 0);
        if self.BC() != 0 {
            self.memory.contend_write_no_mreq_loop(self.DE(), 1, 5);
            self.DecPC(2);
            self.data.cycles += 23;
        } else {
            self.data.cycles += 18;
        }
        self.DecHL();
        self.DecDE();
    }

    /* CPDR */
    pub(crate) fn instrED__CPDR(&mut self) {
        let value: u8 = self.memory.read_byte(self.HL());
        let mut byte_temp: u8 = self.data.A - value;
        let lookup: u8 =
            ((self.data.A & 0x08) >> 3) | ((value & 0x08) >> 2) | ((byte_temp & 0x08) >> 1);
        self.memory.contend_read_no_mreq_loop(self.HL(), 1, 5);
        self.DecBC();
        self.data.F = self.data.F & FLAG_C
            | tern_op_b(self.BC() != 0, FLAG_V | FLAG_N, FLAG_N)
            | HALF_CARRY_SUB_TABLE[lookup as usize]
            | tern_op_b(byte_temp != 0, 0, FLAG_Z)
            | byte_temp & FLAG_S;
        if self.data.F & FLAG_H != 0 {
            byte_temp -= 1;
        }
        self.data.F |= byte_temp & FLAG_3 | tern_op_b((byte_temp & 0x02) != 0, FLAG_5, 0);
        if self.data.F & (FLAG_V | FLAG_Z) == FLAG_V {
            self.memory.contend_read_no_mreq_loop(self.HL(), 1, 5);
            self.DecPC(2);
            self.data.cycles += 18;
        } else {
            self.data.cycles += 23;
        }
        self.DecHL();
    }

    /* INDR */
    pub(crate) fn instrED__INDR(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        let in_i_temp: u8 = self.read_port(self.BC());
        self.memory.write_byte(self.HL(), in_i_temp);

        self.data.B = self.data.B.wrapping_sub(1);
        let in_i_temp2: u8 = in_i_temp + self.data.C - 1;
        self.data.F = tern_op_b(in_i_temp & 0x80 != 0, FLAG_N, 0)
            | tern_op_b(in_i_temp2 < in_i_temp, FLAG_H | FLAG_C, 0)
            | tern_op_b(
                self.tables.parity_table[((in_i_temp2 & 0x07) ^ self.data.B) as usize] != 0,
                FLAG_P,
                0,
            )
            | self.tables.sz53_table[self.data.B as usize];

        if self.data.B != 0 {
            self.memory.contend_write_no_mreq_loop(self.HL(), 1, 5);
            self.DecPC(2);
            self.data.cycles += 23;
        } else {
            self.data.cycles += 18;
        }
        self.DecHL();
    }

    /* OTDR */
    pub(crate) fn instrED__OTDR(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq(_address, 1);
        let address = self.HL();
        let out_i_temp: u8 = self.memory.read_byte(address);
        self.data.B = self.data.B.wrapping_sub(1); /* This does happen first, despite what the specs say */
        self.write_port(self.BC(), out_i_temp);

        self.DecHL();
        let out_i_temp2: u8 = out_i_temp + self.data.L;
        self.data.F = tern_op_b((out_i_temp & 0x80) != 0, FLAG_N, 0)
            | tern_op_b(out_i_temp2 < out_i_temp, FLAG_H | FLAG_C, 0)
            | tern_op_b(
                self.tables.parity_table[((out_i_temp2 & 0x07) ^ self.data.B) as usize] != 0,
                FLAG_P,
                0,
            )
            | self.tables.sz53_table[self.data.B as usize];

        if self.data.B != 0 {
            self.memory.contend_read_no_mreq_loop(self.BC(), 1, 5);
            self.DecPC(2);
            self.data.cycles += 23;
        } else {
            self.data.cycles += 18;
        }
    }

    /* slttrap */
    pub(crate) fn instrED__SLTTRAP(&mut self) {
        self.slt_trap(self.HL() as i16, self.data.A);
    }

    /* ADD ix,BC */
    pub(crate) fn instrDD__ADD_REG_BC(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        // self.add16(self.ix, self.BC())
        let mut ix = Register16::new(self.data.IXH, self.data.IXL);
        let value2 = self.BC();
        self.add16(&mut ix, value2);
        (self.data.IXH, self.data.IXL) = ix.result();
    }

    /* ADD ix,DE */
    pub(crate) fn instrDD__ADD_REG_DE(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        // self.add16(self.ix, self.DE())
        let mut ix = Register16::new(self.data.IXH, self.data.IXL);
        let value2 = self.DE();
        self.add16(&mut ix, value2);
        (self.data.IXH, self.data.IXL) = ix.result();
    }

    /* LD ix,nnnn */
    pub(crate) fn instrDD__LD_REG_NNNN(&mut self) {
        let address = self.PC();
        let b1 = self.memory.read_byte(address);
        self.IncPC(1);
        let address = self.PC();
        let b2 = self.memory.read_byte(address);
        self.IncPC(1);
        self.SetIX(join_bytes(b2, b1));
    }

    /* LD (nnnn),ix */
    fn_instr_ld_i_nnnn_r16!(instrDD__LD_iNNNN_REG, IXL, IXH);

    /* INC ix */
    pub(crate) fn instrDD__INC_REG(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.IncIX()
    }

    /* INC IXH */
    pub(crate) fn instrDD__INC_REGH(&mut self) {
        self.incIXH()
    }

    /* DEC IXH */
    pub(crate) fn instrDD__DEC_REGH(&mut self) {
        self.decIXH()
    }

    /* LD IXH,nn */
    pub(crate) fn instrDD__LD_REGH_NN(&mut self) {
        let address = self.PC();
        self.data.IXH = self.memory.read_byte(address);
        self.IncPC(1);
    }

    /* ADD ix,ix */
    pub(crate) fn instrDD__ADD_REG_REG(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        // self.add16(self.ix, self.IX());
        let mut ix = Register16::new(self.data.IXH, self.data.IXL);
        let value2 = self.IX();
        self.add16(&mut ix, value2);
        (self.data.IXH, self.data.IXL) = ix.result();
    }

    /* LD ix,(nnnn) */
    fn_instr_ld_hl_i_nnnn!(instrDD__LD_REG_iNNNN, IXL, IXH);

    /* DEC ix */
    pub(crate) fn instrDD__DEC_REG(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.DecIX()
    }

    /* INC IXL */
    pub(crate) fn instrDD__INC_REGL(&mut self) {
        self.incIXL()
    }

    /* DEC IXL */
    pub(crate) fn instrDD__DEC_REGL(&mut self) {
        self.decIXL()
    }

    /* LD IXL,nn */
    pub(crate) fn instrDD__LD_REGL_NN(&mut self) {
        let address = self.PC();
        self.data.IXL = self.memory.read_byte(address);
        self.IncPC(1);
    }

    /* INC (ix+dd) */
    fn_instr_dd_op_i_reg_p_dd!(instrDD__INC_iREGpDD, inc);

    /* DEC (ix+dd) */
    fn_instr_dd_op_i_reg_p_dd!(instrDD__DEC_iREGpDD, dec);

    /* LD (ix+dd),nn */
    pub(crate) fn instrDD__LD_iREGpDD_NN(&mut self) {
        let address = self.PC();
        let offset = self.memory.read_byte(address);
        self.IncPC(1);
        let address = self.PC();
        let value = self.memory.read_byte(address);
        let _address = self.PC();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.IncPC(1);
        self.memory
            .write_byte(self.IX() + (sign_extend(offset) as u16), value);
    }

    /* ADD ix,SP */
    pub(crate) fn instrDD__ADD_REG_SP(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        // self.add16(self.ix, self.SP());
        let mut ix = Register16::new(self.data.IXH, self.data.IXL);
        let value2 = self.SP();
        self.add16(&mut ix, value2);
        (self.data.IXH, self.data.IXL) = ix.result();
    }

    /* LD B,IXH */
    pub(crate) fn instrDD__LD_B_REGH(&mut self) {
        self.data.B = self.data.IXH
    }

    /* LD B,IXL */
    pub(crate) fn instrDD__LD_B_REGL(&mut self) {
        self.data.B = self.data.IXL
    }

    /* LD B,(ix+dd) */
    fn_instr_dd_ld_r_i_reg_p_dd!(instrDD__LD_B_iREGpDD, B);

    /* LD C,IXH */
    pub(crate) fn instrDD__LD_C_REGH(&mut self) {
        self.data.C = self.data.IXH
    }

    /* LD C,IXL */
    pub(crate) fn instrDD__LD_C_REGL(&mut self) {
        self.data.C = self.data.IXL
    }

    /* LD C,(ix+dd) */
    fn_instr_dd_ld_r_i_reg_p_dd!(instrDD__LD_C_iREGpDD, C);

    /* LD D,IXH */
    pub(crate) fn instrDD__LD_D_REGH(&mut self) {
        self.data.D = self.data.IXH
    }

    /* LD D,IXL */
    pub(crate) fn instrDD__LD_D_REGL(&mut self) {
        self.data.D = self.data.IXL
    }

    /* LD D,(ix+dd) */
    fn_instr_dd_ld_r_i_reg_p_dd!(instrDD__LD_D_iREGpDD, D);

    /* LD E,IXH */
    pub(crate) fn instrDD__LD_E_REGH(&mut self) {
        self.data.E = self.data.IXH
    }

    /* LD E,IXL */
    pub(crate) fn instrDD__LD_E_REGL(&mut self) {
        self.data.E = self.data.IXL
    }

    /* LD E,(ix+dd) */
    fn_instr_dd_ld_r_i_reg_p_dd!(instrDD__LD_E_iREGpDD, E);

    /* LD IXH,B */
    pub(crate) fn instrDD__LD_REGH_B(&mut self) {
        self.data.IXH = self.data.B
    }

    /* LD IXH,C */
    pub(crate) fn instrDD__LD_REGH_C(&mut self) {
        self.data.IXH = self.data.C
    }

    /* LD IXH,D */
    pub(crate) fn instrDD__LD_REGH_D(&mut self) {
        self.data.IXH = self.data.D
    }

    /* LD IXH,E */
    pub(crate) fn instrDD__LD_REGH_E(&mut self) {
        self.data.IXH = self.data.E
    }

    /* LD IXH,IXH */
    pub(crate) fn instrDD__LD_REGH_REGH(&mut self) {}

    /* LD IXH,IXL */
    pub(crate) fn instrDD__LD_REGH_REGL(&mut self) {
        self.data.IXH = self.data.IXL
    }

    /* LD H,(ix+dd) */
    fn_instr_dd_ld_r_i_reg_p_dd!(instrDD__LD_H_iREGpDD, H);

    /* LD IXH,A */
    pub(crate) fn instrDD__LD_REGH_A(&mut self) {
        self.data.IXH = self.data.A
    }

    /* LD IXL,B */
    pub(crate) fn instrDD__LD_REGL_B(&mut self) {
        self.data.IXL = self.data.B
    }

    /* LD IXL,C */
    pub(crate) fn instrDD__LD_REGL_C(&mut self) {
        self.data.IXL = self.data.C
    }

    /* LD IXL,D */
    pub(crate) fn instrDD__LD_REGL_D(&mut self) {
        self.data.IXL = self.data.D
    }

    /* LD IXL,E */
    pub(crate) fn instrDD__LD_REGL_E(&mut self) {
        self.data.IXL = self.data.E
    }

    /* LD IXL,IXH */
    pub(crate) fn instrDD__LD_REGL_REGH(&mut self) {
        self.data.IXL = self.data.IXH
    }

    /* LD IXL,IXL */
    pub(crate) fn instrDD__LD_REGL_REGL(&mut self) {}

    /* LD L,(ix+dd) */
    fn_instr_dd_ld_r_i_reg_p_dd!(instrDD__LD_L_iREGpDD, L);

    /* LD IXL,A */
    pub(crate) fn instrDD__LD_REGL_A(&mut self) {
        self.data.IXL = self.data.A
    }

    /* LD (ix+dd),B */
    fn_instr_ld_i_reg_p_dd_r8!(instrDD__LD_iREGpDD_B, IX, B);

    /* LD (ix+dd),C */
    fn_instr_ld_i_reg_p_dd_r8!(instrDD__LD_iREGpDD_C, IX, C);

    /* LD (ix+dd),D */
    fn_instr_ld_i_reg_p_dd_r8!(instrDD__LD_iREGpDD_D, IX, D);

    /* LD (ix+dd),E */
    fn_instr_ld_i_reg_p_dd_r8!(instrDD__LD_iREGpDD_E, IX, E);

    /* LD (ix+dd),H */
    fn_instr_ld_i_reg_p_dd_r8!(instrDD__LD_iREGpDD_H, IX, H);

    /* LD (ix+dd),L */
    fn_instr_ld_i_reg_p_dd_r8!(instrDD__LD_iREGpDD_L, IX, L);

    /* LD (ix+dd),A */
    fn_instr_ld_i_reg_p_dd_r8!(instrDD__LD_iREGpDD_A, IX, A);

    /* LD A,IXH */
    fn_instr_ld_a_r8!(instrDD__LD_A_REGH, IXH);

    /* LD A,IXL */
    fn_instr_ld_a_r8!(instrDD__LD_A_REGL, IXL);

    /* LD A,(ix+dd) */
    fn_instr_dd_ld_r_i_reg_p_dd!(instrDD__LD_A_iREGpDD, A);

    /* ADD A,IXH */
    fn_instr_add_r8!(instrDD__ADD_A_REGH, IXH);

    /* ADD A,IXL */
    fn_instr_add_r8!(instrDD__ADD_A_REGL, IXL);

    /* ADD A,(ix+dd) */
    fn_instr_dd_op_a_i_reg_p_dd!(instrDD__ADD_A_iREGpDD, add);

    /* ADC A,IXH */
    fn_instr_adc_r8!(instrDD__ADC_A_REGH, IXH);

    /* ADC A,IXL */
    fn_instr_adc_r8!(instrDD__ADC_A_REGL, IXL);

    /* ADC A,(ix+dd) */
    fn_instr_dd_op_a_i_reg_p_dd!(instrDD__ADC_A_iREGpDD, adc);

    /* SUB A,IXH */
    fn_instr_sub_r8!(instrDD__SUB_A_REGH, IXH);

    /* SUB A,IXL */
    fn_instr_sub_r8!(instrDD__SUB_A_REGL, IXL);

    /* SUB A,(ix+dd) */
    fn_instr_dd_op_a_i_reg_p_dd!(instrDD__SUB_A_iREGpDD, sub);

    /* SBC A,IXH */
    fn_instr_sbc_r8!(instrDD__SBC_A_REGH, IXH);

    /* SBC A,IXL */
    fn_instr_sbc_r8!(instrDD__SBC_A_REGL, IXL);

    /* SBC A,(ix+dd) */
    fn_instr_dd_op_a_i_reg_p_dd!(instrDD__SBC_A_iREGpDD, sbc);

    /* AND A,IXH */
    fn_instr_and_r8!(instrDD__AND_A_REGH, IXH);

    /* AND A,IXL */
    fn_instr_and_r8!(instrDD__AND_A_REGL, IXL);

    /* AND A,(ix+dd) */
    fn_instr_dd_op_a_i_reg_p_dd!(instrDD__AND_A_iREGpDD, and);

    /* XOR A,IXH */
    fn_instr_xor_r8!(instrDD__XOR_A_REGH, IXH);

    /* XOR A,IXL */
    fn_instr_xor_r8!(instrDD__XOR_A_REGL, IXL);

    /* XOR A,(ix+dd) */
    fn_instr_dd_op_a_i_reg_p_dd!(instrDD__XOR_A_iREGpDD, xor);

    /* OR A,IXH */
    fn_instr_or_r8!(instrDD__OR_A_REGH, IXH);

    /* OR A,IXL */
    fn_instr_or_r8!(instrDD__OR_A_REGL, IXL);

    /* OR A,(ix+dd) */
    fn_instr_dd_op_a_i_reg_p_dd!(instrDD__OR_A_iREGpDD, or);

    /* CP A,IXH */
    fn_instr_cp_r8!(instrDD__CP_A_REGH, IXH);

    /* CP A,IXL */
    fn_instr_cp_r8!(instrDD__CP_A_REGL, IXL);

    /* CP A,(ix+dd) */
    fn_instr_dd_op_a_i_reg_p_dd!(instrDD__CP_A_iREGpDD, cp);

    /* shift DDFDCB */
    pub(crate) fn instrDD__SHIFT_DDFDCB(&mut self) {}

    /* POP ix */
    fn_instr_pop_r16!(instrDD__POP_REG, IXL, IXH);

    /* EX (SP),ix */
    pub(crate) fn instrDD__EX_iSP_REG(&mut self) {
        let address = self.SP();
        let byte_temp_l = self.memory.read_byte(address);
        let sp = self.SP();
        let byte_temp_h = self.memory.read_byte(sp + 1);
        let sp = self.SP();
        self.memory.contend_read_no_mreq(sp + 1, 1);
        let sp = self.SP();
        self.memory.write_byte(sp + 1, self.data.IXH);
        let address = self.SP();
        self.memory.write_byte(address, self.data.IXL);
        let _address = self.SP();
        self.memory.contend_write_no_mreq_loop(_address, 1, 2);
        self.data.IXL = byte_temp_l;
        self.data.IXH = byte_temp_h;
    }

    /* PUSH ix */
    fn_instr_push_r16!(instrDD__PUSH_REG, IXL, IXH);

    /* JP ix */
    pub(crate) fn instrDD__JP_REG(&mut self) {
        self.SetPC(self.IX()); /* NB: NOT INDIRECT! */
    }

    /* LD SP,ix */
    pub(crate) fn instrDD__LD_SP_REG(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.SetSP(self.IX());
    }

    /* ADD iy,BC */
    pub(crate) fn instrFD__ADD_REG_BC(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        // self.add16(self.iy, self.BC());
        let mut iy = Register16::new(self.data.IYH, self.data.IYL);
        let value2 = self.BC();
        self.add16(&mut iy, value2);
        (self.data.IYH, self.data.IYL) = iy.result();
    }

    /* ADD iy,DE */
    pub(crate) fn instrFD__ADD_REG_DE(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        // self.add16(self.iy, self.DE());
        let mut iy = Register16::new(self.data.IYH, self.data.IYL);
        let value2 = self.DE();
        self.add16(&mut iy, value2);
        (self.data.IYH, self.data.IYL) = iy.result();
    }

    /* LD iy,nnnn */
    pub(crate) fn instrFD__LD_REG_NNNN(&mut self) {
        let address = self.PC();
        let b1 = self.memory.read_byte(address);
        self.IncPC(1);
        let address = self.PC();
        let b2 = self.memory.read_byte(address);
        self.IncPC(1);
        self.SetIY(join_bytes(b2, b1));
    }

    /* LD (nnnn),iy */
    fn_instr_ld_i_nnnn_r16!(instrFD__LD_iNNNN_REG, IYL, IYH);

    /* INC iy */
    pub(crate) fn instrFD__INC_REG(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.IncIY()
    }

    /* INC IYH */
    pub(crate) fn instrFD__INC_REGH(&mut self) {
        self.incIYH()
    }

    /* DEC IYH */
    pub(crate) fn instrFD__DEC_REGH(&mut self) {
        self.decIYH()
    }

    /* LD IYH,nn */
    pub(crate) fn instrFD__LD_REGH_NN(&mut self) {
        let address = self.PC();
        self.data.IYH = self.memory.read_byte(address);
        self.IncPC(1);
    }

    /* ADD iy,iy */
    pub(crate) fn instrFD__ADD_REG_REG(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        // self.add16(self.iy, self.IY())
        let mut iy = Register16::new(self.data.IYH, self.data.IYL);
        let value2 = self.IY();
        self.add16(&mut iy, value2);
        (self.data.IYH, self.data.IYL) = iy.result();
    }

    /* LD iy,(nnnn) */
    fn_instr_ld_hl_i_nnnn!(instrFD__LD_REG_iNNNN, IYL, IYH);

    /* DEC iy */
    pub(crate) fn instrFD__DEC_REG(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.DecIY()
    }

    /* INC IYL */
    pub(crate) fn instrFD__INC_REGL(&mut self) {
        self.incIYL()
    }

    /* DEC IYL */
    pub(crate) fn instrFD__DEC_REGL(&mut self) {
        self.decIYL()
    }

    /* LD IYL,nn */
    pub(crate) fn instrFD__LD_REGL_NN(&mut self) {
        let address = self.PC();
        self.data.IYL = self.memory.read_byte(address);
        self.IncPC(1);
    }

    /* INC (iy+dd) */
    fn_instr_fd_op_i_reg_p_dd!(instrFD__INC_iREGpDD, inc);

    /* DEC (iy+dd) */
    fn_instr_fd_op_i_reg_p_dd!(instrFD__DEC_iREGpDD, dec);

    /* LD (iy+dd),nn */
    pub(crate) fn instrFD__LD_iREGpDD_NN(&mut self) {
        let address = self.PC();
        let offset = self.memory.read_byte(address);
        self.IncPC(1);
        let address = self.PC();
        let value = self.memory.read_byte(address);
        let _address = self.PC();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.IncPC(1);
        self.memory
            .write_byte(self.IY() + (sign_extend(offset) as u16), value)
    }

    /* ADD iy,SP */
    pub(crate) fn instrFD__ADD_REG_SP(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 7);
        // self.add16(self.iy, self.SP())
        let mut iy = Register16::new(self.data.IYH, self.data.IYL);
        let value2 = self.SP();
        self.add16(&mut iy, value2);
        (self.data.IYH, self.data.IYL) = iy.result();
    }

    /* LD B,IYH */
    pub(crate) fn instrFD__LD_B_REGH(&mut self) {
        self.data.B = self.data.IYH
    }

    /* LD B,IYL */
    pub(crate) fn instrFD__LD_B_REGL(&mut self) {
        self.data.B = self.data.IYL
    }

    /* LD B,(iy+dd) */
    fn_instr_fd_ld_r_i_reg_p_dd!(instrFD__LD_B_iREGpDD, B);

    /* LD C,IYH */
    pub(crate) fn instrFD__LD_C_REGH(&mut self) {
        self.data.C = self.data.IYH
    }

    /* LD C,IYL */
    pub(crate) fn instrFD__LD_C_REGL(&mut self) {
        self.data.C = self.data.IYL
    }

    /* LD C,(iy+dd) */
    fn_instr_fd_ld_r_i_reg_p_dd!(instrFD__LD_C_iREGpDD, C);

    /* LD D,IYH */
    pub(crate) fn instrFD__LD_D_REGH(&mut self) {
        self.data.D = self.data.IYH
    }

    /* LD D,IYL */
    pub(crate) fn instrFD__LD_D_REGL(&mut self) {
        self.data.D = self.data.IYL
    }

    /* LD D,(iy+dd) */
    fn_instr_fd_ld_r_i_reg_p_dd!(instrFD__LD_D_iREGpDD, D);

    /* LD E,IYH */
    pub(crate) fn instrFD__LD_E_REGH(&mut self) {
        self.data.E = self.data.IYH
    }

    /* LD E,IYL */
    pub(crate) fn instrFD__LD_E_REGL(&mut self) {
        self.data.E = self.data.IYL
    }

    /* LD E,(iy+dd) */
    fn_instr_fd_ld_r_i_reg_p_dd!(instrFD__LD_E_iREGpDD, E);

    /* LD IYH,B */
    pub(crate) fn instrFD__LD_REGH_B(&mut self) {
        self.data.IYH = self.data.B
    }

    /* LD IYH,C */
    pub(crate) fn instrFD__LD_REGH_C(&mut self) {
        self.data.IYH = self.data.C
    }

    /* LD IYH,D */
    pub(crate) fn instrFD__LD_REGH_D(&mut self) {
        self.data.IYH = self.data.D
    }

    /* LD IYH,E */
    pub(crate) fn instrFD__LD_REGH_E(&mut self) {
        self.data.IYH = self.data.E
    }

    /* LD IYH,IYH */
    pub(crate) fn instrFD__LD_REGH_REGH(&mut self) {}

    /* LD IYH,IYL */
    pub(crate) fn instrFD__LD_REGH_REGL(&mut self) {
        self.data.IYH = self.data.IYL
    }

    /* LD H,(iy+dd) */
    fn_instr_fd_ld_r_i_reg_p_dd!(instrFD__LD_H_iREGpDD, H);

    /* LD IYH,A */
    pub(crate) fn instrFD__LD_REGH_A(&mut self) {
        self.data.IYH = self.data.A
    }

    /* LD IYL,B */
    pub(crate) fn instrFD__LD_REGL_B(&mut self) {
        self.data.IYL = self.data.B
    }

    /* LD IYL,C */
    pub(crate) fn instrFD__LD_REGL_C(&mut self) {
        self.data.IYL = self.data.C
    }

    /* LD IYL,D */
    pub(crate) fn instrFD__LD_REGL_D(&mut self) {
        self.data.IYL = self.data.D
    }

    /* LD IYL,E */
    pub(crate) fn instrFD__LD_REGL_E(&mut self) {
        self.data.IYL = self.data.E
    }

    /* LD IYL,IYH */
    pub(crate) fn instrFD__LD_REGL_REGH(&mut self) {
        self.data.IYL = self.data.IYH
    }

    /* LD IYL,IYL */
    pub(crate) fn instrFD__LD_REGL_REGL(&mut self) {}

    /* LD L,(iy+dd) */
    fn_instr_fd_ld_r_i_reg_p_dd!(instrFD__LD_L_iREGpDD, L);

    /* LD IYL,A */
    pub(crate) fn instrFD__LD_REGL_A(&mut self) {
        self.data.IYL = self.data.A
    }

    /* LD (iy+dd),B */
    fn_instr_ld_i_reg_p_dd_r8!(instrFD__LD_iREGpDD_B, IY, B);

    /* LD (iy+dd),C */
    fn_instr_ld_i_reg_p_dd_r8!(instrFD__LD_iREGpDD_C, IY, C);

    /* LD (iy+dd),D */
    fn_instr_ld_i_reg_p_dd_r8!(instrFD__LD_iREGpDD_D, IY, D);

    /* LD (iy+dd),E */
    fn_instr_ld_i_reg_p_dd_r8!(instrFD__LD_iREGpDD_E, IY, E);

    /* LD (iy+dd),H */
    fn_instr_ld_i_reg_p_dd_r8!(instrFD__LD_iREGpDD_H, IY, H);

    /* LD (iy+dd),L */
    fn_instr_ld_i_reg_p_dd_r8!(instrFD__LD_iREGpDD_L, IY, L);

    /* LD (iy+dd),A */
    fn_instr_ld_i_reg_p_dd_r8!(instrFD__LD_iREGpDD_A, IY, A);

    /* LD A,IYH */
    fn_instr_ld_a_r8!(instrFD__LD_A_REGH, IYH);

    /* LD A,IYL */
    fn_instr_ld_a_r8!(instrFD__LD_A_REGL, IYL);

    /* LD A,(iy+dd) */
    fn_instr_fd_ld_r_i_reg_p_dd!(instrFD__LD_A_iREGpDD, A);

    /* ADD A,IYH */
    fn_instr_add_r8!(instrFD__ADD_A_REGH, IYH);

    /* ADD A,IYL */
    fn_instr_add_r8!(instrFD__ADD_A_REGL, IYL);

    /* ADD A,(iy+dd) */
    fn_instr_fd_op_a_i_reg_p_dd!(instrFD__ADD_A_iREGpDD, add);

    /* ADC A,IYH */
    fn_instr_adc_r8!(instrFD__ADC_A_REGH, IYH);

    /* ADC A,IYL */
    fn_instr_adc_r8!(instrFD__ADC_A_REGL, IYL);

    /* ADC A,(iy+dd) */
    fn_instr_fd_op_a_i_reg_p_dd!(instrFD__ADC_A_iREGpDD, adc);

    /* SUB A,IYH */
    fn_instr_sub_r8!(instrFD__SUB_A_REGH, IYH);

    /* SUB A,IYL */
    fn_instr_sub_r8!(instrFD__SUB_A_REGL, IYL);

    /* SUB A,(iy+dd) */
    fn_instr_fd_op_a_i_reg_p_dd!(instrFD__SUB_A_iREGpDD, sub);

    /* SBC A,IYH */
    fn_instr_sbc_r8!(instrFD__SBC_A_REGH, IYH);

    /* SBC A,IYL */
    fn_instr_sbc_r8!(instrFD__SBC_A_REGL, IYL);

    /* SBC A,(iy+dd) */
    fn_instr_fd_op_a_i_reg_p_dd!(instrFD__SBC_A_iREGpDD, sbc);

    /* AND A,IYH */
    fn_instr_and_r8!(instrFD__AND_A_REGH, IYH);

    /* AND A,IYL */
    fn_instr_and_r8!(instrFD__AND_A_REGL, IYL);

    /* AND A,(iy+dd) */
    fn_instr_fd_op_a_i_reg_p_dd!(instrFD__AND_A_iREGpDD, and);

    /* XOR A,IYH */
    fn_instr_xor_r8!(instrFD__XOR_A_REGH, IYH);

    /* XOR A,IYL */
    fn_instr_xor_r8!(instrFD__XOR_A_REGL, IYL);

    /* XOR A,(iy+dd) */
    fn_instr_fd_op_a_i_reg_p_dd!(instrFD__XOR_A_iREGpDD, xor);

    /* OR A,IYH */
    fn_instr_or_r8!(instrFD__OR_A_REGH, IYH);

    /* OR A,IYL */
    fn_instr_or_r8!(instrFD__OR_A_REGL, IYL);

    /* OR A,(iy+dd) */
    fn_instr_fd_op_a_i_reg_p_dd!(instrFD__OR_A_iREGpDD, or);

    /* CP A,IYH */
    fn_instr_cp_r8!(instrFD__CP_A_REGH, IYH);

    /* CP A,IYL */
    fn_instr_cp_r8!(instrFD__CP_A_REGL, IYL);

    /* CP A,(iy+dd) */
    fn_instr_fd_op_a_i_reg_p_dd!(instrFD__CP_A_iREGpDD, cp);

    /* shift DDFDCB */
    pub(crate) fn instrFD__SHIFT_DDFDCB(&mut self) {}

    /* POP iy */
    fn_instr_pop_r16!(instrFD__POP_REG, IYL, IYH);

    /* EX (SP),iy */
    pub(crate) fn instrFD__EX_iSP_REG(&mut self) {
        let address = self.SP();
        let byte_temp_l = self.memory.read_byte(address);
        let address = self.SP() + 1;
        let byte_temp_h = self.memory.read_byte(address);
        let _address = self.SP() + 1;
        self.memory.contend_read_no_mreq(_address, 1);
        let address = self.SP() + 1;
        self.memory.write_byte(address, self.data.IYH);
        let address = self.SP();
        self.memory.write_byte(address, self.data.IYL);
        let _address = self.SP();
        self.memory.contend_write_no_mreq_loop(_address, 1, 2);
        self.data.IYL = byte_temp_l;
        self.data.IYH = byte_temp_h;
    }

    /* PUSH iy */
    fn_instr_push_r16!(instrFD__PUSH_REG, IYL, IYH);

    /* JP iy */
    pub(crate) fn instrFD__JP_REG(&mut self) {
        self.SetPC(self.IY()); /* NB: NOT INDIRECT! */
    }

    /* LD SP,iy */
    pub(crate) fn instrFD__LD_SP_REG(&mut self) {
        let _address = self.IR();
        self.memory.contend_read_no_mreq_loop(_address, 1, 2);
        self.SetSP(self.IY());
    }

    /* LD B,RLC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rlc_i_reg_p_dd!(instrDDCB__LD_B_RLC_iREGpDD, B);

    /* LD C,RLC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rlc_i_reg_p_dd!(instrDDCB__LD_C_RLC_iREGpDD, C);

    /* LD D,RLC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rlc_i_reg_p_dd!(instrDDCB__LD_D_RLC_iREGpDD, D);

    /* LD E,RLC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rlc_i_reg_p_dd!(instrDDCB__LD_E_RLC_iREGpDD, E);

    /* LD H,RLC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rlc_i_reg_p_dd!(instrDDCB__LD_H_RLC_iREGpDD, H);

    /* LD L,RLC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rlc_i_reg_p_dd!(instrDDCB__LD_L_RLC_iREGpDD, L);

    /* RLC (REGISTER+dd) */
    fn_instr_ddcb_op_i_reg_p_dd!(instrDDCB__RLC_iREGpDD, rlc);

    /* LD A,RLC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rlc_i_reg_p_dd!(instrDDCB__LD_A_RLC_iREGpDD, A);

    /* LD B,RRC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rrc_i_reg_p_dd!(instrDDCB__LD_B_RRC_iREGpDD, B);

    /* LD C,RRC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rrc_i_reg_p_dd!(instrDDCB__LD_C_RRC_iREGpDD, C);

    /* LD D,RRC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rrc_i_reg_p_dd!(instrDDCB__LD_D_RRC_iREGpDD, D);

    /* LD E,RRC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rrc_i_reg_p_dd!(instrDDCB__LD_E_RRC_iREGpDD, E);

    /* LD H,RRC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rrc_i_reg_p_dd!(instrDDCB__LD_H_RRC_iREGpDD, H);

    /* LD L,RRC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rrc_i_reg_p_dd!(instrDDCB__LD_L_RRC_iREGpDD, L);

    /* RRC (REGISTER+dd) */
    fn_instr_ddcb_op_i_reg_p_dd!(instrDDCB__RRC_iREGpDD, rrc);

    /* LD A,RRC (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rrc_i_reg_p_dd!(instrDDCB__LD_A_RRC_iREGpDD, A);

    /* LD B,RL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rl_i_reg_p_dd!(instrDDCB__LD_B_RL_iREGpDD, B);

    /* LD C,RL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rl_i_reg_p_dd!(instrDDCB__LD_C_RL_iREGpDD, C);

    /* LD D,RL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rl_i_reg_p_dd!(instrDDCB__LD_D_RL_iREGpDD, D);

    /* LD E,RL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rl_i_reg_p_dd!(instrDDCB__LD_E_RL_iREGpDD, E);

    /* LD H,RL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rl_i_reg_p_dd!(instrDDCB__LD_H_RL_iREGpDD, H);

    /* LD L,RL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rl_i_reg_p_dd!(instrDDCB__LD_L_RL_iREGpDD, L);

    /* RL (REGISTER+dd) */
    fn_instr_ddcb_op_i_reg_p_dd!(instrDDCB__RL_iREGpDD, rl);

    /* LD A,RL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rl_i_reg_p_dd!(instrDDCB__LD_A_RL_iREGpDD, A);

    /* LD B,RR (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rr_i_reg_p_dd!(instrDDCB__LD_B_RR_iREGpDD, B);

    /* LD C,RR (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rr_i_reg_p_dd!(instrDDCB__LD_C_RR_iREGpDD, C);

    /* LD D,RR (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rr_i_reg_p_dd!(instrDDCB__LD_D_RR_iREGpDD, D);

    /* LD E,RR (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rr_i_reg_p_dd!(instrDDCB__LD_E_RR_iREGpDD, E);

    /* LD H,RR (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rr_i_reg_p_dd!(instrDDCB__LD_H_RR_iREGpDD, H);

    /* LD L,RR (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rr_i_reg_p_dd!(instrDDCB__LD_L_RR_iREGpDD, L);

    /* RR (REGISTER+dd) */
    fn_instr_ddcb_op_i_reg_p_dd!(instrDDCB__RR_iREGpDD, rr);

    /* LD A,RR (REGISTER+dd) */
    fn_instr_ddcb_ld_r_rr_i_reg_p_dd!(instrDDCB__LD_A_RR_iREGpDD, A);

    /* LD B,SLA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sla_i_reg_p_dd!(instrDDCB__LD_B_SLA_iREGpDD, B);

    /* LD C,SLA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sla_i_reg_p_dd!(instrDDCB__LD_C_SLA_iREGpDD, C);

    /* LD D,SLA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sla_i_reg_p_dd!(instrDDCB__LD_D_SLA_iREGpDD, D);

    /* LD E,SLA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sla_i_reg_p_dd!(instrDDCB__LD_E_SLA_iREGpDD, E);

    /* LD H,SLA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sla_i_reg_p_dd!(instrDDCB__LD_H_SLA_iREGpDD, H);

    /* LD L,SLA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sla_i_reg_p_dd!(instrDDCB__LD_L_SLA_iREGpDD, L);

    /* SLA (REGISTER+dd) */
    fn_instr_ddcb_op_i_reg_p_dd!(instrDDCB__SLA_iREGpDD, sla);

    /* LD A,SLA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sla_i_reg_p_dd!(instrDDCB__LD_A_SLA_iREGpDD, A);

    /* LD B,SRA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sra_i_reg_p_dd!(instrDDCB__LD_B_SRA_iREGpDD, B);

    /* LD C,SRA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sra_i_reg_p_dd!(instrDDCB__LD_C_SRA_iREGpDD, C);

    /* LD D,SRA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sra_i_reg_p_dd!(instrDDCB__LD_D_SRA_iREGpDD, D);

    /* LD E,SRA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sra_i_reg_p_dd!(instrDDCB__LD_E_SRA_iREGpDD, E);

    /* LD H,SRA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sra_i_reg_p_dd!(instrDDCB__LD_H_SRA_iREGpDD, H);

    /* LD L,SRA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sra_i_reg_p_dd!(instrDDCB__LD_L_SRA_iREGpDD, L);

    /* SRA (REGISTER+dd) */
    fn_instr_ddcb_op_i_reg_p_dd!(instrDDCB__SRA_iREGpDD, sra);

    /* LD A,SRA (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sra_i_reg_p_dd!(instrDDCB__LD_A_SRA_iREGpDD, A);

    /* LD B,SLL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sll_i_reg_p_dd!(instrDDCB__LD_B_SLL_iREGpDD, B);

    /* LD C,SLL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sll_i_reg_p_dd!(instrDDCB__LD_C_SLL_iREGpDD, C);

    /* LD D,SLL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sll_i_reg_p_dd!(instrDDCB__LD_D_SLL_iREGpDD, D);

    /* LD E,SLL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sll_i_reg_p_dd!(instrDDCB__LD_E_SLL_iREGpDD, E);

    /* LD H,SLL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sll_i_reg_p_dd!(instrDDCB__LD_H_SLL_iREGpDD, H);

    /* LD L,SLL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sll_i_reg_p_dd!(instrDDCB__LD_L_SLL_iREGpDD, L);

    /* SLL (REGISTER+dd) */
    fn_instr_ddcb_op_i_reg_p_dd!(instrDDCB__SLL_iREGpDD, sll);

    /* LD A,SLL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_sll_i_reg_p_dd!(instrDDCB__LD_A_SLL_iREGpDD, A);

    /* LD B,SRL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_srl_i_reg_p_dd!(instrDDCB__LD_B_SRL_iREGpDD, B);

    /* LD C,SRL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_srl_i_reg_p_dd!(instrDDCB__LD_C_SRL_iREGpDD, C);

    /* LD D,SRL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_srl_i_reg_p_dd!(instrDDCB__LD_D_SRL_iREGpDD, D);

    /* LD E,SRL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_srl_i_reg_p_dd!(instrDDCB__LD_E_SRL_iREGpDD, E);

    /* LD H,SRL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_srl_i_reg_p_dd!(instrDDCB__LD_H_SRL_iREGpDD, H);

    /* LD L,SRL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_srl_i_reg_p_dd!(instrDDCB__LD_L_SRL_iREGpDD, L);

    /* SRL (REGISTER+dd) */
    fn_instr_ddcb_op_i_reg_p_dd!(instrDDCB__SRL_iREGpDD, srl);

    /* LD A,SRL (REGISTER+dd) */
    fn_instr_ddcb_ld_r_srl_i_reg_p_dd!(instrDDCB__LD_A_SRL_iREGpDD, A);

    /* BIT 0,(REGISTER+dd) */
    fn_instr_ddcb_bit_n_i_reg_p_dd!(instrDDCB__BIT_0_iREGpDD, 0);

    /* BIT 1,(REGISTER+dd) */
    fn_instr_ddcb_bit_n_i_reg_p_dd!(instrDDCB__BIT_1_iREGpDD, 1);

    /* BIT 2,(REGISTER+dd) */
    fn_instr_ddcb_bit_n_i_reg_p_dd!(instrDDCB__BIT_2_iREGpDD, 2);

    /* BIT 3,(REGISTER+dd) */
    fn_instr_ddcb_bit_n_i_reg_p_dd!(instrDDCB__BIT_3_iREGpDD, 3);

    /* BIT 4,(REGISTER+dd) */
    fn_instr_ddcb_bit_n_i_reg_p_dd!(instrDDCB__BIT_4_iREGpDD, 4);

    /* BIT 5,(REGISTER+dd) */
    fn_instr_ddcb_bit_n_i_reg_p_dd!(instrDDCB__BIT_5_iREGpDD, 5);

    /* BIT 6,(REGISTER+dd) */
    fn_instr_ddcb_bit_n_i_reg_p_dd!(instrDDCB__BIT_6_iREGpDD, 6);

    /* BIT 7,(REGISTER+dd) */
    fn_instr_ddcb_bit_n_i_reg_p_dd!(instrDDCB__BIT_7_iREGpDD, 7);

    /* LD B,RES 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_0_i_reg_p_dd!(instrDDCB__LD_B_RES_0_iREGpDD, B);

    /* LD C,RES 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_0_i_reg_p_dd!(instrDDCB__LD_C_RES_0_iREGpDD, C);

    /* LD D,RES 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_0_i_reg_p_dd!(instrDDCB__LD_D_RES_0_iREGpDD, D);

    /* LD E,RES 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_0_i_reg_p_dd!(instrDDCB__LD_E_RES_0_iREGpDD, E);

    /* LD H,RES 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_0_i_reg_p_dd!(instrDDCB__LD_H_RES_0_iREGpDD, H);

    /* LD L,RES 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_0_i_reg_p_dd!(instrDDCB__LD_L_RES_0_iREGpDD, L);

    /* RES 0,(REGISTER+dd) */
    fn_instr_ddcb_res_n_i_reg_p_dd!(instrDDCB__RES_0_iREGpDD, 0xfe);

    /* LD A,RES 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_0_i_reg_p_dd!(instrDDCB__LD_A_RES_0_iREGpDD, A);

    /* LD B,RES 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_1_i_reg_p_dd!(instrDDCB__LD_B_RES_1_iREGpDD, B);

    /* LD C,RES 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_1_i_reg_p_dd!(instrDDCB__LD_C_RES_1_iREGpDD, C);

    /* LD D,RES 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_1_i_reg_p_dd!(instrDDCB__LD_D_RES_1_iREGpDD, D);

    /* LD E,RES 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_1_i_reg_p_dd!(instrDDCB__LD_E_RES_1_iREGpDD, E);

    /* LD H,RES 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_1_i_reg_p_dd!(instrDDCB__LD_H_RES_1_iREGpDD, H);

    /* LD L,RES 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_1_i_reg_p_dd!(instrDDCB__LD_L_RES_1_iREGpDD, L);

    /* RES 1,(REGISTER+dd) */
    fn_instr_ddcb_res_n_i_reg_p_dd!(instrDDCB__RES_1_iREGpDD, 0xfd);

    /* LD A,RES 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_1_i_reg_p_dd!(instrDDCB__LD_A_RES_1_iREGpDD, A);

    /* LD B,RES 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_2_i_reg_p_dd!(instrDDCB__LD_B_RES_2_iREGpDD, B);

    /* LD C,RES 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_2_i_reg_p_dd!(instrDDCB__LD_C_RES_2_iREGpDD, C);

    /* LD D,RES 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_2_i_reg_p_dd!(instrDDCB__LD_D_RES_2_iREGpDD, D);

    /* LD E,RES 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_2_i_reg_p_dd!(instrDDCB__LD_E_RES_2_iREGpDD, E);

    /* LD H,RES 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_2_i_reg_p_dd!(instrDDCB__LD_H_RES_2_iREGpDD, H);

    /* LD L,RES 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_2_i_reg_p_dd!(instrDDCB__LD_L_RES_2_iREGpDD, L);

    /* RES 2,(REGISTER+dd) */
    fn_instr_ddcb_res_n_i_reg_p_dd!(instrDDCB__RES_2_iREGpDD, 0xfb);

    /* LD A,RES 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_2_i_reg_p_dd!(instrDDCB__LD_A_RES_2_iREGpDD, A);

    /* LD B,RES 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_3_i_reg_p_dd!(instrDDCB__LD_B_RES_3_iREGpDD, B);

    /* LD C,RES 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_3_i_reg_p_dd!(instrDDCB__LD_C_RES_3_iREGpDD, C);

    /* LD D,RES 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_3_i_reg_p_dd!(instrDDCB__LD_D_RES_3_iREGpDD, D);

    /* LD E,RES 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_3_i_reg_p_dd!(instrDDCB__LD_E_RES_3_iREGpDD, E);

    /* LD H,RES 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_3_i_reg_p_dd!(instrDDCB__LD_H_RES_3_iREGpDD, H);

    /* LD L,RES 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_3_i_reg_p_dd!(instrDDCB__LD_L_RES_3_iREGpDD, L);

    /* RES 3,(REGISTER+dd) */
    fn_instr_ddcb_res_n_i_reg_p_dd!(instrDDCB__RES_3_iREGpDD, 0xf7);

    /* LD A,RES 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_3_i_reg_p_dd!(instrDDCB__LD_A_RES_3_iREGpDD, A);

    /* LD B,RES 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_4_i_reg_p_dd!(instrDDCB__LD_B_RES_4_iREGpDD, B);

    /* LD C,RES 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_4_i_reg_p_dd!(instrDDCB__LD_C_RES_4_iREGpDD, C);

    /* LD D,RES 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_4_i_reg_p_dd!(instrDDCB__LD_D_RES_4_iREGpDD, D);

    /* LD E,RES 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_4_i_reg_p_dd!(instrDDCB__LD_E_RES_4_iREGpDD, E);

    /* LD H,RES 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_4_i_reg_p_dd!(instrDDCB__LD_H_RES_4_iREGpDD, H);

    /* LD L,RES 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_4_i_reg_p_dd!(instrDDCB__LD_L_RES_4_iREGpDD, L);

    /* RES 4,(REGISTER+dd) */
    fn_instr_ddcb_res_n_i_reg_p_dd!(instrDDCB__RES_4_iREGpDD, 0xef);

    /* LD A,RES 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_4_i_reg_p_dd!(instrDDCB__LD_A_RES_4_iREGpDD, A);

    /* LD B,RES 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_5_i_reg_p_dd!(instrDDCB__LD_B_RES_5_iREGpDD, B);

    /* LD C,RES 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_5_i_reg_p_dd!(instrDDCB__LD_C_RES_5_iREGpDD, C);

    /* LD D,RES 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_5_i_reg_p_dd!(instrDDCB__LD_D_RES_5_iREGpDD, D);

    /* LD E,RES 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_5_i_reg_p_dd!(instrDDCB__LD_E_RES_5_iREGpDD, E);

    /* LD H,RES 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_5_i_reg_p_dd!(instrDDCB__LD_H_RES_5_iREGpDD, H);

    /* LD L,RES 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_5_i_reg_p_dd!(instrDDCB__LD_L_RES_5_iREGpDD, L);

    /* RES 5,(REGISTER+dd) */
    fn_instr_ddcb_res_n_i_reg_p_dd!(instrDDCB__RES_5_iREGpDD, 0xdf);

    /* LD A,RES 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_5_i_reg_p_dd!(instrDDCB__LD_A_RES_5_iREGpDD, A);

    /* LD B,RES 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_6_i_reg_p_dd!(instrDDCB__LD_B_RES_6_iREGpDD, B);

    /* LD C,RES 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_6_i_reg_p_dd!(instrDDCB__LD_C_RES_6_iREGpDD, C);

    /* LD D,RES 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_6_i_reg_p_dd!(instrDDCB__LD_D_RES_6_iREGpDD, D);

    /* LD E,RES 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_6_i_reg_p_dd!(instrDDCB__LD_E_RES_6_iREGpDD, E);

    /* LD H,RES 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_6_i_reg_p_dd!(instrDDCB__LD_H_RES_6_iREGpDD, H);

    /* LD L,RES 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_6_i_reg_p_dd!(instrDDCB__LD_L_RES_6_iREGpDD, L);

    /* RES 6,(REGISTER+dd) */
    fn_instr_ddcb_res_n_i_reg_p_dd!(instrDDCB__RES_6_iREGpDD, 0xbf);

    /* LD A,RES 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_6_i_reg_p_dd!(instrDDCB__LD_A_RES_6_iREGpDD, A);

    /* LD B,RES 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_7_i_reg_p_dd!(instrDDCB__LD_B_RES_7_iREGpDD, B);

    /* LD C,RES 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_7_i_reg_p_dd!(instrDDCB__LD_C_RES_7_iREGpDD, C);

    /* LD D,RES 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_7_i_reg_p_dd!(instrDDCB__LD_D_RES_7_iREGpDD, D);

    /* LD E,RES 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_7_i_reg_p_dd!(instrDDCB__LD_E_RES_7_iREGpDD, E);

    /* LD H,RES 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_7_i_reg_p_dd!(instrDDCB__LD_H_RES_7_iREGpDD, H);

    /* LD L,RES 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_7_i_reg_p_dd!(instrDDCB__LD_L_RES_7_iREGpDD, L);

    /* RES 7,(REGISTER+dd) */
    fn_instr_ddcb_res_n_i_reg_p_dd!(instrDDCB__RES_7_iREGpDD, 0x7f);

    /* LD A,RES 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_res_7_i_reg_p_dd!(instrDDCB__LD_A_RES_7_iREGpDD, A);

    /* LD B,SET 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_0_i_reg_p_dd!(instrDDCB__LD_B_SET_0_iREGpDD, B);

    /* LD C,SET 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_0_i_reg_p_dd!(instrDDCB__LD_C_SET_0_iREGpDD, C);

    /* LD D,SET 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_0_i_reg_p_dd!(instrDDCB__LD_D_SET_0_iREGpDD, D);

    /* LD E,SET 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_0_i_reg_p_dd!(instrDDCB__LD_E_SET_0_iREGpDD, E);

    /* LD H,SET 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_0_i_reg_p_dd!(instrDDCB__LD_H_SET_0_iREGpDD, H);

    /* LD L,SET 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_0_i_reg_p_dd!(instrDDCB__LD_L_SET_0_iREGpDD, L);

    /* SET 0,(REGISTER+dd) */
    fn_instr_ddcb_set_n_i_reg_p_dd!(instrDDCB__SET_0_iREGpDD, 0x01);

    /* LD A,SET 0,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_0_i_reg_p_dd!(instrDDCB__LD_A_SET_0_iREGpDD, A);

    /* LD B,SET 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_1_i_reg_p_dd!(instrDDCB__LD_B_SET_1_iREGpDD, B);

    /* LD C,SET 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_1_i_reg_p_dd!(instrDDCB__LD_C_SET_1_iREGpDD, C);

    /* LD D,SET 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_1_i_reg_p_dd!(instrDDCB__LD_D_SET_1_iREGpDD, D);

    /* LD E,SET 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_1_i_reg_p_dd!(instrDDCB__LD_E_SET_1_iREGpDD, E);

    /* LD H,SET 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_1_i_reg_p_dd!(instrDDCB__LD_H_SET_1_iREGpDD, H);

    /* LD L,SET 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_1_i_reg_p_dd!(instrDDCB__LD_L_SET_1_iREGpDD, L);

    /* SET 1,(REGISTER+dd) */
    fn_instr_ddcb_set_n_i_reg_p_dd!(instrDDCB__SET_1_iREGpDD, 0x02);

    /* LD A,SET 1,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_1_i_reg_p_dd!(instrDDCB__LD_A_SET_1_iREGpDD, A);

    /* LD B,SET 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_2_i_reg_p_dd!(instrDDCB__LD_B_SET_2_iREGpDD, B);

    /* LD C,SET 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_2_i_reg_p_dd!(instrDDCB__LD_C_SET_2_iREGpDD, C);

    /* LD D,SET 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_2_i_reg_p_dd!(instrDDCB__LD_D_SET_2_iREGpDD, D);

    /* LD E,SET 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_2_i_reg_p_dd!(instrDDCB__LD_E_SET_2_iREGpDD, E);

    /* LD H,SET 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_2_i_reg_p_dd!(instrDDCB__LD_H_SET_2_iREGpDD, H);

    /* LD L,SET 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_2_i_reg_p_dd!(instrDDCB__LD_L_SET_2_iREGpDD, L);

    /* SET 2,(REGISTER+dd) */
    fn_instr_ddcb_set_n_i_reg_p_dd!(instrDDCB__SET_2_iREGpDD, 0x04);

    /* LD A,SET 2,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_2_i_reg_p_dd!(instrDDCB__LD_A_SET_2_iREGpDD, A);

    /* LD B,SET 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_3_i_reg_p_dd!(instrDDCB__LD_B_SET_3_iREGpDD, B);

    /* LD C,SET 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_3_i_reg_p_dd!(instrDDCB__LD_C_SET_3_iREGpDD, C);

    /* LD D,SET 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_3_i_reg_p_dd!(instrDDCB__LD_D_SET_3_iREGpDD, D);

    /* LD E,SET 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_3_i_reg_p_dd!(instrDDCB__LD_E_SET_3_iREGpDD, E);

    /* LD H,SET 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_3_i_reg_p_dd!(instrDDCB__LD_H_SET_3_iREGpDD, H);

    /* LD L,SET 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_3_i_reg_p_dd!(instrDDCB__LD_L_SET_3_iREGpDD, L);

    /* SET 3,(REGISTER+dd) */
    fn_instr_ddcb_set_n_i_reg_p_dd!(instrDDCB__SET_3_iREGpDD, 0x08);

    /* LD A,SET 3,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_3_i_reg_p_dd!(instrDDCB__LD_A_SET_3_iREGpDD, A);

    /* LD B,SET 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_4_i_reg_p_dd!(instrDDCB__LD_B_SET_4_iREGpDD, B);

    /* LD C,SET 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_4_i_reg_p_dd!(instrDDCB__LD_C_SET_4_iREGpDD, C);

    /* LD D,SET 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_4_i_reg_p_dd!(instrDDCB__LD_D_SET_4_iREGpDD, D);

    /* LD E,SET 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_4_i_reg_p_dd!(instrDDCB__LD_E_SET_4_iREGpDD, E);

    /* LD H,SET 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_4_i_reg_p_dd!(instrDDCB__LD_H_SET_4_iREGpDD, H);

    /* LD L,SET 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_4_i_reg_p_dd!(instrDDCB__LD_L_SET_4_iREGpDD, L);

    /* SET 4,(REGISTER+dd) */
    fn_instr_ddcb_set_n_i_reg_p_dd!(instrDDCB__SET_4_iREGpDD, 0x10);

    /* LD A,SET 4,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_4_i_reg_p_dd!(instrDDCB__LD_A_SET_4_iREGpDD, A);

    /* LD B,SET 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_5_i_reg_p_dd!(instrDDCB__LD_B_SET_5_iREGpDD, B);

    /* LD C,SET 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_5_i_reg_p_dd!(instrDDCB__LD_C_SET_5_iREGpDD, C);

    /* LD D,SET 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_5_i_reg_p_dd!(instrDDCB__LD_D_SET_5_iREGpDD, D);

    /* LD E,SET 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_5_i_reg_p_dd!(instrDDCB__LD_E_SET_5_iREGpDD, E);

    /* LD H,SET 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_5_i_reg_p_dd!(instrDDCB__LD_H_SET_5_iREGpDD, H);

    /* LD L,SET 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_5_i_reg_p_dd!(instrDDCB__LD_L_SET_5_iREGpDD, L);

    /* SET 5,(REGISTER+dd) */
    fn_instr_ddcb_set_n_i_reg_p_dd!(instrDDCB__SET_5_iREGpDD, 0x20);

    /* LD A,SET 5,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_5_i_reg_p_dd!(instrDDCB__LD_A_SET_5_iREGpDD, A);

    /* LD B,SET 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_6_i_reg_p_dd!(instrDDCB__LD_B_SET_6_iREGpDD, B);

    /* LD C,SET 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_6_i_reg_p_dd!(instrDDCB__LD_C_SET_6_iREGpDD, C);

    /* LD D,SET 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_6_i_reg_p_dd!(instrDDCB__LD_D_SET_6_iREGpDD, D);

    /* LD E,SET 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_6_i_reg_p_dd!(instrDDCB__LD_E_SET_6_iREGpDD, E);

    /* LD H,SET 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_6_i_reg_p_dd!(instrDDCB__LD_H_SET_6_iREGpDD, H);

    /* LD L,SET 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_6_i_reg_p_dd!(instrDDCB__LD_L_SET_6_iREGpDD, L);

    /* SET 6,(REGISTER+dd) */
    fn_instr_ddcb_set_n_i_reg_p_dd!(instrDDCB__SET_6_iREGpDD, 0x40);

    /* LD A,SET 6,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_6_i_reg_p_dd!(instrDDCB__LD_A_SET_6_iREGpDD, A);

    /* LD B,SET 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_7_i_reg_p_dd!(instrDDCB__LD_B_SET_7_iREGpDD, B);

    /* LD C,SET 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_7_i_reg_p_dd!(instrDDCB__LD_C_SET_7_iREGpDD, C);

    /* LD D,SET 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_7_i_reg_p_dd!(instrDDCB__LD_D_SET_7_iREGpDD, D);

    /* LD E,SET 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_7_i_reg_p_dd!(instrDDCB__LD_E_SET_7_iREGpDD, E);

    /* LD H,SET 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_7_i_reg_p_dd!(instrDDCB__LD_H_SET_7_iREGpDD, H);

    /* LD L,SET 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_7_i_reg_p_dd!(instrDDCB__LD_L_SET_7_iREGpDD, L);

    /* SET 7,(REGISTER+dd) */
    fn_instr_ddcb_set_n_i_reg_p_dd!(instrDDCB__SET_7_iREGpDD, 0x80);

    /* LD A,SET 7,(REGISTER+dd) */
    fn_instr_ddcb_ld_r_set_7_i_reg_p_dd!(instrDDCB__LD_A_SET_7_iREGpDD, A);
}
