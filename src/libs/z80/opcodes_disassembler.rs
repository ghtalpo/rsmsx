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

use super::z80_base::{
    join_bytes, SHIFT_0X_CB, SHIFT_0X_DD, SHIFT_0X_DDCB, SHIFT_0X_ED, SHIFT_0X_FD, Z80,
};

// */
#[allow(non_snake_case)]
impl Z80 {
    pub fn disassemble_map(&mut self, opcode: u16) {
        match opcode {
            0 => {
                /* NOP */
                self.disassemble__NOP();
            }
            1 => {
                /* LD BC,nnnn */
                self.disassemble__LD_BC_NNNN();
            }
            0x02 => {
                /* LD (BC),A */
                self.disassemble__LD_iBC_A();
            }

            0x03 => {
                /* INC BC */
                self.disassemble__INC_BC();
            }

            0x04 => {
                /* INC B */
                self.disassemble__INC_B();
            }

            0x05 => {
                /* DEC B */
                self.disassemble__DEC_B();
            }

            0x06 => {
                /* LD B,nn */
                self.disassemble__LD_B_NN();
            }

            0x07 => {
                /* RLCA */
                self.disassemble__RLCA();
            }

            0x08 => {
                /* EX AF,AF' */
                self.disassemble__EX_AF_AF();
            }

            0x09 => {
                /* ADD HL,BC */
                self.disassemble__ADD_HL_BC();
            }

            0x0a => {
                /* LD A,(BC) */
                self.disassemble__LD_A_iBC();
            }

            0x0b => {
                /* DEC BC */
                self.disassemble__DEC_BC();
            }

            0x0c => {
                /* INC C */
                self.disassemble__INC_C();
            }

            0x0d => {
                /* DEC C */
                self.disassemble__DEC_C();
            }

            0x0e => {
                /* LD C,nn */
                self.disassemble__LD_C_NN();
            }

            0x0f => {
                /* RRCA */
                self.disassemble__RRCA();
            }

            0x10 => {
                /* DJNZ offset */
                self.disassemble__DJNZ_OFFSET();
            }

            0x11 => {
                /* LD DE,nnnn */
                self.disassemble__LD_DE_NNNN();
            }

            0x12 => {
                /* LD (DE),A */
                self.disassemble__LD_iDE_A();
            }

            0x13 => {
                /* INC DE */
                self.disassemble__INC_DE();
            }

            0x14 => {
                /* INC D */
                self.disassemble__INC_D();
            }

            0x15 => {
                /* DEC D */
                self.disassemble__DEC_D();
            }

            0x16 => {
                /* LD D,nn */
                self.disassemble__LD_D_NN();
            }

            0x17 => {
                /* RLA */
                self.disassemble__RLA();
            }

            0x18 => {
                /* JR offset */
                self.disassemble__JR_OFFSET();
            }

            0x19 => {
                /* ADD HL,DE */
                self.disassemble__ADD_HL_DE();
            }

            0x1a => {
                /* LD A,(DE) */
                self.disassemble__LD_A_iDE();
            }

            0x1b => {
                /* DEC DE */
                self.disassemble__DEC_DE();
            }

            0x1c => {
                /* INC E */
                self.disassemble__INC_E();
            }

            0x1d => {
                /* DEC E */
                self.disassemble__DEC_E();
            }

            0x1e => {
                /* LD E,nn */
                self.disassemble__LD_E_NN();
            }

            0x1f => {
                /* RRA */
                self.disassemble__RRA();
            }

            0x20 => {
                /* JR NZ,offset */
                self.disassemble__JR_NZ_OFFSET();
            }

            0x21 => {
                /* LD HL,nnnn */
                self.disassemble__LD_HL_NNNN();
            }

            0x22 => {
                /* LD (nnnn),HL */
                self.disassemble__LD_iNNNN_HL();
            }

            0x23 => {
                /* INC HL */
                self.disassemble__INC_HL();
            }

            0x24 => {
                /* INC H */
                self.disassemble__INC_H();
            }

            0x25 => {
                /* DEC H */
                self.disassemble__DEC_H();
            }

            0x26 => {
                /* LD H,nn */
                self.disassemble__LD_H_NN();
            }

            0x27 => {
                /* DAA */
                self.disassemble__DAA();
            }

            0x28 => {
                /* JR Z,offset */
                self.disassemble__JR_Z_OFFSET();
            }

            0x29 => {
                /* ADD HL,HL */
                self.disassemble__ADD_HL_HL();
            }

            0x2a => {
                /* LD HL,(nnnn) */
                self.disassemble__LD_HL_iNNNN();
            }

            0x2b => {
                /* DEC HL */
                self.disassemble__DEC_HL();
            }

            0x2c => {
                /* INC L */
                self.disassemble__INC_L();
            }

            0x2d => {
                /* DEC L */
                self.disassemble__DEC_L();
            }

            0x2e => {
                /* LD L,nn */
                self.disassemble__LD_L_NN();
            }

            0x2f => {
                /* CPL */
                self.disassemble__CPL();
            }

            0x30 => {
                /* JR NC,offset */
                self.disassemble__JR_NC_OFFSET();
            }

            0x31 => {
                /* LD SP,nnnn */
                self.disassemble__LD_SP_NNNN();
            }

            0x32 => {
                /* LD (nnnn),A */
                self.disassemble__LD_iNNNN_A();
            }

            0x33 => {
                /* INC SP */
                self.disassemble__INC_SP();
            }

            0x34 => {
                /* INC (HL) */
                self.disassemble__INC_iHL();
            }

            0x35 => {
                /* DEC (HL) */
                self.disassemble__DEC_iHL();
            }

            0x36 => {
                /* LD (HL),nn */
                self.disassemble__LD_iHL_NN();
            }

            0x37 => {
                /* SCF */
                self.disassemble__SCF();
            }

            0x38 => {
                /* JR C,offset */
                self.disassemble__JR_C_OFFSET();
            }

            0x39 => {
                /* ADD HL,SP */
                self.disassemble__ADD_HL_SP();
            }

            0x3a => {
                /* LD A,(nnnn) */
                self.disassemble__LD_A_iNNNN();
            }

            0x3b => {
                /* DEC SP */
                self.disassemble__DEC_SP();
            }

            0x3c => {
                /* INC A */
                self.disassemble__INC_A();
            }

            0x3d => {
                /* DEC A */
                self.disassemble__DEC_A();
            }

            0x3e => {
                /* LD A,nn */
                self.disassemble__LD_A_NN();
            }

            0x3f => {
                /* CCF */
                self.disassemble__CCF();
            }

            0x40 => {
                /* LD B,B */
                self.disassemble__LD_B_B();
            }

            0x41 => {
                /* LD B,C */
                self.disassemble__LD_B_C();
            }

            0x42 => {
                /* LD B,D */
                self.disassemble__LD_B_D();
            }

            0x43 => {
                /* LD B,E */
                self.disassemble__LD_B_E();
            }

            0x44 => {
                /* LD B,H */
                self.disassemble__LD_B_H();
            }

            0x45 => {
                /* LD B,L */
                self.disassemble__LD_B_L();
            }

            0x46 => {
                /* LD B,(HL) */
                self.disassemble__LD_B_iHL();
            }

            0x47 => {
                /* LD B,A */
                self.disassemble__LD_B_A();
            }

            0x48 => {
                /* LD C,B */
                self.disassemble__LD_C_B();
            }

            0x49 => {
                /* LD C,C */
                self.disassemble__LD_C_C();
            }

            0x4a => {
                /* LD C,D */
                self.disassemble__LD_C_D();
            }

            0x4b => {
                /* LD C,E */
                self.disassemble__LD_C_E();
            }

            0x4c => {
                /* LD C,H */
                self.disassemble__LD_C_H();
            }

            0x4d => {
                /* LD C,L */
                self.disassemble__LD_C_L();
            }

            0x4e => {
                /* LD C,(HL) */
                self.disassemble__LD_C_iHL();
            }

            0x4f => {
                /* LD C,A */
                self.disassemble__LD_C_A();
            }

            0x50 => {
                /* LD D,B */
                self.disassemble__LD_D_B();
            }

            0x51 => {
                /* LD D,C */
                self.disassemble__LD_D_C();
            }

            0x52 => {
                /* LD D,D */
                self.disassemble__LD_D_D();
            }

            0x53 => {
                /* LD D,E */
                self.disassemble__LD_D_E();
            }

            0x54 => {
                /* LD D,H */
                self.disassemble__LD_D_H();
            }

            0x55 => {
                /* LD D,L */
                self.disassemble__LD_D_L();
            }

            0x56 => {
                /* LD D,(HL) */
                self.disassemble__LD_D_iHL();
            }

            0x57 => {
                /* LD D,A */
                self.disassemble__LD_D_A();
            }

            0x58 => {
                /* LD E,B */
                self.disassemble__LD_E_B();
            }

            0x59 => {
                /* LD E,C */
                self.disassemble__LD_E_C();
            }

            0x5a => {
                /* LD E,D */
                self.disassemble__LD_E_D();
            }

            0x5b => {
                /* LD E,E */
                self.disassemble__LD_E_E();
            }

            0x5c => {
                /* LD E,H */
                self.disassemble__LD_E_H();
            }

            0x5d => {
                /* LD E,L */
                self.disassemble__LD_E_L();
            }

            0x5e => {
                /* LD E,(HL) */
                self.disassemble__LD_E_iHL();
            }

            0x5f => {
                /* LD E,A */
                self.disassemble__LD_E_A();
            }

            0x60 => {
                /* LD H,B */
                self.disassemble__LD_H_B();
            }

            0x61 => {
                /* LD H,C */
                self.disassemble__LD_H_C();
            }

            0x62 => {
                /* LD H,D */
                self.disassemble__LD_H_D();
            }

            0x63 => {
                /* LD H,E */
                self.disassemble__LD_H_E();
            }

            0x64 => {
                /* LD H,H */
                self.disassemble__LD_H_H();
            }

            0x65 => {
                /* LD H,L */
                self.disassemble__LD_H_L();
            }

            0x66 => {
                /* LD H,(HL) */
                self.disassemble__LD_H_iHL();
            }

            0x67 => {
                /* LD H,A */
                self.disassemble__LD_H_A();
            }

            0x68 => {
                /* LD L,B */
                self.disassemble__LD_L_B();
            }

            0x69 => {
                /* LD L,C */
                self.disassemble__LD_L_C();
            }

            0x6a => {
                /* LD L,D */
                self.disassemble__LD_L_D();
            }

            0x6b => {
                /* LD L,E */
                self.disassemble__LD_L_E();
            }

            0x6c => {
                /* LD L,H */
                self.disassemble__LD_L_H();
            }

            0x6d => {
                /* LD L,L */
                self.disassemble__LD_L_L();
            }

            0x6e => {
                /* LD L,(HL) */
                self.disassemble__LD_L_iHL();
            }

            0x6f => {
                /* LD L,A */
                self.disassemble__LD_L_A();
            }

            0x70 => {
                /* LD (HL),B */
                self.disassemble__LD_iHL_B();
            }

            0x71 => {
                /* LD (HL),C */
                self.disassemble__LD_iHL_C();
            }

            0x72 => {
                /* LD (HL),D */
                self.disassemble__LD_iHL_D();
            }

            0x73 => {
                /* LD (HL),E */
                self.disassemble__LD_iHL_E();
            }

            0x74 => {
                /* LD (HL),H */
                self.disassemble__LD_iHL_H();
            }

            0x75 => {
                /* LD (HL),L */
                self.disassemble__LD_iHL_L();
            }

            0x76 => {
                /* HALT */
                self.disassemble__HALT();
            }

            0x77 => {
                /* LD (HL),A */
                self.disassemble__LD_iHL_A();
            }

            0x78 => {
                /* LD A,B */
                self.disassemble__LD_A_B();
            }

            0x79 => {
                /* LD A,C */
                self.disassemble__LD_A_C();
            }

            0x7a => {
                /* LD A,D */
                self.disassemble__LD_A_D();
            }

            0x7b => {
                /* LD A,E */
                self.disassemble__LD_A_E();
            }

            0x7c => {
                /* LD A,H */
                self.disassemble__LD_A_H();
            }

            0x7d => {
                /* LD A,L */
                self.disassemble__LD_A_L();
            }

            0x7e => {
                /* LD A,(HL) */
                self.disassemble__LD_A_iHL();
            }

            0x7f => {
                /* LD A,A */
                self.disassemble__LD_A_A();
            }

            0x80 => {
                /* ADD A,B */
                self.disassemble__ADD_A_B();
            }

            0x81 => {
                /* ADD A,C */
                self.disassemble__ADD_A_C();
            }

            0x82 => {
                /* ADD A,D */
                self.disassemble__ADD_A_D();
            }

            0x83 => {
                /* ADD A,E */
                self.disassemble__ADD_A_E();
            }

            0x84 => {
                /* ADD A,H */
                self.disassemble__ADD_A_H();
            }

            0x85 => {
                /* ADD A,L */
                self.disassemble__ADD_A_L();
            }

            0x86 => {
                /* ADD A,(HL) */
                self.disassemble__ADD_A_iHL();
            }

            0x87 => {
                /* ADD A,A */
                self.disassemble__ADD_A_A();
            }

            0x88 => {
                /* ADC A,B */
                self.disassemble__ADC_A_B();
            }

            0x89 => {
                /* ADC A,C */
                self.disassemble__ADC_A_C();
            }

            0x8a => {
                /* ADC A,D */
                self.disassemble__ADC_A_D();
            }

            0x8b => {
                /* ADC A,E */
                self.disassemble__ADC_A_E();
            }

            0x8c => {
                /* ADC A,H */
                self.disassemble__ADC_A_H();
            }

            0x8d => {
                /* ADC A,L */
                self.disassemble__ADC_A_L();
            }

            0x8e => {
                /* ADC A,(HL) */
                self.disassemble__ADC_A_iHL();
            }

            0x8f => {
                /* ADC A,A */
                self.disassemble__ADC_A_A();
            }

            0x90 => {
                /* SUB A,B */
                self.disassemble__SUB_A_B();
            }

            0x91 => {
                /* SUB A,C */
                self.disassemble__SUB_A_C();
            }

            0x92 => {
                /* SUB A,D */
                self.disassemble__SUB_A_D();
            }

            0x93 => {
                /* SUB A,E */
                self.disassemble__SUB_A_E();
            }

            0x94 => {
                /* SUB A,H */
                self.disassemble__SUB_A_H();
            }

            0x95 => {
                /* SUB A,L */
                self.disassemble__SUB_A_L();
            }

            0x96 => {
                /* SUB A,(HL) */
                self.disassemble__SUB_A_iHL();
            }

            0x97 => {
                /* SUB A,A */
                self.disassemble__SUB_A_A();
            }

            0x98 => {
                /* SBC A,B */
                self.disassemble__SBC_A_B();
            }

            0x99 => {
                /* SBC A,C */
                self.disassemble__SBC_A_C();
            }

            0x9a => {
                /* SBC A,D */
                self.disassemble__SBC_A_D();
            }

            0x9b => {
                /* SBC A,E */
                self.disassemble__SBC_A_E();
            }

            0x9c => {
                /* SBC A,H */
                self.disassemble__SBC_A_H();
            }

            0x9d => {
                /* SBC A,L */
                self.disassemble__SBC_A_L();
            }

            0x9e => {
                /* SBC A,(HL) */
                self.disassemble__SBC_A_iHL();
            }

            0x9f => {
                /* SBC A,A */
                self.disassemble__SBC_A_A();
            }

            0xa0 => {
                /* AND A,B */
                self.disassemble__AND_A_B();
            }

            0xa1 => {
                /* AND A,C */
                self.disassemble__AND_A_C();
            }

            0xa2 => {
                /* AND A,D */
                self.disassemble__AND_A_D();
            }

            0xa3 => {
                /* AND A,E */
                self.disassemble__AND_A_E();
            }

            0xa4 => {
                /* AND A,H */
                self.disassemble__AND_A_H();
            }

            0xa5 => {
                /* AND A,L */
                self.disassemble__AND_A_L();
            }

            0xa6 => {
                /* AND A,(HL) */
                self.disassemble__AND_A_iHL();
            }

            0xa7 => {
                /* AND A,A */
                self.disassemble__AND_A_A();
            }

            0xa8 => {
                /* XOR A,B */
                self.disassemble__XOR_A_B();
            }

            0xa9 => {
                /* XOR A,C */
                self.disassemble__XOR_A_C();
            }

            0xaa => {
                /* XOR A,D */
                self.disassemble__XOR_A_D();
            }

            0xab => {
                /* XOR A,E */
                self.disassemble__XOR_A_E();
            }

            0xac => {
                /* XOR A,H */
                self.disassemble__XOR_A_H();
            }

            0xad => {
                /* XOR A,L */
                self.disassemble__XOR_A_L();
            }

            0xae => {
                /* XOR A,(HL) */
                self.disassemble__XOR_A_iHL();
            }

            0xaf => {
                /* XOR A,A */
                self.disassemble__XOR_A_A();
            }

            0xb0 => {
                /* OR A,B */
                self.disassemble__OR_A_B();
            }

            0xb1 => {
                /* OR A,C */
                self.disassemble__OR_A_C();
            }

            0xb2 => {
                /* OR A,D */
                self.disassemble__OR_A_D();
            }

            0xb3 => {
                /* OR A,E */
                self.disassemble__OR_A_E();
            }

            0xb4 => {
                /* OR A,H */
                self.disassemble__OR_A_H();
            }

            0xb5 => {
                /* OR A,L */
                self.disassemble__OR_A_L();
            }

            0xb6 => {
                /* OR A,(HL) */
                self.disassemble__OR_A_iHL();
            }

            0xb7 => {
                /* OR A,A */
                self.disassemble__OR_A_A();
            }

            0xb8 => {
                /* CP B */
                self.disassemble__CP_B();
            }

            0xb9 => {
                /* CP C */
                self.disassemble__CP_C();
            }

            0xba => {
                /* CP D */
                self.disassemble__CP_D();
            }

            0xbb => {
                /* CP E */
                self.disassemble__CP_E();
            }

            0xbc => {
                /* CP H */
                self.disassemble__CP_H();
            }

            0xbd => {
                /* CP L */
                self.disassemble__CP_L();
            }

            0xbe => {
                /* CP (HL) */
                self.disassemble__CP_iHL();
            }

            0xbf => {
                /* CP A */
                self.disassemble__CP_A();
            }

            0xc0 => {
                /* RET NZ */
                self.disassemble__RET_NZ();
            }

            0xc1 => {
                /* POP BC */
                self.disassemble__POP_BC();
            }

            0xc2 => {
                /* JP NZ,nnnn */
                self.disassemble__JP_NZ_NNNN();
            }

            0xc3 => {
                /* JP nnnn */
                self.disassemble__JP_NNNN();
            }

            0xc4 => {
                /* CALL NZ,nnnn */
                self.disassemble__CALL_NZ_NNNN();
            }

            0xc5 => {
                /* PUSH BC */
                self.disassemble__PUSH_BC();
            }

            0xc6 => {
                /* ADD A,nn */
                self.disassemble__ADD_A_NN();
            }

            0xc7 => {
                /* RST 00 */
                self.disassemble__RST_00();
            }

            0xc8 => {
                /* RET Z */
                self.disassemble__RET_Z();
            }

            0xc9 => {
                /* RET */
                self.disassemble__RET();
            }

            0xca => {
                /* JP Z,nnnn */
                self.disassemble__JP_Z_NNNN();
            }

            0xcb => {
                /* shift CB */
                self.disassemble__SHIFT_CB();
            }

            0xcc => {
                /* CALL Z,nnnn */
                self.disassemble__CALL_Z_NNNN();
            }

            0xcd => {
                /* CALL nnnn */
                self.disassemble__CALL_NNNN();
            }

            0xce => {
                /* ADC A,nn */
                self.disassemble__ADC_A_NN();
            }

            0xcf => {
                /* RST 8 */
                self.disassemble__RST_8();
            }

            0xd0 => {
                /* RET NC */
                self.disassemble__RET_NC();
            }

            0xd1 => {
                /* POP DE */
                self.disassemble__POP_DE();
            }

            0xd2 => {
                /* JP NC,nnnn */
                self.disassemble__JP_NC_NNNN();
            }

            0xd3 => {
                /* OUT (nn),A */
                self.disassemble__OUT_iNN_A();
            }

            0xd4 => {
                /* CALL NC,nnnn */
                self.disassemble__CALL_NC_NNNN();
            }

            0xd5 => {
                /* PUSH DE */
                self.disassemble__PUSH_DE();
            }

            0xd6 => {
                /* SUB nn */
                self.disassemble__SUB_NN();
            }

            0xd7 => {
                /* RST 10 */
                self.disassemble__RST_10();
            }

            0xd8 => {
                /* RET C */
                self.disassemble__RET_C();
            }

            0xd9 => {
                /* EXX */
                self.disassemble__EXX();
            }

            0xda => {
                /* JP C,nnnn */
                self.disassemble__JP_C_NNNN();
            }

            0xdb => {
                /* IN A,(nn) */
                self.disassemble__IN_A_iNN();
            }

            0xdc => {
                /* CALL C,nnnn */
                self.disassemble__CALL_C_NNNN();
            }

            0xdd => {
                /* shift DD */
                self.disassemble__SHIFT_DD();
            }

            0xde => {
                /* SBC A,nn */
                self.disassemble__SBC_A_NN();
            }

            0xdf => {
                /* RST 18 */
                self.disassemble__RST_18();
            }

            0xe0 => {
                /* RET PO */
                self.disassemble__RET_PO();
            }

            0xe1 => {
                /* POP HL */
                self.disassemble__POP_HL();
            }

            0xe2 => {
                /* JP PO,nnnn */
                self.disassemble__JP_PO_NNNN();
            }

            0xe3 => {
                /* EX (SP),HL */
                self.disassemble__EX_iSP_HL();
            }

            0xe4 => {
                /* CALL PO,nnnn */
                self.disassemble__CALL_PO_NNNN();
            }

            0xe5 => {
                /* PUSH HL */
                self.disassemble__PUSH_HL();
            }

            0xe6 => {
                /* AND nn */
                self.disassemble__AND_NN();
            }

            0xe7 => {
                /* RST 20 */
                self.disassemble__RST_20();
            }

            0xe8 => {
                /* RET PE */
                self.disassemble__RET_PE();
            }

            0xe9 => {
                /* JP HL */
                self.disassemble__JP_HL();
            }

            0xea => {
                /* JP PE,nnnn */
                self.disassemble__JP_PE_NNNN();
            }

            0xeb => {
                /* EX DE,HL */
                self.disassemble__EX_DE_HL();
            }

            0xec => {
                /* CALL PE,nnnn */
                self.disassemble__CALL_PE_NNNN();
            }

            0xed => {
                /* shift ED */
                self.disassemble__SHIFT_ED();
            }

            0xee => {
                /* XOR A,nn */
                self.disassemble__XOR_A_NN();
            }

            0xef => {
                /* RST 28 */
                self.disassemble__RST_28();
            }

            0xf0 => {
                /* RET P */
                self.disassemble__RET_P();
            }

            0xf1 => {
                /* POP AF */
                self.disassemble__POP_AF();
            }

            0xf2 => {
                /* JP P,nnnn */
                self.disassemble__JP_P_NNNN();
            }

            0xf3 => {
                /* DI */
                self.disassemble__DI();
            }

            0xf4 => {
                /* CALL P,nnnn */
                self.disassemble__CALL_P_NNNN();
            }

            0xf5 => {
                /* PUSH AF */
                self.disassemble__PUSH_AF();
            }

            0xf6 => {
                /* OR nn */
                self.disassemble__OR_NN();
            }

            0xf7 => {
                /* RST 30 */
                self.disassemble__RST_30();
            }

            0xf8 => {
                /* RET M */
                self.disassemble__RET_M();
            }

            0xf9 => {
                /* LD SP,HL */
                self.disassemble__LD_SP_HL();
            }

            0xfa => {
                /* JP M,nnnn */
                self.disassemble__JP_M_NNNN();
            }

            0xfb => {
                /* EI */
                self.disassemble__EI();
            }

            0xfc => {
                /* CALL M,nnnn */
                self.disassemble__CALL_M_NNNN();
            }

            0xfd => {
                /* shift FD */
                self.disassemble__SHIFT_FD();
            }

            0xfe => {
                /* CP nn */
                self.disassemble__CP_NN();
            }

            0xff => {
                /* RST 38 */
                self.disassemble__RST_38();
            }

            val if val == SHIFT_0X_CB => {
                /* RLC B */
                self.disassembleCB__RLC_B();
            }

            val if val == SHIFT_0X_CB + 0x01 => {
                /* RLC C */
                self.disassembleCB__RLC_C();
            }

            val if val == SHIFT_0X_CB + 0x02 => {
                /* RLC D */
                self.disassembleCB__RLC_D();
            }

            val if val == SHIFT_0X_CB + 0x03 => {
                /* RLC E */
                self.disassembleCB__RLC_E();
            }

            val if val == SHIFT_0X_CB + 0x04 => {
                /* RLC H */
                self.disassembleCB__RLC_H();
            }

            val if val == SHIFT_0X_CB + 0x05 => {
                /* RLC L */
                self.disassembleCB__RLC_L();
            }

            val if val == SHIFT_0X_CB + 0x06 => {
                /* RLC (HL) */
                self.disassembleCB__RLC_iHL();
            }

            val if val == SHIFT_0X_CB + 0x07 => {
                /* RLC A */
                self.disassembleCB__RLC_A();
            }

            val if val == SHIFT_0X_CB + 0x08 => {
                /* RRC B */
                self.disassembleCB__RRC_B();
            }

            val if val == SHIFT_0X_CB + 0x09 => {
                /* RRC C */
                self.disassembleCB__RRC_C();
            }

            val if val == SHIFT_0X_CB + 0x0a => {
                /* RRC D */
                self.disassembleCB__RRC_D();
            }

            val if val == SHIFT_0X_CB + 0x0b => {
                /* RRC E */
                self.disassembleCB__RRC_E();
            }

            val if val == SHIFT_0X_CB + 0x0c => {
                /* RRC H */
                self.disassembleCB__RRC_H();
            }

            val if val == SHIFT_0X_CB + 0x0d => {
                /* RRC L */
                self.disassembleCB__RRC_L();
            }

            val if val == SHIFT_0X_CB + 0x0e => {
                /* RRC (HL) */
                self.disassembleCB__RRC_iHL();
            }

            val if val == SHIFT_0X_CB + 0x0f => {
                /* RRC A */
                self.disassembleCB__RRC_A();
            }

            val if val == SHIFT_0X_CB + 0x10 => {
                /* RL B */
                self.disassembleCB__RL_B();
            }

            val if val == SHIFT_0X_CB + 0x11 => {
                /* RL C */
                self.disassembleCB__RL_C();
            }

            val if val == SHIFT_0X_CB + 0x12 => {
                /* RL D */
                self.disassembleCB__RL_D();
            }

            val if val == SHIFT_0X_CB + 0x13 => {
                /* RL E */
                self.disassembleCB__RL_E();
            }

            val if val == SHIFT_0X_CB + 0x14 => {
                /* RL H */
                self.disassembleCB__RL_H();
            }

            val if val == SHIFT_0X_CB + 0x15 => {
                /* RL L */
                self.disassembleCB__RL_L();
            }

            val if val == SHIFT_0X_CB + 0x16 => {
                /* RL (HL) */
                self.disassembleCB__RL_iHL();
            }

            val if val == SHIFT_0X_CB + 0x17 => {
                /* RL A */
                self.disassembleCB__RL_A();
            }

            val if val == SHIFT_0X_CB + 0x18 => {
                /* RR B */
                self.disassembleCB__RR_B();
            }

            val if val == SHIFT_0X_CB + 0x19 => {
                /* RR C */
                self.disassembleCB__RR_C();
            }

            val if val == SHIFT_0X_CB + 0x1a => {
                /* RR D */
                self.disassembleCB__RR_D();
            }

            val if val == SHIFT_0X_CB + 0x1b => {
                /* RR E */
                self.disassembleCB__RR_E();
            }

            val if val == SHIFT_0X_CB + 0x1c => {
                /* RR H */
                self.disassembleCB__RR_H();
            }

            val if val == SHIFT_0X_CB + 0x1d => {
                /* RR L */
                self.disassembleCB__RR_L();
            }

            val if val == SHIFT_0X_CB + 0x1e => {
                /* RR (HL) */
                self.disassembleCB__RR_iHL();
            }

            val if val == SHIFT_0X_CB + 0x1f => {
                /* RR A */
                self.disassembleCB__RR_A();
            }

            val if val == SHIFT_0X_CB + 0x20 => {
                /* SLA B */
                self.disassembleCB__SLA_B();
            }

            val if val == SHIFT_0X_CB + 0x21 => {
                /* SLA C */
                self.disassembleCB__SLA_C();
            }

            val if val == SHIFT_0X_CB + 0x22 => {
                /* SLA D */
                self.disassembleCB__SLA_D();
            }

            val if val == SHIFT_0X_CB + 0x23 => {
                /* SLA E */
                self.disassembleCB__SLA_E();
            }

            val if val == SHIFT_0X_CB + 0x24 => {
                /* SLA H */
                self.disassembleCB__SLA_H();
            }

            val if val == SHIFT_0X_CB + 0x25 => {
                /* SLA L */
                self.disassembleCB__SLA_L();
            }

            val if val == SHIFT_0X_CB + 0x26 => {
                /* SLA (HL) */
                self.disassembleCB__SLA_iHL();
            }

            val if val == SHIFT_0X_CB + 0x27 => {
                /* SLA A */
                self.disassembleCB__SLA_A();
            }

            val if val == SHIFT_0X_CB + 0x28 => {
                /* SRA B */
                self.disassembleCB__SRA_B();
            }

            val if val == SHIFT_0X_CB + 0x29 => {
                /* SRA C */
                self.disassembleCB__SRA_C();
            }

            val if val == SHIFT_0X_CB + 0x2a => {
                /* SRA D */
                self.disassembleCB__SRA_D();
            }

            val if val == SHIFT_0X_CB + 0x2b => {
                /* SRA E */
                self.disassembleCB__SRA_E();
            }

            val if val == SHIFT_0X_CB + 0x2c => {
                /* SRA H */
                self.disassembleCB__SRA_H();
            }

            val if val == SHIFT_0X_CB + 0x2d => {
                /* SRA L */
                self.disassembleCB__SRA_L();
            }

            val if val == SHIFT_0X_CB + 0x2e => {
                /* SRA (HL) */
                self.disassembleCB__SRA_iHL();
            }

            val if val == SHIFT_0X_CB + 0x2f => {
                /* SRA A */
                self.disassembleCB__SRA_A();
            }

            val if val == SHIFT_0X_CB + 0x30 => {
                /* SLL B */
                self.disassembleCB__SLL_B();
            }

            val if val == SHIFT_0X_CB + 0x31 => {
                /* SLL C */
                self.disassembleCB__SLL_C();
            }

            val if val == SHIFT_0X_CB + 0x32 => {
                /* SLL D */
                self.disassembleCB__SLL_D();
            }

            val if val == SHIFT_0X_CB + 0x33 => {
                /* SLL E */
                self.disassembleCB__SLL_E();
            }

            val if val == SHIFT_0X_CB + 0x34 => {
                /* SLL H */
                self.disassembleCB__SLL_H();
            }

            val if val == SHIFT_0X_CB + 0x35 => {
                /* SLL L */
                self.disassembleCB__SLL_L();
            }

            val if val == SHIFT_0X_CB + 0x36 => {
                /* SLL (HL) */
                self.disassembleCB__SLL_iHL();
            }

            val if val == SHIFT_0X_CB + 0x37 => {
                /* SLL A */
                self.disassembleCB__SLL_A();
            }

            val if val == SHIFT_0X_CB + 0x38 => {
                /* SRL B */
                self.disassembleCB__SRL_B();
            }

            val if val == SHIFT_0X_CB + 0x39 => {
                /* SRL C */
                self.disassembleCB__SRL_C();
            }

            val if val == SHIFT_0X_CB + 0x3a => {
                /* SRL D */
                self.disassembleCB__SRL_D();
            }

            val if val == SHIFT_0X_CB + 0x3b => {
                /* SRL E */
                self.disassembleCB__SRL_E();
            }

            val if val == SHIFT_0X_CB + 0x3c => {
                /* SRL H */
                self.disassembleCB__SRL_H();
            }

            val if val == SHIFT_0X_CB + 0x3d => {
                /* SRL L */
                self.disassembleCB__SRL_L();
            }

            val if val == SHIFT_0X_CB + 0x3e => {
                /* SRL (HL) */
                self.disassembleCB__SRL_iHL();
            }

            val if val == SHIFT_0X_CB + 0x3f => {
                /* SRL A */
                self.disassembleCB__SRL_A();
            }

            val if val == SHIFT_0X_CB + 0x40 => {
                /* BIT 0,B */
                self.disassembleCB__BIT_0_B();
            }

            val if val == SHIFT_0X_CB + 0x41 => {
                /* BIT 0,C */
                self.disassembleCB__BIT_0_C();
            }

            val if val == SHIFT_0X_CB + 0x42 => {
                /* BIT 0,D */
                self.disassembleCB__BIT_0_D();
            }

            val if val == SHIFT_0X_CB + 0x43 => {
                /* BIT 0,E */
                self.disassembleCB__BIT_0_E();
            }

            val if val == SHIFT_0X_CB + 0x44 => {
                /* BIT 0,H */
                self.disassembleCB__BIT_0_H();
            }

            val if val == SHIFT_0X_CB + 0x45 => {
                /* BIT 0,L */
                self.disassembleCB__BIT_0_L();
            }

            val if val == SHIFT_0X_CB + 0x46 => {
                /* BIT 0,(HL) */
                self.disassembleCB__BIT_0_iHL();
            }

            val if val == SHIFT_0X_CB + 0x47 => {
                /* BIT 0,A */
                self.disassembleCB__BIT_0_A();
            }

            val if val == SHIFT_0X_CB + 0x48 => {
                /* BIT 1,B */
                self.disassembleCB__BIT_1_B();
            }

            val if val == SHIFT_0X_CB + 0x49 => {
                /* BIT 1,C */
                self.disassembleCB__BIT_1_C();
            }

            val if val == SHIFT_0X_CB + 0x4a => {
                /* BIT 1,D */
                self.disassembleCB__BIT_1_D();
            }

            val if val == SHIFT_0X_CB + 0x4b => {
                /* BIT 1,E */
                self.disassembleCB__BIT_1_E();
            }

            val if val == SHIFT_0X_CB + 0x4c => {
                /* BIT 1,H */
                self.disassembleCB__BIT_1_H();
            }

            val if val == SHIFT_0X_CB + 0x4d => {
                /* BIT 1,L */
                self.disassembleCB__BIT_1_L();
            }

            val if val == SHIFT_0X_CB + 0x4e => {
                /* BIT 1,(HL) */
                self.disassembleCB__BIT_1_iHL();
            }

            val if val == SHIFT_0X_CB + 0x4f => {
                /* BIT 1,A */
                self.disassembleCB__BIT_1_A();
            }

            val if val == SHIFT_0X_CB + 0x50 => {
                /* BIT 2,B */
                self.disassembleCB__BIT_2_B();
            }

            val if val == SHIFT_0X_CB + 0x51 => {
                /* BIT 2,C */
                self.disassembleCB__BIT_2_C();
            }

            val if val == SHIFT_0X_CB + 0x52 => {
                /* BIT 2,D */
                self.disassembleCB__BIT_2_D();
            }

            val if val == SHIFT_0X_CB + 0x53 => {
                /* BIT 2,E */
                self.disassembleCB__BIT_2_E();
            }

            val if val == SHIFT_0X_CB + 0x54 => {
                /* BIT 2,H */
                self.disassembleCB__BIT_2_H();
            }

            val if val == SHIFT_0X_CB + 0x55 => {
                /* BIT 2,L */
                self.disassembleCB__BIT_2_L();
            }

            val if val == SHIFT_0X_CB + 0x56 => {
                /* BIT 2,(HL) */
                self.disassembleCB__BIT_2_iHL();
            }

            val if val == SHIFT_0X_CB + 0x57 => {
                /* BIT 2,A */
                self.disassembleCB__BIT_2_A();
            }

            val if val == SHIFT_0X_CB + 0x58 => {
                /* BIT 3,B */
                self.disassembleCB__BIT_3_B();
            }

            val if val == SHIFT_0X_CB + 0x59 => {
                /* BIT 3,C */
                self.disassembleCB__BIT_3_C();
            }

            val if val == SHIFT_0X_CB + 0x5a => {
                /* BIT 3,D */
                self.disassembleCB__BIT_3_D();
            }

            val if val == SHIFT_0X_CB + 0x5b => {
                /* BIT 3,E */
                self.disassembleCB__BIT_3_E();
            }

            val if val == SHIFT_0X_CB + 0x5c => {
                /* BIT 3,H */
                self.disassembleCB__BIT_3_H();
            }

            val if val == SHIFT_0X_CB + 0x5d => {
                /* BIT 3,L */
                self.disassembleCB__BIT_3_L();
            }

            val if val == SHIFT_0X_CB + 0x5e => {
                /* BIT 3,(HL) */
                self.disassembleCB__BIT_3_iHL();
            }

            val if val == SHIFT_0X_CB + 0x5f => {
                /* BIT 3,A */
                self.disassembleCB__BIT_3_A();
            }

            val if val == SHIFT_0X_CB + 0x60 => {
                /* BIT 4,B */
                self.disassembleCB__BIT_4_B();
            }

            val if val == SHIFT_0X_CB + 0x61 => {
                /* BIT 4,C */
                self.disassembleCB__BIT_4_C();
            }

            val if val == SHIFT_0X_CB + 0x62 => {
                /* BIT 4,D */
                self.disassembleCB__BIT_4_D();
            }

            val if val == SHIFT_0X_CB + 0x63 => {
                /* BIT 4,E */
                self.disassembleCB__BIT_4_E();
            }

            val if val == SHIFT_0X_CB + 0x64 => {
                /* BIT 4,H */
                self.disassembleCB__BIT_4_H();
            }

            val if val == SHIFT_0X_CB + 0x65 => {
                /* BIT 4,L */
                self.disassembleCB__BIT_4_L();
            }

            val if val == SHIFT_0X_CB + 0x66 => {
                /* BIT 4,(HL) */
                self.disassembleCB__BIT_4_iHL();
            }

            val if val == SHIFT_0X_CB + 0x67 => {
                /* BIT 4,A */
                self.disassembleCB__BIT_4_A();
            }

            val if val == SHIFT_0X_CB + 0x68 => {
                /* BIT 5,B */
                self.disassembleCB__BIT_5_B();
            }

            val if val == SHIFT_0X_CB + 0x69 => {
                /* BIT 5,C */
                self.disassembleCB__BIT_5_C();
            }

            val if val == SHIFT_0X_CB + 0x6a => {
                /* BIT 5,D */
                self.disassembleCB__BIT_5_D();
            }

            val if val == SHIFT_0X_CB + 0x6b => {
                /* BIT 5,E */
                self.disassembleCB__BIT_5_E();
            }

            val if val == SHIFT_0X_CB + 0x6c => {
                /* BIT 5,H */
                self.disassembleCB__BIT_5_H();
            }

            val if val == SHIFT_0X_CB + 0x6d => {
                /* BIT 5,L */
                self.disassembleCB__BIT_5_L();
            }

            val if val == SHIFT_0X_CB + 0x6e => {
                /* BIT 5,(HL) */
                self.disassembleCB__BIT_5_iHL();
            }

            val if val == SHIFT_0X_CB + 0x6f => {
                /* BIT 5,A */
                self.disassembleCB__BIT_5_A();
            }

            val if val == SHIFT_0X_CB + 0x70 => {
                /* BIT 6,B */
                self.disassembleCB__BIT_6_B();
            }

            val if val == SHIFT_0X_CB + 0x71 => {
                /* BIT 6,C */
                self.disassembleCB__BIT_6_C();
            }

            val if val == SHIFT_0X_CB + 0x72 => {
                /* BIT 6,D */
                self.disassembleCB__BIT_6_D();
            }

            val if val == SHIFT_0X_CB + 0x73 => {
                /* BIT 6,E */
                self.disassembleCB__BIT_6_E();
            }

            val if val == SHIFT_0X_CB + 0x74 => {
                /* BIT 6,H */
                self.disassembleCB__BIT_6_H();
            }

            val if val == SHIFT_0X_CB + 0x75 => {
                /* BIT 6,L */
                self.disassembleCB__BIT_6_L();
            }

            val if val == SHIFT_0X_CB + 0x76 => {
                /* BIT 6,(HL) */
                self.disassembleCB__BIT_6_iHL();
            }

            val if val == SHIFT_0X_CB + 0x77 => {
                /* BIT 6,A */
                self.disassembleCB__BIT_6_A();
            }

            val if val == SHIFT_0X_CB + 0x78 => {
                /* BIT 7,B */
                self.disassembleCB__BIT_7_B();
            }

            val if val == SHIFT_0X_CB + 0x79 => {
                /* BIT 7,C */
                self.disassembleCB__BIT_7_C();
            }

            val if val == SHIFT_0X_CB + 0x7a => {
                /* BIT 7,D */
                self.disassembleCB__BIT_7_D();
            }

            val if val == SHIFT_0X_CB + 0x7b => {
                /* BIT 7,E */
                self.disassembleCB__BIT_7_E();
            }

            val if val == SHIFT_0X_CB + 0x7c => {
                /* BIT 7,H */
                self.disassembleCB__BIT_7_H();
            }

            val if val == SHIFT_0X_CB + 0x7d => {
                /* BIT 7,L */
                self.disassembleCB__BIT_7_L();
            }

            val if val == SHIFT_0X_CB + 0x7e => {
                /* BIT 7,(HL) */
                self.disassembleCB__BIT_7_iHL();
            }

            val if val == SHIFT_0X_CB + 0x7f => {
                /* BIT 7,A */
                self.disassembleCB__BIT_7_A();
            }

            val if val == SHIFT_0X_CB + 0x80 => {
                /* RES 0,B */
                self.disassembleCB__RES_0_B();
            }

            val if val == SHIFT_0X_CB + 0x81 => {
                /* RES 0,C */
                self.disassembleCB__RES_0_C();
            }

            val if val == SHIFT_0X_CB + 0x82 => {
                /* RES 0,D */
                self.disassembleCB__RES_0_D();
            }

            val if val == SHIFT_0X_CB + 0x83 => {
                /* RES 0,E */
                self.disassembleCB__RES_0_E();
            }

            val if val == SHIFT_0X_CB + 0x84 => {
                /* RES 0,H */
                self.disassembleCB__RES_0_H();
            }

            val if val == SHIFT_0X_CB + 0x85 => {
                /* RES 0,L */
                self.disassembleCB__RES_0_L();
            }

            val if val == SHIFT_0X_CB + 0x86 => {
                /* RES 0,(HL) */
                self.disassembleCB__RES_0_iHL();
            }

            val if val == SHIFT_0X_CB + 0x87 => {
                /* RES 0,A */
                self.disassembleCB__RES_0_A();
            }

            val if val == SHIFT_0X_CB + 0x88 => {
                /* RES 1,B */
                self.disassembleCB__RES_1_B();
            }

            val if val == SHIFT_0X_CB + 0x89 => {
                /* RES 1,C */
                self.disassembleCB__RES_1_C();
            }

            val if val == SHIFT_0X_CB + 0x8a => {
                /* RES 1,D */
                self.disassembleCB__RES_1_D();
            }

            val if val == SHIFT_0X_CB + 0x8b => {
                /* RES 1,E */
                self.disassembleCB__RES_1_E();
            }

            val if val == SHIFT_0X_CB + 0x8c => {
                /* RES 1,H */
                self.disassembleCB__RES_1_H();
            }

            val if val == SHIFT_0X_CB + 0x8d => {
                /* RES 1,L */
                self.disassembleCB__RES_1_L();
            }

            val if val == SHIFT_0X_CB + 0x8e => {
                /* RES 1,(HL) */
                self.disassembleCB__RES_1_iHL();
            }

            val if val == SHIFT_0X_CB + 0x8f => {
                /* RES 1,A */
                self.disassembleCB__RES_1_A();
            }

            val if val == SHIFT_0X_CB + 0x90 => {
                /* RES 2,B */
                self.disassembleCB__RES_2_B();
            }

            val if val == SHIFT_0X_CB + 0x91 => {
                /* RES 2,C */
                self.disassembleCB__RES_2_C();
            }

            val if val == SHIFT_0X_CB + 0x92 => {
                /* RES 2,D */
                self.disassembleCB__RES_2_D();
            }

            val if val == SHIFT_0X_CB + 0x93 => {
                /* RES 2,E */
                self.disassembleCB__RES_2_E();
            }

            val if val == SHIFT_0X_CB + 0x94 => {
                /* RES 2,H */
                self.disassembleCB__RES_2_H();
            }

            val if val == SHIFT_0X_CB + 0x95 => {
                /* RES 2,L */
                self.disassembleCB__RES_2_L();
            }

            val if val == SHIFT_0X_CB + 0x96 => {
                /* RES 2,(HL) */
                self.disassembleCB__RES_2_iHL();
            }

            val if val == SHIFT_0X_CB + 0x97 => {
                /* RES 2,A */
                self.disassembleCB__RES_2_A();
            }

            val if val == SHIFT_0X_CB + 0x98 => {
                /* RES 3,B */
                self.disassembleCB__RES_3_B();
            }

            val if val == SHIFT_0X_CB + 0x99 => {
                /* RES 3,C */
                self.disassembleCB__RES_3_C();
            }

            val if val == SHIFT_0X_CB + 0x9a => {
                /* RES 3,D */
                self.disassembleCB__RES_3_D();
            }

            val if val == SHIFT_0X_CB + 0x9b => {
                /* RES 3,E */
                self.disassembleCB__RES_3_E();
            }

            val if val == SHIFT_0X_CB + 0x9c => {
                /* RES 3,H */
                self.disassembleCB__RES_3_H();
            }

            val if val == SHIFT_0X_CB + 0x9d => {
                /* RES 3,L */
                self.disassembleCB__RES_3_L();
            }

            val if val == SHIFT_0X_CB + 0x9e => {
                /* RES 3,(HL) */
                self.disassembleCB__RES_3_iHL();
            }

            val if val == SHIFT_0X_CB + 0x9f => {
                /* RES 3,A */
                self.disassembleCB__RES_3_A();
            }

            val if val == SHIFT_0X_CB + 0xa0 => {
                /* RES 4,B */
                self.disassembleCB__RES_4_B();
            }

            val if val == SHIFT_0X_CB + 0xa1 => {
                /* RES 4,C */
                self.disassembleCB__RES_4_C();
            }

            val if val == SHIFT_0X_CB + 0xa2 => {
                /* RES 4,D */
                self.disassembleCB__RES_4_D();
            }

            val if val == SHIFT_0X_CB + 0xa3 => {
                /* RES 4,E */
                self.disassembleCB__RES_4_E();
            }

            val if val == SHIFT_0X_CB + 0xa4 => {
                /* RES 4,H */
                self.disassembleCB__RES_4_H();
            }

            val if val == SHIFT_0X_CB + 0xa5 => {
                /* RES 4,L */
                self.disassembleCB__RES_4_L();
            }

            val if val == SHIFT_0X_CB + 0xa6 => {
                /* RES 4,(HL) */
                self.disassembleCB__RES_4_iHL();
            }

            val if val == SHIFT_0X_CB + 0xa7 => {
                /* RES 4,A */
                self.disassembleCB__RES_4_A();
            }

            val if val == SHIFT_0X_CB + 0xa8 => {
                /* RES 5,B */
                self.disassembleCB__RES_5_B();
            }

            val if val == SHIFT_0X_CB + 0xa9 => {
                /* RES 5,C */
                self.disassembleCB__RES_5_C();
            }

            val if val == SHIFT_0X_CB + 0xaa => {
                /* RES 5,D */
                self.disassembleCB__RES_5_D();
            }

            val if val == SHIFT_0X_CB + 0xab => {
                /* RES 5,E */
                self.disassembleCB__RES_5_E();
            }

            val if val == SHIFT_0X_CB + 0xac => {
                /* RES 5,H */
                self.disassembleCB__RES_5_H();
            }

            val if val == SHIFT_0X_CB + 0xad => {
                /* RES 5,L */
                self.disassembleCB__RES_5_L();
            }

            val if val == SHIFT_0X_CB + 0xae => {
                /* RES 5,(HL) */
                self.disassembleCB__RES_5_iHL();
            }

            val if val == SHIFT_0X_CB + 0xaf => {
                /* RES 5,A */
                self.disassembleCB__RES_5_A();
            }

            val if val == SHIFT_0X_CB + 0xb0 => {
                /* RES 6,B */
                self.disassembleCB__RES_6_B();
            }

            val if val == SHIFT_0X_CB + 0xb1 => {
                /* RES 6,C */
                self.disassembleCB__RES_6_C();
            }

            val if val == SHIFT_0X_CB + 0xb2 => {
                /* RES 6,D */
                self.disassembleCB__RES_6_D();
            }

            val if val == SHIFT_0X_CB + 0xb3 => {
                /* RES 6,E */
                self.disassembleCB__RES_6_E();
            }

            val if val == SHIFT_0X_CB + 0xb4 => {
                /* RES 6,H */
                self.disassembleCB__RES_6_H();
            }

            val if val == SHIFT_0X_CB + 0xb5 => {
                /* RES 6,L */
                self.disassembleCB__RES_6_L();
            }

            val if val == SHIFT_0X_CB + 0xb6 => {
                /* RES 6,(HL) */
                self.disassembleCB__RES_6_iHL();
            }

            val if val == SHIFT_0X_CB + 0xb7 => {
                /* RES 6,A */
                self.disassembleCB__RES_6_A();
            }

            val if val == SHIFT_0X_CB + 0xb8 => {
                /* RES 7,B */
                self.disassembleCB__RES_7_B();
            }

            val if val == SHIFT_0X_CB + 0xb9 => {
                /* RES 7,C */
                self.disassembleCB__RES_7_C();
            }

            val if val == SHIFT_0X_CB + 0xba => {
                /* RES 7,D */
                self.disassembleCB__RES_7_D();
            }

            val if val == SHIFT_0X_CB + 0xbb => {
                /* RES 7,E */
                self.disassembleCB__RES_7_E();
            }

            val if val == SHIFT_0X_CB + 0xbc => {
                /* RES 7,H */
                self.disassembleCB__RES_7_H();
            }

            val if val == SHIFT_0X_CB + 0xbd => {
                /* RES 7,L */
                self.disassembleCB__RES_7_L();
            }

            val if val == SHIFT_0X_CB + 0xbe => {
                /* RES 7,(HL) */
                self.disassembleCB__RES_7_iHL();
            }

            val if val == SHIFT_0X_CB + 0xbf => {
                /* RES 7,A */
                self.disassembleCB__RES_7_A();
            }

            val if val == SHIFT_0X_CB + 0xc0 => {
                /* SET 0,B */
                self.disassembleCB__SET_0_B();
            }

            val if val == SHIFT_0X_CB + 0xc1 => {
                /* SET 0,C */
                self.disassembleCB__SET_0_C();
            }

            val if val == SHIFT_0X_CB + 0xc2 => {
                /* SET 0,D */
                self.disassembleCB__SET_0_D();
            }

            val if val == SHIFT_0X_CB + 0xc3 => {
                /* SET 0,E */
                self.disassembleCB__SET_0_E();
            }

            val if val == SHIFT_0X_CB + 0xc4 => {
                /* SET 0,H */
                self.disassembleCB__SET_0_H();
            }

            val if val == SHIFT_0X_CB + 0xc5 => {
                /* SET 0,L */
                self.disassembleCB__SET_0_L();
            }

            val if val == SHIFT_0X_CB + 0xc6 => {
                /* SET 0,(HL) */
                self.disassembleCB__SET_0_iHL();
            }

            val if val == SHIFT_0X_CB + 0xc7 => {
                /* SET 0,A */
                self.disassembleCB__SET_0_A();
            }

            val if val == SHIFT_0X_CB + 0xc8 => {
                /* SET 1,B */
                self.disassembleCB__SET_1_B();
            }

            val if val == SHIFT_0X_CB + 0xc9 => {
                /* SET 1,C */
                self.disassembleCB__SET_1_C();
            }

            val if val == SHIFT_0X_CB + 0xca => {
                /* SET 1,D */
                self.disassembleCB__SET_1_D();
            }

            val if val == SHIFT_0X_CB + 0xcb => {
                /* SET 1,E */
                self.disassembleCB__SET_1_E();
            }

            val if val == SHIFT_0X_CB + 0xcc => {
                /* SET 1,H */
                self.disassembleCB__SET_1_H();
            }

            val if val == SHIFT_0X_CB + 0xcd => {
                /* SET 1,L */
                self.disassembleCB__SET_1_L();
            }

            val if val == SHIFT_0X_CB + 0xce => {
                /* SET 1,(HL) */
                self.disassembleCB__SET_1_iHL();
            }

            val if val == SHIFT_0X_CB + 0xcf => {
                /* SET 1,A */
                self.disassembleCB__SET_1_A();
            }

            val if val == SHIFT_0X_CB + 0xd0 => {
                /* SET 2,B */
                self.disassembleCB__SET_2_B();
            }

            val if val == SHIFT_0X_CB + 0xd1 => {
                /* SET 2,C */
                self.disassembleCB__SET_2_C();
            }

            val if val == SHIFT_0X_CB + 0xd2 => {
                /* SET 2,D */
                self.disassembleCB__SET_2_D();
            }

            val if val == SHIFT_0X_CB + 0xd3 => {
                /* SET 2,E */
                self.disassembleCB__SET_2_E();
            }

            val if val == SHIFT_0X_CB + 0xd4 => {
                /* SET 2,H */
                self.disassembleCB__SET_2_H();
            }

            val if val == SHIFT_0X_CB + 0xd5 => {
                /* SET 2,L */
                self.disassembleCB__SET_2_L();
            }

            val if val == SHIFT_0X_CB + 0xd6 => {
                /* SET 2,(HL) */
                self.disassembleCB__SET_2_iHL();
            }

            val if val == SHIFT_0X_CB + 0xd7 => {
                /* SET 2,A */
                self.disassembleCB__SET_2_A();
            }

            val if val == SHIFT_0X_CB + 0xd8 => {
                /* SET 3,B */
                self.disassembleCB__SET_3_B();
            }

            val if val == SHIFT_0X_CB + 0xd9 => {
                /* SET 3,C */
                self.disassembleCB__SET_3_C();
            }

            val if val == SHIFT_0X_CB + 0xda => {
                /* SET 3,D */
                self.disassembleCB__SET_3_D();
            }

            val if val == SHIFT_0X_CB + 0xdb => {
                /* SET 3,E */
                self.disassembleCB__SET_3_E();
            }

            val if val == SHIFT_0X_CB + 0xdc => {
                /* SET 3,H */
                self.disassembleCB__SET_3_H();
            }

            val if val == SHIFT_0X_CB + 0xdd => {
                /* SET 3,L */
                self.disassembleCB__SET_3_L();
            }

            val if val == SHIFT_0X_CB + 0xde => {
                /* SET 3,(HL) */
                self.disassembleCB__SET_3_iHL();
            }

            val if val == SHIFT_0X_CB + 0xdf => {
                /* SET 3,A */
                self.disassembleCB__SET_3_A();
            }

            val if val == SHIFT_0X_CB + 0xe0 => {
                /* SET 4,B */
                self.disassembleCB__SET_4_B();
            }

            val if val == SHIFT_0X_CB + 0xe1 => {
                /* SET 4,C */
                self.disassembleCB__SET_4_C();
            }

            val if val == SHIFT_0X_CB + 0xe2 => {
                /* SET 4,D */
                self.disassembleCB__SET_4_D();
            }

            val if val == SHIFT_0X_CB + 0xe3 => {
                /* SET 4,E */
                self.disassembleCB__SET_4_E();
            }

            val if val == SHIFT_0X_CB + 0xe4 => {
                /* SET 4,H */
                self.disassembleCB__SET_4_H();
            }

            val if val == SHIFT_0X_CB + 0xe5 => {
                /* SET 4,L */
                self.disassembleCB__SET_4_L();
            }

            val if val == SHIFT_0X_CB + 0xe6 => {
                /* SET 4,(HL) */
                self.disassembleCB__SET_4_iHL();
            }

            val if val == SHIFT_0X_CB + 0xe7 => {
                /* SET 4,A */
                self.disassembleCB__SET_4_A();
            }

            val if val == SHIFT_0X_CB + 0xe8 => {
                /* SET 5,B */
                self.disassembleCB__SET_5_B();
            }

            val if val == SHIFT_0X_CB + 0xe9 => {
                /* SET 5,C */
                self.disassembleCB__SET_5_C();
            }

            val if val == SHIFT_0X_CB + 0xea => {
                /* SET 5,D */
                self.disassembleCB__SET_5_D();
            }

            val if val == SHIFT_0X_CB + 0xeb => {
                /* SET 5,E */
                self.disassembleCB__SET_5_E();
            }

            val if val == SHIFT_0X_CB + 0xec => {
                /* SET 5,H */
                self.disassembleCB__SET_5_H();
            }

            val if val == SHIFT_0X_CB + 0xed => {
                /* SET 5,L */
                self.disassembleCB__SET_5_L();
            }

            val if val == SHIFT_0X_CB + 0xee => {
                /* SET 5,(HL) */
                self.disassembleCB__SET_5_iHL();
            }

            val if val == SHIFT_0X_CB + 0xef => {
                /* SET 5,A */
                self.disassembleCB__SET_5_A();
            }

            val if val == SHIFT_0X_CB + 0xf0 => {
                /* SET 6,B */
                self.disassembleCB__SET_6_B();
            }

            val if val == SHIFT_0X_CB + 0xf1 => {
                /* SET 6,C */
                self.disassembleCB__SET_6_C();
            }

            val if val == SHIFT_0X_CB + 0xf2 => {
                /* SET 6,D */
                self.disassembleCB__SET_6_D();
            }

            val if val == SHIFT_0X_CB + 0xf3 => {
                /* SET 6,E */
                self.disassembleCB__SET_6_E();
            }

            val if val == SHIFT_0X_CB + 0xf4 => {
                /* SET 6,H */
                self.disassembleCB__SET_6_H();
            }

            val if val == SHIFT_0X_CB + 0xf5 => {
                /* SET 6,L */
                self.disassembleCB__SET_6_L();
            }

            val if val == SHIFT_0X_CB + 0xf6 => {
                /* SET 6,(HL) */
                self.disassembleCB__SET_6_iHL();
            }

            val if val == SHIFT_0X_CB + 0xf7 => {
                /* SET 6,A */
                self.disassembleCB__SET_6_A();
            }

            val if val == SHIFT_0X_CB + 0xf8 => {
                /* SET 7,B */
                self.disassembleCB__SET_7_B();
            }

            val if val == SHIFT_0X_CB + 0xf9 => {
                /* SET 7,C */
                self.disassembleCB__SET_7_C();
            }

            val if val == SHIFT_0X_CB + 0xfa => {
                /* SET 7,D */
                self.disassembleCB__SET_7_D();
            }

            val if val == SHIFT_0X_CB + 0xfb => {
                /* SET 7,E */
                self.disassembleCB__SET_7_E();
            }

            val if val == SHIFT_0X_CB + 0xfc => {
                /* SET 7,H */
                self.disassembleCB__SET_7_H();
            }

            val if val == SHIFT_0X_CB + 0xfd => {
                /* SET 7,L */
                self.disassembleCB__SET_7_L();
            }

            val if val == SHIFT_0X_CB + 0xfe => {
                /* SET 7,(HL) */
                self.disassembleCB__SET_7_iHL();
            }

            val if val == SHIFT_0X_CB + 0xff => {
                /* SET 7,A */
                self.disassembleCB__SET_7_A();
            }

            val if val == SHIFT_0X_ED + 0x40 => {
                /* IN B,(C) */
                self.disassembleED__IN_B_iC();
            }

            val if val == SHIFT_0X_ED + 0x41 => {
                /* OUT (C),B */
                self.disassembleED__OUT_iC_B();
            }

            val if val == SHIFT_0X_ED + 0x42 => {
                /* SBC HL,BC */
                self.disassembleED__SBC_HL_BC();
            }

            val if val == SHIFT_0X_ED + 0x43 => {
                /* LD (nnnn),BC */
                self.disassembleED__LD_iNNNN_BC();
            }

            val if val == SHIFT_0X_ED + 0x7c => {
                /* NEG */
                self.disassembleED__NEG();
            }

            val if val == SHIFT_0X_ED + 0x44 => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();
                self.disassembleED__NEG();
            }

            val if val == SHIFT_0X_ED + 0x4c => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();
                self.disassembleED__NEG();
            }

