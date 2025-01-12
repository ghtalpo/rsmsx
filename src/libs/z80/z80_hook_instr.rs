use super::z80_base::Z80;

#[allow(non_snake_case, dead_code)]
impl Z80 {
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
    pub(crate) fn instr_hk__ADD_HL_BC(&mut self) {
        self.IncPC(1);
        self.instr__ADD_HL_BC();
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
}
