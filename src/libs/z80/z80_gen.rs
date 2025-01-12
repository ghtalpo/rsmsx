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

use super::z80_base::{join_bytes, split_word, tern_op_b, FLAG_C, FLAG_H, FLAG_N, FLAG_V, Z80};

macro_rules! fn_inc_reg {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.$r = self.data.$r.wrapping_add(1);
            self.data.F = self.data.F & FLAG_C
                | tern_op_b(self.data.$r == 0x80, FLAG_V, 0)
                | tern_op_b((self.data.$r & 0x0f) != 0, 0, FLAG_H)
                | self.tables.sz53_table[self.data.$r as usize];
        }
    };
}

macro_rules! fn_dec_reg {
    ($fn:tt, $r:ident) => {
        pub(crate) fn $fn(&mut self) {
            self.data.F =
                self.data.F & FLAG_C | tern_op_b(self.data.$r & 0x0f != 0, 0, FLAG_H) | FLAG_N;
            self.data.$r = self.data.$r.wrapping_sub(1);
            self.data.F |= tern_op_b(self.data.$r == 0x7f, FLAG_V, 0)
                | self.tables.sz53_table[self.data.$r as usize];
        }
    };
}

macro_rules! fn_get_reg16 {
    ($fn:tt, $rh:ident, $rl:ident) => {
        pub(crate) fn $fn(&self) -> u16 {
            // return self.bc.get();
            join_bytes(self.data.$rh, self.data.$rl)
        }
    };
}

macro_rules! fn_set_reg16 {
    ($fn:tt, $rh:ident, $rl:ident) => {
        pub(crate) fn $fn(&mut self, value: u16) {
            // return self.bc.get();
            (self.data.$rh, self.data.$rl) = split_word(value);
        }
    };
}

macro_rules! fn_dec_reg16 {
    ($fn:tt, $r:ident, $fs:ident) => {
        pub(crate) fn $fn(&mut self) {
            // self.bc.dec()
            let r = self.$r();
            self.$fs(r - 1);
        }
    };
}

macro_rules! fn_inc_reg16 {
    ($fn:tt, $r:ident, $fs:ident) => {
        pub(crate) fn $fn(&mut self) {
            // self.bc.inc()
            let r = self.$r();
            self.$fs(r + 1);
        }
    };
}

#[allow(non_snake_case, dead_code)]
impl Z80 {
    fn_inc_reg!(incA, A);
    fn_dec_reg!(decA, A);

    fn_inc_reg!(incB, B);
    fn_dec_reg!(decB, B);

    fn_inc_reg!(incC, C);
    fn_dec_reg!(decC, C);

    fn_inc_reg!(incD, D);
    fn_dec_reg!(decD, D);

    fn_inc_reg!(incE, E);
    fn_dec_reg!(decE, E);

    fn_inc_reg!(incF, F);
    fn_dec_reg!(decF, F);

    fn_inc_reg!(incH, H);
    fn_dec_reg!(decH, H);

    fn_inc_reg!(incI, I);
    fn_dec_reg!(decI, I);

    fn_inc_reg!(incL, L);
    fn_dec_reg!(decL, L);

    fn_inc_reg!(incR7, R7);
    fn_dec_reg!(decR7, R7);

    fn_inc_reg!(incA_, A_);
    fn_dec_reg!(decA_, A_);

    fn_inc_reg!(incB_, B_);
    fn_dec_reg!(decB_, B_);

    fn_inc_reg!(incC_, C_);
    fn_dec_reg!(decC_, C_);

    fn_inc_reg!(incD_, D_);
    fn_dec_reg!(decD_, D_);

    fn_inc_reg!(incE_, E_);
    fn_dec_reg!(decE_, E_);

    fn_inc_reg!(incF_, F_);
    fn_dec_reg!(decF_, F_);

    fn_inc_reg!(incH_, H_);
    fn_dec_reg!(decH_, H_);

    fn_inc_reg!(incL_, L_);
    fn_dec_reg!(decL_, L_);

    fn_inc_reg!(incIXL, IXL);
    fn_dec_reg!(decIXL, IXL);

    fn_inc_reg!(incIXH, IXH);
    fn_dec_reg!(decIXH, IXH);

    fn_inc_reg!(incIYL, IYL);
    fn_dec_reg!(decIYL, IYL);

    fn_inc_reg!(incIYH, IYH);
    fn_dec_reg!(decIYH, IYH);

    // Generated getters/setters and INC/DEC functions for 16bit registers

    fn_get_reg16!(BC, B, C);
    fn_set_reg16!(SetBC, B, C);
    fn_dec_reg16!(DecBC, BC, SetBC);
    fn_inc_reg16!(IncBC, BC, SetBC);

    fn_get_reg16!(DE, D, E);
    fn_set_reg16!(SetDE, D, E);
    fn_dec_reg16!(DecDE, DE, SetDE);
    fn_inc_reg16!(IncDE, DE, SetDE);

    fn_get_reg16!(HL, H, L);
    fn_set_reg16!(SetHL, H, L);
    fn_dec_reg16!(DecHL, HL, SetHL);
    fn_inc_reg16!(IncHL, HL, SetHL);

    fn_get_reg16!(BC_, B_, C_);
    fn_set_reg16!(SetBC_, B_, C_);
    fn_dec_reg16!(DecBC_, BC_, SetBC_);
    fn_inc_reg16!(IncBC_, BC_, SetBC_);

    fn_get_reg16!(DE_, D_, E_);
    fn_set_reg16!(SetDE_, D_, E_);
    fn_dec_reg16!(DecDE_, DE_, SetDE_);
    fn_inc_reg16!(IncDE_, DE_, SetDE_);

    fn_get_reg16!(HL_, H_, L_);
    fn_set_reg16!(SetHL_, H_, L_);
    fn_dec_reg16!(DecHL_, HL_, SetHL_);
    fn_inc_reg16!(IncHL_, HL_, SetHL_);

    fn_get_reg16!(IX, IXH, IXL);
    fn_set_reg16!(SetIX, IXH, IXL);
    fn_dec_reg16!(DecIX, IX, SetIX);
    fn_inc_reg16!(IncIX, IX, SetIX);

    fn_get_reg16!(IY, IYH, IYL);
    fn_set_reg16!(SetIY, IYH, IYL);
    fn_dec_reg16!(DecIY, IY, SetIY);
    fn_inc_reg16!(IncIY, IY, SetIY);
}
