use super::z80_base::Z80;

#[allow(non_snake_case, dead_code)]
impl Z80 {
    pub(crate) fn increase_cycles(&mut self, n: u64) {
        self.data.cycles = self.data.cycles + n;
    }
    // add
    pub(crate) fn instr_hk__ADD_HL_BC(&mut self) {
        self.IncPC(1);
        self.instr__ADD_HL_BC();
        self.increase_cycles(11);
    }
    pub(crate) fn instr_hk__ADD_HL_DE(&mut self) {
        self.IncPC(1);
        self.instr__ADD_HL_DE();
        self.increase_cycles(11);
    }
    pub(crate) fn instr_hk__ADD_HL_HL(&mut self) {
        self.IncPC(1);
        self.instr__ADD_HL_HL();
        self.increase_cycles(11);
    }
    pub(crate) fn instr_hk__ADD_A_A(&mut self) {
        self.IncPC(1);
        self.instr__ADD_A_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__ADD_A_E(&mut self) {
        self.IncPC(1);
        self.instr__ADD_A_E();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__ADD_A_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.add(nn);
        self.increase_cycles(7);
    }
    // and
    pub(crate) fn instr_hk__AND_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.and(nn);
        self.increase_cycles(7);
    }

    // bit
    pub(crate) fn instr_hk__BIT_7_B(&mut self) {
        self.IncPC(2);
        self.instrCB__BIT_7_B();
        self.increase_cycles(8);
    }