            val if val == SHIFT_0X_ED + 0x54 => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();
                self.disassembleED__NEG();
            }

            val if val == SHIFT_0X_ED + 0x5c => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();
                self.disassembleED__NEG();
            }

            val if val == SHIFT_0X_ED + 0x64 => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();
                self.disassembleED__NEG();
            }

            val if val == SHIFT_0X_ED + 0x6c => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();
                self.disassembleED__NEG();
            }

            val if val == SHIFT_0X_ED + 0x74 => {
                /* NEG */
                // self.OpcodesMap[SHIFT_0xED + 0x7c]();
                self.disassembleED__NEG();
            }

            val if val == SHIFT_0X_ED + 0x7d => {
                /* RETN */
                self.disassembleED__RETN();
            }

            val if val == SHIFT_0X_ED + 0x45 => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();
                self.disassembleED__RETN();
            }

            val if val == SHIFT_0X_ED + 0x4d => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();
                self.disassembleED__RETN();
            }

            val if val == SHIFT_0X_ED + 0x55 => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();
                self.disassembleED__RETN();
            }

            val if val == SHIFT_0X_ED + 0x5d => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();
                self.disassembleED__RETN();
            }

            val if val == SHIFT_0X_ED + 0x65 => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();
                self.disassembleED__RETN();
            }

            val if val == SHIFT_0X_ED + 0x6d => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();
                self.disassembleED__RETN();
            }

            val if val == SHIFT_0X_ED + 0x75 => {
                /* RETN */
                // self.OpcodesMap[SHIFT_0xED + 0x7d]();
                self.disassembleED__RETN();
            }

            val if val == SHIFT_0X_ED + 0x6e => {
                /* IM 0 */
                self.disassembleED__IM_0();
            }

            val if val == SHIFT_0X_ED + 0x46 => {
                /* IM 0 */
                // self.OpcodesMap[SHIFT_0xED + 0x6e]();
                self.disassembleED__IM_0();
            }

            val if val == SHIFT_0X_ED + 0x4e => {
                /* IM 0 */
                // self.OpcodesMap[SHIFT_0xED + 0x6e]();
                self.disassembleED__IM_0();
            }

            val if val == SHIFT_0X_ED + 0x66 => {
                /* IM 0 */
                // self.OpcodesMap[SHIFT_0xED + 0x6e]();
                self.disassembleED__IM_0();
            }

            val if val == SHIFT_0X_ED + 0x47 => {
                /* LD I,A */
                self.disassembleED__LD_I_A();
            }

            val if val == SHIFT_0X_ED + 0x48 => {
                /* IN C,(C) */
                self.disassembleED__IN_C_iC();
            }

            val if val == SHIFT_0X_ED + 0x49 => {
                /* OUT (C),C */
                self.disassembleED__OUT_iC_C();
            }

            val if val == SHIFT_0X_ED + 0x4a => {
                /* ADC HL,BC */
                self.disassembleED__ADC_HL_BC();
            }

            val if val == SHIFT_0X_ED + 0x4b => {
                /* LD BC,(nnnn) */
                self.disassembleED__LD_BC_iNNNN();
            }

            val if val == SHIFT_0X_ED + 0x4f => {
                /* LD R,A */
                self.disassembleED__LD_R_A();
            }

            val if val == SHIFT_0X_ED + 0x50 => {
                /* IN D,(C) */
                self.disassembleED__IN_D_iC();
            }

            val if val == SHIFT_0X_ED + 0x51 => {
                /* OUT (C),D */
                self.disassembleED__OUT_iC_D();
            }

            val if val == SHIFT_0X_ED + 0x52 => {
                /* SBC HL,DE */
                self.disassembleED__SBC_HL_DE();
            }

            val if val == SHIFT_0X_ED + 0x53 => {
                /* LD (nnnn),DE */
                self.disassembleED__LD_iNNNN_DE();
            }

            val if val == SHIFT_0X_ED + 0x76 => {
                /* IM 1 */
                self.disassembleED__IM_1();
            }

            val if val == SHIFT_0X_ED + 0x56 => {
                /* IM 1 */
                // self.OpcodesMap[SHIFT_0xED + 0x76]();
                self.disassembleED__IM_1();
            }

            val if val == SHIFT_0X_ED + 0x57 => {
                /* LD A,I */
                self.disassembleED__LD_A_I();
            }

            val if val == SHIFT_0X_ED + 0x58 => {
                /* IN E,(C) */
                self.disassembleED__IN_E_iC();
            }

            val if val == SHIFT_0X_ED + 0x59 => {
                /* OUT (C),E */
                self.disassembleED__OUT_iC_E();
            }

            val if val == SHIFT_0X_ED + 0x5a => {
                /* ADC HL,DE */
                self.disassembleED__ADC_HL_DE();
            }

            val if val == SHIFT_0X_ED + 0x5b => {
                /* LD DE,(nnnn) */
                self.disassembleED__LD_DE_iNNNN();
            }

            val if val == SHIFT_0X_ED + 0x7e => {
                /* IM 2 */
                self.disassembleED__IM_2();
            }

            val if val == SHIFT_0X_ED + 0x5e => {
                /* IM 2 */
                // self.OpcodesMap[SHIFT_0xED + 0x7e]();
                self.disassembleED__IM_2();
            }

            val if val == SHIFT_0X_ED + 0x5f => {
                /* LD A,R */
                self.disassembleED__LD_A_R();
            }

            val if val == SHIFT_0X_ED + 0x60 => {
                /* IN H,(C) */
                self.disassembleED__IN_H_iC();
            }

            val if val == SHIFT_0X_ED + 0x61 => {
                /* OUT (C),H */
                self.disassembleED__OUT_iC_H();
            }

            val if val == SHIFT_0X_ED + 0x62 => {
                /* SBC HL,HL */
                self.disassembleED__SBC_HL_HL();
            }

            val if val == SHIFT_0X_ED + 0x63 => {
                /* LD (nnnn),HL */
                self.disassembleED__LD_iNNNN_HL();
            }

            val if val == SHIFT_0X_ED + 0x67 => {
                /* RRD */
                self.disassembleED__RRD();
            }

            val if val == SHIFT_0X_ED + 0x68 => {
                /* IN L,(C) */
                self.disassembleED__IN_L_iC();
            }

            val if val == SHIFT_0X_ED + 0x69 => {
                /* OUT (C),L */
                self.disassembleED__OUT_iC_L();
            }

            val if val == SHIFT_0X_ED + 0x6a => {
                /* ADC HL,HL */
                self.disassembleED__ADC_HL_HL();
            }

            val if val == SHIFT_0X_ED + 0x6b => {
                /* LD HL,(nnnn) */
                self.disassembleED__LD_HL_iNNNN();
            }

            val if val == SHIFT_0X_ED + 0x6f => {
                /* RLD */
                self.disassembleED__RLD();
            }

            val if val == SHIFT_0X_ED + 0x70 => {
                /* IN F,(C) */
                self.disassembleED__IN_F_iC();
            }

            val if val == SHIFT_0X_ED + 0x71 => {
                /* OUT (C),0 */
                self.disassembleED__OUT_iC_0();
            }

            val if val == SHIFT_0X_ED + 0x72 => {
                /* SBC HL,SP */
                self.disassembleED__SBC_HL_SP();
            }

            val if val == SHIFT_0X_ED + 0x73 => {
                /* LD (nnnn),SP */
                self.disassembleED__LD_iNNNN_SP();
            }

            val if val == SHIFT_0X_ED + 0x78 => {
                /* IN A,(C) */
                self.disassembleED__IN_A_iC();
            }

            val if val == SHIFT_0X_ED + 0x79 => {
                /* OUT (C),A */
                self.disassembleED__OUT_iC_A();
            }

            val if val == SHIFT_0X_ED + 0x7a => {
                /* ADC HL,SP */
                self.disassembleED__ADC_HL_SP();
            }

            val if val == SHIFT_0X_ED + 0x7b => {
                /* LD SP,(nnnn) */
                self.disassembleED__LD_SP_iNNNN();
            }

            val if val == SHIFT_0X_ED + 0xa0 => {
                /* LDI */
                self.disassembleED__LDI();
            }

            val if val == SHIFT_0X_ED + 0xa1 => {
                /* CPI */
                self.disassembleED__CPI();
            }

            val if val == SHIFT_0X_ED + 0xa2 => {
                /* INI */
                self.disassembleED__INI();
            }

            val if val == SHIFT_0X_ED + 0xa3 => {
                /* OUTI */
                self.disassembleED__OUTI();
            }

            val if val == SHIFT_0X_ED + 0xa8 => {
                /* LDD */
                self.disassembleED__LDD();
            }

            val if val == SHIFT_0X_ED + 0xa9 => {
                /* CPD */
                self.disassembleED__CPD();
            }

            val if val == SHIFT_0X_ED + 0xaa => {
                /* IND */
                self.disassembleED__IND();
            }

            val if val == SHIFT_0X_ED + 0xab => {
                /* OUTD */
                self.disassembleED__OUTD();
            }

            val if val == SHIFT_0X_ED + 0xb0 => {
                /* LDIR */
                self.disassembleED__LDIR();
            }

            val if val == SHIFT_0X_ED + 0xb1 => {
                /* CPIR */
                self.disassembleED__CPIR();
            }

            val if val == SHIFT_0X_ED + 0xb2 => {
                /* INIR */
                self.disassembleED__INIR();
            }

            val if val == SHIFT_0X_ED + 0xb3 => {
                /* OTIR */
                self.disassembleED__OTIR();
            }

            val if val == SHIFT_0X_ED + 0xb8 => {
                /* LDDR */
                self.disassembleED__LDDR();
            }

            val if val == SHIFT_0X_ED + 0xb9 => {
                /* CPDR */
                self.disassembleED__CPDR();
            }

            val if val == SHIFT_0X_ED + 0xba => {
                /* INDR */
                self.disassembleED__INDR();
            }

            val if val == SHIFT_0X_ED + 0xbb => {
                /* OTDR */
                self.disassembleED__OTDR();
            }

            val if val == SHIFT_0X_ED + 0xfb => {
                /* slttrap */
                self.disassembleED__SLTTRAP();
            }

            val if val == SHIFT_0X_DD + 0x09 => {
                /* ADD REGISTER,BC */
                self.disassembleDD__ADD_REG_BC();
            }

            val if val == SHIFT_0X_DD + 0x19 => {
                /* ADD REGISTER,DE */
                self.disassembleDD__ADD_REG_DE();
            }

            val if val == SHIFT_0X_DD + 0x21 => {
                /* LD REGISTER,nnnn */
                self.disassembleDD__LD_REG_NNNN();
            }

            val if val == SHIFT_0X_DD + 0x22 => {
                /* LD (nnnn),REGISTER */
                self.disassembleDD__LD_iNNNN_REG();
            }

            val if val == SHIFT_0X_DD + 0x23 => {
                /* INC REGISTER */
                self.disassembleDD__INC_REG();
            }

            val if val == SHIFT_0X_DD + 0x24 => {
                /* INC REGISTERH */
                self.disassembleDD__INC_REGH();
            }

            val if val == SHIFT_0X_DD + 0x25 => {
                /* DEC REGISTERH */
                self.disassembleDD__DEC_REGH();
            }

            val if val == SHIFT_0X_DD + 0x26 => {
                /* LD REGISTERH,nn */
                self.disassembleDD__LD_REGH_NN();
            }

            val if val == SHIFT_0X_DD + 0x29 => {
                /* ADD REGISTER,REGISTER */
                self.disassembleDD__ADD_REG_REG();
            }

            val if val == SHIFT_0X_DD + 0x2a => {
                /* LD REGISTER,(nnnn) */
                self.disassembleDD__LD_REG_iNNNN();
            }

            val if val == SHIFT_0X_DD + 0x2b => {
                /* DEC REGISTER */
                self.disassembleDD__DEC_REG();
            }

            val if val == SHIFT_0X_DD + 0x2c => {
                /* INC REGISTERL */
                self.disassembleDD__INC_REGL();
            }

            val if val == SHIFT_0X_DD + 0x2d => {
                /* DEC REGISTERL */
                self.disassembleDD__DEC_REGL();
            }

            val if val == SHIFT_0X_DD + 0x2e => {
                /* LD REGISTERL,nn */
                self.disassembleDD__LD_REGL_NN();
            }

            val if val == SHIFT_0X_DD + 0x34 => {
                /* INC (REGISTER+dd) */
                self.disassembleDD__INC_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x35 => {
                /* DEC (REGISTER+dd) */
                self.disassembleDD__DEC_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x36 => {
                /* LD (REGISTER+dd),nn */
                self.disassembleDD__LD_iREGpDD_NN();
            }

            val if val == SHIFT_0X_DD + 0x39 => {
                /* ADD REGISTER,SP */
                self.disassembleDD__ADD_REG_SP();
            }

            val if val == SHIFT_0X_DD + 0x44 => {
                /* LD B,REGISTERH */
                self.disassembleDD__LD_B_REGH();
            }

            val if val == SHIFT_0X_DD + 0x45 => {
                /* LD B,REGISTERL */
                self.disassembleDD__LD_B_REGL();
            }

            val if val == SHIFT_0X_DD + 0x46 => {
                /* LD B,(REGISTER+dd) */
                self.disassembleDD__LD_B_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x4c => {
                /* LD C,REGISTERH */
                self.disassembleDD__LD_C_REGH();
            }

            val if val == SHIFT_0X_DD + 0x4d => {
                /* LD C,REGISTERL */
                self.disassembleDD__LD_C_REGL();
            }

            val if val == SHIFT_0X_DD + 0x4e => {
                /* LD C,(REGISTER+dd) */
                self.disassembleDD__LD_C_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x54 => {
                /* LD D,REGISTERH */
                self.disassembleDD__LD_D_REGH();
            }

            val if val == SHIFT_0X_DD + 0x55 => {
                /* LD D,REGISTERL */
                self.disassembleDD__LD_D_REGL();
            }

            val if val == SHIFT_0X_DD + 0x56 => {
                /* LD D,(REGISTER+dd) */
                self.disassembleDD__LD_D_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x5c => {
                /* LD E,REGISTERH */
                self.disassembleDD__LD_E_REGH();
            }

            val if val == SHIFT_0X_DD + 0x5d => {
                /* LD E,REGISTERL */
                self.disassembleDD__LD_E_REGL();
            }

            val if val == SHIFT_0X_DD + 0x5e => {
                /* LD E,(REGISTER+dd) */
                self.disassembleDD__LD_E_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x60 => {
                /* LD REGISTERH,B */
                self.disassembleDD__LD_REGH_B();
            }

            val if val == SHIFT_0X_DD + 0x61 => {
                /* LD REGISTERH,C */
                self.disassembleDD__LD_REGH_C();
            }

            val if val == SHIFT_0X_DD + 0x62 => {
                /* LD REGISTERH,D */
                self.disassembleDD__LD_REGH_D();
            }

            val if val == SHIFT_0X_DD + 0x63 => {
                /* LD REGISTERH,E */
                self.disassembleDD__LD_REGH_E();
            }

            val if val == SHIFT_0X_DD + 0x64 => {
                /* LD REGISTERH,REGISTERH */
                self.disassembleDD__LD_REGH_REGH();
            }

            val if val == SHIFT_0X_DD + 0x65 => {
                /* LD REGISTERH,REGISTERL */
                self.disassembleDD__LD_REGH_REGL();
            }

            val if val == SHIFT_0X_DD + 0x66 => {
                /* LD H,(REGISTER+dd) */
                self.disassembleDD__LD_H_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x67 => {
                /* LD REGISTERH,A */
                self.disassembleDD__LD_REGH_A();
            }

            val if val == SHIFT_0X_DD + 0x68 => {
                /* LD REGISTERL,B */
                self.disassembleDD__LD_REGL_B();
            }

            val if val == SHIFT_0X_DD + 0x69 => {
                /* LD REGISTERL,C */
                self.disassembleDD__LD_REGL_C();
            }

            val if val == SHIFT_0X_DD + 0x6a => {
                /* LD REGISTERL,D */
                self.disassembleDD__LD_REGL_D();
            }

            val if val == SHIFT_0X_DD + 0x6b => {
                /* LD REGISTERL,E */
                self.disassembleDD__LD_REGL_E();
            }

            val if val == SHIFT_0X_DD + 0x6c => {
                /* LD REGISTERL,REGISTERH */
                self.disassembleDD__LD_REGL_REGH();
            }

            val if val == SHIFT_0X_DD + 0x6d => {
                /* LD REGISTERL,REGISTERL */
                self.disassembleDD__LD_REGL_REGL();
            }

            val if val == SHIFT_0X_DD + 0x6e => {
                /* LD L,(REGISTER+dd) */
                self.disassembleDD__LD_L_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x6f => {
                /* LD REGISTERL,A */
                self.disassembleDD__LD_REGL_A();
            }

            val if val == SHIFT_0X_DD + 0x70 => {
                /* LD (REGISTER+dd),B */
                self.disassembleDD__LD_iREGpDD_B();
            }

            val if val == SHIFT_0X_DD + 0x71 => {
                /* LD (REGISTER+dd),C */
                self.disassembleDD__LD_iREGpDD_C();
            }

            val if val == SHIFT_0X_DD + 0x72 => {
                /* LD (REGISTER+dd),D */
                self.disassembleDD__LD_iREGpDD_D();
            }

            val if val == SHIFT_0X_DD + 0x73 => {
                /* LD (REGISTER+dd),E */
                self.disassembleDD__LD_iREGpDD_E();
            }

            val if val == SHIFT_0X_DD + 0x74 => {
                /* LD (REGISTER+dd),H */
                self.disassembleDD__LD_iREGpDD_H();
            }

            val if val == SHIFT_0X_DD + 0x75 => {
                /* LD (REGISTER+dd),L */
                self.disassembleDD__LD_iREGpDD_L();
            }

            val if val == SHIFT_0X_DD + 0x77 => {
                /* LD (REGISTER+dd),A */
                self.disassembleDD__LD_iREGpDD_A();
            }

            val if val == SHIFT_0X_DD + 0x7c => {
                /* LD A,REGISTERH */
                self.disassembleDD__LD_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0x7d => {
                /* LD A,REGISTERL */
                self.disassembleDD__LD_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0x7e => {
                /* LD A,(REGISTER+dd) */
                self.disassembleDD__LD_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x84 => {
                /* ADD A,REGISTERH */
                self.disassembleDD__ADD_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0x85 => {
                /* ADD A,REGISTERL */
                self.disassembleDD__ADD_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0x86 => {
                /* ADD A,(REGISTER+dd) */
                self.disassembleDD__ADD_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x8c => {
                /* ADC A,REGISTERH */
                self.disassembleDD__ADC_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0x8d => {
                /* ADC A,REGISTERL */
                self.disassembleDD__ADC_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0x8e => {
                /* ADC A,(REGISTER+dd) */
                self.disassembleDD__ADC_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x94 => {
                /* SUB A,REGISTERH */
                self.disassembleDD__SUB_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0x95 => {
                /* SUB A,REGISTERL */
                self.disassembleDD__SUB_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0x96 => {
                /* SUB A,(REGISTER+dd) */
                self.disassembleDD__SUB_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0x9c => {
                /* SBC A,REGISTERH */
                self.disassembleDD__SBC_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0x9d => {
                /* SBC A,REGISTERL */
                self.disassembleDD__SBC_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0x9e => {
                /* SBC A,(REGISTER+dd) */
                self.disassembleDD__SBC_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0xa4 => {
                /* AND A,REGISTERH */
                self.disassembleDD__AND_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0xa5 => {
                /* AND A,REGISTERL */
                self.disassembleDD__AND_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0xa6 => {
                /* AND A,(REGISTER+dd) */
                self.disassembleDD__AND_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0xac => {
                /* XOR A,REGISTERH */
                self.disassembleDD__XOR_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0xad => {
                /* XOR A,REGISTERL */
                self.disassembleDD__XOR_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0xae => {
                /* XOR A,(REGISTER+dd) */
                self.disassembleDD__XOR_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0xb4 => {
                /* OR A,REGISTERH */
                self.disassembleDD__OR_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0xb5 => {
                /* OR A,REGISTERL */
                self.disassembleDD__OR_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0xb6 => {
                /* OR A,(REGISTER+dd) */
                self.disassembleDD__OR_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0xbc => {
                /* CP A,REGISTERH */
                self.disassembleDD__CP_A_REGH();
            }

            val if val == SHIFT_0X_DD + 0xbd => {
                /* CP A,REGISTERL */
                self.disassembleDD__CP_A_REGL();
            }

            val if val == SHIFT_0X_DD + 0xbe => {
                /* CP A,(REGISTER+dd) */
                self.disassembleDD__CP_A_iREGpDD();
            }

            val if val == SHIFT_0X_DD + 0xcb => {
                /* shift DDFDCB */
                self.disassembleDD__SHIFT_DDFDCB();
            }

            val if val == SHIFT_0X_DD + 0xe1 => {
                /* POP REGISTER */
                self.disassembleDD__POP_REG();
            }

            val if val == SHIFT_0X_DD + 0xe3 => {
                /* EX (SP),REGISTER */
                self.disassembleDD__EX_iSP_REG();
            }

            val if val == SHIFT_0X_DD + 0xe5 => {
                /* PUSH REGISTER */
                self.disassembleDD__PUSH_REG();
            }

            val if val == SHIFT_0X_DD + 0xe9 => {
                /* JP REGISTER */
                self.disassembleDD__JP_REG();
            }

            val if val == SHIFT_0X_DD + 0xf9 => {
                /* LD SP,REGISTER */
                self.disassembleDD__LD_SP_REG();
            }

            val if val == SHIFT_0X_FD + 0x09 => {
                /* ADD REGISTER,BC */
                self.disassembleFD__ADD_REG_BC();
            }

            val if val == SHIFT_0X_FD + 0x19 => {
                /* ADD REGISTER,DE */
                self.disassembleFD__ADD_REG_DE();
            }

            val if val == SHIFT_0X_FD + 0x21 => {
                /* LD REGISTER,nnnn */
                self.disassembleFD__LD_REG_NNNN();
            }

            val if val == SHIFT_0X_FD + 0x22 => {
                /* LD (nnnn),REGISTER */
                self.disassembleFD__LD_iNNNN_REG();
            }

            val if val == SHIFT_0X_FD + 0x23 => {
                /* INC REGISTER */
                self.disassembleFD__INC_REG();
            }

            val if val == SHIFT_0X_FD + 0x24 => {
                /* INC REGISTERH */
                self.disassembleFD__INC_REGH();
            }

            val if val == SHIFT_0X_FD + 0x25 => {
                /* DEC REGISTERH */
                self.disassembleFD__DEC_REGH();
            }

            val if val == SHIFT_0X_FD + 0x26 => {
                /* LD REGISTERH,nn */
                self.disassembleFD__LD_REGH_NN();
            }

            val if val == SHIFT_0X_FD + 0x29 => {
                /* ADD REGISTER,REGISTER */
                self.disassembleFD__ADD_REG_REG();
            }

            val if val == SHIFT_0X_FD + 0x2a => {
                /* LD REGISTER,(nnnn) */
                self.disassembleFD__LD_REG_iNNNN();
            }

            val if val == SHIFT_0X_FD + 0x2b => {
                /* DEC REGISTER */
                self.disassembleFD__DEC_REG();
            }

            val if val == SHIFT_0X_FD + 0x2c => {
                /* INC REGISTERL */
                self.disassembleFD__INC_REGL();
            }

            val if val == SHIFT_0X_FD + 0x2d => {
                /* DEC REGISTERL */
                self.disassembleFD__DEC_REGL();
            }

            val if val == SHIFT_0X_FD + 0x2e => {
                /* LD REGISTERL,nn */
                self.disassembleFD__LD_REGL_NN();
            }

            val if val == SHIFT_0X_FD + 0x34 => {
                /* INC (REGISTER+dd) */
                self.disassembleFD__INC_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x35 => {
                /* DEC (REGISTER+dd) */
                self.disassembleFD__DEC_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x36 => {
                /* LD (REGISTER+dd),nn */
                self.disassembleFD__LD_iREGpDD_NN();
            }

            val if val == SHIFT_0X_FD + 0x39 => {
                /* ADD REGISTER,SP */
                self.disassembleFD__ADD_REG_SP();
            }

            val if val == SHIFT_0X_FD + 0x44 => {
                /* LD B,REGISTERH */
                self.disassembleFD__LD_B_REGH();
            }

            val if val == SHIFT_0X_FD + 0x45 => {
                /* LD B,REGISTERL */
                self.disassembleFD__LD_B_REGL();
            }

            val if val == SHIFT_0X_FD + 0x46 => {
                /* LD B,(REGISTER+dd) */
                self.disassembleFD__LD_B_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x4c => {
                /* LD C,REGISTERH */
                self.disassembleFD__LD_C_REGH();
            }

            val if val == SHIFT_0X_FD + 0x4d => {
                /* LD C,REGISTERL */
                self.disassembleFD__LD_C_REGL();
            }

            val if val == SHIFT_0X_FD + 0x4e => {
                /* LD C,(REGISTER+dd) */
                self.disassembleFD__LD_C_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x54 => {
                /* LD D,REGISTERH */
                self.disassembleFD__LD_D_REGH();
            }

            val if val == SHIFT_0X_FD + 0x55 => {
                /* LD D,REGISTERL */
                self.disassembleFD__LD_D_REGL();
            }

            val if val == SHIFT_0X_FD + 0x56 => {
                /* LD D,(REGISTER+dd) */
                self.disassembleFD__LD_D_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x5c => {
                /* LD E,REGISTERH */
                self.disassembleFD__LD_E_REGH();
            }

            val if val == SHIFT_0X_FD + 0x5d => {
                /* LD E,REGISTERL */
                self.disassembleFD__LD_E_REGL();
            }

            val if val == SHIFT_0X_FD + 0x5e => {
                /* LD E,(REGISTER+dd) */
                self.disassembleFD__LD_E_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x60 => {
                /* LD REGISTERH,B */
                self.disassembleFD__LD_REGH_B();
            }

            val if val == SHIFT_0X_FD + 0x61 => {
                /* LD REGISTERH,C */
                self.disassembleFD__LD_REGH_C();
            }

            val if val == SHIFT_0X_FD + 0x62 => {
                /* LD REGISTERH,D */
                self.disassembleFD__LD_REGH_D();
            }

            val if val == SHIFT_0X_FD + 0x63 => {
                /* LD REGISTERH,E */
                self.disassembleFD__LD_REGH_E();
            }

            val if val == SHIFT_0X_FD + 0x64 => {
                /* LD REGISTERH,REGISTERH */
                self.disassembleFD__LD_REGH_REGH();
            }

            val if val == SHIFT_0X_FD + 0x65 => {
                /* LD REGISTERH,REGISTERL */
                self.disassembleFD__LD_REGH_REGL();
            }

            val if val == SHIFT_0X_FD + 0x66 => {
                /* LD H,(REGISTER+dd) */
                self.disassembleFD__LD_H_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x67 => {
                /* LD REGISTERH,A */
                self.disassembleFD__LD_REGH_A();
            }

            val if val == SHIFT_0X_FD + 0x68 => {
                /* LD REGISTERL,B */
                self.disassembleFD__LD_REGL_B();
            }

            val if val == SHIFT_0X_FD + 0x69 => {
                /* LD REGISTERL,C */
                self.disassembleFD__LD_REGL_C();
            }

            val if val == SHIFT_0X_FD + 0x6a => {
                /* LD REGISTERL,D */
                self.disassembleFD__LD_REGL_D();
            }

            val if val == SHIFT_0X_FD + 0x6b => {
                /* LD REGISTERL,E */
                self.disassembleFD__LD_REGL_E();
            }

            val if val == SHIFT_0X_FD + 0x6c => {
                /* LD REGISTERL,REGISTERH */
                self.disassembleFD__LD_REGL_REGH();
            }

            val if val == SHIFT_0X_FD + 0x6d => {
                /* LD REGISTERL,REGISTERL */
                self.disassembleFD__LD_REGL_REGL();
            }

            val if val == SHIFT_0X_FD + 0x6e => {
                /* LD L,(REGISTER+dd) */
                self.disassembleFD__LD_L_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x6f => {
                /* LD REGISTERL,A */
                self.disassembleFD__LD_REGL_A();
            }

            val if val == SHIFT_0X_FD + 0x70 => {
                /* LD (REGISTER+dd),B */
                self.disassembleFD__LD_iREGpDD_B();
            }

            val if val == SHIFT_0X_FD + 0x71 => {
                /* LD (REGISTER+dd),C */
                self.disassembleFD__LD_iREGpDD_C();
            }

            val if val == SHIFT_0X_FD + 0x72 => {
                /* LD (REGISTER+dd),D */
                self.disassembleFD__LD_iREGpDD_D();
            }

            val if val == SHIFT_0X_FD + 0x73 => {
                /* LD (REGISTER+dd),E */
                self.disassembleFD__LD_iREGpDD_E();
            }

            val if val == SHIFT_0X_FD + 0x74 => {
                /* LD (REGISTER+dd),H */
                self.disassembleFD__LD_iREGpDD_H();
            }

            val if val == SHIFT_0X_FD + 0x75 => {
                /* LD (REGISTER+dd),L */
                self.disassembleFD__LD_iREGpDD_L();
            }

            val if val == SHIFT_0X_FD + 0x77 => {
                /* LD (REGISTER+dd),A */
                self.disassembleFD__LD_iREGpDD_A();
            }

            val if val == SHIFT_0X_FD + 0x7c => {
                /* LD A,REGISTERH */
                self.disassembleFD__LD_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0x7d => {
                /* LD A,REGISTERL */
                self.disassembleFD__LD_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0x7e => {
                /* LD A,(REGISTER+dd) */
                self.disassembleFD__LD_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x84 => {
                /* ADD A,REGISTERH */
                self.disassembleFD__ADD_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0x85 => {
                /* ADD A,REGISTERL */
                self.disassembleFD__ADD_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0x86 => {
                /* ADD A,(REGISTER+dd) */
                self.disassembleFD__ADD_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x8c => {
                /* ADC A,REGISTERH */
                self.disassembleFD__ADC_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0x8d => {
                /* ADC A,REGISTERL */
                self.disassembleFD__ADC_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0x8e => {
                /* ADC A,(REGISTER+dd) */
                self.disassembleFD__ADC_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x94 => {
                /* SUB A,REGISTERH */
                self.disassembleFD__SUB_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0x95 => {
                /* SUB A,REGISTERL */
                self.disassembleFD__SUB_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0x96 => {
                /* SUB A,(REGISTER+dd) */
                self.disassembleFD__SUB_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0x9c => {
                /* SBC A,REGISTERH */
                self.disassembleFD__SBC_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0x9d => {
                /* SBC A,REGISTERL */
                self.disassembleFD__SBC_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0x9e => {
                /* SBC A,(REGISTER+dd) */
                self.disassembleFD__SBC_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0xa4 => {
                /* AND A,REGISTERH */
                self.disassembleFD__AND_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0xa5 => {
                /* AND A,REGISTERL */
                self.disassembleFD__AND_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0xa6 => {
                /* AND A,(REGISTER+dd) */
                self.disassembleFD__AND_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0xac => {
                /* XOR A,REGISTERH */
                self.disassembleFD__XOR_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0xad => {
                /* XOR A,REGISTERL */
                self.disassembleFD__XOR_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0xae => {
                /* XOR A,(REGISTER+dd) */
                self.disassembleFD__XOR_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0xb4 => {
                /* OR A,REGISTERH */
                self.disassembleFD__OR_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0xb5 => {
                /* OR A,REGISTERL */
                self.disassembleFD__OR_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0xb6 => {
                /* OR A,(REGISTER+dd) */
                self.disassembleFD__OR_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0xbc => {
                /* CP A,REGISTERH */
                self.disassembleFD__CP_A_REGH();
            }

            val if val == SHIFT_0X_FD + 0xbd => {
                /* CP A,REGISTERL */
                self.disassembleFD__CP_A_REGL();
            }

            val if val == SHIFT_0X_FD + 0xbe => {
                /* CP A,(REGISTER+dd) */
                self.disassembleFD__CP_A_iREGpDD();
            }

            val if val == SHIFT_0X_FD + 0xcb => {
                /* shift DDFDCB */
                self.disassembleFD__SHIFT_DDFDCB();
            }

            val if val == SHIFT_0X_FD + 0xe1 => {
                /* POP REGISTER */
                self.disassembleFD__POP_REG();
            }

            val if val == SHIFT_0X_FD + 0xe3 => {
                /* EX (SP),REGISTER */
                self.disassembleFD__EX_iSP_REG();
            }

            val if val == SHIFT_0X_FD + 0xe5 => {
                /* PUSH REGISTER */
                self.disassembleFD__PUSH_REG();
            }

            val if val == SHIFT_0X_FD + 0xe9 => {
                /* JP REGISTER */
                self.disassembleFD__JP_REG();
            }

            val if val == SHIFT_0X_FD + 0xf9 => {
                /* LD SP,REGISTER */
                self.disassembleFD__LD_SP_REG();
            }

            val if val == SHIFT_0X_DDCB => {
                /* LD B,RLC (REGISTER+dd) */
                self.disassembleDDCB__LD_B_RLC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x01 => {
                /* LD C,RLC (REGISTER+dd) */
                self.disassembleDDCB__LD_C_RLC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x02 => {
                /* LD D,RLC (REGISTER+dd) */
                self.disassembleDDCB__LD_D_RLC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x03 => {
                /* LD E,RLC (REGISTER+dd) */
                self.disassembleDDCB__LD_E_RLC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x04 => {
                /* LD H,RLC (REGISTER+dd) */
                self.disassembleDDCB__LD_H_RLC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x05 => {
                /* LD L,RLC (REGISTER+dd) */
                self.disassembleDDCB__LD_L_RLC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x06 => {
                /* RLC (REGISTER+dd) */
                self.disassembleDDCB__RLC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x07 => {
                /* LD A,RLC (REGISTER+dd) */
                self.disassembleDDCB__LD_A_RLC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x08 => {
                /* LD B,RRC (REGISTER+dd) */
                self.disassembleDDCB__LD_B_RRC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x09 => {
                /* LD C,RRC (REGISTER+dd) */
                self.disassembleDDCB__LD_C_RRC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x0a => {
                /* LD D,RRC (REGISTER+dd) */
                self.disassembleDDCB__LD_D_RRC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x0b => {
                /* LD E,RRC (REGISTER+dd) */
                self.disassembleDDCB__LD_E_RRC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x0c => {
                /* LD H,RRC (REGISTER+dd) */
                self.disassembleDDCB__LD_H_RRC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x0d => {
                /* LD L,RRC (REGISTER+dd) */
                self.disassembleDDCB__LD_L_RRC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x0e => {
                /* RRC (REGISTER+dd) */
                self.disassembleDDCB__RRC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x0f => {
                /* LD A,RRC (REGISTER+dd) */
                self.disassembleDDCB__LD_A_RRC_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x10 => {
                /* LD B,RL (REGISTER+dd) */
                self.disassembleDDCB__LD_B_RL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x11 => {
                /* LD C,RL (REGISTER+dd) */
                self.disassembleDDCB__LD_C_RL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x12 => {
                /* LD D,RL (REGISTER+dd) */
                self.disassembleDDCB__LD_D_RL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x13 => {
                /* LD E,RL (REGISTER+dd) */
                self.disassembleDDCB__LD_E_RL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x14 => {
                /* LD H,RL (REGISTER+dd) */
                self.disassembleDDCB__LD_H_RL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x15 => {
                /* LD L,RL (REGISTER+dd) */
                self.disassembleDDCB__LD_L_RL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x16 => {
                /* RL (REGISTER+dd) */
                self.disassembleDDCB__RL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x17 => {
                /* LD A,RL (REGISTER+dd) */
                self.disassembleDDCB__LD_A_RL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x18 => {
                /* LD B,RR (REGISTER+dd) */
                self.disassembleDDCB__LD_B_RR_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x19 => {
                /* LD C,RR (REGISTER+dd) */
                self.disassembleDDCB__LD_C_RR_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x1a => {
                /* LD D,RR (REGISTER+dd) */
                self.disassembleDDCB__LD_D_RR_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x1b => {
                /* LD E,RR (REGISTER+dd) */
                self.disassembleDDCB__LD_E_RR_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x1c => {
                /* LD H,RR (REGISTER+dd) */
                self.disassembleDDCB__LD_H_RR_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x1d => {
                /* LD L,RR (REGISTER+dd) */
                self.disassembleDDCB__LD_L_RR_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x1e => {
                /* RR (REGISTER+dd) */
                self.disassembleDDCB__RR_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x1f => {
                /* LD A,RR (REGISTER+dd) */
                self.disassembleDDCB__LD_A_RR_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x20 => {
                /* LD B,SLA (REGISTER+dd) */
                self.disassembleDDCB__LD_B_SLA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x21 => {
                /* LD C,SLA (REGISTER+dd) */
                self.disassembleDDCB__LD_C_SLA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x22 => {
                /* LD D,SLA (REGISTER+dd) */
                self.disassembleDDCB__LD_D_SLA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x23 => {
                /* LD E,SLA (REGISTER+dd) */
                self.disassembleDDCB__LD_E_SLA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x24 => {
                /* LD H,SLA (REGISTER+dd) */
                self.disassembleDDCB__LD_H_SLA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x25 => {
                /* LD L,SLA (REGISTER+dd) */
                self.disassembleDDCB__LD_L_SLA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x26 => {
                /* SLA (REGISTER+dd) */
                self.disassembleDDCB__SLA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x27 => {
                /* LD A,SLA (REGISTER+dd) */
                self.disassembleDDCB__LD_A_SLA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x28 => {
                /* LD B,SRA (REGISTER+dd) */
                self.disassembleDDCB__LD_B_SRA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x29 => {
                /* LD C,SRA (REGISTER+dd) */
                self.disassembleDDCB__LD_C_SRA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x2a => {
                /* LD D,SRA (REGISTER+dd) */
                self.disassembleDDCB__LD_D_SRA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x2b => {
                /* LD E,SRA (REGISTER+dd) */
                self.disassembleDDCB__LD_E_SRA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x2c => {
                /* LD H,SRA (REGISTER+dd) */
                self.disassembleDDCB__LD_H_SRA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x2d => {
                /* LD L,SRA (REGISTER+dd) */
                self.disassembleDDCB__LD_L_SRA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x2e => {
                /* SRA (REGISTER+dd) */
                self.disassembleDDCB__SRA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x2f => {
                /* LD A,SRA (REGISTER+dd) */
                self.disassembleDDCB__LD_A_SRA_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x30 => {
                /* LD B,SLL (REGISTER+dd) */
                self.disassembleDDCB__LD_B_SLL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x31 => {
                /* LD C,SLL (REGISTER+dd) */
                self.disassembleDDCB__LD_C_SLL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x32 => {
                /* LD D,SLL (REGISTER+dd) */
                self.disassembleDDCB__LD_D_SLL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x33 => {
                /* LD E,SLL (REGISTER+dd) */
                self.disassembleDDCB__LD_E_SLL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x34 => {
                /* LD H,SLL (REGISTER+dd) */
                self.disassembleDDCB__LD_H_SLL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x35 => {
                /* LD L,SLL (REGISTER+dd) */
                self.disassembleDDCB__LD_L_SLL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x36 => {
                /* SLL (REGISTER+dd) */
                self.disassembleDDCB__SLL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x37 => {
                /* LD A,SLL (REGISTER+dd) */
                self.disassembleDDCB__LD_A_SLL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x38 => {
                /* LD B,SRL (REGISTER+dd) */
                self.disassembleDDCB__LD_B_SRL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x39 => {
                /* LD C,SRL (REGISTER+dd) */
                self.disassembleDDCB__LD_C_SRL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x3a => {
                /* LD D,SRL (REGISTER+dd) */
                self.disassembleDDCB__LD_D_SRL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x3b => {
                /* LD E,SRL (REGISTER+dd) */
                self.disassembleDDCB__LD_E_SRL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x3c => {
                /* LD H,SRL (REGISTER+dd) */
                self.disassembleDDCB__LD_H_SRL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x3d => {
                /* LD L,SRL (REGISTER+dd) */
                self.disassembleDDCB__LD_L_SRL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x3e => {
                /* SRL (REGISTER+dd) */
                self.disassembleDDCB__SRL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x3f => {
                /* LD A,SRL (REGISTER+dd) */
                self.disassembleDDCB__LD_A_SRL_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x47 => {
                /* BIT 0,(REGISTER+dd) */
                self.disassembleDDCB__BIT_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x40 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();
                self.disassembleDDCB__BIT_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x41 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();
                self.disassembleDDCB__BIT_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x42 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();
                self.disassembleDDCB__BIT_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x43 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();
                self.disassembleDDCB__BIT_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x44 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();
                self.disassembleDDCB__BIT_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x45 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();
                self.disassembleDDCB__BIT_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x46 => {
                /* BIT 0,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x47]();
                self.disassembleDDCB__BIT_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x4f => {
                /* BIT 1,(REGISTER+dd) */
                self.disassembleDDCB__BIT_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x48 => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();
                self.disassembleDDCB__BIT_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x49 => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();
                self.disassembleDDCB__BIT_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x4a => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();
                self.disassembleDDCB__BIT_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x4b => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();
                self.disassembleDDCB__BIT_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x4c => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();
                self.disassembleDDCB__BIT_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x4d => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();
                self.disassembleDDCB__BIT_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x4e => {
                /* BIT 1,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x4f]();
                self.disassembleDDCB__BIT_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x57 => {
                /* BIT 2,(REGISTER+dd) */
                self.disassembleDDCB__BIT_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x50 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();
                self.disassembleDDCB__BIT_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x51 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();
                self.disassembleDDCB__BIT_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x52 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();
                self.disassembleDDCB__BIT_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x53 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();
                self.disassembleDDCB__BIT_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x54 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();
                self.disassembleDDCB__BIT_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x55 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();
                self.disassembleDDCB__BIT_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x56 => {
                /* BIT 2,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x57]();
                self.disassembleDDCB__BIT_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x5f => {
                /* BIT 3,(REGISTER+dd) */
                self.disassembleDDCB__BIT_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x58 => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();
                self.disassembleDDCB__BIT_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x59 => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();
                self.disassembleDDCB__BIT_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x5a => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();
                self.disassembleDDCB__BIT_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x5b => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();
                self.disassembleDDCB__BIT_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x5c => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();
                self.disassembleDDCB__BIT_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x5d => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();
                self.disassembleDDCB__BIT_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x5e => {
                /* BIT 3,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x5f]();
                self.disassembleDDCB__BIT_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x67 => {
                /* BIT 4,(REGISTER+dd) */
                self.disassembleDDCB__BIT_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x60 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();
                self.disassembleDDCB__BIT_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x61 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();
                self.disassembleDDCB__BIT_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x62 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();
                self.disassembleDDCB__BIT_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x63 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();
                self.disassembleDDCB__BIT_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x64 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();
                self.disassembleDDCB__BIT_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x65 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();
                self.disassembleDDCB__BIT_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x66 => {
                /* BIT 4,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x67]();
                self.disassembleDDCB__BIT_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x6f => {
                /* BIT 5,(REGISTER+dd) */
                self.disassembleDDCB__BIT_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x68 => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();
                self.disassembleDDCB__BIT_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x69 => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();
                self.disassembleDDCB__BIT_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x6a => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();
                self.disassembleDDCB__BIT_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x6b => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();
                self.disassembleDDCB__BIT_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x6c => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();
                self.disassembleDDCB__BIT_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x6d => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();
                self.disassembleDDCB__BIT_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x6e => {
                /* BIT 5,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x6f]();
                self.disassembleDDCB__BIT_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x77 => {
                /* BIT 6,(REGISTER+dd) */
                self.disassembleDDCB__BIT_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x70 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();
                self.disassembleDDCB__BIT_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x71 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();
                self.disassembleDDCB__BIT_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x72 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();
                self.disassembleDDCB__BIT_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x73 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();
                self.disassembleDDCB__BIT_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x74 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();
                self.disassembleDDCB__BIT_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x75 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();
                self.disassembleDDCB__BIT_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x76 => {
                /* BIT 6,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x77]();
                self.disassembleDDCB__BIT_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x7f => {
                /* BIT 7,(REGISTER+dd) */
                self.disassembleDDCB__BIT_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x78 => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();
                self.disassembleDDCB__BIT_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x79 => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();
                self.disassembleDDCB__BIT_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x7a => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();
                self.disassembleDDCB__BIT_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x7b => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();
                self.disassembleDDCB__BIT_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x7c => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();
                self.disassembleDDCB__BIT_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x7d => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();
                self.disassembleDDCB__BIT_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x7e => {
                /* BIT 7,(REGISTER+dd) */
                // self.OpcodesMap[SHIFT_0xDDCB + 0x7f]();
                self.disassembleDDCB__BIT_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x80 => {
                /* LD B,RES 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_RES_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x81 => {
                /* LD C,RES 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_RES_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x82 => {
                /* LD D,RES 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_RES_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x83 => {
                /* LD E,RES 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_RES_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x84 => {
                /* LD H,RES 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_RES_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x85 => {
                /* LD L,RES 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_RES_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x86 => {
                /* RES 0,(REGISTER+dd) */
                self.disassembleDDCB__RES_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x87 => {
                /* LD A,RES 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_RES_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x88 => {
                /* LD B,RES 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_RES_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x89 => {
                /* LD C,RES 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_RES_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x8a => {
                /* LD D,RES 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_RES_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x8b => {
                /* LD E,RES 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_RES_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x8c => {
                /* LD H,RES 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_RES_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x8d => {
                /* LD L,RES 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_RES_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x8e => {
                /* RES 1,(REGISTER+dd) */
                self.disassembleDDCB__RES_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x8f => {
                /* LD A,RES 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_RES_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x90 => {
                /* LD B,RES 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_RES_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x91 => {
                /* LD C,RES 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_RES_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x92 => {
                /* LD D,RES 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_RES_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x93 => {
                /* LD E,RES 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_RES_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x94 => {
                /* LD H,RES 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_RES_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x95 => {
                /* LD L,RES 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_RES_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x96 => {
                /* RES 2,(REGISTER+dd) */
                self.disassembleDDCB__RES_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x97 => {
                /* LD A,RES 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_RES_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x98 => {
                /* LD B,RES 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_RES_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x99 => {
                /* LD C,RES 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_RES_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x9a => {
                /* LD D,RES 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_RES_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x9b => {
                /* LD E,RES 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_RES_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x9c => {
                /* LD H,RES 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_RES_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x9d => {
                /* LD L,RES 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_RES_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x9e => {
                /* RES 3,(REGISTER+dd) */
                self.disassembleDDCB__RES_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0x9f => {
                /* LD A,RES 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_RES_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa0 => {
                /* LD B,RES 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_RES_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa1 => {
                /* LD C,RES 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_RES_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa2 => {
                /* LD D,RES 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_RES_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa3 => {
                /* LD E,RES 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_RES_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa4 => {
                /* LD H,RES 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_RES_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa5 => {
                /* LD L,RES 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_RES_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa6 => {
                /* RES 4,(REGISTER+dd) */
                self.disassembleDDCB__RES_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa7 => {
                /* LD A,RES 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_RES_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa8 => {
                /* LD B,RES 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_RES_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xa9 => {
                /* LD C,RES 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_RES_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xaa => {
                /* LD D,RES 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_RES_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xab => {
                /* LD E,RES 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_RES_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xac => {
                /* LD H,RES 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_RES_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xad => {
                /* LD L,RES 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_RES_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xae => {
                /* RES 5,(REGISTER+dd) */
                self.disassembleDDCB__RES_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xaf => {
                /* LD A,RES 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_RES_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb0 => {
                /* LD B,RES 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_RES_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb1 => {
                /* LD C,RES 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_RES_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb2 => {
                /* LD D,RES 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_RES_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb3 => {
                /* LD E,RES 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_RES_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb4 => {
                /* LD H,RES 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_RES_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb5 => {
                /* LD L,RES 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_RES_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb6 => {
                /* RES 6,(REGISTER+dd) */
                self.disassembleDDCB__RES_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb7 => {
                /* LD A,RES 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_RES_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb8 => {
                /* LD B,RES 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_RES_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xb9 => {
                /* LD C,RES 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_RES_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xba => {
                /* LD D,RES 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_RES_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xbb => {
                /* LD E,RES 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_RES_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xbc => {
                /* LD H,RES 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_RES_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xbd => {
                /* LD L,RES 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_RES_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xbe => {
                /* RES 7,(REGISTER+dd) */
                self.disassembleDDCB__RES_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xbf => {
                /* LD A,RES 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_RES_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc0 => {
                /* LD B,SET 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_SET_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc1 => {
                /* LD C,SET 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_SET_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc2 => {
                /* LD D,SET 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_SET_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc3 => {
                /* LD E,SET 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_SET_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc4 => {
                /* LD H,SET 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_SET_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc5 => {
                /* LD L,SET 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_SET_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc6 => {
                /* SET 0,(REGISTER+dd) */
                self.disassembleDDCB__SET_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc7 => {
                /* LD A,SET 0,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_SET_0_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc8 => {
                /* LD B,SET 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_SET_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xc9 => {
                /* LD C,SET 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_SET_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xca => {
                /* LD D,SET 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_SET_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xcb => {
                /* LD E,SET 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_SET_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xcc => {
                /* LD H,SET 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_SET_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xcd => {
                /* LD L,SET 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_SET_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xce => {
                /* SET 1,(REGISTER+dd) */
                self.disassembleDDCB__SET_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xcf => {
                /* LD A,SET 1,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_SET_1_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd0 => {
                /* LD B,SET 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_SET_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd1 => {
                /* LD C,SET 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_SET_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd2 => {
                /* LD D,SET 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_SET_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd3 => {
                /* LD E,SET 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_SET_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd4 => {
                /* LD H,SET 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_SET_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd5 => {
                /* LD L,SET 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_SET_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd6 => {
                /* SET 2,(REGISTER+dd) */
                self.disassembleDDCB__SET_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd7 => {
                /* LD A,SET 2,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_SET_2_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd8 => {
                /* LD B,SET 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_SET_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xd9 => {
                /* LD C,SET 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_SET_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xda => {
                /* LD D,SET 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_SET_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xdb => {
                /* LD E,SET 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_SET_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xdc => {
                /* LD H,SET 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_SET_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xdd => {
                /* LD L,SET 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_SET_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xde => {
                /* SET 3,(REGISTER+dd) */
                self.disassembleDDCB__SET_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xdf => {
                /* LD A,SET 3,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_SET_3_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe0 => {
                /* LD B,SET 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_SET_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe1 => {
                /* LD C,SET 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_SET_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe2 => {
                /* LD D,SET 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_SET_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe3 => {
                /* LD E,SET 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_SET_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe4 => {
                /* LD H,SET 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_SET_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe5 => {
                /* LD L,SET 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_SET_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe6 => {
                /* SET 4,(REGISTER+dd) */
                self.disassembleDDCB__SET_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe7 => {
                /* LD A,SET 4,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_SET_4_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe8 => {
                /* LD B,SET 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_SET_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xe9 => {
                /* LD C,SET 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_SET_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xea => {
                /* LD D,SET 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_SET_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xeb => {
                /* LD E,SET 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_SET_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xec => {
                /* LD H,SET 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_SET_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xed => {
                /* LD L,SET 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_SET_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xee => {
                /* SET 5,(REGISTER+dd) */
                self.disassembleDDCB__SET_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xef => {
                /* LD A,SET 5,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_SET_5_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf0 => {
                /* LD B,SET 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_SET_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf1 => {
                /* LD C,SET 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_SET_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf2 => {
                /* LD D,SET 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_SET_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf3 => {
                /* LD E,SET 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_SET_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf4 => {
                /* LD H,SET 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_SET_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf5 => {
                /* LD L,SET 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_SET_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf6 => {
                /* SET 6,(REGISTER+dd) */
                self.disassembleDDCB__SET_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf7 => {
                /* LD A,SET 6,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_SET_6_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf8 => {
                /* LD B,SET 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_B_SET_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xf9 => {
                /* LD C,SET 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_C_SET_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xfa => {
                /* LD D,SET 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_D_SET_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xfb => {
                /* LD E,SET 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_E_SET_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xfc => {
                /* LD H,SET 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_H_SET_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xfd => {
                /* LD L,SET 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_L_SET_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xfe => {
                /* SET 7,(REGISTER+dd) */
                self.disassembleDDCB__SET_7_iREGpDD();
            }

            val if val == SHIFT_0X_DDCB + 0xff => {
                /* LD A,SET 7,(REGISTER+dd) */
                self.disassembleDDCB__LD_A_SET_7_iREGpDD();
            }

            _ => {
                unimplemented!();
            }
        }
    }
    /* NOP */
    fn disassemble__NOP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) NOP", address);
    }
    /* LD BC,nnnn */
    fn disassemble__LD_BC_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD BC,0x{:04x}", address, nnnn);
    }

    /* LD (BC),A */
    fn disassemble__LD_iBC_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (BC),A", address);
    }

    /* INC BC */
    fn disassemble__INC_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC BC", address);
    }

    /* INC B */
    fn disassemble__INC_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC B", address);
    }

    /* DEC B */
    fn disassemble__DEC_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC B", address);
    }

    /* LD B,nn */
    fn disassemble__LD_B_NN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,nn", address);
    }

    /* RLCA */
    fn disassemble__RLCA(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLCA", address);
    }

    /* EX AF,AF' */
    fn disassemble__EX_AF_AF(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) EX AF,AF'", address);
    }

    /* ADD HL,BC */
    fn disassemble__ADD_HL_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD HL,BC", address);
    }

    /* LD A,(BC) */
    fn disassemble__LD_A_iBC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,(BC)", address);
    }

    /* DEC BC */
    fn disassemble__DEC_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC BC", address);
    }

    /* INC C */
    fn disassemble__INC_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC C", address);
    }

    /* DEC C */
    fn disassemble__DEC_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC C", address);
    }

    /* LD C,nn */
    fn disassemble__LD_C_NN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,nn", address);
    }

    /* RRCA */
    fn disassemble__RRCA(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRCA", address);
    }

    /* DJNZ offset */
    fn disassemble__DJNZ_OFFSET(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DJNZ offset", address);
    }

    /* LD DE,nnnn */
    fn disassemble__LD_DE_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD DE,0x{:04x}", address, nnnn);
    }

    /* LD (DE),A */
    fn disassemble__LD_iDE_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (DE),A", address);
    }

    /* INC DE */
    fn disassemble__INC_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC DE", address);
    }

    /* INC D */
    fn disassemble__INC_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC D", address);
    }

    /* DEC D */
    fn disassemble__DEC_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC D", address);
    }

    /* LD D,nn */
    fn disassemble__LD_D_NN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,nn", address);
    }

    /* RLA */
    fn disassemble__RLA(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLA", address);
    }

    /* JR offset */
    fn disassemble__JR_OFFSET(&mut self) {
        let address = self.PC() - 1;
        let offset = self.memory.read_byte(address + 1);
        println!("({:04x}) JR 0x{:02x}", address, offset);
    }

    /* ADD HL,DE */
    fn disassemble__ADD_HL_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD HL,DE", address);
    }

    /* LD A,(DE) */
    fn disassemble__LD_A_iDE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,(DE)", address);
    }

    /* DEC DE */
    fn disassemble__DEC_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC DE", address);
    }

    /* INC E */
    fn disassemble__INC_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC E", address);
    }

    /* DEC E */
    fn disassemble__DEC_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC E", address);
    }

    /* LD E,nn */
    fn disassemble__LD_E_NN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,nn", address);
    }

    /* RRA */
    fn disassemble__RRA(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRA", address);
    }

    /* JR NZ,offset */
    fn disassemble__JR_NZ_OFFSET(&mut self) {
        let address = self.PC() - 1;
        let offset = self.memory.read_byte(address + 1);
        println!("({:04x}) JR NZ,0x{:02x}", address, offset);
    }

    /* LD HL,nnnn */
    fn disassemble__LD_HL_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD HL,0x{:04x}", address, nnnn);
    }

    /* LD (nnnn),HL */
    fn disassemble__LD_iNNNN_HL(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD (0x{:04x}),HL", address, nnnn);
    }

    /* INC HL */
    fn disassemble__INC_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC HL", address);
    }

    /* INC H */
    fn disassemble__INC_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC H", address);
    }

    /* DEC H */
    fn disassemble__DEC_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC H", address);
    }

    /* LD H,nn */
    fn disassemble__LD_H_NN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,nn", address);
    }

    /* DAA */
    fn disassemble__DAA(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DAA", address);
    }

    /* JR Z,offset */
    fn disassemble__JR_Z_OFFSET(&mut self) {
        let address = self.PC() - 1;
        let offset = self.memory.read_byte(address + 1);
        println!("({:04x}) JR Z,0x{:02x}", address, offset);
    }

    /* ADD HL,HL */
    fn disassemble__ADD_HL_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD HL,HL", address);
    }

    /* LD HL,(nnnn) */
    fn disassemble__LD_HL_iNNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD HL,(0x{:04x})", address, nnnn);
    }

    /* DEC HL */
    fn disassemble__DEC_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC HL", address);
    }

    /* INC L */
    fn disassemble__INC_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC L", address);
    }

    /* DEC L */
    fn disassemble__DEC_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC L", address);
    }

    /* LD L,nn */
    fn disassemble__LD_L_NN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,nn", address);
    }

    /* CPL */
    fn disassemble__CPL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CPL", address);
    }

    /* JR NC,offset */
    fn disassemble__JR_NC_OFFSET(&mut self) {
        let address = self.PC() - 1;
        let offset = self.memory.read_byte(address + 1);
        println!("({:04x}) JR NC,0x{:02x}", address, offset);
    }

    /* LD SP,nnnn */
    fn disassemble__LD_SP_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD SP,0x{:04x}", address, nnnn);
    }

    /* LD (nnnn),A */
    fn disassemble__LD_iNNNN_A(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD (0x{:04x}),A", address, nnnn);
    }

    /* INC SP */
    fn disassemble__INC_SP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC SP", address);
    }

    /* INC (HL) */
    fn disassemble__INC_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC (HL)", address);
    }

    /* DEC (HL) */
    fn disassemble__DEC_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC (HL)", address);
    }

    /* LD (HL),nn */
    fn disassemble__LD_iHL_NN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (HL),nn", address);
    }

    /* SCF */
    fn disassemble__SCF(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SCF", address);
    }

    /* JR C,offset */
    fn disassemble__JR_C_OFFSET(&mut self) {
        let address = self.PC() - 1;
        let offset = self.memory.read_byte(address + 1);
        println!("({:04x}) JR C,0x{:02x}", address, offset);
    }

    /* ADD HL,SP */
    fn disassemble__ADD_HL_SP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD HL,SP", address);
    }

    /* LD A,(nnnn) */
    fn disassemble__LD_A_iNNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD A,(0x{:04x})", address, nnnn);
    }

    /* DEC SP */
    fn disassemble__DEC_SP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC SP", address);
    }

    /* INC A */
    fn disassemble__INC_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC A", address);
    }

    /* DEC A */
    fn disassemble__DEC_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC A", address);
    }

    /* LD A,nn */
    fn disassemble__LD_A_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) LD A,0x{:02x}", address, nn);
    }

    /* CCF */
    fn disassemble__CCF(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CCF", address);
    }

    /* LD B,B */
    fn disassemble__LD_B_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,B", address);
    }

    /* LD B,C */
    fn disassemble__LD_B_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,C", address);
    }

    /* LD B,D */
    fn disassemble__LD_B_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,D", address);
    }

    /* LD B,E */
    fn disassemble__LD_B_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,E", address);
    }

    /* LD B,H */
    fn disassemble__LD_B_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,H", address);
    }

    /* LD B,L */
    fn disassemble__LD_B_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,L", address);
    }

    /* LD B,(HL) */
    fn disassemble__LD_B_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,(HL)", address);
    }

    /* LD B,A */
    fn disassemble__LD_B_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,A", address);
    }

    /* LD C,B */
    fn disassemble__LD_C_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,B", address);
    }

    /* LD C,C */
    fn disassemble__LD_C_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,C", address);
    }

    /* LD C,D */
    fn disassemble__LD_C_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,D", address);
    }

    /* LD C,E */
    fn disassemble__LD_C_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,E", address);
    }

    /* LD C,H */
    fn disassemble__LD_C_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,H", address);
    }

    /* LD C,L */
    fn disassemble__LD_C_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,L", address);
    }

    /* LD C,(HL) */
    fn disassemble__LD_C_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,(HL)", address);
    }

    /* LD C,A */
    fn disassemble__LD_C_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,A", address);
    }

    /* LD D,B */
    fn disassemble__LD_D_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,B", address);
    }

    /* LD D,C */
    fn disassemble__LD_D_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,C", address);
    }

    /* LD D,D */
    fn disassemble__LD_D_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,D", address);
    }

    /* LD D,E */
    fn disassemble__LD_D_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,E", address);
    }

    /* LD D,H */
    fn disassemble__LD_D_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,H", address);
    }

    /* LD D,L */
    fn disassemble__LD_D_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,L", address);
    }

    /* LD D,(HL) */
    fn disassemble__LD_D_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,(HL)", address);
    }

    /* LD D,A */
    fn disassemble__LD_D_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,A", address);
    }

    /* LD E,B */
    fn disassemble__LD_E_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,B", address);
    }

    /* LD E,C */
    fn disassemble__LD_E_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,C", address);
    }

    /* LD E,D */
    fn disassemble__LD_E_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,D", address);
    }

    /* LD E,E */
    fn disassemble__LD_E_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,E", address);
    }

    /* LD E,H */
    fn disassemble__LD_E_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,H", address);
    }

    /* LD E,L */
    fn disassemble__LD_E_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,L", address);
    }

    /* LD E,(HL) */
    fn disassemble__LD_E_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,(HL)", address);
    }

    /* LD E,A */
    fn disassemble__LD_E_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,A", address);
    }

    /* LD H,B */
    fn disassemble__LD_H_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,B", address);
    }

    /* LD H,C */
    fn disassemble__LD_H_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,C", address);
    }

    /* LD H,D */
    fn disassemble__LD_H_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,D", address);
    }

    /* LD H,E */
    fn disassemble__LD_H_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,E", address);
    }

    /* LD H,H */
    fn disassemble__LD_H_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,H", address);
    }

    /* LD H,L */
    fn disassemble__LD_H_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,L", address);
    }

    /* LD H,(HL) */
    fn disassemble__LD_H_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,(HL)", address);
    }

    /* LD H,A */
    fn disassemble__LD_H_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,A", address);
    }

    /* LD L,B */
    fn disassemble__LD_L_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,B", address);
    }

    /* LD L,C */
    fn disassemble__LD_L_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,C", address);
    }

    /* LD L,D */
    fn disassemble__LD_L_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,D", address);
    }

    /* LD L,E */
    fn disassemble__LD_L_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,E", address);
    }

    /* LD L,H */
    fn disassemble__LD_L_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,H", address);
    }

    /* LD L,L */
    fn disassemble__LD_L_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,L", address);
    }

    /* LD L,(HL) */
    fn disassemble__LD_L_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,(HL)", address);
    }

    /* LD L,A */
    fn disassemble__LD_L_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,A", address);
    }

    /* LD (HL),B */
    fn disassemble__LD_iHL_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (HL),B", address);
    }

    /* LD (HL),C */
    fn disassemble__LD_iHL_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (HL),C", address);
    }

    /* LD (HL),D */
    fn disassemble__LD_iHL_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (HL),D", address);
    }

    /* LD (HL),E */
    fn disassemble__LD_iHL_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (HL),E", address);
    }

    /* LD (HL),H */
    fn disassemble__LD_iHL_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (HL),H", address);
    }

    /* LD (HL),L */
    fn disassemble__LD_iHL_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (HL),L", address);
    }

    /* HALT */
    fn disassemble__HALT(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) HALT", address);
    }

    /* LD (HL),A */
    fn disassemble__LD_iHL_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (HL),A", address);
    }

    /* LD A,B */
    fn disassemble__LD_A_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,B", address);
    }

    /* LD A,C */
    fn disassemble__LD_A_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,C", address);
    }

    /* LD A,D */
    fn disassemble__LD_A_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,D", address);
    }

    /* LD A,E */
    fn disassemble__LD_A_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,E", address);
    }

    /* LD A,H */
    fn disassemble__LD_A_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,H", address);
    }

    /* LD A,L */
    fn disassemble__LD_A_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,L", address);
    }

    /* LD A,(HL) */
    fn disassemble__LD_A_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,(HL)", address);
    }

    /* LD A,A */
    fn disassemble__LD_A_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,A", address);
    }

    /* ADD A,B */
    fn disassemble__ADD_A_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,B", address);
    }

    /* ADD A,C */
    fn disassemble__ADD_A_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,C", address);
    }

    /* ADD A,D */
    fn disassemble__ADD_A_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,D", address);
    }

    /* ADD A,E */
    fn disassemble__ADD_A_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,E", address);
    }

    /* ADD A,H */
    fn disassemble__ADD_A_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,H", address);
    }

    /* ADD A,L */
    fn disassemble__ADD_A_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,L", address);
    }

    /* ADD A,(HL) */
    fn disassemble__ADD_A_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,(HL)", address);
    }

    /* ADD A,A */
    fn disassemble__ADD_A_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,A", address);
    }

    /* ADC A,B */
    fn disassemble__ADC_A_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,B", address);
    }

    /* ADC A,C */
    fn disassemble__ADC_A_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,C", address);
    }

    /* ADC A,D */
    fn disassemble__ADC_A_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,D", address);
    }

    /* ADC A,E */
    fn disassemble__ADC_A_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,E", address);
    }

    /* ADC A,H */
    fn disassemble__ADC_A_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,H", address);
    }

    /* ADC A,L */
    fn disassemble__ADC_A_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,L", address);
    }

    /* ADC A,(HL) */
    fn disassemble__ADC_A_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,(HL)", address);
    }

    /* ADC A,A */
    fn disassemble__ADC_A_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,A", address);
    }

    /* SUB A,B */
    fn disassemble__SUB_A_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,B", address);
    }

    /* SUB A,C */
    fn disassemble__SUB_A_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,C", address);
    }

    /* SUB A,D */
    fn disassemble__SUB_A_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,D", address);
    }

    /* SUB A,E */
    fn disassemble__SUB_A_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,E", address);
    }

    /* SUB A,H */
    fn disassemble__SUB_A_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,H", address);
    }

    /* SUB A,L */
    fn disassemble__SUB_A_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,L", address);
    }

    /* SUB A,(HL) */
    fn disassemble__SUB_A_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,(HL)", address);
    }

    /* SUB A,A */
    fn disassemble__SUB_A_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,A", address);
    }

    /* SBC A,B */
    fn disassemble__SBC_A_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,B", address);
    }

    /* SBC A,C */
    fn disassemble__SBC_A_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,C", address);
    }

    /* SBC A,D */
    fn disassemble__SBC_A_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,D", address);
    }

    /* SBC A,E */
    fn disassemble__SBC_A_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,E", address);
    }

    /* SBC A,H */
    fn disassemble__SBC_A_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,H", address);
    }

    /* SBC A,L */
    fn disassemble__SBC_A_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,L", address);
    }

    /* SBC A,(HL) */
    fn disassemble__SBC_A_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,(HL)", address);
    }

    /* SBC A,A */
    fn disassemble__SBC_A_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,A", address);
    }

    /* AND A,B */
    fn disassemble__AND_A_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,B", address);
    }

    /* AND A,C */
    fn disassemble__AND_A_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,C", address);
    }

    /* AND A,D */
    fn disassemble__AND_A_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,D", address);
    }

    /* AND A,E */
    fn disassemble__AND_A_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,E", address);
    }

    /* AND A,H */
    fn disassemble__AND_A_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,H", address);
    }

    /* AND A,L */
    fn disassemble__AND_A_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,L", address);
    }

    /* AND A,(HL) */
    fn disassemble__AND_A_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,(HL)", address);
    }

    /* AND A,A */
    fn disassemble__AND_A_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,A", address);
    }

    /* XOR A,B */
    fn disassemble__XOR_A_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,B", address);
    }

    /* XOR A,C */
    fn disassemble__XOR_A_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,C", address);
    }

    /* XOR A,D */
    fn disassemble__XOR_A_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,D", address);
    }

    /* XOR A,E */
    fn disassemble__XOR_A_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,E", address);
    }

    /* XOR A,H */
    fn disassemble__XOR_A_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,H", address);
    }

    /* XOR A,L */
    fn disassemble__XOR_A_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,L", address);
    }

    /* XOR A,(HL) */
    fn disassemble__XOR_A_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,(HL)", address);
    }

    /* XOR A,A */
    fn disassemble__XOR_A_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,A", address);
    }

    /* OR A,B */
    fn disassemble__OR_A_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,B", address);
    }

    /* OR A,C */
    fn disassemble__OR_A_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,C", address);
    }

    /* OR A,D */
    fn disassemble__OR_A_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,D", address);
    }

    /* OR A,E */
    fn disassemble__OR_A_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,E", address);
    }

    /* OR A,H */
    fn disassemble__OR_A_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,H", address);
    }

    /* OR A,L */
    fn disassemble__OR_A_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,L", address);
    }

    /* OR A,(HL) */
    fn disassemble__OR_A_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,(HL)", address);
    }

    /* OR A,A */
    fn disassemble__OR_A_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,A", address);
    }

    /* CP B */
    fn disassemble__CP_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP B", address);
    }

    /* CP C */
    fn disassemble__CP_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP C", address);
    }

    /* CP D */
    fn disassemble__CP_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP D", address);
    }

    /* CP E */
    fn disassemble__CP_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP E", address);
    }

    /* CP H */
    fn disassemble__CP_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP H", address);
    }

    /* CP L */
    fn disassemble__CP_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP L", address);
    }

    /* CP (HL) */
    fn disassemble__CP_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP (HL)", address);
    }

    /* CP A */
    fn disassemble__CP_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP A", address);
    }

    /* RET NZ */
    fn disassemble__RET_NZ(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET NZ", address);
    }

    /* POP BC */
    fn disassemble__POP_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) POP BC", address);
    }

    /* JP NZ,nnnn */
    fn disassemble__JP_NZ_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP NZ,0x{:04x}", address, nnnn);
    }

    /* JP nnnn */
    fn disassemble__JP_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP 0x{:04x}", address, nnnn);
    }

    /* CALL NZ,nnnn */
    fn disassemble__CALL_NZ_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL NZ,0x{:04x}", address, nnnn);
    }

    /* PUSH BC */
    fn disassemble__PUSH_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) PUSH BC", address);
    }

    /* ADD A,nn */
    fn disassemble__ADD_A_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) ADD A,0x{:02x}", address, nn);
    }

    /* RST 00 */
    fn disassemble__RST_00(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RST 00", address);
    }

    /* RET Z */
    fn disassemble__RET_Z(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET Z", address);
    }

    /* RET */
    fn disassemble__RET(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET", address);
    }

    /* JP Z,nnnn */
    fn disassemble__JP_Z_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP Z,0x{:04x}", address, nnnn);
    }

    /* shift CB */
    fn disassemble__SHIFT_CB(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) shift CB", address);
    }

    /* CALL Z,nnnn */
    fn disassemble__CALL_Z_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL Z,0x{:04x}", address, nnnn);
    }

    /* CALL nnnn */
    fn disassemble__CALL_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL 0x{:04x}", address, nnnn);
    }

    /* ADC A,nn */
    fn disassemble__ADC_A_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) ADC A,0x{:02x}", address, nn);
    }

    /* RST 8 */
    fn disassemble__RST_8(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RST 8", address);
    }

    /* RET NC */
    fn disassemble__RET_NC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET NC", address);
    }

    /* POP DE */
    fn disassemble__POP_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) POP DE", address);
    }

    /* JP NC,nnnn */
    fn disassemble__JP_NC_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP NC,0x{:04x}", address, nnnn);
    }

    /* OUT (nn),A */
    fn disassemble__OUT_iNN_A(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) OUT (0x{:02x}),A", address, nn);
    }

    /* CALL NC,nnnn */
    fn disassemble__CALL_NC_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL NC,0x{:04x}", address, nnnn);
    }

    /* PUSH DE */
    fn disassemble__PUSH_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) PUSH DE", address);
    }

    /* SUB nn */
    fn disassemble__SUB_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) SUB 0x{:02x}", address, nn);
    }

    /* RST 10 */
    fn disassemble__RST_10(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RST 10", address);
    }

    /* RET C */
    fn disassemble__RET_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET C", address);
    }

    /* EXX */
    fn disassemble__EXX(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) EXX", address);
    }

    /* JP C,nnnn */
    fn disassemble__JP_C_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP C,0x{:04x}", address, nnnn);
    }

    /* IN A,(nn) */
    fn disassemble__IN_A_iNN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) IN A,(0x{:02x})", address, nn);
    }

    /* CALL C,nnnn */
    fn disassemble__CALL_C_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL C,0x{:04x}", address, nnnn);
    }

    /* shift DD */
    fn disassemble__SHIFT_DD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) shift DD", address);
    }

    /* SBC A,nn */
    fn disassemble__SBC_A_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) SBC A,0x{:02x}", address, nn);
    }

    /* RST 18 */
    fn disassemble__RST_18(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RST 18", address);
    }

    /* RET PO */
    fn disassemble__RET_PO(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET PO", address);
    }

    /* POP HL */
    fn disassemble__POP_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) POP HL", address);
    }

    /* JP PO,nnnn */
    fn disassemble__JP_PO_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP PO,0x{:04x}", address, nnnn);
    }

    /* EX (SP),HL */
    fn disassemble__EX_iSP_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) EX (SP),HL", address);
    }

    /* CALL PO,nnnn */
    fn disassemble__CALL_PO_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL PO,0x{:04x}", address, nnnn);
    }

    /* PUSH HL */
    fn disassemble__PUSH_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) PUSH HL", address);
    }

    /* AND nn */
    fn disassemble__AND_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) AND 0x{:02x}", address, nn);
    }

    /* RST 20 */
    fn disassemble__RST_20(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RST 20", address);
    }

    /* RET PE */
    fn disassemble__RET_PE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET PE", address);
    }

    /* JP HL */
    fn disassemble__JP_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) JP HL", address);
    }

    /* JP PE,nnnn */
    fn disassemble__JP_PE_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP PE,0x{:04x}", address, nnnn);
    }

    /* EX DE,HL */
    fn disassemble__EX_DE_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) EX DE,HL", address);
    }

    /* CALL PE,nnnn */
    fn disassemble__CALL_PE_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL PE,0x{:04x}", address, nnnn);
    }

    /* shift ED */
    fn disassemble__SHIFT_ED(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) shift ED", address);
    }

    /* XOR A,nn */
    fn disassemble__XOR_A_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) XOR A,0x{:02x}", address, nn);
    }

    /* RST 28 */
    fn disassemble__RST_28(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RST 28", address);
    }

    /* RET P */
    fn disassemble__RET_P(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET P", address);
    }

    /* POP AF */
    fn disassemble__POP_AF(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) POP AF", address);
    }

    /* JP P,nnnn */
    fn disassemble__JP_P_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP P,0x{:04x}", address, nnnn);
    }

    /* DI */
    fn disassemble__DI(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DI", address);
    }

    /* CALL P,nnnn */
    fn disassemble__CALL_P_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL P,0x{:04x}", address, nnnn);
    }

    /* PUSH AF */
    fn disassemble__PUSH_AF(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) PUSH AF", address);
    }

    /* OR nn */
    fn disassemble__OR_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) OR 0x{:02x}", address, nn);
    }

    /* RST 30 */
    fn disassemble__RST_30(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RST 30", address);
    }

    /* RET M */
    fn disassemble__RET_M(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RET M", address);
    }

    /* LD SP,HL */
    fn disassemble__LD_SP_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD SP,HL", address);
    }

    /* JP M,nnnn */
    fn disassemble__JP_M_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) JP M,0x{:04x}", address, nnnn);
    }

    /* EI */
    fn disassemble__EI(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) EI", address);
    }

    /* CALL M,nnnn */
    fn disassemble__CALL_M_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) CALL M,0x{:04x}", address, nnnn);
    }

    /* shift FD */
    fn disassemble__SHIFT_FD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) shift FD", address);
    }

    /* CP nn */
    fn disassemble__CP_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) CP 0x{:02x}", address, nn);
    }

    /* RST 38 */
    fn disassemble__RST_38(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RST 38", address);
    }

    /* RLC B */
    fn disassembleCB__RLC_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC B", address);
    }

    /* RLC C */
    fn disassembleCB__RLC_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC C", address);
    }

    /* RLC D */
    fn disassembleCB__RLC_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC D", address);
    }

    /* RLC E */
    fn disassembleCB__RLC_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC E", address);
    }

    /* RLC H */
    fn disassembleCB__RLC_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC H", address);
    }

    /* RLC L */
    fn disassembleCB__RLC_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC L", address);
    }

    /* RLC (HL) */
    fn disassembleCB__RLC_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC (HL)", address);
    }

    /* RLC A */
    fn disassembleCB__RLC_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC A", address);
    }

    /* RRC B */
    fn disassembleCB__RRC_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC B", address);
    }

    /* RRC C */
    fn disassembleCB__RRC_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC C", address);
    }

    /* RRC D */
    fn disassembleCB__RRC_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC D", address);
    }

    /* RRC E */
    fn disassembleCB__RRC_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC E", address);
    }

    /* RRC H */
    fn disassembleCB__RRC_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC H", address);
    }

    /* RRC L */
    fn disassembleCB__RRC_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC L", address);
    }

    /* RRC (HL) */
    fn disassembleCB__RRC_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC (HL)", address);
    }

    /* RRC A */
    fn disassembleCB__RRC_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC A", address);
    }

    /* RL B */
    fn disassembleCB__RL_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL B", address);
    }

    /* RL C */
    fn disassembleCB__RL_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL C", address);
    }

    /* RL D */
    fn disassembleCB__RL_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL D", address);
    }

    /* RL E */
    fn disassembleCB__RL_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL E", address);
    }

    /* RL H */
    fn disassembleCB__RL_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL H", address);
    }

    /* RL L */
    fn disassembleCB__RL_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL L", address);
    }

    /* RL (HL) */
    fn disassembleCB__RL_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL (HL)", address);
    }

    /* RL A */
    fn disassembleCB__RL_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL A", address);
    }

    /* RR B */
    fn disassembleCB__RR_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR B", address);
    }

    /* RR C */
    fn disassembleCB__RR_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR C", address);
    }

    /* RR D */
    fn disassembleCB__RR_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR D", address);
    }

    /* RR E */
    fn disassembleCB__RR_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR E", address);
    }

    /* RR H */
    fn disassembleCB__RR_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR H", address);
    }

    /* RR L */
    fn disassembleCB__RR_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR L", address);
    }

    /* RR (HL) */
    fn disassembleCB__RR_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR (HL)", address);
    }

    /* RR A */
    fn disassembleCB__RR_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR A", address);
    }

    /* SLA B */
    fn disassembleCB__SLA_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA B", address);
    }

    /* SLA C */
    fn disassembleCB__SLA_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA C", address);
    }

    /* SLA D */
    fn disassembleCB__SLA_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA D", address);
    }

    /* SLA E */
    fn disassembleCB__SLA_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA E", address);
    }

    /* SLA H */
    fn disassembleCB__SLA_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA H", address);
    }

    /* SLA L */
    fn disassembleCB__SLA_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA L", address);
    }

    /* SLA (HL) */
    fn disassembleCB__SLA_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA (HL)", address);
    }

    /* SLA A */
    fn disassembleCB__SLA_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA A", address);
    }

    /* SRA B */
    fn disassembleCB__SRA_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA B", address);
    }

    /* SRA C */
    fn disassembleCB__SRA_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA C", address);
    }

    /* SRA D */
    fn disassembleCB__SRA_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA D", address);
    }

    /* SRA E */
    fn disassembleCB__SRA_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA E", address);
    }

    /* SRA H */
    fn disassembleCB__SRA_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA H", address);
    }

    /* SRA L */
    fn disassembleCB__SRA_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA L", address);
    }

    /* SRA (HL) */
    fn disassembleCB__SRA_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA (HL)", address);
    }

    /* SRA A */
    fn disassembleCB__SRA_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA A", address);
    }

    /* SLL B */
    fn disassembleCB__SLL_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL B", address);
    }

    /* SLL C */
    fn disassembleCB__SLL_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL C", address);
    }

    /* SLL D */
    fn disassembleCB__SLL_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL D", address);
    }

    /* SLL E */
    fn disassembleCB__SLL_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL E", address);
    }

    /* SLL H */
    fn disassembleCB__SLL_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL H", address);
    }

    /* SLL L */
    fn disassembleCB__SLL_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL L", address);
    }

    /* SLL (HL) */
    fn disassembleCB__SLL_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL (HL)", address);
    }

    /* SLL A */
    fn disassembleCB__SLL_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL A", address);
    }

    /* SRL B */
    fn disassembleCB__SRL_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL B", address);
    }

    /* SRL C */
    fn disassembleCB__SRL_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL C", address);
    }

    /* SRL D */
    fn disassembleCB__SRL_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL D", address);
    }

    /* SRL E */
    fn disassembleCB__SRL_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL E", address);
    }

    /* SRL H */
    fn disassembleCB__SRL_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL H", address);
    }

    /* SRL L */
    fn disassembleCB__SRL_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL L", address);
    }

    /* SRL (HL) */
    fn disassembleCB__SRL_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL (HL)", address);
    }

    /* SRL A */
    fn disassembleCB__SRL_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL A", address);
    }

    /* BIT 0,B */
    fn disassembleCB__BIT_0_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,B", address);
    }

    /* BIT 0,C */
    fn disassembleCB__BIT_0_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,C", address);
    }

    /* BIT 0,D */
    fn disassembleCB__BIT_0_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,D", address);
    }

    /* BIT 0,E */
    fn disassembleCB__BIT_0_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,E", address);
    }

    /* BIT 0,H */
    fn disassembleCB__BIT_0_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,H", address);
    }

    /* BIT 0,L */
    fn disassembleCB__BIT_0_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,L", address);
    }

    /* BIT 0,(HL) */
    fn disassembleCB__BIT_0_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,(HL)", address);
    }

    /* BIT 0,A */
    fn disassembleCB__BIT_0_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,A", address);
    }

    /* BIT 1,B */
    fn disassembleCB__BIT_1_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,B", address);
    }

    /* BIT 1,C */
    fn disassembleCB__BIT_1_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,C", address);
    }

    /* BIT 1,D */
    fn disassembleCB__BIT_1_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,D", address);
    }

    /* BIT 1,E */
    fn disassembleCB__BIT_1_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,E", address);
    }

    /* BIT 1,H */
    fn disassembleCB__BIT_1_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,H", address);
    }

    /* BIT 1,L */
    fn disassembleCB__BIT_1_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,L", address);
    }

    /* BIT 1,(HL) */
    fn disassembleCB__BIT_1_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,(HL)", address);
    }

    /* BIT 1,A */
    fn disassembleCB__BIT_1_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,A", address);
    }

    /* BIT 2,B */
    fn disassembleCB__BIT_2_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,B", address);
    }

    /* BIT 2,C */
    fn disassembleCB__BIT_2_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,C", address);
    }

    /* BIT 2,D */
    fn disassembleCB__BIT_2_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,D", address);
    }

    /* BIT 2,E */
    fn disassembleCB__BIT_2_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,E", address);
    }

    /* BIT 2,H */
    fn disassembleCB__BIT_2_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,H", address);
    }

    /* BIT 2,L */
    fn disassembleCB__BIT_2_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,L", address);
    }

    /* BIT 2,(HL) */
    fn disassembleCB__BIT_2_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,(HL)", address);
    }

    /* BIT 2,A */
    fn disassembleCB__BIT_2_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,A", address);
    }

    /* BIT 3,B */
    fn disassembleCB__BIT_3_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,B", address);
    }

    /* BIT 3,C */
    fn disassembleCB__BIT_3_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,C", address);
    }

    /* BIT 3,D */
    fn disassembleCB__BIT_3_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,D", address);
    }

    /* BIT 3,E */
    fn disassembleCB__BIT_3_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,E", address);
    }

    /* BIT 3,H */
    fn disassembleCB__BIT_3_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,H", address);
    }

    /* BIT 3,L */
    fn disassembleCB__BIT_3_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,L", address);
    }

    /* BIT 3,(HL) */
    fn disassembleCB__BIT_3_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,(HL)", address);
    }

    /* BIT 3,A */
    fn disassembleCB__BIT_3_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,A", address);
    }

    /* BIT 4,B */
    fn disassembleCB__BIT_4_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,B", address);
    }

    /* BIT 4,C */
    fn disassembleCB__BIT_4_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,C", address);
    }

    /* BIT 4,D */
    fn disassembleCB__BIT_4_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,D", address);
    }

    /* BIT 4,E */
    fn disassembleCB__BIT_4_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,E", address);
    }

    /* BIT 4,H */
    fn disassembleCB__BIT_4_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,H", address);
    }

    /* BIT 4,L */
    fn disassembleCB__BIT_4_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,L", address);
    }

    /* BIT 4,(HL) */
    fn disassembleCB__BIT_4_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,(HL)", address);
    }

    /* BIT 4,A */
    fn disassembleCB__BIT_4_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,A", address);
    }

    /* BIT 5,B */
    fn disassembleCB__BIT_5_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,B", address);
    }

    /* BIT 5,C */
    fn disassembleCB__BIT_5_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,C", address);
    }

    /* BIT 5,D */
    fn disassembleCB__BIT_5_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,D", address);
    }

    /* BIT 5,E */
    fn disassembleCB__BIT_5_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,E", address);
    }

    /* BIT 5,H */
    fn disassembleCB__BIT_5_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,H", address);
    }

    /* BIT 5,L */
    fn disassembleCB__BIT_5_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,L", address);
    }

    /* BIT 5,(HL) */
    fn disassembleCB__BIT_5_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,(HL)", address);
    }

    /* BIT 5,A */
    fn disassembleCB__BIT_5_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,A", address);
    }

    /* BIT 6,B */
    fn disassembleCB__BIT_6_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,B", address);
    }

    /* BIT 6,C */
    fn disassembleCB__BIT_6_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,C", address);
    }

    /* BIT 6,D */
    fn disassembleCB__BIT_6_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,D", address);
    }

    /* BIT 6,E */
    fn disassembleCB__BIT_6_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,E", address);
    }

    /* BIT 6,H */
    fn disassembleCB__BIT_6_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,H", address);
    }

    /* BIT 6,L */
    fn disassembleCB__BIT_6_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,L", address);
    }

    /* BIT 6,(HL) */
    fn disassembleCB__BIT_6_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,(HL)", address);
    }

    /* BIT 6,A */
    fn disassembleCB__BIT_6_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,A", address);
    }

    /* BIT 7,B */
    fn disassembleCB__BIT_7_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,B", address);
    }

    /* BIT 7,C */
    fn disassembleCB__BIT_7_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,C", address);
    }

    /* BIT 7,D */
    fn disassembleCB__BIT_7_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,D", address);
    }

    /* BIT 7,E */
    fn disassembleCB__BIT_7_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,E", address);
    }

    /* BIT 7,H */
    fn disassembleCB__BIT_7_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,H", address);
    }

    /* BIT 7,L */
    fn disassembleCB__BIT_7_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,L", address);
    }

    /* BIT 7,(HL) */
    fn disassembleCB__BIT_7_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,(HL)", address);
    }

    /* BIT 7,A */
    fn disassembleCB__BIT_7_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,A", address);
    }

    /* RES 0,B */
    fn disassembleCB__RES_0_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,B", address);
    }

    /* RES 0,C */
    fn disassembleCB__RES_0_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,C", address);
    }

    /* RES 0,D */
    fn disassembleCB__RES_0_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,D", address);
    }

    /* RES 0,E */
    fn disassembleCB__RES_0_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,E", address);
    }

    /* RES 0,H */
    fn disassembleCB__RES_0_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,H", address);
    }

    /* RES 0,L */
    fn disassembleCB__RES_0_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,L", address);
    }

    /* RES 0,(HL) */
    fn disassembleCB__RES_0_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,(HL)", address);
    }

    /* RES 0,A */
    fn disassembleCB__RES_0_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,A", address);
    }

    /* RES 1,B */
    fn disassembleCB__RES_1_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,B", address);
    }

    /* RES 1,C */
    fn disassembleCB__RES_1_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,C", address);
    }

    /* RES 1,D */
    fn disassembleCB__RES_1_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,D", address);
    }

    /* RES 1,E */
    fn disassembleCB__RES_1_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,E", address);
    }

    /* RES 1,H */
    fn disassembleCB__RES_1_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,H", address);
    }

    /* RES 1,L */
    fn disassembleCB__RES_1_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,L", address);
    }

    /* RES 1,(HL) */
    fn disassembleCB__RES_1_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,(HL)", address);
    }

    /* RES 1,A */
    fn disassembleCB__RES_1_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,A", address);
    }

    /* RES 2,B */
    fn disassembleCB__RES_2_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,B", address);
    }

    /* RES 2,C */
    fn disassembleCB__RES_2_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,C", address);
    }

    /* RES 2,D */
    fn disassembleCB__RES_2_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,D", address);
    }

    /* RES 2,E */
    fn disassembleCB__RES_2_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,E", address);
    }

    /* RES 2,H */
    fn disassembleCB__RES_2_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,H", address);
    }

    /* RES 2,L */
    fn disassembleCB__RES_2_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,L", address);
    }

    /* RES 2,(HL) */
    fn disassembleCB__RES_2_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,(HL)", address);
    }

    /* RES 2,A */
    fn disassembleCB__RES_2_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,A", address);
    }

    /* RES 3,B */
    fn disassembleCB__RES_3_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,B", address);
    }

    /* RES 3,C */
    fn disassembleCB__RES_3_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,C", address);
    }

    /* RES 3,D */
    fn disassembleCB__RES_3_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,D", address);
    }

    /* RES 3,E */
    fn disassembleCB__RES_3_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,E", address);
    }

    /* RES 3,H */
    fn disassembleCB__RES_3_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,H", address);
    }

    /* RES 3,L */
    fn disassembleCB__RES_3_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,L", address);
    }

    /* RES 3,(HL) */
    fn disassembleCB__RES_3_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,(HL)", address);
    }

    /* RES 3,A */
    fn disassembleCB__RES_3_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,A", address);
    }

    /* RES 4,B */
    fn disassembleCB__RES_4_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,B", address);
    }

    /* RES 4,C */
    fn disassembleCB__RES_4_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,C", address);
    }

    /* RES 4,D */
    fn disassembleCB__RES_4_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,D", address);
    }

    /* RES 4,E */
    fn disassembleCB__RES_4_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,E", address);
    }

    /* RES 4,H */
    fn disassembleCB__RES_4_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,H", address);
    }

    /* RES 4,L */
    fn disassembleCB__RES_4_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,L", address);
    }

    /* RES 4,(HL) */
    fn disassembleCB__RES_4_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,(HL)", address);
    }

    /* RES 4,A */
    fn disassembleCB__RES_4_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,A", address);
    }

    /* RES 5,B */
    fn disassembleCB__RES_5_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,B", address);
    }

    /* RES 5,C */
    fn disassembleCB__RES_5_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,C", address);
    }

    /* RES 5,D */
    fn disassembleCB__RES_5_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,D", address);
    }

    /* RES 5,E */
    fn disassembleCB__RES_5_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,E", address);
    }

    /* RES 5,H */
    fn disassembleCB__RES_5_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,H", address);
    }

    /* RES 5,L */
    fn disassembleCB__RES_5_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,L", address);
    }

    /* RES 5,(HL) */
    fn disassembleCB__RES_5_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,(HL)", address);
    }

    /* RES 5,A */
    fn disassembleCB__RES_5_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,A", address);
    }

    /* RES 6,B */
    fn disassembleCB__RES_6_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,B", address);
    }

    /* RES 6,C */
    fn disassembleCB__RES_6_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,C", address);
    }

    /* RES 6,D */
    fn disassembleCB__RES_6_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,D", address);
    }

    /* RES 6,E */
    fn disassembleCB__RES_6_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,E", address);
    }

    /* RES 6,H */
    fn disassembleCB__RES_6_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,H", address);
    }

    /* RES 6,L */
    fn disassembleCB__RES_6_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,L", address);
    }

    /* RES 6,(HL) */
    fn disassembleCB__RES_6_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,(HL)", address);
    }

    /* RES 6,A */
    fn disassembleCB__RES_6_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,A", address);
    }

    /* RES 7,B */
    fn disassembleCB__RES_7_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,B", address);
    }

    /* RES 7,C */
    fn disassembleCB__RES_7_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,C", address);
    }

    /* RES 7,D */
    fn disassembleCB__RES_7_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,D", address);
    }

    /* RES 7,E */
    fn disassembleCB__RES_7_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,E", address);
    }

    /* RES 7,H */
    fn disassembleCB__RES_7_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,H", address);
    }

    /* RES 7,L */
    fn disassembleCB__RES_7_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,L", address);
    }

    /* RES 7,(HL) */
    fn disassembleCB__RES_7_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,(HL)", address);
    }

    /* RES 7,A */
    fn disassembleCB__RES_7_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,A", address);
    }

    /* SET 0,B */
    fn disassembleCB__SET_0_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,B", address);
    }

    /* SET 0,C */
    fn disassembleCB__SET_0_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,C", address);
    }

    /* SET 0,D */
    fn disassembleCB__SET_0_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,D", address);
    }

    /* SET 0,E */
    fn disassembleCB__SET_0_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,E", address);
    }

    /* SET 0,H */
    fn disassembleCB__SET_0_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,H", address);
    }

    /* SET 0,L */
    fn disassembleCB__SET_0_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,L", address);
    }

    /* SET 0,(HL) */
    fn disassembleCB__SET_0_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,(HL)", address);
    }

    /* SET 0,A */
    fn disassembleCB__SET_0_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,A", address);
    }

    /* SET 1,B */
    fn disassembleCB__SET_1_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,B", address);
    }

    /* SET 1,C */
    fn disassembleCB__SET_1_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,C", address);
    }

    /* SET 1,D */
    fn disassembleCB__SET_1_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,D", address);
    }

    /* SET 1,E */
    fn disassembleCB__SET_1_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,E", address);
    }

    /* SET 1,H */
    fn disassembleCB__SET_1_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,H", address);
    }

    /* SET 1,L */
    fn disassembleCB__SET_1_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,L", address);
    }

    /* SET 1,(HL) */
    fn disassembleCB__SET_1_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,(HL)", address);
    }

    /* SET 1,A */
    fn disassembleCB__SET_1_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,A", address);
    }

    /* SET 2,B */
    fn disassembleCB__SET_2_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,B", address);
    }

    /* SET 2,C */
    fn disassembleCB__SET_2_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,C", address);
    }

    /* SET 2,D */
    fn disassembleCB__SET_2_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,D", address);
    }

    /* SET 2,E */
    fn disassembleCB__SET_2_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,E", address);
    }

    /* SET 2,H */
    fn disassembleCB__SET_2_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,H", address);
    }

    /* SET 2,L */
    fn disassembleCB__SET_2_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,L", address);
    }

    /* SET 2,(HL) */
    fn disassembleCB__SET_2_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,(HL)", address);
    }

    /* SET 2,A */
    fn disassembleCB__SET_2_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,A", address);
    }

    /* SET 3,B */
    fn disassembleCB__SET_3_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,B", address);
    }

    /* SET 3,C */
    fn disassembleCB__SET_3_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,C", address);
    }

    /* SET 3,D */
    fn disassembleCB__SET_3_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,D", address);
    }

    /* SET 3,E */
    fn disassembleCB__SET_3_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,E", address);
    }

    /* SET 3,H */
    fn disassembleCB__SET_3_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,H", address);
    }

    /* SET 3,L */
    fn disassembleCB__SET_3_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,L", address);
    }

    /* SET 3,(HL) */
    fn disassembleCB__SET_3_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,(HL)", address);
    }

    /* SET 3,A */
    fn disassembleCB__SET_3_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,A", address);
    }

    /* SET 4,B */
    fn disassembleCB__SET_4_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,B", address);
    }

    /* SET 4,C */
    fn disassembleCB__SET_4_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,C", address);
    }

    /* SET 4,D */
    fn disassembleCB__SET_4_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,D", address);
    }

    /* SET 4,E */
    fn disassembleCB__SET_4_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,E", address);
    }

    /* SET 4,H */
    fn disassembleCB__SET_4_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,H", address);
    }

    /* SET 4,L */
    fn disassembleCB__SET_4_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,L", address);
    }

    /* SET 4,(HL) */
    fn disassembleCB__SET_4_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,(HL)", address);
    }

    /* SET 4,A */
    fn disassembleCB__SET_4_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,A", address);
    }

    /* SET 5,B */
    fn disassembleCB__SET_5_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,B", address);
    }

    /* SET 5,C */
    fn disassembleCB__SET_5_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,C", address);
    }

    /* SET 5,D */
    fn disassembleCB__SET_5_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,D", address);
    }

    /* SET 5,E */
    fn disassembleCB__SET_5_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,E", address);
    }

    /* SET 5,H */
    fn disassembleCB__SET_5_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,H", address);
    }

    /* SET 5,L */
    fn disassembleCB__SET_5_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,L", address);
    }

    /* SET 5,(HL) */
    fn disassembleCB__SET_5_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,(HL)", address);
    }

    /* SET 5,A */
    fn disassembleCB__SET_5_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,A", address);
    }

    /* SET 6,B */
    fn disassembleCB__SET_6_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,B", address);
    }

    /* SET 6,C */
    fn disassembleCB__SET_6_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,C", address);
    }

    /* SET 6,D */
    fn disassembleCB__SET_6_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,D", address);
    }

    /* SET 6,E */
    fn disassembleCB__SET_6_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,E", address);
    }

    /* SET 6,H */
    fn disassembleCB__SET_6_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,H", address);
    }

    /* SET 6,L */
    fn disassembleCB__SET_6_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,L", address);
    }

    /* SET 6,(HL) */
    fn disassembleCB__SET_6_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,(HL)", address);
    }

    /* SET 6,A */
    fn disassembleCB__SET_6_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,A", address);
    }

    /* SET 7,B */
    fn disassembleCB__SET_7_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,B", address);
    }

    /* SET 7,C */
    fn disassembleCB__SET_7_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,C", address);
    }

    /* SET 7,D */
    fn disassembleCB__SET_7_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,D", address);
    }

    /* SET 7,E */
    fn disassembleCB__SET_7_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,E", address);
    }

    /* SET 7,H */
    fn disassembleCB__SET_7_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,H", address);
    }

    /* SET 7,L */
    fn disassembleCB__SET_7_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,L", address);
    }

    /* SET 7,(HL) */
    fn disassembleCB__SET_7_iHL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,(HL)", address);
    }

    /* SET 7,A */
    fn disassembleCB__SET_7_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,A", address);
    }

    /* IN B,(C) */
    fn disassembleED__IN_B_iC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IN B,(C)", address);
    }

    /* OUT (C),B */
    fn disassembleED__OUT_iC_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUT (C),B", address);
    }

    /* SBC HL,BC */
    fn disassembleED__SBC_HL_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC HL,BC", address);
    }

    /* LD (nnnn),BC */
    fn disassembleED__LD_iNNNN_BC(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD (0x{:04x}),BC", address, nnnn);
    }

    /* NEG */
    fn disassembleED__NEG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) NEG", address);
    }

    /* RETN */
    fn disassembleED__RETN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RETN", address);
    }

    /* IM 0 */
    fn disassembleED__IM_0(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IM 0", address);
    }

    /* LD I,A */
    fn disassembleED__LD_I_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD I,A", address);
    }

    /* IN C,(C) */
    fn disassembleED__IN_C_iC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IN C,(C)", address);
    }

    /* OUT (C),C */
    fn disassembleED__OUT_iC_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUT (C),C", address);
    }

    /* ADC HL,BC */
    fn disassembleED__ADC_HL_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC HL,BC", address);
    }

    /* LD BC,(nnnn) */
    fn disassembleED__LD_BC_iNNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD BC,(0x{:04x})", address, nnnn);
    }

    /* LD R,A */
    fn disassembleED__LD_R_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD R,A", address);
    }

    /* IN D,(C) */
    fn disassembleED__IN_D_iC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IN D,(C)", address);
    }

    /* OUT (C),D */
    fn disassembleED__OUT_iC_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUT (C),D", address);
    }

    /* SBC HL,DE */
    fn disassembleED__SBC_HL_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC HL,DE", address);
    }

    /* LD (nnnn),DE */
    fn disassembleED__LD_iNNNN_DE(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD (0x{:04x}),DE", address, nnnn);
    }

    /* IM 1 */
    fn disassembleED__IM_1(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IM 1", address);
    }

    /* LD A,I */
    fn disassembleED__LD_A_I(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,I", address);
    }

    /* IN E,(C) */
    fn disassembleED__IN_E_iC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IN E,(C)", address);
    }

    /* OUT (C),E */
    fn disassembleED__OUT_iC_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUT (C),E", address);
    }

    /* ADC HL,DE */
    fn disassembleED__ADC_HL_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC HL,DE", address);
    }

    /* LD DE,(nnnn) */
    fn disassembleED__LD_DE_iNNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD DE,(0x{:04x})", address, nnnn);
    }

    /* IM 2 */
    fn disassembleED__IM_2(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IM 2", address);
    }

    /* LD A,R */
    fn disassembleED__LD_A_R(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,R", address);
    }

    /* IN H,(C) */
    fn disassembleED__IN_H_iC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IN H,(C)", address);
    }

    /* OUT (C),H */
    fn disassembleED__OUT_iC_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUT (C),H", address);
    }

    /* SBC HL,HL */
    fn disassembleED__SBC_HL_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC HL,HL", address);
    }

    /* LD (nnnn),HL */
    fn disassembleED__LD_iNNNN_HL(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD (0x{:04x}),HL", address, nnnn);
    }

    /* RRD */
    fn disassembleED__RRD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRD", address);
    }

    /* IN L,(C) */
    fn disassembleED__IN_L_iC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IN L,(C)", address);
    }

    /* OUT (C),L */
    fn disassembleED__OUT_iC_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUT (C),L", address);
    }

    /* ADC HL,HL */
    fn disassembleED__ADC_HL_HL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC HL,HL", address);
    }

    /* LD HL,(nnnn) */
    fn disassembleED__LD_HL_iNNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD HL,(0x{:04x})", address, nnnn);
    }

    /* RLD */
    fn disassembleED__RLD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLD", address);
    }

    /* IN F,(C) */
    fn disassembleED__IN_F_iC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IN F,(C)", address);
    }

    /* OUT (C),0 */
    fn disassembleED__OUT_iC_0(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUT (C),0", address);
    }

    /* SBC HL,SP */
    fn disassembleED__SBC_HL_SP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC HL,SP", address);
    }

    /* LD (nnnn),SP */
    fn disassembleED__LD_iNNNN_SP(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD (0x{:04x}),SP", address, nnnn);
    }

    /* IN A,(C) */
    fn disassembleED__IN_A_iC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IN A,(C)", address);
    }

    /* OUT (C),A */
    fn disassembleED__OUT_iC_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUT (C),A", address);
    }

    /* ADC HL,SP */
    fn disassembleED__ADC_HL_SP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC HL,SP", address);
    }

    /* LD SP,(nnnn) */
    fn disassembleED__LD_SP_iNNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD SP,(0x{:04x})", address, nnnn);
    }

    /* LDI */
    fn disassembleED__LDI(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LDI", address);
    }

    /* CPI */
    fn disassembleED__CPI(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CPI", address);
    }

    /* INI */
    fn disassembleED__INI(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INI", address);
    }

    /* OUTI */
    fn disassembleED__OUTI(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUTI", address);
    }

    /* LDD */
    fn disassembleED__LDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LDD", address);
    }

    /* CPD */
    fn disassembleED__CPD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CPD", address);
    }

    /* IND */
    fn disassembleED__IND(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) IND", address);
    }

    /* OUTD */
    fn disassembleED__OUTD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OUTD", address);
    }

    /* LDIR */
    fn disassembleED__LDIR(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LDIR", address);
    }

    /* CPIR */
    fn disassembleED__CPIR(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CPIR", address);
    }

    /* INIR */
    fn disassembleED__INIR(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INIR", address);
    }

    /* OTIR */
    fn disassembleED__OTIR(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OTIR", address);
    }

    /* LDDR */
    fn disassembleED__LDDR(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LDDR", address);
    }

    /* CPDR */
    fn disassembleED__CPDR(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CPDR", address);
    }

    /* INDR */
    fn disassembleED__INDR(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INDR", address);
    }

    /* OTDR */
    fn disassembleED__OTDR(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OTDR", address);
    }

    /* slttrap */
    fn disassembleED__SLTTRAP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) slttrap", address);
    }

    /* ADD REGISTER,BC */
    fn disassembleDD__ADD_REG_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD ix,BC", address);
    }

    /* ADD REGISTER,DE */
    fn disassembleDD__ADD_REG_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD ix,DE", address);
    }

    /* LD REGISTER,nnnn */
    fn disassembleDD__LD_REG_NNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD ix,0x{:04x}", address, nnnn);
    }

    /* LD (nnnn),REGISTER */
    fn disassembleDD__LD_iNNNN_REG(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD (0x{:04x}),ix", address, nnnn);
    }

    /* INC REGISTER */
    fn disassembleDD__INC_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC ix", address);
    }

    /* INC REGISTERH */
    fn disassembleDD__INC_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC IXH", address);
    }

    /* DEC REGISTERH */
    fn disassembleDD__DEC_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC IXH", address);
    }

    /* LD REGISTERH,nn */
    fn disassembleDD__LD_REGH_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) LD IXH,0x{:04x}", address, nn);
    }

    /* ADD REGISTER,REGISTER */
    fn disassembleDD__ADD_REG_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD REGISTER,REGISTER", address);
    }

    /* LD REGISTER,(nnnn) */
    fn disassembleDD__LD_REG_iNNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD ix,(0x{:04x})", address, nnnn);
    }

    /* DEC REGISTER */
    fn disassembleDD__DEC_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC REGISTER", address);
    }

    /* INC REGISTERL */
    fn disassembleDD__INC_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC REGISTERL", address);
    }

    /* DEC REGISTERL */
    fn disassembleDD__DEC_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC REGISTERL", address);
    }

    /* LD REGISTERL,nn */
    fn disassembleDD__LD_REGL_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) LD IXL,0x{:02x}", address, nn);
    }

    /* INC (REGISTER+dd) */
    fn disassembleDD__INC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC (REGISTER+dd)", address);
    }

    /* DEC (REGISTER+dd) */
    fn disassembleDD__DEC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC (REGISTER+dd)", address);
    }

    /* LD (REGISTER+dd),nn */
    fn disassembleDD__LD_iREGpDD_NN(&mut self) {
        let address = self.PC() - 1;
        let ix_dd = self.memory.read_byte(address + 1);
        let nn = self.memory.read_byte(address + 2);
        println!("({:04x}) LD (ix+0x{:02x}),0x{:02x}", address, ix_dd, nn);
    }

    /* ADD REGISTER,SP */
    fn disassembleDD__ADD_REG_SP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD REGISTER,SP", address);
    }

    /* LD B,REGISTERH */
    fn disassembleDD__LD_B_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,REGISTERH", address);
    }

    /* LD B,REGISTERL */
    fn disassembleDD__LD_B_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,REGISTERL", address);
    }

    /* LD B,(REGISTER+dd) */
    fn disassembleDD__LD_B_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,(REGISTER+dd)", address);
    }

    /* LD C,REGISTERH */
    fn disassembleDD__LD_C_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,REGISTERH", address);
    }

    /* LD C,REGISTERL */
    fn disassembleDD__LD_C_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,REGISTERL", address);
    }

    /* LD C,(REGISTER+dd) */
    fn disassembleDD__LD_C_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,(REGISTER+dd)", address);
    }

    /* LD D,REGISTERH */
    fn disassembleDD__LD_D_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,REGISTERH", address);
    }

    /* LD D,REGISTERL */
    fn disassembleDD__LD_D_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,REGISTERL", address);
    }

    /* LD D,(REGISTER+dd) */
    fn disassembleDD__LD_D_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,(REGISTER+dd)", address);
    }

    /* LD E,REGISTERH */
    fn disassembleDD__LD_E_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,REGISTERH", address);
    }

    /* LD E,REGISTERL */
    fn disassembleDD__LD_E_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,REGISTERL", address);
    }

    /* LD E,(REGISTER+dd) */
    fn disassembleDD__LD_E_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,(REGISTER+dd)", address);
    }

    /* LD REGISTERH,B */
    fn disassembleDD__LD_REGH_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,B", address);
    }

    /* LD REGISTERH,C */
    fn disassembleDD__LD_REGH_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,C", address);
    }

    /* LD REGISTERH,D */
    fn disassembleDD__LD_REGH_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,D", address);
    }

    /* LD REGISTERH,E */
    fn disassembleDD__LD_REGH_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,E", address);
    }

    /* LD REGISTERH,REGISTERH */
    fn disassembleDD__LD_REGH_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,REGISTERH", address);
    }

    /* LD REGISTERH,REGISTERL */
    fn disassembleDD__LD_REGH_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,REGISTERL", address);
    }

    /* LD H,(REGISTER+dd) */
    fn disassembleDD__LD_H_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,(REGISTER+dd)", address);
    }

    /* LD REGISTERH,A */
    fn disassembleDD__LD_REGH_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,A", address);
    }

    /* LD REGISTERL,B */
    fn disassembleDD__LD_REGL_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,B", address);
    }

    /* LD REGISTERL,C */
    fn disassembleDD__LD_REGL_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,C", address);
    }

    /* LD REGISTERL,D */
    fn disassembleDD__LD_REGL_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,D", address);
    }

    /* LD REGISTERL,E */
    fn disassembleDD__LD_REGL_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,E", address);
    }

    /* LD REGISTERL,REGISTERH */
    fn disassembleDD__LD_REGL_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,REGISTERH", address);
    }

    /* LD REGISTERL,REGISTERL */
    fn disassembleDD__LD_REGL_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,REGISTERL", address);
    }

    /* LD L,(REGISTER+dd) */
    fn disassembleDD__LD_L_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,(REGISTER+dd)", address);
    }

    /* LD REGISTERL,A */
    fn disassembleDD__LD_REGL_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,A", address);
    }

    /* LD (REGISTER+dd),B */
    fn disassembleDD__LD_iREGpDD_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),B", address);
    }

    /* LD (REGISTER+dd),C */
    fn disassembleDD__LD_iREGpDD_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),C", address);
    }

    /* LD (REGISTER+dd),D */
    fn disassembleDD__LD_iREGpDD_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),D", address);
    }

    /* LD (REGISTER+dd),E */
    fn disassembleDD__LD_iREGpDD_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),E", address);
    }

    /* LD (REGISTER+dd),H */
    fn disassembleDD__LD_iREGpDD_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),H", address);
    }

    /* LD (REGISTER+dd),L */
    fn disassembleDD__LD_iREGpDD_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),L", address);
    }

    /* LD (REGISTER+dd),A */
    fn disassembleDD__LD_iREGpDD_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),A", address);
    }

    /* LD A,REGISTERH */
    fn disassembleDD__LD_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,REGISTERH", address);
    }

    /* LD A,REGISTERL */
    fn disassembleDD__LD_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,REGISTERL", address);
    }

    /* LD A,(REGISTER+dd) */
    fn disassembleDD__LD_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,(REGISTER+dd)", address);
    }

    /* ADD A,REGISTERH */
    fn disassembleDD__ADD_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,REGISTERH", address);
    }

    /* ADD A,REGISTERL */
    fn disassembleDD__ADD_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,REGISTERL", address);
    }

    /* ADD A,(REGISTER+dd) */
    fn disassembleDD__ADD_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,(REGISTER+dd)", address);
    }

    /* ADC A,REGISTERH */
    fn disassembleDD__ADC_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,REGISTERH", address);
    }

    /* ADC A,REGISTERL */
    fn disassembleDD__ADC_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,REGISTERL", address);
    }

    /* ADC A,(REGISTER+dd) */
    fn disassembleDD__ADC_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,(REGISTER+dd)", address);
    }

    /* SUB A,REGISTERH */
    fn disassembleDD__SUB_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,REGISTERH", address);
    }

    /* SUB A,REGISTERL */
    fn disassembleDD__SUB_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,REGISTERL", address);
    }

    /* SUB A,(REGISTER+dd) */
    fn disassembleDD__SUB_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,(REGISTER+dd)", address);
    }

    /* SBC A,REGISTERH */
    fn disassembleDD__SBC_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,REGISTERH", address);
    }

    /* SBC A,REGISTERL */
    fn disassembleDD__SBC_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,REGISTERL", address);
    }

    /* SBC A,(REGISTER+dd) */
    fn disassembleDD__SBC_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,(REGISTER+dd)", address);
    }

    /* AND A,REGISTERH */
    fn disassembleDD__AND_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,REGISTERH", address);
    }

    /* AND A,REGISTERL */
    fn disassembleDD__AND_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,REGISTERL", address);
    }

    /* AND A,(REGISTER+dd) */
    fn disassembleDD__AND_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,(REGISTER+dd)", address);
    }

    /* XOR A,REGISTERH */
    fn disassembleDD__XOR_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,REGISTERH", address);
    }

    /* XOR A,REGISTERL */
    fn disassembleDD__XOR_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,REGISTERL", address);
    }

    /* XOR A,(REGISTER+dd) */
    fn disassembleDD__XOR_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,(REGISTER+dd)", address);
    }

    /* OR A,REGISTERH */
    fn disassembleDD__OR_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,REGISTERH", address);
    }

    /* OR A,REGISTERL */
    fn disassembleDD__OR_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,REGISTERL", address);
    }

    /* OR A,(REGISTER+dd) */
    fn disassembleDD__OR_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,(REGISTER+dd)", address);
    }

    /* CP A,REGISTERH */
    fn disassembleDD__CP_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP A,REGISTERH", address);
    }

    /* CP A,REGISTERL */
    fn disassembleDD__CP_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP A,REGISTERL", address);
    }

    /* CP A,(REGISTER+dd) */
    fn disassembleDD__CP_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP A,(REGISTER+dd)", address);
    }

    /* shift DDFDCB */
    fn disassembleDD__SHIFT_DDFDCB(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) shift DDFDCB", address);
    }

    /* POP REGISTER */
    fn disassembleDD__POP_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) POP REGISTER", address);
    }

    /* EX (SP),REGISTER */
    fn disassembleDD__EX_iSP_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) EX (SP),REGISTER", address);
    }

    /* PUSH REGISTER */
    fn disassembleDD__PUSH_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) PUSH REGISTER", address);
    }

    /* JP REGISTER */
    fn disassembleDD__JP_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) JP REGISTER", address);
    }

    /* LD SP,REGISTER */
    fn disassembleDD__LD_SP_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD SP,REGISTER", address);
    }

    /* ADD REGISTER,BC */
    fn disassembleFD__ADD_REG_BC(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD REGISTER,BC", address);
    }

    /* ADD REGISTER,DE */
    fn disassembleFD__ADD_REG_DE(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD REGISTER,DE", address);
    }

    /* LD REGISTER,nnnn */
    fn disassembleFD__LD_REG_NNNN(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTER,nnnn", address);
    }

    /* LD (nnnn),REGISTER */
    fn disassembleFD__LD_iNNNN_REG(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD (0x{:04x}),iy", address, nnnn);
    }

    /* INC REGISTER */
    fn disassembleFD__INC_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC REGISTER", address);
    }

    /* INC REGISTERH */
    fn disassembleFD__INC_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC REGISTERH", address);
    }

    /* DEC REGISTERH */
    fn disassembleFD__DEC_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC REGISTERH", address);
    }

    /* LD REGISTERH,nn */
    fn disassembleFD__LD_REGH_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) LD IYH,0x{:02x}", address, nn);
    }

    /* ADD REGISTER,REGISTER */
    fn disassembleFD__ADD_REG_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD REGISTER,REGISTER", address);
    }

    /* LD REGISTER,(nnnn) */
    fn disassembleFD__LD_REG_iNNNN(&mut self) {
        let address = self.PC() - 1;
        let b1 = self.memory.read_byte(address + 1);
        let b2 = self.memory.read_byte(address + 2);
        let nnnn = join_bytes(b2, b1);
        println!("({:04x}) LD iy,(0x{:04x})", address, nnnn);
    }

    /* DEC REGISTER */
    fn disassembleFD__DEC_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC REGISTER", address);
    }

    /* INC REGISTERL */
    fn disassembleFD__INC_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC REGISTERL", address);
    }

    /* DEC REGISTERL */
    fn disassembleFD__DEC_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC REGISTERL", address);
    }

    /* LD REGISTERL,nn */
    fn disassembleFD__LD_REGL_NN(&mut self) {
        let address = self.PC() - 1;
        let nn = self.memory.read_byte(address + 1);
        println!("({:04x}) LD IYL,0x{:02x}", address, nn);
    }

    /* INC (REGISTER+dd) */
    fn disassembleFD__INC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) INC (REGISTER+dd)", address);
    }

    /* DEC (REGISTER+dd) */
    fn disassembleFD__DEC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) DEC (REGISTER+dd)", address);
    }

    /* LD (REGISTER+dd),nn */
    fn disassembleFD__LD_iREGpDD_NN(&mut self) {
        let address = self.PC() - 1;
        let iy_dd = self.memory.read_byte(address + 1);
        let nn = self.memory.read_byte(address + 2);
        println!("({:04x}) LD (iy+0x{:02x}),0x{:02x}", address, iy_dd, nn);
    }

    /* ADD REGISTER,SP */
    fn disassembleFD__ADD_REG_SP(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD REGISTER,SP", address);
    }

    /* LD B,REGISTERH */
    fn disassembleFD__LD_B_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,REGISTERH", address);
    }

    /* LD B,REGISTERL */
    fn disassembleFD__LD_B_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,REGISTERL", address);
    }

    /* LD B,(REGISTER+dd) */
    fn disassembleFD__LD_B_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,(REGISTER+dd)", address);
    }

    /* LD C,REGISTERH */
    fn disassembleFD__LD_C_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,REGISTERH", address);
    }

    /* LD C,REGISTERL */
    fn disassembleFD__LD_C_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,REGISTERL", address);
    }

    /* LD C,(REGISTER+dd) */
    fn disassembleFD__LD_C_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,(REGISTER+dd)", address);
    }

    /* LD D,REGISTERH */
    fn disassembleFD__LD_D_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,REGISTERH", address);
    }

    /* LD D,REGISTERL */
    fn disassembleFD__LD_D_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,REGISTERL", address);
    }

    /* LD D,(REGISTER+dd) */
    fn disassembleFD__LD_D_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,(REGISTER+dd)", address);
    }

    /* LD E,REGISTERH */
    fn disassembleFD__LD_E_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,REGISTERH", address);
    }

    /* LD E,REGISTERL */
    fn disassembleFD__LD_E_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,REGISTERL", address);
    }

    /* LD E,(REGISTER+dd) */
    fn disassembleFD__LD_E_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,(REGISTER+dd)", address);
    }

    /* LD REGISTERH,B */
    fn disassembleFD__LD_REGH_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,B", address);
    }

    /* LD REGISTERH,C */
    fn disassembleFD__LD_REGH_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,C", address);
    }

    /* LD REGISTERH,D */
    fn disassembleFD__LD_REGH_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,D", address);
    }

    /* LD REGISTERH,E */
    fn disassembleFD__LD_REGH_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,E", address);
    }

    /* LD REGISTERH,REGISTERH */
    fn disassembleFD__LD_REGH_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,REGISTERH", address);
    }

    /* LD REGISTERH,REGISTERL */
    fn disassembleFD__LD_REGH_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,REGISTERL", address);
    }

    /* LD H,(REGISTER+dd) */
    fn disassembleFD__LD_H_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,(REGISTER+dd)", address);
    }

    /* LD REGISTERH,A */
    fn disassembleFD__LD_REGH_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERH,A", address);
    }

    /* LD REGISTERL,B */
    fn disassembleFD__LD_REGL_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,B", address);
    }

    /* LD REGISTERL,C */
    fn disassembleFD__LD_REGL_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,C", address);
    }

    /* LD REGISTERL,D */
    fn disassembleFD__LD_REGL_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,D", address);
    }

    /* LD REGISTERL,E */
    fn disassembleFD__LD_REGL_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,E", address);
    }

    /* LD REGISTERL,REGISTERH */
    fn disassembleFD__LD_REGL_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,REGISTERH", address);
    }

    /* LD REGISTERL,REGISTERL */
    fn disassembleFD__LD_REGL_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,REGISTERL", address);
    }

    /* LD L,(REGISTER+dd) */
    fn disassembleFD__LD_L_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,(REGISTER+dd)", address);
    }

    /* LD REGISTERL,A */
    fn disassembleFD__LD_REGL_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD REGISTERL,A", address);
    }

    /* LD (REGISTER+dd),B */
    fn disassembleFD__LD_iREGpDD_B(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),B", address);
    }

    /* LD (REGISTER+dd),C */
    fn disassembleFD__LD_iREGpDD_C(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),C", address);
    }

    /* LD (REGISTER+dd),D */
    fn disassembleFD__LD_iREGpDD_D(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),D", address);
    }

    /* LD (REGISTER+dd),E */
    fn disassembleFD__LD_iREGpDD_E(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),E", address);
    }

    /* LD (REGISTER+dd),H */
    fn disassembleFD__LD_iREGpDD_H(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),H", address);
    }

    /* LD (REGISTER+dd),L */
    fn disassembleFD__LD_iREGpDD_L(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),L", address);
    }

    /* LD (REGISTER+dd),A */
    fn disassembleFD__LD_iREGpDD_A(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD (REGISTER+dd),A", address);
    }

    /* LD A,REGISTERH */
    fn disassembleFD__LD_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,REGISTERH", address);
    }

    /* LD A,REGISTERL */
    fn disassembleFD__LD_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,REGISTERL", address);
    }

    /* LD A,(REGISTER+dd) */
    fn disassembleFD__LD_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,(REGISTER+dd)", address);
    }

    /* ADD A,REGISTERH */
    fn disassembleFD__ADD_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,REGISTERH", address);
    }

    /* ADD A,REGISTERL */
    fn disassembleFD__ADD_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,REGISTERL", address);
    }

    /* ADD A,(REGISTER+dd) */
    fn disassembleFD__ADD_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADD A,(REGISTER+dd)", address);
    }

    /* ADC A,REGISTERH */
    fn disassembleFD__ADC_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,REGISTERH", address);
    }

    /* ADC A,REGISTERL */
    fn disassembleFD__ADC_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,REGISTERL", address);
    }

    /* ADC A,(REGISTER+dd) */
    fn disassembleFD__ADC_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) ADC A,(REGISTER+dd)", address);
    }

    /* SUB A,REGISTERH */
    fn disassembleFD__SUB_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,REGISTERH", address);
    }

    /* SUB A,REGISTERL */
    fn disassembleFD__SUB_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,REGISTERL", address);
    }

    /* SUB A,(REGISTER+dd) */
    fn disassembleFD__SUB_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SUB A,(REGISTER+dd)", address);
    }

    /* SBC A,REGISTERH */
    fn disassembleFD__SBC_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,REGISTERH", address);
    }

    /* SBC A,REGISTERL */
    fn disassembleFD__SBC_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,REGISTERL", address);
    }

    /* SBC A,(REGISTER+dd) */
    fn disassembleFD__SBC_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SBC A,(REGISTER+dd)", address);
    }

    /* AND A,REGISTERH */
    fn disassembleFD__AND_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,REGISTERH", address);
    }

    /* AND A,REGISTERL */
    fn disassembleFD__AND_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,REGISTERL", address);
    }

    /* AND A,(REGISTER+dd) */
    fn disassembleFD__AND_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) AND A,(REGISTER+dd)", address);
    }

    /* XOR A,REGISTERH */
    fn disassembleFD__XOR_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,REGISTERH", address);
    }

    /* XOR A,REGISTERL */
    fn disassembleFD__XOR_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,REGISTERL", address);
    }

    /* XOR A,(REGISTER+dd) */
    fn disassembleFD__XOR_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) XOR A,(REGISTER+dd)", address);
    }

    /* OR A,REGISTERH */
    fn disassembleFD__OR_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,REGISTERH", address);
    }

    /* OR A,REGISTERL */
    fn disassembleFD__OR_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,REGISTERL", address);
    }

    /* OR A,(REGISTER+dd) */
    fn disassembleFD__OR_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) OR A,(REGISTER+dd)", address);
    }

    /* CP A,REGISTERH */
    fn disassembleFD__CP_A_REGH(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP A,REGISTERH", address);
    }

    /* CP A,REGISTERL */
    fn disassembleFD__CP_A_REGL(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP A,REGISTERL", address);
    }

    /* CP A,(REGISTER+dd) */
    fn disassembleFD__CP_A_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) CP A,(REGISTER+dd)", address);
    }

    /* shift DDFDCB */
    fn disassembleFD__SHIFT_DDFDCB(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) shift DDFDCB", address);
    }

    /* POP REGISTER */
    fn disassembleFD__POP_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) POP REGISTER", address);
    }

    /* EX (SP),REGISTER */
    fn disassembleFD__EX_iSP_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) EX (SP),REGISTER", address);
    }

    /* PUSH REGISTER */
    fn disassembleFD__PUSH_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) PUSH REGISTER", address);
    }

    /* JP REGISTER */
    fn disassembleFD__JP_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) JP REGISTER", address);
    }

    /* LD SP,REGISTER */
    fn disassembleFD__LD_SP_REG(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD SP,REGISTER", address);
    }

    /* LD B,RLC (REGISTER+dd) */
    fn disassembleDDCB__LD_B_RLC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RLC (REGISTER+dd)", address);
    }

    /* LD C,RLC (REGISTER+dd) */
    fn disassembleDDCB__LD_C_RLC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RLC (REGISTER+dd)", address);
    }

    /* LD D,RLC (REGISTER+dd) */
    fn disassembleDDCB__LD_D_RLC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RLC (REGISTER+dd)", address);
    }

    /* LD E,RLC (REGISTER+dd) */
    fn disassembleDDCB__LD_E_RLC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RLC (REGISTER+dd)", address);
    }

    /* LD H,RLC (REGISTER+dd) */
    fn disassembleDDCB__LD_H_RLC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RLC (REGISTER+dd)", address);
    }

    /* LD L,RLC (REGISTER+dd) */
    fn disassembleDDCB__LD_L_RLC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RLC (REGISTER+dd)", address);
    }

    /* RLC (REGISTER+dd) */
    fn disassembleDDCB__RLC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RLC (REGISTER+dd)", address);
    }

    /* LD A,RLC (REGISTER+dd) */
    fn disassembleDDCB__LD_A_RLC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RLC (REGISTER+dd)", address);
    }

    /* LD B,RRC (REGISTER+dd) */
    fn disassembleDDCB__LD_B_RRC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RRC (REGISTER+dd)", address);
    }

    /* LD C,RRC (REGISTER+dd) */
    fn disassembleDDCB__LD_C_RRC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RRC (REGISTER+dd)", address);
    }

    /* LD D,RRC (REGISTER+dd) */
    fn disassembleDDCB__LD_D_RRC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RRC (REGISTER+dd)", address);
    }

    /* LD E,RRC (REGISTER+dd) */
    fn disassembleDDCB__LD_E_RRC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RRC (REGISTER+dd)", address);
    }

    /* LD H,RRC (REGISTER+dd) */
    fn disassembleDDCB__LD_H_RRC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RRC (REGISTER+dd)", address);
    }

    /* LD L,RRC (REGISTER+dd) */
    fn disassembleDDCB__LD_L_RRC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RRC (REGISTER+dd)", address);
    }

    /* RRC (REGISTER+dd) */
    fn disassembleDDCB__RRC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RRC (REGISTER+dd)", address);
    }

    /* LD A,RRC (REGISTER+dd) */
    fn disassembleDDCB__LD_A_RRC_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RRC (REGISTER+dd)", address);
    }

    /* LD B,RL (REGISTER+dd) */
    fn disassembleDDCB__LD_B_RL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RL (REGISTER+dd)", address);
    }

    /* LD C,RL (REGISTER+dd) */
    fn disassembleDDCB__LD_C_RL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RL (REGISTER+dd)", address);
    }

    /* LD D,RL (REGISTER+dd) */
    fn disassembleDDCB__LD_D_RL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RL (REGISTER+dd)", address);
    }

    /* LD E,RL (REGISTER+dd) */
    fn disassembleDDCB__LD_E_RL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RL (REGISTER+dd)", address);
    }

    /* LD H,RL (REGISTER+dd) */
    fn disassembleDDCB__LD_H_RL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RL (REGISTER+dd)", address);
    }

    /* LD L,RL (REGISTER+dd) */
    fn disassembleDDCB__LD_L_RL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RL (REGISTER+dd)", address);
    }

    /* RL (REGISTER+dd) */
    fn disassembleDDCB__RL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RL (REGISTER+dd)", address);
    }

    /* LD A,RL (REGISTER+dd) */
    fn disassembleDDCB__LD_A_RL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RL (REGISTER+dd)", address);
    }

    /* LD B,RR (REGISTER+dd) */
    fn disassembleDDCB__LD_B_RR_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RR (REGISTER+dd)", address);
    }

    /* LD C,RR (REGISTER+dd) */
    fn disassembleDDCB__LD_C_RR_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RR (REGISTER+dd)", address);
    }

    /* LD D,RR (REGISTER+dd) */
    fn disassembleDDCB__LD_D_RR_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RR (REGISTER+dd)", address);
    }

    /* LD E,RR (REGISTER+dd) */
    fn disassembleDDCB__LD_E_RR_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RR (REGISTER+dd)", address);
    }

    /* LD H,RR (REGISTER+dd) */
    fn disassembleDDCB__LD_H_RR_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RR (REGISTER+dd)", address);
    }

    /* LD L,RR (REGISTER+dd) */
    fn disassembleDDCB__LD_L_RR_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RR (REGISTER+dd)", address);
    }

    /* RR (REGISTER+dd) */
    fn disassembleDDCB__RR_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RR (REGISTER+dd)", address);
    }

    /* LD A,RR (REGISTER+dd) */
    fn disassembleDDCB__LD_A_RR_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RR (REGISTER+dd)", address);
    }

    /* LD B,SLA (REGISTER+dd) */
    fn disassembleDDCB__LD_B_SLA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SLA (REGISTER+dd)", address);
    }

    /* LD C,SLA (REGISTER+dd) */
    fn disassembleDDCB__LD_C_SLA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SLA (REGISTER+dd)", address);
    }

    /* LD D,SLA (REGISTER+dd) */
    fn disassembleDDCB__LD_D_SLA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SLA (REGISTER+dd)", address);
    }

    /* LD E,SLA (REGISTER+dd) */
    fn disassembleDDCB__LD_E_SLA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SLA (REGISTER+dd)", address);
    }

    /* LD H,SLA (REGISTER+dd) */
    fn disassembleDDCB__LD_H_SLA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SLA (REGISTER+dd)", address);
    }

    /* LD L,SLA (REGISTER+dd) */
    fn disassembleDDCB__LD_L_SLA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SLA (REGISTER+dd)", address);
    }

    /* SLA (REGISTER+dd) */
    fn disassembleDDCB__SLA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLA (REGISTER+dd)", address);
    }

    /* LD A,SLA (REGISTER+dd) */
    fn disassembleDDCB__LD_A_SLA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SLA (REGISTER+dd)", address);
    }

    /* LD B,SRA (REGISTER+dd) */
    fn disassembleDDCB__LD_B_SRA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SRA (REGISTER+dd)", address);
    }

    /* LD C,SRA (REGISTER+dd) */
    fn disassembleDDCB__LD_C_SRA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SRA (REGISTER+dd)", address);
    }

    /* LD D,SRA (REGISTER+dd) */
    fn disassembleDDCB__LD_D_SRA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SRA (REGISTER+dd)", address);
    }

    /* LD E,SRA (REGISTER+dd) */
    fn disassembleDDCB__LD_E_SRA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SRA (REGISTER+dd)", address);
    }

    /* LD H,SRA (REGISTER+dd) */
    fn disassembleDDCB__LD_H_SRA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SRA (REGISTER+dd)", address);
    }

    /* LD L,SRA (REGISTER+dd) */
    fn disassembleDDCB__LD_L_SRA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SRA (REGISTER+dd)", address);
    }

    /* SRA (REGISTER+dd) */
    fn disassembleDDCB__SRA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRA (REGISTER+dd)", address);
    }

    /* LD A,SRA (REGISTER+dd) */
    fn disassembleDDCB__LD_A_SRA_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SRA (REGISTER+dd)", address);
    }

    /* LD B,SLL (REGISTER+dd) */
    fn disassembleDDCB__LD_B_SLL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SLL (REGISTER+dd)", address);
    }

    /* LD C,SLL (REGISTER+dd) */
    fn disassembleDDCB__LD_C_SLL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SLL (REGISTER+dd)", address);
    }

    /* LD D,SLL (REGISTER+dd) */
    fn disassembleDDCB__LD_D_SLL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SLL (REGISTER+dd)", address);
    }

    /* LD E,SLL (REGISTER+dd) */
    fn disassembleDDCB__LD_E_SLL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SLL (REGISTER+dd)", address);
    }

    /* LD H,SLL (REGISTER+dd) */
    fn disassembleDDCB__LD_H_SLL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SLL (REGISTER+dd)", address);
    }

    /* LD L,SLL (REGISTER+dd) */
    fn disassembleDDCB__LD_L_SLL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SLL (REGISTER+dd)", address);
    }

    /* SLL (REGISTER+dd) */
    fn disassembleDDCB__SLL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SLL (REGISTER+dd)", address);
    }

    /* LD A,SLL (REGISTER+dd) */
    fn disassembleDDCB__LD_A_SLL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SLL (REGISTER+dd)", address);
    }

    /* LD B,SRL (REGISTER+dd) */
    fn disassembleDDCB__LD_B_SRL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SRL (REGISTER+dd)", address);
    }

    /* LD C,SRL (REGISTER+dd) */
    fn disassembleDDCB__LD_C_SRL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SRL (REGISTER+dd)", address);
    }

    /* LD D,SRL (REGISTER+dd) */
    fn disassembleDDCB__LD_D_SRL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SRL (REGISTER+dd)", address);
    }

    /* LD E,SRL (REGISTER+dd) */
    fn disassembleDDCB__LD_E_SRL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SRL (REGISTER+dd)", address);
    }

    /* LD H,SRL (REGISTER+dd) */
    fn disassembleDDCB__LD_H_SRL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SRL (REGISTER+dd)", address);
    }

    /* LD L,SRL (REGISTER+dd) */
    fn disassembleDDCB__LD_L_SRL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SRL (REGISTER+dd)", address);
    }

    /* SRL (REGISTER+dd) */
    fn disassembleDDCB__SRL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SRL (REGISTER+dd)", address);
    }

    /* LD A,SRL (REGISTER+dd) */
    fn disassembleDDCB__LD_A_SRL_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SRL (REGISTER+dd)", address);
    }

    /* BIT 0,(REGISTER+dd) */
    fn disassembleDDCB__BIT_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 0,(REGISTER+dd)", address);
    }

    /* BIT 1,(REGISTER+dd) */
    fn disassembleDDCB__BIT_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 1,(REGISTER+dd)", address);
    }

    /* BIT 2,(REGISTER+dd) */
    fn disassembleDDCB__BIT_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 2,(REGISTER+dd)", address);
    }

    /* BIT 3,(REGISTER+dd) */
    fn disassembleDDCB__BIT_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 3,(REGISTER+dd)", address);
    }

    /* BIT 4,(REGISTER+dd) */
    fn disassembleDDCB__BIT_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 4,(REGISTER+dd)", address);
    }

    /* BIT 5,(REGISTER+dd) */
    fn disassembleDDCB__BIT_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 5,(REGISTER+dd)", address);
    }

    /* BIT 6,(REGISTER+dd) */
    fn disassembleDDCB__BIT_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 6,(REGISTER+dd)", address);
    }

    /* BIT 7,(REGISTER+dd) */
    fn disassembleDDCB__BIT_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) BIT 7,(REGISTER+dd)", address);
    }

    /* LD B,RES 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_RES_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RES 0,(REGISTER+dd)", address);
    }

    /* LD C,RES 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_RES_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RES 0,(REGISTER+dd)", address);
    }

    /* LD D,RES 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_RES_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RES 0,(REGISTER+dd)", address);
    }

    /* LD E,RES 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_RES_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RES 0,(REGISTER+dd)", address);
    }

    /* LD H,RES 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_RES_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RES 0,(REGISTER+dd)", address);
    }

    /* LD L,RES 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_RES_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RES 0,(REGISTER+dd)", address);
    }

    /* RES 0,(REGISTER+dd) */
    fn disassembleDDCB__RES_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 0,(REGISTER+dd)", address);
    }

    /* LD A,RES 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_RES_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RES 0,(REGISTER+dd)", address);
    }

    /* LD B,RES 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_RES_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RES 1,(REGISTER+dd)", address);
    }

    /* LD C,RES 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_RES_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RES 1,(REGISTER+dd)", address);
    }

    /* LD D,RES 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_RES_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RES 1,(REGISTER+dd)", address);
    }

    /* LD E,RES 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_RES_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RES 1,(REGISTER+dd)", address);
    }

    /* LD H,RES 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_RES_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RES 1,(REGISTER+dd)", address);
    }

    /* LD L,RES 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_RES_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RES 1,(REGISTER+dd)", address);
    }

    /* RES 1,(REGISTER+dd) */
    fn disassembleDDCB__RES_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 1,(REGISTER+dd)", address);
    }

    /* LD A,RES 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_RES_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RES 1,(REGISTER+dd)", address);
    }

    /* LD B,RES 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_RES_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RES 2,(REGISTER+dd)", address);
    }

    /* LD C,RES 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_RES_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RES 2,(REGISTER+dd)", address);
    }

    /* LD D,RES 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_RES_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RES 2,(REGISTER+dd)", address);
    }

    /* LD E,RES 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_RES_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RES 2,(REGISTER+dd)", address);
    }

    /* LD H,RES 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_RES_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RES 2,(REGISTER+dd)", address);
    }

    /* LD L,RES 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_RES_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RES 2,(REGISTER+dd)", address);
    }

    /* RES 2,(REGISTER+dd) */
    fn disassembleDDCB__RES_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 2,(REGISTER+dd)", address);
    }

    /* LD A,RES 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_RES_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RES 2,(REGISTER+dd)", address);
    }

    /* LD B,RES 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_RES_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RES 3,(REGISTER+dd)", address);
    }

    /* LD C,RES 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_RES_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RES 3,(REGISTER+dd)", address);
    }

    /* LD D,RES 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_RES_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RES 3,(REGISTER+dd)", address);
    }

    /* LD E,RES 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_RES_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RES 3,(REGISTER+dd)", address);
    }

    /* LD H,RES 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_RES_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RES 3,(REGISTER+dd)", address);
    }

    /* LD L,RES 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_RES_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RES 3,(REGISTER+dd)", address);
    }

    /* RES 3,(REGISTER+dd) */
    fn disassembleDDCB__RES_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 3,(REGISTER+dd)", address);
    }

    /* LD A,RES 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_RES_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RES 3,(REGISTER+dd)", address);
    }

    /* LD B,RES 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_RES_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RES 4,(REGISTER+dd)", address);
    }

    /* LD C,RES 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_RES_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RES 4,(REGISTER+dd)", address);
    }

    /* LD D,RES 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_RES_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RES 4,(REGISTER+dd)", address);
    }

    /* LD E,RES 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_RES_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RES 4,(REGISTER+dd)", address);
    }

    /* LD H,RES 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_RES_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RES 4,(REGISTER+dd)", address);
    }

    /* LD L,RES 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_RES_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RES 4,(REGISTER+dd)", address);
    }

    /* RES 4,(REGISTER+dd) */
    fn disassembleDDCB__RES_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 4,(REGISTER+dd)", address);
    }

    /* LD A,RES 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_RES_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RES 4,(REGISTER+dd)", address);
    }

    /* LD B,RES 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_RES_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RES 5,(REGISTER+dd)", address);
    }

    /* LD C,RES 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_RES_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RES 5,(REGISTER+dd)", address);
    }

    /* LD D,RES 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_RES_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RES 5,(REGISTER+dd)", address);
    }

    /* LD E,RES 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_RES_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RES 5,(REGISTER+dd)", address);
    }

    /* LD H,RES 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_RES_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RES 5,(REGISTER+dd)", address);
    }

    /* LD L,RES 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_RES_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RES 5,(REGISTER+dd)", address);
    }

    /* RES 5,(REGISTER+dd) */
    fn disassembleDDCB__RES_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 5,(REGISTER+dd)", address);
    }

    /* LD A,RES 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_RES_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RES 5,(REGISTER+dd)", address);
    }

    /* LD B,RES 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_RES_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RES 6,(REGISTER+dd)", address);
    }

    /* LD C,RES 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_RES_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RES 6,(REGISTER+dd)", address);
    }

    /* LD D,RES 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_RES_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RES 6,(REGISTER+dd)", address);
    }

    /* LD E,RES 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_RES_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RES 6,(REGISTER+dd)", address);
    }

    /* LD H,RES 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_RES_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RES 6,(REGISTER+dd)", address);
    }

    /* LD L,RES 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_RES_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RES 6,(REGISTER+dd)", address);
    }

    /* RES 6,(REGISTER+dd) */
    fn disassembleDDCB__RES_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 6,(REGISTER+dd)", address);
    }

    /* LD A,RES 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_RES_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RES 6,(REGISTER+dd)", address);
    }

    /* LD B,RES 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_RES_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,RES 7,(REGISTER+dd)", address);
    }

    /* LD C,RES 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_RES_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,RES 7,(REGISTER+dd)", address);
    }

    /* LD D,RES 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_RES_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,RES 7,(REGISTER+dd)", address);
    }

    /* LD E,RES 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_RES_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,RES 7,(REGISTER+dd)", address);
    }

    /* LD H,RES 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_RES_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,RES 7,(REGISTER+dd)", address);
    }

    /* LD L,RES 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_RES_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,RES 7,(REGISTER+dd)", address);
    }

    /* RES 7,(REGISTER+dd) */
    fn disassembleDDCB__RES_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) RES 7,(REGISTER+dd)", address);
    }

    /* LD A,RES 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_RES_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,RES 7,(REGISTER+dd)", address);
    }

    /* LD B,SET 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_SET_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SET 0,(REGISTER+dd)", address);
    }

    /* LD C,SET 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_SET_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SET 0,(REGISTER+dd)", address);
    }

    /* LD D,SET 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_SET_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SET 0,(REGISTER+dd)", address);
    }

    /* LD E,SET 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_SET_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SET 0,(REGISTER+dd)", address);
    }

    /* LD H,SET 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_SET_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SET 0,(REGISTER+dd)", address);
    }

    /* LD L,SET 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_SET_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SET 0,(REGISTER+dd)", address);
    }

    /* SET 0,(REGISTER+dd) */
    fn disassembleDDCB__SET_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 0,(REGISTER+dd)", address);
    }

    /* LD A,SET 0,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_SET_0_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SET 0,(REGISTER+dd)", address);
    }

    /* LD B,SET 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_SET_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SET 1,(REGISTER+dd)", address);
    }

    /* LD C,SET 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_SET_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SET 1,(REGISTER+dd)", address);
    }

    /* LD D,SET 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_SET_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SET 1,(REGISTER+dd)", address);
    }

    /* LD E,SET 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_SET_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SET 1,(REGISTER+dd)", address);
    }

    /* LD H,SET 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_SET_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SET 1,(REGISTER+dd)", address);
    }

    /* LD L,SET 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_SET_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SET 1,(REGISTER+dd)", address);
    }

    /* SET 1,(REGISTER+dd) */
    fn disassembleDDCB__SET_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 1,(REGISTER+dd)", address);
    }

    /* LD A,SET 1,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_SET_1_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SET 1,(REGISTER+dd)", address);
    }

    /* LD B,SET 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_SET_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SET 2,(REGISTER+dd)", address);
    }

    /* LD C,SET 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_SET_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SET 2,(REGISTER+dd)", address);
    }

    /* LD D,SET 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_SET_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SET 2,(REGISTER+dd)", address);
    }

    /* LD E,SET 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_SET_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SET 2,(REGISTER+dd)", address);
    }

    /* LD H,SET 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_SET_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SET 2,(REGISTER+dd)", address);
    }

    /* LD L,SET 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_SET_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SET 2,(REGISTER+dd)", address);
    }

    /* SET 2,(REGISTER+dd) */
    fn disassembleDDCB__SET_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 2,(REGISTER+dd)", address);
    }

    /* LD A,SET 2,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_SET_2_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SET 2,(REGISTER+dd)", address);
    }

    /* LD B,SET 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_SET_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SET 3,(REGISTER+dd)", address);
    }

    /* LD C,SET 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_SET_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SET 3,(REGISTER+dd)", address);
    }

    /* LD D,SET 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_SET_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SET 3,(REGISTER+dd)", address);
    }

    /* LD E,SET 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_SET_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SET 3,(REGISTER+dd)", address);
    }

    /* LD H,SET 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_SET_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SET 3,(REGISTER+dd)", address);
    }

    /* LD L,SET 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_SET_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SET 3,(REGISTER+dd)", address);
    }

    /* SET 3,(REGISTER+dd) */
    fn disassembleDDCB__SET_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 3,(REGISTER+dd)", address);
    }

    /* LD A,SET 3,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_SET_3_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SET 3,(REGISTER+dd)", address);
    }

    /* LD B,SET 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_SET_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SET 4,(REGISTER+dd)", address);
    }

    /* LD C,SET 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_SET_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SET 4,(REGISTER+dd)", address);
    }

    /* LD D,SET 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_SET_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SET 4,(REGISTER+dd)", address);
    }

    /* LD E,SET 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_SET_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SET 4,(REGISTER+dd)", address);
    }

    /* LD H,SET 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_SET_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SET 4,(REGISTER+dd)", address);
    }

    /* LD L,SET 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_SET_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SET 4,(REGISTER+dd)", address);
    }

    /* SET 4,(REGISTER+dd) */
    fn disassembleDDCB__SET_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 4,(REGISTER+dd)", address);
    }

    /* LD A,SET 4,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_SET_4_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SET 4,(REGISTER+dd)", address);
    }

    /* LD B,SET 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_SET_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SET 5,(REGISTER+dd)", address);
    }

    /* LD C,SET 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_SET_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SET 5,(REGISTER+dd)", address);
    }

    /* LD D,SET 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_SET_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SET 5,(REGISTER+dd)", address);
    }

    /* LD E,SET 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_SET_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SET 5,(REGISTER+dd)", address);
    }

    /* LD H,SET 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_SET_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SET 5,(REGISTER+dd)", address);
    }

    /* LD L,SET 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_SET_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SET 5,(REGISTER+dd)", address);
    }

    /* SET 5,(REGISTER+dd) */
    fn disassembleDDCB__SET_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 5,(REGISTER+dd)", address);
    }

    /* LD A,SET 5,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_SET_5_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SET 5,(REGISTER+dd)", address);
    }

    /* LD B,SET 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_SET_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SET 6,(REGISTER+dd)", address);
    }

    /* LD C,SET 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_SET_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SET 6,(REGISTER+dd)", address);
    }

    /* LD D,SET 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_SET_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SET 6,(REGISTER+dd)", address);
    }

    /* LD E,SET 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_SET_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SET 6,(REGISTER+dd)", address);
    }

    /* LD H,SET 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_SET_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SET 6,(REGISTER+dd)", address);
    }

    /* LD L,SET 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_SET_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SET 6,(REGISTER+dd)", address);
    }

    /* SET 6,(REGISTER+dd) */
    fn disassembleDDCB__SET_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 6,(REGISTER+dd)", address);
    }

    /* LD A,SET 6,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_SET_6_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SET 6,(REGISTER+dd)", address);
    }

    /* LD B,SET 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_B_SET_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD B,SET 7,(REGISTER+dd)", address);
    }

    /* LD C,SET 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_C_SET_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD C,SET 7,(REGISTER+dd)", address);
    }

    /* LD D,SET 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_D_SET_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD D,SET 7,(REGISTER+dd)", address);
    }

    /* LD E,SET 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_E_SET_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD E,SET 7,(REGISTER+dd)", address);
    }

    /* LD H,SET 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_H_SET_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD H,SET 7,(REGISTER+dd)", address);
    }

    /* LD L,SET 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_L_SET_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD L,SET 7,(REGISTER+dd)", address);
    }

    /* SET 7,(REGISTER+dd) */
    fn disassembleDDCB__SET_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) SET 7,(REGISTER+dd)", address);
    }

    /* LD A,SET 7,(REGISTER+dd) */
    fn disassembleDDCB__LD_A_SET_7_iREGpDD(&mut self) {
        let address = self.PC() - 1;
        println!("({:04x}) LD A,SET 7,(REGISTER+dd)", address);
    }
}
