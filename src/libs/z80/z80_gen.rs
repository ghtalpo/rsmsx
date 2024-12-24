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

#[allow(non_snake_case, dead_code)]
impl Z80 {
    pub(crate) fn incA(&mut self) {
        self.A = self.A.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.A == 0x80, FLAG_V, 0))
            | (tern_op_b((self.A & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.A as usize];
    }

    pub(crate) fn decA(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.A & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.A = self.A.wrapping_sub(1);
        self.F |= (tern_op_b(self.A == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.A as usize];
    }

    pub(crate) fn incB(&mut self) {
        self.B = self.B.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.B == 0x80, FLAG_V, 0))
            | (tern_op_b((self.B & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.B as usize];
    }

    pub(crate) fn decB(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.B & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.B = self.B.wrapping_sub(1);
        self.F |= (tern_op_b(self.B == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.B as usize];
    }

    pub(crate) fn incC(&mut self) {
        self.C = self.C.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.C == 0x80, FLAG_V, 0))
            | (tern_op_b((self.C & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.C as usize];
    }

    pub(crate) fn decC(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.C & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.C = self.C.wrapping_sub(1);
        self.F |= (tern_op_b(self.C == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.C as usize];
    }

    pub(crate) fn incD(&mut self) {
        self.D = self.D.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.D == 0x80, FLAG_V, 0))
            | (tern_op_b((self.D & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.D as usize];
    }

    pub(crate) fn decD(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.D & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.D = self.D.wrapping_sub(1);
        self.F |= (tern_op_b(self.D == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.D as usize];
    }

    pub(crate) fn incE(&mut self) {
        self.E = self.E.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.E == 0x80, FLAG_V, 0))
            | (tern_op_b((self.E & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.E as usize];
    }

    pub(crate) fn decE(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.E & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.E = self.E.wrapping_sub(1);
        self.F |= (tern_op_b(self.E == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.E as usize];
    }

    pub(crate) fn incF(&mut self) {
        self.F = self.F.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.F == 0x80, FLAG_V, 0))
            | (tern_op_b((self.F & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.F as usize];
    }

    pub(crate) fn decF(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.F & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.F = self.F.wrapping_sub(1);
        self.F |= (tern_op_b(self.F == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.F as usize];
    }

    pub(crate) fn incH(&mut self) {
        self.H = self.H.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.H == 0x80, FLAG_V, 0))
            | (tern_op_b((self.H & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.H as usize];
    }

    pub(crate) fn decH(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.H & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.H = self.H.wrapping_sub(1);
        self.F |= (tern_op_b(self.H == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.H as usize];
    }

    pub(crate) fn incI(&mut self) {
        self.I = self.I.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.I == 0x80, FLAG_V, 0))
            | (tern_op_b((self.I & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.I as usize];
    }

    pub(crate) fn decI(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.I & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.I = self.I.wrapping_sub(1);
        self.F |= (tern_op_b(self.I == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.I as usize];
    }

    pub(crate) fn incL(&mut self) {
        self.L = self.L.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.L == 0x80, FLAG_V, 0))
            | (tern_op_b((self.L & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.L as usize];
    }

    pub(crate) fn decL(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.L & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.L = self.L.wrapping_sub(1);
        self.F |= (tern_op_b(self.L == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.L as usize];
    }

    pub(crate) fn incR7(&mut self) {
        self.R7 = self.R7.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.R7 == 0x80, FLAG_V, 0))
            | (tern_op_b((self.R7 & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.R7 as usize];
    }

    pub(crate) fn decR7(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.R7 & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.R7 = self.R7.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.R7 == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.R7 as usize];
    }

    pub(crate) fn incA_(&mut self) {
        self.A_ = self.A_.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.A_ == 0x80, FLAG_V, 0))
            | (tern_op_b((self.A_ & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.A_ as usize];
    }

    pub(crate) fn decA_(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.A_ & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.A_ = self.A_.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.A_ == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.A_ as usize];
    }

    pub(crate) fn incB_(&mut self) {
        self.B_ = self.B_.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.B_ == 0x80, FLAG_V, 0))
            | (tern_op_b((self.B_ & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.B_ as usize];
    }

    pub(crate) fn decB_(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.B_ & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.B_ = self.B_.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.B_ == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.B_ as usize];
    }

    pub(crate) fn incC_(&mut self) {
        self.C_ = self.C_.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.C_ == 0x80, FLAG_V, 0))
            | (tern_op_b((self.C_ & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.C_ as usize];
    }

    pub(crate) fn decC_(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.C_ & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.C_ = self.C_.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.C_ == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.C_ as usize];
    }

    pub(crate) fn incD_(&mut self) {
        self.D_ = self.D_.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.D_ == 0x80, FLAG_V, 0))
            | (tern_op_b((self.D_ & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.D_ as usize];
    }

    pub(crate) fn decD_(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.D_ & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.D_ = self.D_.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.D_ == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.D_ as usize];
    }

    pub(crate) fn incE_(&mut self) {
        self.E_ = self.E_.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.E_ == 0x80, FLAG_V, 0))
            | (tern_op_b((self.E_ & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.E_ as usize];
    }

    pub(crate) fn decE_(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.E_ & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.E_ = self.E_.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.E_ == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.E_ as usize];
    }

    pub(crate) fn incF_(&mut self) {
        self.F_ = self.F_.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.F_ == 0x80, FLAG_V, 0))
            | (tern_op_b((self.F_ & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.F_ as usize];
    }

    pub(crate) fn decF_(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.F_ & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.F_ = self.F_.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.F_ == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.F_ as usize];
    }

    pub(crate) fn incH_(&mut self) {
        self.H_ = self.H_.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.H_ == 0x80, FLAG_V, 0))
            | (tern_op_b((self.H_ & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.H_ as usize];
    }

    pub(crate) fn decH_(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.H_ & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.H_ = self.H_.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.H_ == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.H_ as usize];
    }

    pub(crate) fn incL_(&mut self) {
        self.L_ = self.L_.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.L_ == 0x80, FLAG_V, 0))
            | (tern_op_b((self.L_ & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.L_ as usize];
    }

    pub(crate) fn decL_(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.L_ & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.L_ = self.L_.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.L_ == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.L_ as usize];
    }

    pub(crate) fn incIXL(&mut self) {
        self.IXL = self.IXL.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.IXL == 0x80, FLAG_V, 0))
            | (tern_op_b((self.IXL & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.IXL as usize];
    }

    pub(crate) fn decIXL(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.IXL & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.IXL = self.IXL.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.IXL == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.IXL as usize];
    }

    pub(crate) fn incIXH(&mut self) {
        self.IXH = self.IXH.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.IXH == 0x80, FLAG_V, 0))
            | (tern_op_b((self.IXH & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.IXH as usize];
    }

    pub(crate) fn decIXH(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.IXH & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.IXH = self.IXH.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.IXH == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.IXH as usize];
    }

    pub(crate) fn incIYL(&mut self) {
        self.IYL = self.IYL.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.IYL == 0x80, FLAG_V, 0))
            | (tern_op_b((self.IYL & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.IYL as usize];
    }

    pub(crate) fn decIYL(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.IYL & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.IYL = self.IYL.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.IYL == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.IYL as usize];
    }

    pub(crate) fn incIYH(&mut self) {
        self.IYH = self.IYH.wrapping_add(1);
        self.F = (self.F & FLAG_C)
            | (tern_op_b(self.IYH == 0x80, FLAG_V, 0))
            | (tern_op_b((self.IYH & 0x0f) != 0, 0, FLAG_H))
            | self.tables.sz53_table[self.IYH as usize];
    }

    pub(crate) fn decIYH(&mut self) {
        self.F = (self.F & FLAG_C) | (tern_op_b(self.IYH & 0x0f != 0, 0, FLAG_H)) | FLAG_N;
        self.IYH = self.IYH.wrapping_sub(1);
        self.F |=
            (tern_op_b(self.IYH == 0x7f, FLAG_V, 0)) | self.tables.sz53_table[self.IYH as usize];
    }

    // Generated getters/setters and INC/DEC functions for 16bit registers

    pub(crate) fn BC(&self) -> u16 {
        // return self.bc.get();
        join_bytes(self.B, self.C)
    }

    pub(crate) fn SetBC(&mut self, value: u16) {
        // self.bc.set(value)
        (self.B, self.C) = split_word(value);
    }

    pub(crate) fn DecBC(&mut self) {
        // self.bc.dec()
        let r = self.BC();
        self.SetBC(r - 1);
    }

    pub(crate) fn IncBC(&mut self) {
        // self.bc.inc()
        let r = self.BC();
        self.SetBC(r + 1);
    }

    pub(crate) fn DE(&self) -> u16 {
        join_bytes(self.D, self.E)
    }

    pub(crate) fn SetDE(&mut self, value: u16) {
        (self.D, self.E) = split_word(value);
    }

    pub(crate) fn DecDE(&mut self) {
        let r = self.DE();
        self.SetDE(r - 1);
    }

    pub(crate) fn IncDE(&mut self) {
        let r = self.DE();
        self.SetDE(r + 1);
    }

    pub(crate) fn HL(&self) -> u16 {
        join_bytes(self.H, self.L)
    }

    pub(crate) fn SetHL(&mut self, value: u16) {
        (self.H, self.L) = split_word(value);
    }

    pub(crate) fn DecHL(&mut self) {
        let r = self.HL();
        self.SetHL(r - 1);
    }

    pub(crate) fn IncHL(&mut self) {
        let r = self.HL();
        self.SetHL(r + 1);
    }

    pub(crate) fn BC_(&self) -> u16 {
        join_bytes(self.B_, self.C_)
    }

    pub(crate) fn SetBC_(&mut self, value: u16) {
        (self.B_, self.C_) = split_word(value);
    }

    pub(crate) fn DecBC_(&mut self) {
        let r = self.BC_();
        self.SetBC_(r - 1);
    }

    pub(crate) fn IncBC_(&mut self) {
        let r = self.BC_();
        self.SetBC_(r + 1);
    }

    pub(crate) fn DE_(&self) -> u16 {
        join_bytes(self.D_, self.E_)
    }

    pub(crate) fn SetDE_(&mut self, value: u16) {
        (self.D_, self.E_) = split_word(value);
    }

    pub(crate) fn DecDE_(&mut self) {
        let r = self.DE_();
        self.SetDE_(r - 1);
    }

    pub(crate) fn IncDE_(&mut self) {
        let r = self.DE_();
        self.SetDE_(r + 1);
    }

    pub(crate) fn HL_(&self) -> u16 {
        join_bytes(self.H_, self.L_)
    }

    pub(crate) fn SetHL_(&mut self, value: u16) {
        (self.H_, self.L_) = split_word(value);
    }

    pub(crate) fn DecHL_(&mut self) {
        let r = self.HL_();
        self.SetHL_(r - 1);
    }

    pub(crate) fn IncHL_(&mut self) {
        let r = self.HL_();
        self.SetHL_(r + 1);
    }

    pub(crate) fn IX(&self) -> u16 {
        join_bytes(self.IXH, self.IXL)
    }

    pub(crate) fn SetIX(&mut self, value: u16) {
        (self.IXH, self.IXL) = split_word(value);
    }

    pub(crate) fn DecIX(&mut self) {
        let r = self.IX();
        self.SetIX(r - 1);
    }

    pub(crate) fn IncIX(&mut self) {
        let r = self.IX();
        self.SetIX(r + 1);
    }

    pub(crate) fn IY(&self) -> u16 {
        join_bytes(self.IYH, self.IYL)
    }

    pub(crate) fn SetIY(&mut self, value: u16) {
        (self.IYH, self.IYL) = split_word(value);
    }

    pub(crate) fn DecIY(&mut self) {
        let r = self.IY();
        self.SetIY(r - 1);
    }

    pub(crate) fn IncIY(&mut self) {
        let r = self.IY();
        self.SetIY(r + 1);
    }
}
