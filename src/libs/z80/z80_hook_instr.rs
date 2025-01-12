use super::z80_base::Z80;

#[allow(non_snake_case, dead_code)]
impl Z80 {
    // add
    fn increase_cycles(&mut self, n: u64) {
        self.data.cycles = self.data.cycles + n;
    }
    pub(crate) fn instr_hk__ADD_HL_BC(&mut self) {
        self.IncPC(1);
        self.instr__ADD_HL_BC();
        self.increase_cycles(11);
    }
    pub(crate) fn instr_hk__ADD_A_A(&mut self) {
        self.IncPC(1);
        self.instr__ADD_A_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__ADD_A_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.add(nn);
        self.increase_cycles(7);
    }
    // cp
    pub(crate) fn instr_hk__CP_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.cp(nn);
        self.increase_cycles(7);
    }
    // inc
    pub(crate) fn instr_hk__INC_A(&mut self) {
        self.IncPC(1);
        self.instr__INC_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__INC_HL(&mut self) {
        self.IncPC(1);
        self.instr__INC_HL();
        self.increase_cycles(6);
    }
    // ld
    pub(crate) fn instr_hk__LD_HL_NNNN(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.SetHL(nnnn);
        self.increase_cycles(10);
    }
    pub(crate) fn instr_hk__LD_A_iHL(&mut self) {
        self.IncPC(1);
        self.instr__LD_A_iHL();
        self.increase_cycles(7);
    }
    pub(crate) fn instr_hk__LD_D_iHL(&mut self) {
        self.IncPC(1);
        self.instr__LD_D_iHL();
        self.increase_cycles(7);
    }
    pub(crate) fn instr_hk__LD_E_iHL(&mut self) {
        self.IncPC(1);
        self.instr__LD_E_iHL();
        self.increase_cycles(7);
    }
    pub(crate) fn instr_hk__LD_A_iNNNN(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.data.A = self.memory.read_byte(nnnn);
        self.increase_cycles(13);
    }
    pub(crate) fn instr_hk__LD_C_A(&mut self) {
        self.IncPC(1);
        self.instr__LD_C_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_B_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.data.B = nn;
        self.increase_cycles(7);
    }
    pub(crate) fn instr_hk__LD_C_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.data.C = nn;
        self.increase_cycles(7);
    }
    pub(crate) fn instr_hk__LD_HL_iNNNN(&mut self, mut nnnn: u16) {
        self.IncPC(3);
        self.data.L = self.memory.read_byte(nnnn);
        nnnn += 1;
        self.data.H = self.memory.read_byte(nnnn);
        self.increase_cycles(16);
    }
    pub(crate) fn instr_hk__LD_iHL_B(&mut self) {
        self.IncPC(1);
        self.instr__LD_iHL_B();
        self.increase_cycles(7);
    }
    pub(crate) fn instr_hk__LD_iNNNN_HL(&mut self, mut nnnn: u16) {
        self.IncPC(3);
        self.memory.write_byte(nnnn, self.data.L);
        nnnn += 1;
        self.memory.write_byte(nnnn, self.data.H);
        self.increase_cycles(16);
    }
    pub(crate) fn instr_hk__LD_iNNNN_A(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.memory.write_byte(nnnn, self.data.A);
        self.increase_cycles(13);
    }
    // or
    pub(crate) fn instr_hk__OR_A_A(&mut self) {
        self.IncPC(1);
        self.instr__OR_A_A();
        self.increase_cycles(4);
    }
    // pop
    pub(crate) fn instr_hk__POP_BC(&mut self) {
        self.IncPC(1);
        self.instr__POP_BC();
        self.increase_cycles(10);
    }
    // push
    pub(crate) fn instr_hk__PUSH_BC(&mut self) {
        self.IncPC(1);
        self.instr__PUSH_BC();
        self.increase_cycles(11);
    }
    // sbc
    pub(crate) fn instr_hk__SBC_HL_DE(&mut self) {
        self.IncPC(2);
        self.instrED__SBC_HL_DE();
        self.increase_cycles(15);
    }
    // scf
    pub(crate) fn instr_hk__SCF(&mut self) {
        self.IncPC(1);
        self.instr__SCF();
        self.increase_cycles(4);
    }
    // xor
    pub(crate) fn instr_hk__XOR_A_A(&mut self) {
        self.IncPC(1);
        self.instr__XOR_A_A();
        self.increase_cycles(4);
    }
}
