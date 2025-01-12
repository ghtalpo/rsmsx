// package z80

// http://map.grauw.nl/resources/z80instr.php

use super::z80_base::{
    FLAG_C, FLAG_P, FLAG_S, FLAG_Z, SHIFT_0X_CB, SHIFT_0X_DD, SHIFT_0X_DDCB, SHIFT_0X_ED,
    SHIFT_0X_FD, Z80,
};

impl Z80 {
    pub fn get_timings(&self, opcode: u16) -> u64 {
        match opcode {
            0 => {
                /* NOP */
                5
            }
            0x2 => {
                /* LD (BC),A */
                8
            }
            0x3 => {
                /* INC BC */
                7
            }
            0x4 => {
                /* INC B */
                5
            }
            0x5 => {
                /* DEC B */
                5
            }
            0x7 => {
                /* RLCA */
                5
            }
            0x8 => {
                /* EX AF,AF' */
                5
            }
            0x9 => {
                /* ADD HL,BC */
                12
            }
            0x12 => {
                /* LD (DE),A */
                8
            }
            0x13 => {
                /* INC DE */
                7
            }
            0x14 => {
                /* INC D */
                5
            }
            0x15 => {
                /* DEC D */
                5
            }
            0x17 => {
                /* RLA */
                5
            }
            0x19 => {
                /* ADD HL,DE */
                12
            }
            0x23 => {
                /* INC HL */
                7
            }
            0x24 => {
                /* INC H */
                5
            }
            0x25 => {
                /* DEC H */
                5
            }
            0x27 => {
                /* DAA */
                5
            }
            0x29 => {
                /* ADD HL,HL */
                12
            }
            0x33 => {
                /* INC SP */
                7
            }
            0x34 => {
                /* INC (HL) */
                12
            }
            0x35 => {
                /* DEC (HL) */
                12
            }
            0x37 => {
                /* SCF */
                5
            }
            0x39 => {
                /* ADD HL,SP */
                12
            }
            0x46 => {
                /* LD B,(HL) */
                8
            }
            0x56 => {
                /* LD D,(HL) */
                8
            }
            0x66 => {
                /* LD H,(HL) */
                8
            }
            0x76 => {
                /* HALT */
                5
            }
            0x86 => {
                /* LD A,(HL) */
                8
            }
            0x96 => {
                /* SUB (HL) */
                8
            }
            0x1 => {
                /* LD BC,nn */
                11
            }
            0x6 => {
                /* LD B,n */
                8
            }
            0x0a => {
                /* LD A,(BC) */
                8
            }
            0x0b => {
                /* DEC BC */
                7
            }
            0x0c => {
                /* INC C */
                5
            }
            0x0d => {
                /* DEC C */
                5
            }
            0x0e => {
                /* LD C,n */
                8
            }
            0x0f => {
                /* RRCA */
                5
            }
            0x10 => {
                /* DJNZ o */
                0
            }
            0x11 => {
                /* LD DE,nn */
                11
            }
            0x16 => {
                /* LD D,n */
                8
            }
            0x18 => {
                /* JR o */
                13
            }
            0x1a => {
                /* LD A,(DE) */
                8
            }
            0x1b => {
                /* DEC DE */
                7
            }
            0x1c => {
                /* INC E */
                5
            }
            0x1d => {
                /* DEC E */
                5
            }
            0x1e => {
                /* LD E,n */
                8
            }
            0x1f => {
                /* RRA */
                5
            }
            0x21 => {
                /* LD HL,nn */
                11
            }
            0x22 => {
                /* LD (nn),HL */
                17
            }
            0x26 => {
                /* LD H,n */
                8
            }
            0x2a => {
                /* LD HL,(nn) */
                17
            }
            0x2b => {
                /* DEC HL */
                7
            }
            0x2c => {
                /* INC L */
                5
            }
            0x2d => {
                /* DEC L */
                5
            }
            0x2e => {
                /* LD L,n */
                8
            }
            0x2f => {
                /* CPL */
                5
            }
            0x31 => {
                /* LD SP,nn */
                11
            }
            0x32 => {
                /* LD (nn),A */
                14
            }
            0x36 => {
                /* LD (HL),n */
                11
            }
            0x3a => {
                /* LD A,(nn) */
                14
            }
            0x3b => {
                /* DEC SP */
                7
            }
            0x3c => {
                /* INC A */
                5
            }
            0x3d => {
                /* DEC A */
                5
            }
            0x3e => {
                /* LD A,n */
                8
            }
            0x3f => {
                /* CCF */
                5
            }
            0x40 => {
                /* LD B,r */
                5
            }
            0x48 => {
                /* LD C,r */
                5
            }
            0x4e => {
                /* LD C,(HL) */
                8
            }
            0x50 => {
                /* LD D,r */
                5
            }
            0x58 => {
                /* LD E,r */
                5
            }
            0x5e => {
                /* LD E,(HL) */
                8
            }
            0x60 => {
                /* LD H,r */
                5
            }
            0x68 => {
                /* LD L,r */
                5
            }
            0x6e => {
                /* LD L,(HL) */
                8
            }
            0x70 => {
                /* LD (HL),r */
                8
            }
            0x78 => {
                /* LD A,r */
                5
            }
            0x7e => {
                /* LD A,(HL) */
                8
            }
            0x80 => {
                /* ADD A,r */
                5
            }
            0x88 => {
                /* ADC A,r */
                5
            }
            0x8e => {
                /* ADC A,(HL) */
                8
            }
            0x90 => {
                /* SUB r */
                5
            }
            0x98 => {
                /* SBC A,r */
                5
            }
            0x9e => {
                /* SBC A,(HL) */
                8
            }
            0xa0 => {
                /* AND r */
                5
            }
            0xa6 => {
                /* AND (HL) */
                8
            }
            0xa8 => {
                /* XOR r */
                5
            }
            0xae => {
                /* XOR (HL) */
                8
            }
            0xb0 => {
                /* OR r */
                5
            }
            0xb6 => {
                /* OR (HL) */
                8
            }
            0xb8 => {
                /* CP r */
                5
            }
            0xbe => {
                /* CP (HL) */
                8
            }
            0xc1 => {
                /* POP BC */
                11
            }
            0xc2 => {
                /* JP NZ,nn */
                11
            }
            0xc3 => {
                /* JP nn */
                11
            }
            0xc5 => {
                /* PUSH BC */
                12
            }
            0xc6 => {
                /* ADD A,n */
                8
            }
            0xc7 => {
                /* RST 0 */
                12
            }
            0xc9 => {
                /* RET */
                11
            }
            0xca => {
                /* JP Z,nn */
                11
            }
            val if val == SHIFT_0X_CB => {
                /* RLC r */
                10
            }
            val if val == SHIFT_0X_CB + 0x06 => {
                /* RLC (HL) */
                17
            }
            val if val == SHIFT_0X_CB + 0x08 => {
                /* RRC r */
                10
            }
            val if val == SHIFT_0X_CB + 0x0e => {
                /* RRC (HL) */
                17
            }
            val if val == SHIFT_0X_CB + 0x10 => {
                /* RL r */
                10
            }
            val if val == SHIFT_0X_CB + 0x16 => {
                /* RL (HL) */
                17
            }
            val if val == SHIFT_0X_CB + 0x18 => {
                /* RR r */
                10
            }
            val if val == SHIFT_0X_CB + 0x1e => {
                /* RR (HL) */
                17
            }
            val if val == SHIFT_0X_CB + 0x20 => {
                /* SLA r */
                10
            }
            val if val == SHIFT_0X_CB + 0x26 => {
                /* SLA (HL) */
                17
            }
            val if val == SHIFT_0X_CB + 0x28 => {
                /* SRA r */
                10
            }
            val if val == SHIFT_0X_CB + 0x2e => {
                /* SRA (HL) */
                17
            }
            val if val == SHIFT_0X_CB + 0x38 => {
                /* SRL r */
                10
            }
            val if val == SHIFT_0X_CB + 0x3e => {
                /* SRL (HL) */
                17
            }
            val if val == SHIFT_0X_CB + 0x40 => {
                /* BIT b,r */
                10
            }
            val if val == SHIFT_0X_CB + 0x46 => {
                /* BIT b,(HL) */
                14
            }
            val if val == SHIFT_0X_CB + 0x80 => {
                /* RES b,r */
                10
            }
            val if val == SHIFT_0X_CB + 0x86 => {
                /* RES b,(HL) */
                17
            }
            val if val == SHIFT_0X_CB + 0xc0 => {
                /* SET b,r */
                10
            }
            val if val == SHIFT_0X_CB + 0xc6 => {
                /* SET b,(HL) */
                17
            }
            0xcd => {
                /* CALL nn */
                18
            }
            0xce => {
                /* ADC A,n */
                8
            }
            0xcf => {
                /* RST 8H */
                12
            }
            0xd1 => {
                /* POP DE */
                11
            }
            0xd2 => {
                /* JP NC,nn */
                11
            }
            0xd3 => {
                /* OUT (n),A */
                12
            }
            0xd5 => {
                /* PUSH DE */
                12
            }
            0xd6 => {
                /* SUB n */
                8
            }
            0xd7 => {
                /* RST 10H */
                12
            }
            0xd9 => {
                /* EXX */
                5
            }
            0xda => {
                /* JP C,nn */
                11
            }
            0xdb => {
                /* IN A,(n) */
                12
            }
            val if val == SHIFT_0X_DD + 0x04 => {
                /* INC IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x05 => {
                /* DEC IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x09 => {
                /* ADD IX,BC */
                17
            }
            val if val == SHIFT_0X_DD + 0x19 => {
                /* ADD IX,DE */
                17
            }
            val if val == SHIFT_0X_DD + 0x21 => {
                /* LD IX,nn */
                16
            }
            val if val == SHIFT_0X_DD + 0x22 => {
                /* LD (nn),IX */
                22
            }
            val if val == SHIFT_0X_DD + 0x23 => {
                /* INC IX */
                12
            }
            val if val == SHIFT_0X_DD + 0x26 => {
                /* LD IXh,n */
                13
            }
            val if val == SHIFT_0X_DD + 0x29 => {
                /* ADD IX,IX */
                17
            }
            val if val == SHIFT_0X_DD + 0x2a => {
                /* LD IX,(nn) */
                22
            }
            val if val == SHIFT_0X_DD + 0x2b => {
                /* DEC IX */
                12
            }
            val if val == SHIFT_0X_DD + 0x2e => {
                /* LD IXl,n */
                13
            }
            val if val == SHIFT_0X_DD + 0x34 => {
                /* INC (IX+o) */
                25
            }
            val if val == SHIFT_0X_DD + 0x35 => {
                /* DEC (IX+o) */
                25
            }
            val if val == SHIFT_0X_DD + 0x36 => {
                /* LD (IX+o),n */
                21
            }
            val if val == SHIFT_0X_DD + 0x39 => {
                /* ADD IX,SP */
                17
            }
            val if val == SHIFT_0X_DD + 0x40 => {
                /* LD B,IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x46 => {
                /* LD B,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x48 => {
                /* LD C,IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x4e => {
                /* LD C,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x50 => {
                /* LD D,IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x56 => {
                /* LD D,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x58 => {
                /* LD E,IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x5e => {
                /* LD E,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x60 => {
                /* LD IXh,p */
                10
            }
            val if val == SHIFT_0X_DD + 0x66 => {
                /* LD H,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x68 => {
                /* LD IXl,p */
                10
            }
            val if val == SHIFT_0X_DD + 0x6e => {
                /* LD L,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x70 => {
                /* LD (IX+o),r */
                21
            }
            val if val == SHIFT_0X_DD + 0x78 => {
                /* LD A,IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x7e => {
                /* LD A,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x80 => {
                /* ADD A,IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x86 => {
                /* ADD A,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x88 => {
                /* ADC A,IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x8e => {
                /* ADC A,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x90 => {
                /* SUB IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x96 => {
                /* SUB (IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0x98 => {
                /* SBC A,IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0x9e => {
                /* SBC A,(IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0xa0 => {
                /* AND IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0xa6 => {
                /* AND (IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0xa8 => {
                /* XOR IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0xae => {
                /* XOR (IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0xb0 => {
                /* OR IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0xb6 => {
                /* OR (IX+o) */
                21
            }
            val if val == SHIFT_0X_DD + 0xb8 => {
                /* CP IXp */
                10
            }
            val if val == SHIFT_0X_DD + 0xbe => {
                /* CP (IX+o) */
                21
            }
            val if val == SHIFT_0X_DDCB + 0x06 => {
                /* RLC (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0x0e => {
                /* RRC (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0x16 => {
                /* RL (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0x1e => {
                /* RR (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0x26 => {
                /* SLA (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0x2e => {
                /* SRA (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0x3e => {
                /* SRL (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0x46 => {
                /* BIT (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0x86 => {
                /* RES (IX+o) */
                25
            }
            val if val == SHIFT_0X_DDCB + 0xc6 => {
                /* SET (IX+o) */
                25
            }
            val if val == SHIFT_0X_DD + 0xe1 => {
                /* POP IX */
                16
            }
            val if val == SHIFT_0X_DD + 0xe3 => {
                /* EX (SP),IX */
                25
            }
            val if val == SHIFT_0X_DD + 0xe5 => {
                /* PUSH IX */
                17
            }
            val if val == SHIFT_0X_DD + 0xe9 => {
                /* JP (IX) */
                10
            }
            val if val == SHIFT_0X_DD + 0xf9 => {
                /* LD SP,IX */
                12
            }
            0xde => {
                /* SBC A,n */
                8
            }
            0xdf => {
                /* RST 18H */
                12
            }
            0xe1 => {
                /* POP HL */
                11
            }
            0xe2 => {
                /* JP PO,nn */
                11
            }
            0xe3 => {
                /* EX (SP),HL */
                20
            }
            0xe5 => {
                /* PUSH HL */
                12
            }
            0xe6 => {
                /* AND n */
                8
            }
            0xe7 => {
                /* RST 20H */
                12
            }
            0xe9 => {
                /* JP (HL) */
                5
            }
            0xea => {
                /* JP PE,nn */
                11
            }
            0xeb => {
                /* EX DE,HL */
                5
            }
            val if val == SHIFT_0X_ED + 0x40 => {
                /* IN B,(C) */
                14
            }
            val if val == SHIFT_0X_ED + 0x41 => {
                /* OUT (C),B */
                14
            }
            val if val == SHIFT_0X_ED + 0x42 => {
                /* SBC HL,BC */
                17
            }
            val if val == SHIFT_0X_ED + 0x43 => {
                /* LD (nn),BC */
                22
            }
            val if val == SHIFT_0X_ED + 0x44 => {
                /* NEG */
                10
            }

            val if val == SHIFT_0X_ED + 0x45 => {
                /* RETN */
                16
            }

            val if val == SHIFT_0X_ED + 0x46 => {
                /* IM 0 */
                10
            }

            val if val == SHIFT_0X_ED + 0x47 => {
                /* LD I,A */
                11
            }

            val if val == SHIFT_0X_ED + 0x48 => {
                /* IN C,(C) */
                14
            }

            val if val == SHIFT_0X_ED + 0x49 => {
                /* OUT (C),C */
                14
            }

            val if val == SHIFT_0X_ED + 0x4a => {
                /* ADC HL,BC */
                17
            }

            val if val == SHIFT_0X_ED + 0x4b => {
                /* LD BC,(nn) */
                22
            }

            val if val == SHIFT_0X_ED + 0x4d => {
                /* RETI */
                16
            }

            val if val == SHIFT_0X_ED + 0x4f => {
                /* LD R,A */
                11
            }

            val if val == SHIFT_0X_ED + 0x50 => {
                /* IN D,(C) */
                14
            }

            val if val == SHIFT_0X_ED + 0x51 => {
                /* OUT (C),D */
                14
            }

            val if val == SHIFT_0X_ED + 0x52 => {
                /* SBC HL,DE */
                17
            }

            val if val == SHIFT_0X_ED + 0x53 => {
                /* LD (nn),DE */
                22
            }

            val if val == SHIFT_0X_ED + 0x56 => {
                /* IM 1 */
                10
            }

            val if val == SHIFT_0X_ED + 0x57 => {
                /* LD A,I */
                11
            }

            val if val == SHIFT_0X_ED + 0x58 => {
                /* IN E,(C) */
                14
            }

            val if val == SHIFT_0X_ED + 0x59 => {
                /* OUT (C),E */
                14
            }

            val if val == SHIFT_0X_ED + 0x5a => {
                /* ADC HL,DE */
                17
            }

            val if val == SHIFT_0X_ED + 0x5b => {
                /* LD DE,(nn) */
                22
            }

            val if val == SHIFT_0X_ED + 0x5e => {
                /* IM 2 */
                10
            }

            val if val == SHIFT_0X_ED + 0x5f => {
                /* LD A,R */
                11
            }

            val if val == SHIFT_0X_ED + 0x60 => {
                /* IN H,(C) */
                14
            }

            val if val == SHIFT_0X_ED + 0x61 => {
                /* OUT (C),H */
                14
            }

            val if val == SHIFT_0X_ED + 0x62 => {
                /* SBC HL,HL */
                17
            }

            val if val == SHIFT_0X_ED + 0x67 => {
                /* RRD */
                20
            }

            val if val == SHIFT_0X_ED + 0x68 => {
                /* IN L,(C) */
                14
            }

            val if val == SHIFT_0X_ED + 0x69 => {
                /* OUT (C),L */
                14
            }

            val if val == SHIFT_0X_ED + 0x6a => {
                /* ADC HL,HL */
                17
            }

            val if val == SHIFT_0X_ED + 0x6f => {
                /* RLD */
                20
            }

            val if val == SHIFT_0X_ED + 0x70 => {
                /* IN F,(C) */
                14
            }

            val if val == SHIFT_0X_ED + 0x72 => {
                /* SBC HL,SP */
                17
            }

            val if val == SHIFT_0X_ED + 0x73 => {
                /* LD (nn),SP */
                22
            }

            val if val == SHIFT_0X_ED + 0x78 => {
                /* IN A,(C) */
                14
            }

            val if val == SHIFT_0X_ED + 0x79 => {
                /* OUT (C),A */
                14
            }

            val if val == SHIFT_0X_ED + 0x7a => {
                /* ADC HL,SP */
                17
            }

            val if val == SHIFT_0X_ED + 0x7b => {
                /* LD SP,(nn) */
                22
            }

            val if val == SHIFT_0X_ED + 0xa0 => {
                /* LDI */
                18
            }

            val if val == SHIFT_0X_ED + 0xa1 => {
                /* CPI */
                18
            }

            val if val == SHIFT_0X_ED + 0xa2 => {
                /* INI */
                18
            }

            val if val == SHIFT_0X_ED + 0xa3 => {
                /* OUTI */
                18
            }

            val if val == SHIFT_0X_ED + 0xa8 => {
                /* LDD */
                18
            }

            val if val == SHIFT_0X_ED + 0xa9 => {
                /* CPD */
                18
            }

            val if val == SHIFT_0X_ED + 0xaa => {
                /* IND */
                18
            }

            val if val == SHIFT_0X_ED + 0xab => {
                /* OUTD */
                18
            }

            val if val == SHIFT_0X_ED + 0xb0 => {
                /* LDIR */
                0
            }

            val if val == SHIFT_0X_ED + 0xb1 => {
                /* CPIR */
                0
            }

            val if val == SHIFT_0X_ED + 0xb2 => {
                /* INIR */
                0
            }

            val if val == SHIFT_0X_ED + 0xb3 => {
                /* OTIR */
                0
            }

            val if val == SHIFT_0X_ED + 0xb8 => {
                /* LDDR */
                0
            }

            val if val == SHIFT_0X_ED + 0xb9 => {
                /* CPDR */
                0
            }

            val if val == SHIFT_0X_ED + 0xba => {
                /* INDR */
                0
            }

            val if val == SHIFT_0X_ED + 0xbb => {
                /* OTDR */
                0
            }

            val if val == SHIFT_0X_ED + 0xc1 => {
                /* MULUB A,r */
                0
            }

            val if val == SHIFT_0X_ED + 0xc3 => {
                /* MULUW HL,BC */
                0
            }

            val if val == SHIFT_0X_ED + 0xf3 => {
                /* MULUW HL,SP */
                0
            }
            0xee => {
                /* XOR n */
                8
            }

            0xef => {
                /* RST 28H */
                12
            }

            0xf1 => {
                /* POP AF */
                11
            }

            0xf2 => {
                /* JP P,nn */
                11
            }

            0xf3 => {
                /* DI */
                5
            }

            0xf5 => {
                /* PUSH AF */
                12
            }

            0xf6 => {
                /* OR n */
                8
            }

            0xf7 => {
                /* RST 30H */
                12
            }

            0xf9 => {
                /* LD SP,HL */
                7
            }

            0xfa => {
                /* JP M,nn */
                11
            }

            0xfb => {
                /* EI */
                5
            }

            val if val == SHIFT_0X_FD + 0x04 => {
                /* INC IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x05 => {
                /* DEC IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x09 => {
                /* ADD IY,BC */
                17
            }

            val if val == SHIFT_0X_FD + 0x19 => {
                /* ADD IY,DE */
                17
            }

            val if val == SHIFT_0X_FD + 0x21 => {
                /* LD IY,nn */
                16
            }

            val if val == SHIFT_0X_FD + 0x22 => {
                /* LD (nn),IY */
                22
            }

            val if val == SHIFT_0X_FD + 0x23 => {
                /* INC IY */
                12
            }

            val if val == SHIFT_0X_FD + 0x26 => {
                /* LD IYh,n */
                13
            }

            val if val == SHIFT_0X_FD + 0x29 => {
                /* ADD IY,IY */
                17
            }

            val if val == SHIFT_0X_FD + 0x2a => {
                /* LD IY,(nn) */
                22
            }

            val if val == SHIFT_0X_FD + 0x2b => {
                /* DEC IY */
                12
            }

            val if val == SHIFT_0X_FD + 0x2e => {
                /* LD IYl,n */
                13
            }

            val if val == SHIFT_0X_FD + 0x34 => {
                /* INC (IY+o) */
                25
            }

            val if val == SHIFT_0X_FD + 0x35 => {
                /* DEC (IY+o) */
                25
            }

            val if val == SHIFT_0X_FD + 0x36 => {
                /* LD (IY+o),n */
                21
            }

            val if val == SHIFT_0X_FD + 0x39 => {
                /* ADD IY,SP */
                17
            }

            val if val == SHIFT_0X_FD + 0x40 => {
                /* LD B,IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x46 => {
                /* LD B,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x48 => {
                /* LD C,IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x4e => {
                /* LD C,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x50 => {
                /* LD D,IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x56 => {
                /* LD D,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x58 => {
                /* LD E,IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x5e => {
                /* LD E,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x60 => {
                /* LD IYh,q */
                10
            }

            val if val == SHIFT_0X_FD + 0x66 => {
                /* LD H,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x68 => {
                /* LD IYl,q */
                10
            }

            val if val == SHIFT_0X_FD + 0x6e => {
                /* LD L,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x70 => {
                /* LD (IY+o),r */
                21
            }

            val if val == SHIFT_0X_FD + 0x78 => {
                /* LD A,IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x7e => {
                /* LD A,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x80 => {
                /* ADD A,IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x86 => {
                /* ADD A,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x88 => {
                /* ADC A,IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x8e => {
                /* ADC A,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x90 => {
                /* SUB IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x96 => {
                /* SUB (IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0x98 => {
                /* SBC A,IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0x9e => {
                /* SBC A,(IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0xa0 => {
                /* AND IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0xa6 => {
                /* AND (IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0xa8 => {
                /* XOR IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0xae => {
                /* XOR (IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0xb0 => {
                /* OR IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0xb6 => {
                /* OR (IY+o) */
                21
            }

            val if val == SHIFT_0X_FD + 0xb8 => {
                /* CP IYq */
                10
            }

            val if val == SHIFT_0X_FD + 0xbe => {
                /* CP (IY+o) */
                21
            }

            val if val == SHIFT_0X_DDCB + 0x06 => {
                /* RLC (IY+o) */
                25
            }

            val if val == SHIFT_0X_DDCB + 0x0e => {
                /* RRC (IY+o) */
                25
            }

            val if val == SHIFT_0X_DDCB + 0x16 => {
                /* RL (IY+o) */
                25
            }

            val if val == SHIFT_0X_DDCB + 0x1e => {
                /* RR (IY+o) */
                25
            }

            val if val == SHIFT_0X_DDCB + 0x26 => {
                /* SLA (IY+o) */
                25
            }

            val if val == SHIFT_0X_DDCB + 0x2e => {
                /* SRA (IY+o) */
                25
            }

            val if val == SHIFT_0X_DDCB + 0x3e => {
                /* SRL (IY+o) */
                25
            }

            val if val == SHIFT_0X_DDCB + 0x46 => {
                /* BIT b,(IY+o) */
                22
            }

            val if val == SHIFT_0X_DDCB + 0x86 => {
                /* RES b,(IY+o) */
                25
            }

            val if val == SHIFT_0X_DDCB + 0xc6 => {
                /* SET b,(IY+o) */
                25
            }

            val if val == SHIFT_0X_FD + 0xe1 => {
                /* POP IY */
                16
            }

            val if val == SHIFT_0X_FD + 0xe3 => {
                /* EX (SP),IY */
                25
            }

            val if val == SHIFT_0X_FD + 0xe5 => {
                /* PUSH IY */
                17
            }

            val if val == SHIFT_0X_FD + 0xe9 => {
                /* JP (IY) */
                10
            }

            val if val == SHIFT_0X_FD + 0xf9 => {
                /* LD SP,IY */
                12
            }

            0xfe => {
                /* CP n */
                8
            }

            0xff => {
                /* RST 38H */
                12
            }
            0xdc => {
                // CALL C, nn
                if self.data.F & FLAG_C != 0 {
                    18
                } else {
                    11
                }
            }
            0xfc => {
                // CALL M, nn
                if self.data.F & FLAG_S != 0 {
                    18
                } else {
                    11
                }
            }
            0xd4 => {
                // CALL NC, nn
                if self.data.F & FLAG_C == 0 {
                    18
                } else {
                    11
                }
            }
            0xc4 => {
                // CALL NZ, nn
                if self.data.F & FLAG_Z == 0 {
                    18
                } else {
                    11
                }
            }
            0xf4 => {
                // CALL P, nn
                if self.data.F & FLAG_S == 0 {
                    18
                } else {
                    11
                }
            }
            0xec => {
                // CALL PE, nn
                if self.data.F & FLAG_P != 0 {
                    18
                } else {
                    11
                }
            }
            0xe4 => {
                // CALL PO, nn
                if self.data.F & FLAG_P == 0 {
                    18
                } else {
                    11
                }
            }
            0xcc => {
                // CALL Z, nn
                if self.data.F & FLAG_Z != 0 {
                    18
                } else {
                    11
                }
            }
            // 0x10 => {
            //     // DJNZ o
            //     if self.B != 0 {
            //         return 14;
            //     }
            //     return 9;
            // }
            0x38 => {
                // JR C, nn
                if self.data.F & FLAG_C != 0 {
                    13
                } else {
                    8
                }
            }
            0x30 => {
                // JR NC, nn
                if self.data.F & FLAG_C == 0 {
                    13
                } else {
                    8
                }
            }
            0x28 => {
                // JR Z, nn
                if self.data.F & FLAG_Z != 0 {
                    13
                } else {
                    8
                }
            }
            0x20 => {
                // JR NZ, nn
                if self.data.F & FLAG_Z == 0 {
                    13
                } else {
                    8
                }
            }
            0xd8 => {
                // RET C
                if self.data.F & FLAG_C != 0 {
                    12
                } else {
                    6
                }
            }
            0xf8 => {
                // RET M
                if self.data.F & FLAG_S != 0 {
                    12
                } else {
                    6
                }
            }
            0xd0 => {
                // RET NC
                if self.data.F & FLAG_C == 0 {
                    12
                } else {
                    6
                }
            }
            0xc0 => {
                // RET NZ
                if self.data.F & FLAG_Z == 0 {
                    12
                } else {
                    6
                }
            }
            0xf0 => {
                // RET P
                if self.data.F & FLAG_S == 0 {
                    12
                } else {
                    6
                }
            }
            0xe8 => {
                // RET PE
                if self.data.F & FLAG_P != 0 {
                    12
                } else {
                    6
                }
            }
            0xe0 => {
                // RET PO
                if self.data.F & FLAG_P == 0 {
                    12
                } else {
                    6
                }
            }
            0xc8 => {
                // RET Z
                if self.data.F & FLAG_Z != 0 {
                    12
                } else {
                    6
                }
            }
            _ => 0,
        }
    }
}