    // cp
    pub(crate) fn instr_hk__CP_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.cp(nn);
        self.increase_cycles(7);
    }
    pub(crate) fn instr_hk__CP_iHL(&mut self) {
        self.IncPC(1);
        self.instr__CP_iHL();
        self.increase_cycles(7);
    }
    // dec
    pub(crate) fn instr_hk__DEC_A(&mut self) {
        self.IncPC(1);
        self.instr__DEC_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__DEC_BC(&mut self) {
        self.IncPC(1);
        self.instr__DEC_BC();
        self.increase_cycles(6);
    }
    // di
    pub(crate) fn instr_hk__DI(&mut self) {
        self.IncPC(1);
        self.instr__DI();
        self.increase_cycles(4);
    }
    // ei
    pub(crate) fn instr_hk__EI(&mut self) {
        self.IncPC(1);
        self.instr__EI();
        self.increase_cycles(4);
    }
    // ex
    pub(crate) fn instr_hk__EX_DE_HL(&mut self) {
        self.IncPC(1);
        self.instr__EX_DE_HL();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__EX_iSP_HL(&mut self) {
        self.IncPC(1);
        self.instr__EX_iSP_HL();
        self.increase_cycles(19);
    }
    // inc
    pub(crate) fn instr_hk__INC_A(&mut self) {
        self.IncPC(1);
        self.instr__INC_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__INC_C(&mut self) {
        self.IncPC(1);
        self.instr__INC_C();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__INC_D(&mut self) {
        self.IncPC(1);
        self.instr__INC_D();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__INC_DE(&mut self) {
        self.IncPC(1);
        self.instr__INC_DE();
        self.increase_cycles(6);
    }
    pub(crate) fn instr_hk__INC_HL(&mut self) {
        self.IncPC(1);
        self.instr__INC_HL();
        self.increase_cycles(6);
    }
    // ld
    pub(crate) fn instr_hk__LD_BC_NNNN(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.SetBC(nnnn);
        self.increase_cycles(10);
    }
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
    pub(crate) fn instr_hk__LD_A_iDE(&mut self) {
        self.IncPC(1);
        self.instr__LD_A_iDE();
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
    pub(crate) fn instr_hk__LD_A_B(&mut self) {
        self.IncPC(1);
        self.instr__LD_A_B();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_A_C(&mut self) {
        self.IncPC(1);
        self.instr__LD_A_C();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_A_H(&mut self) {
        self.IncPC(1);
        self.instr__LD_A_H();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_A_L(&mut self) {
        self.IncPC(1);
        self.instr__LD_A_L();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_C_A(&mut self) {
        self.IncPC(1);
        self.instr__LD_C_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_D_H(&mut self) {
        self.IncPC(1);
        self.instr__LD_D_H();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_E_A(&mut self) {
        self.IncPC(1);
        self.instr__LD_E_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_E_H(&mut self) {
        self.IncPC(1);
        self.instr__LD_E_H();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_H_B(&mut self) {
        self.IncPC(1);
        self.instr__LD_H_B();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_H_NN(&mut self, nn: u8) {
        self.IncPC(1);
        self.data.H = nn;
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_L_A(&mut self) {
        self.IncPC(1);
        self.instr__LD_L_A();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__LD_L_C(&mut self) {
        self.IncPC(1);
        self.instr__LD_L_C();
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
    pub(crate) fn instr_hk__LD_DE_NNNN(&mut self, nnnn: u16) {
        self.IncPC(3);
        self.SetDE(nnnn);
        self.increase_cycles(10);
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
    pub(crate) fn instr_hk__LD_iHL_E(&mut self) {
        self.IncPC(1);
        self.instr__LD_iHL_E();
        self.increase_cycles(7);
    }
    pub(crate) fn instr_hk__LD_iNNNN_HL(&mut self, mut nnnn: u16) {
        self.IncPC(4);
        self.memory.write_byte(nnnn, self.data.L);
        nnnn += 1;
        self.memory.write_byte(nnnn, self.data.H);
        self.increase_cycles(20);
    }
    pub(crate) fn instr_hk__LD_iNNNN_DE(&mut self, mut nnnn: u16) {
        self.IncPC(4);
        self.memory.write_byte(nnnn, self.data.E);
        nnnn += 1;
        self.memory.write_byte(nnnn, self.data.D);
        self.increase_cycles(20);
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
    pub(crate) fn instr_hk__OR_A_B(&mut self) {
        self.IncPC(1);
        self.instr__OR_A_B();
        self.increase_cycles(4);
    }
    pub(crate) fn instr_hk__OR_NN(&mut self, nn: u8) {
        self.IncPC(2);
        self.or(nn);
        self.increase_cycles(7);
    }
    // out
    pub(crate) fn instr_hk__OUT_iNN_A(&mut self, nn: u8) {
        self.IncPC(2);
        let out_temp: u16 = (nn as u16) + ((self.data.A as u16) << 8);
        self.write_port(out_temp, self.data.A);
        self.increase_cycles(11);
    }
    // pop
    pub(crate) fn instr_hk__POP_AF(&mut self) {
        self.IncPC(1);
        self.instr__POP_AF();
        self.increase_cycles(10);
    }
    pub(crate) fn instr_hk__POP_BC(&mut self) {
        self.IncPC(1);
        self.instr__POP_BC();
        self.increase_cycles(10);
    }
    pub(crate) fn instr_hk__POP_DE(&mut self) {
        self.IncPC(1);
        self.instr__POP_DE();
        self.increase_cycles(10);
    }
    pub(crate) fn instr_hk__POP_HL(&mut self) {
        self.IncPC(1);
        self.instr__POP_HL();
        self.increase_cycles(10);
    }
    // push
    pub(crate) fn instr_hk__PUSH_AF(&mut self) {
        self.IncPC(1);
        self.instr__PUSH_AF();
        self.increase_cycles(11);
    }
    pub(crate) fn instr_hk__PUSH_BC(&mut self) {
        self.IncPC(1);
        self.instr__PUSH_BC();
        self.increase_cycles(11);
    }
    pub(crate) fn instr_hk__PUSH_DE(&mut self) {
        self.IncPC(1);
        self.instr__PUSH_DE();
        self.increase_cycles(11);
    }
    pub(crate) fn instr_hk__PUSH_HL(&mut self) {
        self.IncPC(1);
        self.instr__PUSH_HL();
        self.increase_cycles(11);
    }
    // res
    pub(crate) fn instr_hk__RES_7_B(&mut self) {
        self.IncPC(2);
        self.instrCB__RES_7_B();
        self.increase_cycles(8);
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
