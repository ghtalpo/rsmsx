use super::z80_base::Z80;

#[allow(non_snake_case, dead_code)]
impl Z80 {
    // add
    pub(crate) fn instr_hk__ADD_HL_BC(&mut self) {
        self.IncPC(1);
        self.instr__ADD_HL_BC();
    }
    pub(crate) fn instr_hk__ADD_A_A(&mut self) {
        self.IncPC(1);
        self.instr__ADD_A_A();
    }
    pub(crate) fn instr_hk__ADD_A_NN(&mut self, nn: u8) {
        self.IncPC(1);
        self.IncPC(1);
        self.add(nn);
    }
    // cp
    pub(crate) fn instr_hk__CP_NN(&mut self, nn: u8) {
        self.IncPC(1);
        self.IncPC(1);
        self.cp(nn);
    }
    // inc
    pub(crate) fn instr_hk__INC_A(&mut self) {
        self.IncPC(1);
        self.instr__INC_A();
    }
    pub(crate) fn instr_hk__INC_HL(&mut self) {
        self.IncPC(1);
        self.instr__INC_HL();
    }
    // ld
    pub(crate) fn instr_hk__LD_HL_NNNN(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.SetHL(nnnn);
    }
    pub(crate) fn instr_hk__LD_A_iHL(&mut self) {
        self.IncPC(1);
        self.instr__LD_A_iHL();
    }
    pub(crate) fn instr_hk__LD_A_iNNNN(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.data.A = self.memory.read_byte(nnnn);
    }
    pub(crate) fn instr_hk__LD_D_iNNNN(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.data.D = self.memory.read_byte(nnnn);
    }
    pub(crate) fn instr_hk__LD_E_iNNNN(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.data.E = self.memory.read_byte(nnnn);
    }
    pub(crate) fn instr_hk__LD_C_A(&mut self) {
        self.IncPC(1);
        self.instr__LD_C_A();
    }
    pub(crate) fn instr_hk__LD_B_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.data.B = nn;
    }
    pub(crate) fn instr_hk__LD_C_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.data.C = nn;
    }
    pub(crate) fn instr_hk__LD_HL_iNNNN(&mut self, mut nnnn: u16) {
        self.IncPC(3);
        self.data.L = self.memory.read_byte(nnnn);
        nnnn += 1;
        self.data.H = self.memory.read_byte(nnnn);
    }
    pub(crate) fn instr_hk__LD_iHL_B(&mut self) {
        self.IncPC(1);
        self.instr__LD_iHL_B();
    }
    pub(crate) fn instr_hk__LD_iNNNN_HL(&mut self, mut nnnn: u16) {
        self.IncPC(1);
        self.IncPC(2);
        self.memory.write_byte(nnnn, self.data.L);
        nnnn += 1;
        self.memory.write_byte(nnnn, self.data.H);
    }
    pub(crate) fn instr_hk__LD_iNNNN_A(&mut self, nnnn: u16) {
        self.IncPC(1);
        self.IncPC(2);
        self.memory.write_byte(nnnn, self.data.A);
    }
    // or
    pub(crate) fn instr_hk__OR_A_A(&mut self) {
        self.IncPC(1);
        self.instr__OR_A_A();
    }
    // pop
    pub(crate) fn instr_hk__POP_BC(&mut self) {
        self.IncPC(1);
        self.instr__POP_BC();
    }
    // push
    pub(crate) fn instr_hk__PUSH_BC(&mut self) {
        self.IncPC(1);
        self.instr__PUSH_BC();
    }
    // sbc
    pub(crate) fn instr_hk__SBC_HL_DE(&mut self) {
        self.IncPC(1);
        self.instrED__SBC_HL_DE();
    }
    // scf
    pub(crate) fn instr_hk__SCF(&mut self) {
        self.IncPC(1);
        self.instr__SCF();
    }
    // xor
    pub(crate) fn instr_hk__XOR_A_A(&mut self) {
        self.IncPC(1);
        self.instr__XOR_A_A();
    }
}
