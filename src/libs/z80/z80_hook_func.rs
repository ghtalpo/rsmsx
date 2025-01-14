use crate::libs::z80::z80_base::{FLAG_C, FLAG_S, FLAG_Z};

use super::z80_base::{join_bytes, Z80};

const SCREEN2_PATTERN_GENERATOR_TABLE_SIZE: u16 = 0x1800;
const SCREEN_2_VRAM_SPRITE_TABLE_BEGIN: u16 = 0x1b00;
const DAT_IO_0098: u8 = 0x98;
const DAT_IO_0099: u8 = 0x99;

#[allow(non_snake_case, dead_code)]
impl Z80 {
    fn assert_pc(&self, addr: u16) {
        assert!(
            self.PC() == addr,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            addr
        );
    }
    pub(crate) fn read_word(&self, addr: u16) -> u16 {
        let l = self.memory.read_byte(addr);
        let h = self.memory.read_byte(addr + 1);
        join_bytes(h, l)
    }
    pub(crate) fn peek_stack(&self, depth: usize) {
        let sp = self.data.sp;
        let mut s = format!("SP(0x{:04x}): ", sp);
        for i in 0..depth {
            s.push_str(format!("0x{:04x} ", self.read_word(sp + 2 * i as u16)).as_str());
        }
        println!("{}", s);
    }

    pub(crate) fn hook_0093(&mut self) -> bool {
        let psgReg = self.data.A;
        let data = self.data.E;
        log::debug!("[FBIOS] hook_0093: reg:{} data:{}", psgReg, data);
        self.write_port(0xa0, psgReg);
        self.write_port(0xa1, data);
        true
    }

    pub(crate) fn hook_0096(&mut self) -> bool {
        let psgReg = self.data.A;
        self.write_port(0xa0, psgReg);
        let data = self.read_port(0xa2);
        log::debug!("[FBIOS] hook_0096: reg:{} data:{}", psgReg, data);
        self.data.A = data;
        true
    }

    pub(crate) fn hook_42ba(&mut self) -> bool {
        //         ram:42ba cd 0e b6        CALL       sb_rand_guess_B60E
        assert!(self.call_hook(0xb60e));
        //         ram:42bd e6 01           AND        0x1
        self.instr_hk__AND_NN(0x1);
        //         ram:42bf ca d2 42        JP         Z,loop_2_init
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP(loop_2_init);
            //                              loop_2_init
            //         ram:42d2 cd 0e b6        CALL       sb_rand_guess_B60E
            assert!(self.call_hook(0xb60e));
            //         ram:42d5 06 04           LD         B,0x4
            self.instr_hk__LD_B_NN(0x4);
            //         ram:42d7 21 6b c3        LD         HL,BYTE_ram_c36b
            self.instr_hk__LD_HL_NNNN(0xc36b);
            //                              loop_2
            loop {
                self.SetPC(0x42da);
                //         ram:42da e6 03           AND        0x3
                self.instr_hk__AND_NN(0x3);
                //         ram:42dc 77              LD         (HL=>BYTE_ram_c36b),A
                self.instr_hk__LD_iHL_A();
                //         ram:42dd 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:42de 3c              INC        A
                self.instr_hk__INC_A();
                //         ram:42df 10 f9           DJNZ       loop_2
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP loop_2;
                } else {
                    self.increase_cycles(8);
                    break;
                }
            }
            //         ram:42e1 c9              RET

            self.assert_pc(0x42e1);
            true
        } else {
            self.SetPC(0x42c2);
            //         ram:42c2 cd 0e b6        CALL       sb_rand_guess_B60E
            assert!(self.call_hook(0xb60e));
            //         ram:42c5 06 04           LD         B,0x4
            self.instr_hk__LD_B_NN(0x4);
            //         ram:42c7 21 6b c3        LD         HL,BYTE_ram_c36b
            self.instr_hk__LD_HL_NNNN(0xc36b);
            //                              loop_1
            loop {
                self.SetPC(0x42ca);
                //         ram:42ca e6 03           AND        0x3
                self.instr_hk__AND_NN(0x3);
                //         ram:42cc 77              LD         (HL=>BYTE_ram_c36b),A
                self.instr_hk__LD_iHL_A();
                //         ram:42cd 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:42ce 3d              DEC        A
                self.instr_hk__DEC_A();
                //         ram:42cf 10 f9           DJNZ       loop_1
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP loop_1;
                } else {
                    self.increase_cycles(8);
                    break;
                }
            }
            self.assert_pc(0x42d1);
            //         ram:42d1 c9              RET
            true
        }
    }
    // fn hook_4308(&mut self) -> bool {
    //     //
    //     //                              **************************************************************
    //     //                              *                          FUNCTION                          *
    //     //                              **************************************************************
    //     //                              ()
    //     //              undefined         A:1            <RETURN>
    //     //                              FUN_ram_4308
    //     //         ram:4308 21 6b c3        LD         HL,BYTE_ram_c36b
    //     self.instr_hk__LD_HL_NNNN(0xc36b);
    //     //         ram:430b 06 04           LD         B,0x4
    //     self.instr_hk__LD_B_NN(0x4);
    //     //                              loop
    //     loop {
    //         //         ram:430d c5              PUSH       BC
    //         self.instr_hk__PUSH_BC();
    //         //         ram:430e e5              PUSH       HL=>BYTE_ram_c36b
    //         self.instr_hk__PUSH_HL();
    //         //         ram:430f 7e              LD         A,(HL=>BYTE_ram_c36b)
    //         self.instr_hk__LD_A_iHL();
    //         //         ram:4310 cd 1c 43        CALL       FUN_ram_431c                                     byte FUN_ram_431c(void)
    //         assert!(self.call_hook(0x431c));
    //         //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
    //         //         ram:4313 e1              POP        HL
    //         self.instr_hk__POP_HL();
    //         //         ram:4314 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:4315 c1              POP        BC
    //         self.instr_hk__POP_BC();
    //         //         ram:4316 10 f5           DJNZ       FUN_ram_4308::loop
    //         self.IncPC(2);
    //         self.decB();
    //         if self.data.B != 0 {
    //             self.increase_cycles(13);
    //             //JP FUN_ram_4308::loop;
    //         } else {
    //             self.increase_cycles(8);
    //             break;
    //         }
    //     }

    //     //         ram:4318 cd 24 88        CALL       FUN_ram_8824                                     ()
    //     assert!(self.call_hook(0x8824));
    //     //         ram:431b c9              RET
    //     // return true;
    //     //
    //     self.assert_pc(0x431b);
    //     true
    // }
    pub(crate) fn hook_4453(&mut self) -> bool {
        //         ram:4453 eb              EX         DE,HL                                            IN hl
        self.instr_hk__EX_DE_HL();
        //         ram:4454 2a b8 c8        LD         HL,(BYTE_ram_c8b8)
        self.instr_hk__LD_HL_iNNNN(0xc8b8);
        //         ram:4457 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:4458 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:4459 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:445a e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:445b 6e              LD         L,(HL)
        self.instr_hk__LD_L_iHL();
        //         ram:445c 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:445e b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:445f ed 52           SBC        HL,DE
        self.instr_hk__SBC_HL_DE();
        //         ram:4461 38 08           JR         C,LAB_ram_446b
        self.IncPC(2);
        if (self.data.F & FLAG_C) == 0 {
            self.increase_cycles(7);
            //         ram:4463 28 08           JR         Z,LAB_ram_446d
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(7);
                //         ram:4465 7d              LD         A,L
                self.instr_hk__LD_A_L();
                //         ram:4466 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:4467 77              LD         (HL),A
                self.instr_hk__LD_iHL_A();
                //         ram:4468 c3 a9 44        JP         l_exit_x
                self.IncPC(3);
                self.increase_cycles(10);
                // JP(l_exit_x);
                self.SetPC(0x44a9);
                //                              l_exit_x                                        XREF[2]:     ram:4468(j), ram:448b(j)
                //         ram:44a9 cd 60 47        CALL       sb_read_mem_for_player_4760                      hl <- addr
                assert!(self.call_hook(0x4760));
                //                                                                                              bc <- player_idx
                //         ram:44ac 7e              LD         A,(HL)
                self.instr_hk__LD_A_iHL();
                //         ram:44ad e6 03           AND        0x3
                self.instr_hk__AND_NN(0x3);
                //         ram:44af 08              EX         AF,AF_
                self.instr_hk__EX_AF_AF_();
                //         ram:44b0 cd ab 47        CALL       sb_read_mem_47ab                                 hl <- c8c2 + *c8b7
                assert!(self.call_hook(0x47ab));
                //         ram:44b3 08              EX         AF,AF_
                self.instr_hk__EX_AF_AF_();
                //         ram:44b4 77              LD         (HL),A
                self.instr_hk__LD_iHL_A();
                self.assert_pc(0x44b5);
                //         ram:44b5 c9              RET
                return true;
            //
            // true
            } else {
                self.increase_cycles(12);
                // JR(LAB_ram_446d);
            }
        } else {
            self.increase_cycles(12);
            // JR(LAB_ram_446b);
            //                              LAB_ram_446b                                    XREF[1]:     ram:4461(j)
            //         ram:446b 2e 00           LD         L,0x0
            self.instr_hk__LD_L_NN(0x0);
            // JR(LAB_ram_446d);
        }

        //                              LAB_ram_446d                                    XREF[1]:     ram:4463(j)
        //         ram:446d 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:446e e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:446f 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:4470 3a b5 c8        LD         A,(BYTE_ram_c8b5)
        self.instr_hk__LD_A_iNNNN(0xc8b5);
        //         ram:4473 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:4474 28 04           JR         Z,LAB_ram_447a
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_447a);
        } else {
            self.increase_cycles(7);
            //         ram:4476 3d              DEC        A
            self.instr_hk__DEC_A();
            //         ram:4477 32 b5 c8        LD         (BYTE_ram_c8b5),A
            self.instr_hk__LD_iNNNN_A(0xc8b5);
        }

        //                              LAB_ram_447a                                    XREF[1]:     ram:4474(j)
        //         ram:447a 2a ba c8        LD         HL,(wd_addr_c8ba)
        self.instr_hk__LD_HL_iNNNN(0xc8ba);
        //         ram:447d 11 05 00        LD         DE,0x5_c8ba
        self.instr_hk__LD_DE_NNNN(0x5);
        //         ram:4480 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:4481 4e              LD         C,(HL)
        self.instr_hk__LD_C_iHL();
        //         ram:4482 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:4485 13              INC        DE
        self.instr_hk__INC_DE();
        //         ram:4486 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:4487 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:4488 3d              DEC        A
        self.instr_hk__DEC_A();
        //         ram:4489 0c              INC        C
        self.instr_hk__INC_C();
        //         ram:448a 91              SUB        C
        self.instr_hk__SUB_A_C();
        //         ram:448b d2 a9 44        JP         NC,l_exit_x
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) == 0 {
            // JP(l_exit_x);
            self.SetPC(0x44a9);
            //                              l_exit_x                                        XREF[2]:     ram:4468(j), ram:448b(j)
            //         ram:44a9 cd 60 47        CALL       sb_read_mem_for_player_4760                      hl <- addr
            assert!(self.call_hook(0x4760));
            //                                                                                              bc <- player_idx
            //         ram:44ac 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:44ad e6 03           AND        0x3
            self.instr_hk__AND_NN(0x3);
            //         ram:44af 08              EX         AF,AF_
            self.instr_hk__EX_AF_AF_();
            //         ram:44b0 cd ab 47        CALL       sb_read_mem_47ab                                 hl <- c8c2 + *c8b7
            assert!(self.call_hook(0x47ab));
            //         ram:44b3 08              EX         AF,AF_
            self.instr_hk__EX_AF_AF_();
            //         ram:44b4 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:44b5 c9              RET
            self.assert_pc(0x44b5);
            return true;
            //
        }

        //         ram:448e 41              LD         B,C
        self.instr_hk__LD_B_C();
        //         ram:448f cb 38           SRL        B
        self.instr_hk__SRL_B();
        //         ram:4491 04              INC        B
        self.instr_hk__INC_B();
        //         ram:4492 3e 04           LD         A,0x4
        self.instr_hk__LD_A_NN(0x4);
        //         ram:4494 cd 34 b6        CALL       sb_rand_calc_B634                                IN a:
        assert!(self.call_hook(0xb634));
        //                                                                                                 b: cnt
        //                                                                                              OUT hl
        //         ram:4497 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:4498 cd 09 48        CALL       sb_get_char_exp_4809                             OUT hl: addr
        assert!(self.call_hook(0x4809));
        //         ram:449b 4e              LD         C,(HL=>DAT_ram_0006)                             = ??
        self.instr_hk__LD_C_iHL();
        //         ram:449c 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:449d 46              LD         B,(HL=>DAT_ram_0007)                             = ??
        self.instr_hk__LD_B_iHL();
        //         ram:449e eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:449f 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:44a0 30 03           JR         NC,LAB_ram_44a5
        self.IncPC(2);
        if (self.data.F & FLAG_C) == 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_44a5);
        } else {
            self.increase_cycles(7);
            //         ram:44a2 21 ff ff        LD         HL,0xffff
            self.instr_hk__LD_HL_NNNN(0xffff);
        }

        self.SetPC(0x44a5);
        //                              LAB_ram_44a5                                    XREF[1]:     ram:44a0(j)
        //         ram:44a5 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:44a6 72              LD         (HL=>DAT_ram_0007),D                             = ??
        self.instr_hk__LD_iHL_D();
        //         ram:44a7 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:44a8 73              LD         (HL=>DAT_ram_0006),E                             = ??
        self.instr_hk__LD_iHL_E();
        // JP(l_exit_x);
        //                              l_exit_x                                        XREF[2]:     ram:4468(j), ram:448b(j)
        //         ram:44a9 cd 60 47        CALL       sb_read_mem_for_player_4760                      hl <- addr
        assert!(self.call_hook(0x4760));
        //                                                                                              bc <- player_idx
        //         ram:44ac 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:44ad e6 03           AND        0x3
        self.instr_hk__AND_NN(0x3);
        //         ram:44af 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:44b0 cd ab 47        CALL       sb_read_mem_47ab                                 hl <- c8c2 + *c8b7
        assert!(self.call_hook(0x47ab));
        //         ram:44b3 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:44b4 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:44b5 c9              RET
        self.assert_pc(0x44b5);
        // return true;
        //
        true
    }
    pub(crate) fn hook_44b6(&mut self) -> bool {
        println!("hook_44b6");
        //         ram:44b6 2a ba c8        LD         HL,(wd_addr_c8ba)                                OUT a
        self.instr_hk__LD_HL_iNNNN(0xc8ba);
        //         ram:44b9 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:44ba 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:44bb 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:44bc 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:44bd 4e              LD         C,(HL)
        self.instr_hk__LD_C_iHL();
        //         ram:44be 3e 12           LD         A,0x12
        self.instr_hk__LD_A_NN(0x12);
        //         ram:44c0 91              SUB        C
        self.instr_hk__SUB_A_C();
        //         ram:44c1 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:44c2 3e 1c           LD         A,0x1c
        self.instr_hk__LD_A_NN(0x1c);
        //         ram:44c4 91              SUB        C
        self.instr_hk__SUB_A_C();
        //         ram:44c5 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:44c6 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:44c7 cd 78 47        CALL       sb_read_mem_for_player_4778                      hl <- c1a8 + 6 * player_idx
        assert!(self.call_hook(0x4778));
        //         ram:44ca 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:44cb 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:44cc c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:44cd b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:44ce 28 04           JR         Z,LAB_ram_44d4
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_44d4);
        } else {
            self.increase_cycles(7);
            //         ram:44d0 3e fc           LD         A,0xfc
            self.instr_hk__LD_A_NN(0xfc);
            //         ram:44d2 81              ADD        A,C
            self.instr_hk__ADD_A_C();
            //         ram:44d3 4f              LD         C,A
            self.instr_hk__LD_C_A();
        }

        //                              LAB_ram_44d4                                    XREF[1]:     ram:44ce(j)
        //         ram:44d4 d9              EXX
        self.instr_hk__EXX();
        //         ram:44d5 cd f1 47        CALL       sb_get_char_level_47F1                           OUT hl: addr
        assert!(self.call_hook(0x47f1));
        //         ram:44d8 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:44d9 d9              EXX
        self.instr_hk__EXX();
        //         ram:44da 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:44db 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:44dc 79              LD         A,C
        self.instr_hk__LD_A_C();
        //         ram:44dd 90              SUB        B
        self.instr_hk__SUB_A_B();
        //         ram:44de 30 01           JR         NC,l_exit_x
        self.IncPC(2);
        if (self.data.F & FLAG_C) == 0 {
            self.increase_cycles(12);
            // JR(l_exit_x);
        } else {
            self.increase_cycles(7);
            //         ram:44e0 af              XOR        A
            self.instr_hk__XOR_A_A();
        }

        self.SetPC(0x44e1);
        //                              l_exit_x                                        XREF[3]:     ram:44de(j), ram:4514(j),
        //                                                                                           ram:4517(j)
        //         ram:44e1 f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:44e2 06 01           LD         B,0x1
        self.instr_hk__LD_B_NN(0x1);
        //         ram:44e4 3e 14           LD         A,0x14
        self.instr_hk__LD_A_NN(0x14);
        //         ram:44e6 cd 34 b6        CALL       sb_rand_calc_B634                                IN a:
        assert!(self.call_hook(0xb634));
        //                                                                                                 b: cnt
        //                                                                                              OUT hl
        //         ram:44e9 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:44ea 2c              INC        L
        self.instr_hk__INC_L();
        //         ram:44eb 95              SUB        L
        self.instr_hk__SUB_A_L();
        //         ram:44ec c9              RET
        self.assert_pc(0x44ec);
        // return true;
        //
        true
    }
    pub(crate) fn hook_46ac(&mut self) -> bool {
        //         ram:46ac c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:46ad 22  be  c8       LD         (BYTE_ram_c8be ),HL
        self.instr_hk__LD_iNNNN_HL(0xc8be);
        //         ram:46b0 af              XOR        A
        self.instr_hk__XOR_A_A();
        //                              LAB_ram_46b1
        loop {
            self.SetPC(0x46b1);
            //         ram:46b1 32  1b  c2       LD         (bt_player_idx_c21b ),A
            self.instr_hk__LD_iNNNN_A(0xc21b);
            //         ram:46b4 cd  05  47       CALL       sb_read_mem_for_player_4705
            assert!(self.call_hook(0x4705));
            //         ram:46b7 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:46b8 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:46b9 28  0b           JR         Z,LAB_ram_46c6
            if (self.data.F & FLAG_Z) != 0 {
                // JR LAB_ram_46c6
            } else {
                // JR 46bb
                //         ram:46bb cd  ea  46       CALL       sb_read_mem_for_player_46ea                      OUT d,e
                assert!(self.call_hook(0x46ea));
                //         ram:46be 2a  be  c8       LD         HL,(BYTE_ram_c8be )
                self.instr_hk__LD_HL_iNNNN(0xc8be);
                //         ram:46c1 b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:46c2 ed  52           SBC        HL,DE
                self.instr_hk__SBC_HL_DE();
                //         ram:46c4 28  0e           JR         Z,LAB_ram_46d4
                if (self.data.F & FLAG_Z) != 0 {
                    // JR LAB_ram_46d4
                    //                              LAB_ram_46d4
                    //         ram:46d4 2a  be  c8       LD         HL,(BYTE_ram_c8be )
                    self.instr_hk__LD_HL_iNNNN(0xc8be);
                    //         ram:46d7 c1              POP        BC
                    self.instr_hk__POP_BC();
                    //         ram:46d8 37              SCF
                    self.instr_hk__SCF();
                    //         ram:46d9 c9              RET
                    // self.instr_hk__RET();

                    return true;
                } else {
                    // JR 46c6
                }
            }
            //                              LAB_ram_46c6
            //         ram:46c6 3a  1b  c2       LD         A,(bt_player_idx_c21b )
            self.instr_hk__LD_A_iNNNN(0xc21b);
            //         ram:46c9 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:46ca fe  03           CP         0x3
            self.instr_hk__CP_NN(0x3);
            //         ram:46cc 20  e3           JR         NZ,LAB_ram_46b1
            if (self.data.F & FLAG_Z) == 0 {
                // JR LAB_ram_46b1
            } else {
                // JR 46ce
                break;
            }
        }
        self.SetPC(0x46ce);
        //         ram:46ce 2a  be  c8       LD         HL,(BYTE_ram_c8be )
        self.instr_hk__LD_HL_iNNNN(0xc8be);
        //         ram:46d1 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:46d2 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:46d3 c9              RET
        // self.instr_hk__RET();
        self.assert_pc(0x46d3);
        true
    }
    pub(crate) fn hook_46da(&mut self) -> bool {
        //         ram:46da cd 15 48        CALL       sb_get_char_items_4815                           OUT CF: set
        assert!(self.call_hook(0x4815));
        //                                                                                                  hl: addr for empty pos
        //                                                                                                  CF: unset if item bag is full
        //         ram:46dd 06 06           LD         B,0x6
        self.instr_hk__LD_B_NN(0x6);
        //                              loop_or_exit_clear_cf                           XREF[1]:     ram:46e4(j)
        loop {
            self.SetPC(0x46df);
            //         ram:46df be              CP         (HL)
            self.instr_hk__CP_iHL();
            //         ram:46e0 ca e8 46        JP         Z,l_exit_scf
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(l_exit_scf);
                self.SetPC(0x46e8);
                //                              l_exit_scf                                      XREF[1]:     ram:46e0(j)
                //         ram:46e8 37              SCF
                self.instr_hk__SCF();
                //         ram:46e9 c9              RET
                self.assert_pc(0x46e9);
                return true;
                //
            }

            //         ram:46e3 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:46e4 10 f9           DJNZ       loop_or_exit_clear_cf
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_or_exit_clear_cf;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:46e6 af              XOR        A
        self.instr_hk__XOR_A_A();
        self.assert_pc(0x46e7);
        //         ram:46e7 c9              RET
        true
    }
    pub(crate) fn hook_46ea(&mut self) -> bool {
        //         ram:46ea 21  ee  c1       LD         HL,BYTE_ram_c1ee                                 OUT d,e
        self.instr_hk__LD_HL_NNNN(0xc1ee);
        self.internal_46ed();
        true
    }
    fn internal_46ed(&mut self) {
        self.SetPC(0x46ed);
        //                              LAB_ram_46ed                                         sb_read_mem_for_player_46fa:46fd
        //         ram:46ed 3a  1b  c2       LD         A,(bt_player_idx_c21b )
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:46f0 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:46f1 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:46f2 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:46f3 06  00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:46f5 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:46f6 56              LD         D,(HL=>BYTE_ram_c1ee )
        self.instr_hk__LD_D_iHL();
        //         ram:46f7 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:46f8 5e              LD         E,(HL=>BYTE_ram_c1ef )
        self.instr_hk__LD_E_iHL();
        //         ram:46f9 c9              RET
        self.assert_pc(0x46f9);
    }
    pub(crate) fn hook_46fa(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1ec);
        self.internal_46ed();
        true
    }
    pub(crate) fn hook_4700(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1fb);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4705(&mut self) -> bool {
        // ram:4705 3a  1b  c2       LD         A,(bt_player_idx_c21b )
        self.instr_hk__LD_A_iNNNN(0xc21b);
        // ram:4708 c6  03           ADD        A,0x3
        self.instr_hk__ADD_A_NN(0x3);
        // ram:470a 4f              LD         C,A
        self.instr_hk__LD_C_A();
        // ram:470b 06  00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        // ram:470d 21  49  c3       LD         HL,BYTE_ram_c349
        self.instr_hk__LD_HL_NNNN(0xc349);
        // ram:4710 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        // ram:4711 c9              RET
        // self.instr_hk__RET();
        self.assert_pc(0x4711);
        true
    }
    pub(crate) fn hook_4712(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1e3);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4717(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1e0);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_471c(&mut self) -> bool {
        // log::info!("hook_471c");
        //         ram:471c 21  bd  c1       LD         HL,by_player_controller_c1bd
        self.instr_hk__LD_HL_NNNN(0xc1bd);
        //         ram:471f cd  63  47       CALL       fn_add_player_idx_to_addr_4763
        assert!(self.call_hook(0x4763));
        //         ram:4722 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:4723 c9              RET
        self.assert_pc(0x4723);
        true
    }
    pub(crate) fn hook_4724(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1a2);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4729(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1cd);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_472e(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1c4);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4733(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1c7);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4738(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1ca);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_473d(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc196);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4742(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc199);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4747(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc19c);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_474c(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1ba);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4751(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1c0);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4756(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc19f);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_475b(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1a5);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4760(&mut self) -> bool {
        self.instr_hk__LD_HL_NNNN(0xc1f8);
        assert!(self.call_hook(0x4763));
        true
    }
    pub(crate) fn hook_4763(&mut self) -> bool {
        // log::info!("hook_4763");
        //         ram:4763 3a  1b  c2       LD         A,(bt_player_idx_c21b )
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:4766 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:4767 06  00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:4769 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:476a c9              RET
        self.assert_pc(0x476a);
        true
    }
    pub(crate) fn hook_476b(&mut self) -> bool {
        //         ram:476b 3a 1b c2        LD         A,(bt_player_idx_c21b)                           ;
        self.instr_hk__LD_A_iNNNN(0xc21b);

        //         ram:476e 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:476f 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:4770 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:4771 06 00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:4773 21 d0 c1        LD         HL,bt_char_spell_guess_c1d0
        self.instr_hk__LD_HL_NNNN(0xc1d0);
        //         ram:4776 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:4777 c9              RET
        self.assert_pc(0x4777);

        true
    }
    pub(crate) fn hook_4778(&mut self) -> bool {
        //         ram:4778 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:477b 87              ADD        A,A                                              a <- 2 * player_idx
        self.instr_hk__ADD_A_A();
        //         ram:477c 47              LD         B,A                                              b <- 2 * player_idx
        self.instr_hk__LD_B_A();
        //         ram:477d 87              ADD        A,A                                              a <- 4 * player_idx
        self.instr_hk__ADD_A_A();
        //         ram:477e 80              ADD        A,B                                              a <- 6 * player_idx
        self.instr_hk__ADD_A_B();
        //         ram:477f 4f              LD         C,A                                              bc <- 6 * player_idx
        self.instr_hk__LD_C_A();
        //         ram:4780 06 00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:4782 21 a8 c1        LD         HL,BYTE_ram_c1a8
        self.instr_hk__LD_HL_NNNN(0xc1a8);
        //         ram:4785 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:4786 c9              RET

        self.assert_pc(0x4786);
        true
    }
    pub(crate) fn hook_4787(&mut self) -> bool {
        //         ram:4787 cd 29 47        CALL       sb_read_mem_for_player_4729                      OUT hl: addr
        assert!(self.call_hook(0x4729));
        //         ram:478a 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:478b d6 0a           SUB        0xa
        self.instr_hk__SUB_NN(0xa);
        //         ram:478d 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:4790 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:4791 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:4792 01 11 00        LD         BC,0x11
        self.instr_hk__LD_BC_NNNN(0x11);
        //         ram:4795 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:4796 c9              RET
        self.assert_pc(0x4796);
        true
    }
    pub(crate) fn hook_4797(&mut self) -> bool {
        //         ram:4797 21 d2 c8        LD         HL,BYTE_ram_c8d2                                 hl <- c8ca + *c8b7 if *c23e == 0
        self.instr_hk__LD_HL_NNNN(0xc8d2);
        //                                                                                                    c8d2 + *c8b7 otherwise
        //         ram:479a 3a 3e c2        LD         A,(DAT_ram_c23e)
        self.instr_hk__LD_A_iNNNN(0xc23e);
        //         ram:479d b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:479e 20 03           JR         NZ,l_exit_x
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12);
            // JR(l_exit_x);
        } else {
            self.increase_cycles(7);
            //         ram:47a0 21 ca c8        LD         HL,BYTE_ram_c8ca
            self.instr_hk__LD_HL_NNNN(0xc8ca);
        }

        self.internal_47a3();
        true
    }
    fn internal_47a3(&mut self) {
        self.SetPC(0x47a3);
        //                              l_exit_x                                        XREF[2]:     ram:479e(j),
        //                                                                                           sb_read_mem_47ab:47ae(j)
        //         ram:47a3 3a b7 c8        LD         A,(BYTE_ram_c8b7)
        self.instr_hk__LD_A_iNNNN(0xc8b7);
        //         ram:47a6 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:47a7 06 00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:47a9 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:47aa c9              RET
        self.assert_pc(0x47aa);
    }
    pub(crate) fn hook_47ab(&mut self) -> bool {
        // ram:47ab 21 c2 c8        LD         HL,BYTE_ram_c8c2                                 hl <- c8c2 + *c8b7
        self.instr_hk__LD_HL_NNNN(0xc8c2);
        // ram:47ae 18 f3           JR         sb_read_mem_4797::l_exit_x
        self.internal_47a3();
        true
    }
    pub(crate) fn hook_47b0(&mut self) -> bool {
        // println!("hook_47b0");
        //         ram:47b0 21 da c8        LD         HL,BYTE_ram_c8da
        self.instr_hk__LD_HL_NNNN(0xc8da);

        //         ram:47b3 3a 3e c2        LD         A,(DAT_ram_c23e)
        self.instr_hk__LD_A_iNNNN(0xc23e);
        //         ram:47b6 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:47b7 28 03           JR         Z,l_exit_x
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(l_exit_x);
        } else {
            self.increase_cycles(7);
            //         ram:47b9 21 fa c8        LD         HL,DAT_ram_c8fa
            self.instr_hk__LD_HL_NNNN(0xc8fa);
        }
        self.SetPC(0x47bc);
        //                              l_exit_x
        //         ram:47bc 3a b7 c8        LD         A,(BYTE_ram_c8b7)
        self.instr_hk__LD_A_iNNNN(0xc8b7);
        //         ram:47bf 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:47c0 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:47c1 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:47c2 06 00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:47c4 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:47c5 c9              RET
        self.assert_pc(0x47c5);

        true
    }
    pub(crate) fn hook_47c6(&mut self) -> bool {
        // println!("hook_47c6");

        //         ram:47c6 cd b0 47        CALL       sb_read_mem_47b0
        assert!(self.call_hook(0x47b0));

        //         ram:47c9 22 b8 c8        LD         (BYTE_ram_c8b8),HL
        self.instr_hk__LD_iNNNN_HL(0xc8b8);
        //         ram:47cc 6e              LD         L,(HL)
        self.instr_hk__LD_L_iHL();
        //         ram:47cd 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:47cf 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:47d0 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:47d1 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:47d2 01 67 a7        LD         BC,DAT_ram_a767                                  = FAh
        self.instr_hk__LD_BC_NNNN(0xa767);
        //         ram:47d5 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:47d6 22 ba c8        LD         (wd_addr_c8ba),HL
        self.instr_hk__LD_iNNNN_HL(0xc8ba);
        //         ram:47d9 c9              RET
        self.assert_pc(0x47d9);

        true
    }
    pub(crate) fn hook_47da(&mut self) -> bool {
        //         ram:47da 2a ba c8        LD         HL,(wd_addr_c8ba)                                OUT bc
        self.instr_hk__LD_HL_iNNNN(0xc8ba);
        //                                                                                                  ak
        //         ram:47dd 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:47de 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:47df 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:47e0 01 02 02        LD         BC,0x202
        self.instr_hk__LD_BC_NNNN(0x202);
        //         ram:47e3 fe 04           CP         0x4
        self.instr_hk__CP_NN(0x4);
        //         ram:47e5 c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:47e6 0c              INC        C
        self.instr_hk__INC_C();
        //         ram:47e7 04              INC        B
        self.instr_hk__INC_B();
        //         ram:47e8 fe 09           CP         0x9
        self.instr_hk__CP_NN(0x9);
        //         ram:47ea c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:47eb 0c              INC        C
        self.instr_hk__INC_C();
        //         ram:47ec fe 0c           CP         0xC
        self.instr_hk__CP_NN(0xC);
        //         ram:47ee c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:47ef 04              INC        B
        self.instr_hk__INC_B();
        //         ram:47f0 c9              RET
        self.assert_pc(0x47f0);
        //
        true
    }
    pub(crate) fn hook_47f1(&mut self) -> bool {
        // log::info!("hook_47f1");
        self.instr_hk__LD_C_NN(0x06);
        // ram:47f3 18  22           JR         sb_get_char_internal_4817
        self.internal_4817();
        true
    }
    pub(crate) fn hook_47f5(&mut self) -> bool {
        // log::info!("hook_47f5");
        self.instr_hk__LD_C_NN(0x07);
        self.internal_4817();
        true
    }
    pub(crate) fn hook_47f9(&mut self) -> bool {
        // log::info!("hook_47f9");
        self.instr_hk__LD_C_NN(0x08);
        self.internal_4817();
        true
    }
    pub(crate) fn hook_47fd(&mut self) -> bool {
        // log::info!("hook_47fd");
        self.instr_hk__LD_C_NN(0x09);
        self.internal_4817();
        true
    }
    pub(crate) fn hook_4801(&mut self) -> bool {
        // log::info!("hook_4801");
        self.instr_hk__LD_C_NN(0xa);
        // ram:4803 18  12           JR         sb_get_char_internal_4817
        self.internal_4817();
        true
    }
    pub(crate) fn hook_4805(&mut self) -> bool {
        // log::info!("hook_4805");
        self.instr_hk__LD_C_NN(0x0b);
        self.internal_4817();
        true
    }
    pub(crate) fn hook_4809(&mut self) -> bool {
        // log::info!("hook_4809");
        self.instr_hk__LD_C_NN(0x0d);
        self.internal_4817();
        true
    }
    pub(crate) fn hook_480d(&mut self) -> bool {
        // log::info!("hook_480d");
        self.instr_hk__LD_C_NN(0x0f);
        self.internal_4817();
        true
    }
    pub(crate) fn hook_4811(&mut self) -> bool {
        // log::info!("hook_4811");
        self.instr_hk__LD_C_NN(0x10);
        self.internal_4817();
        true
    }
    fn internal_4817(&mut self) {
        self.SetPC(0x4817);
        //         ram:4817 06  00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:4819 2a  54  c2       LD         HL,(pt_char_c254 )
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:481c 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:481d c9              RET
        self.assert_pc(0x481d);
    }
    pub(crate) fn hook_4815(&mut self) -> bool {
        // log::info!("hook_4811");
        self.instr_hk__LD_C_NN(0x11);
        self.internal_4817();
        true
    }
    pub(crate) fn hook_487f(&mut self) -> bool {
        //         ram:487f 5f              LD         E,A                                              IN: a
        self.instr_hk__LD_E_A();
        //                                                                                              OUT: hl <- a * a
        //         ram:4880 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:4881 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:4883 54              LD         D,H
        self.instr_hk__LD_D_H();
        //         ram:4884 cd a9 b7        CALL       sb_multiply_guess_B7A9                           hl <- hl * de ?
        assert!(self.call_hook(0xb7a9));
        //         ram:4887 c9              RET
        self.assert_pc(0x4887);
        true
    }
    pub(crate) fn hook_49a1(&mut self) -> bool {
        //         ram:49a1 2a b8 c8        LD         HL,(BYTE_ram_c8b8)                               OUT cf
        self.instr_hk__LD_HL_iNNNN(0xc8b8);
        //         ram:49a4 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:49a5 56              LD         D,(HL)
        self.instr_hk__LD_D_iHL();
        //         ram:49a6 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:49a7 5e              LD         E,(HL)
        self.instr_hk__LD_E_iHL();
        //         ram:49a8 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:49a9 f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:49aa b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:49ab 20 03           JR         NZ,LAB_ram_49b0
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_49b0);
            //                              LAB_ram_49b0                                    XREF[1]:     ram:49ab(j)
            //         ram:49b0 fe 01           CP         0x1
            self.instr_hk__CP_NN(0x1);
            //         ram:49b2 20 03           JR         NZ,LAB_ram_49b7
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_49b7);
                //                              LAB_ram_49b7                                    XREF[1]:     ram:49b2(j)
                //         ram:49b7 fe 02           CP         0x2
                self.instr_hk__CP_NN(0x2);
                //         ram:49b9 20 03           JR         NZ,LAB_ram_49be
                self.IncPC(2);
                if (self.data.F & FLAG_Z) == 0 {
                    self.increase_cycles(12);
                    // JR(LAB_ram_49be);
                    //                              LAB_ram_49be                                    XREF[1]:     ram:49b9(j)
                    //         ram:49be 25              DEC        H
                    self.instr_hk__DEC_H();
                } else {
                    self.increase_cycles(7);
                    //         ram:49bb 2c              INC        L
                    self.instr_hk__INC_L();
                    //         ram:49bc 18 01           JR         LAB_ram_49bf
                    self.IncPC(2);
                    self.increase_cycles(12);
                    // JR(LAB_ram_49bf);
                }
            } else {
                self.increase_cycles(7);
                //         ram:49b4 24              INC        H
                self.instr_hk__INC_H();
                //         ram:49b5 18 08           JR         LAB_ram_49bf
                self.IncPC(2);
                self.increase_cycles(12);
            }
        } else {
            self.increase_cycles(7);
            //         ram:49ad 2d              DEC        L
            self.instr_hk__DEC_L();
            //         ram:49ae 18 0f           JR         LAB_ram_49bf
            self.IncPC(2);
            self.increase_cycles(12);
        }
        // JR(LAB_ram_49bf);

        //         ram:49bf 7c              LD         A,H
        self.instr_hk__LD_A_H();
        //         ram:49c0 fe 1d           CP         0x1d
        self.instr_hk__CP_NN(0x1d);
        //         ram:49c2 30 5a           JR         NC,l_exit
        self.IncPC(2);
        if (self.data.F & FLAG_C) == 0 {
            self.increase_cycles(12);
            // JR(l_exit);
            self.SetPC(0x4a1e);
            //         ram:4a1e f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:4a1f 37              SCF
            self.instr_hk__SCF();
            //         ram:4a20 c9              RET
            self.assert_pc(0x4a20);
            return true;
        }
        self.increase_cycles(7);
        //         ram:49c4 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:49c5 fe 11           CP         0x11
        self.instr_hk__CP_NN(0x11);
        //         ram:49c7 30 55           JR         NC,l_exit
        self.IncPC(2);
        if (self.data.F & FLAG_C) == 0 {
            self.increase_cycles(12);
            // JR(l_exit);
            self.SetPC(0x4a1e);
            //         ram:4a1e f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:4a1f 37              SCF
            self.instr_hk__SCF();
            //         ram:4a20 c9              RET
            self.assert_pc(0x4a20);
            return true;
        }
        self.increase_cycles(7);

        //         ram:49c9 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:49ca cd bc 89        CALL       fn_calc_voffset_89BC                             hl <- (hl & 0xff) * 20 + (hl >> 8)
        assert!(self.call_hook(0x89bc));
        //                                                                                              de <- (hl >> 8) << 8 + (hl & 0xff)
        //         ram:49cd 11 9a c9        LD         DE,DAT_ram_c99a
        self.instr_hk__LD_DE_NNNN(0xc99a);
        //         ram:49d0 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:49d1 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:49d2 cd da 47        CALL       sb_read_mem_calc_47DA                            OUT bc
        assert!(self.call_hook(0x47da));
        //                                                                                                  ak
        //         ram:49d5 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:49d6 cd 4c b3        CALL       sb_check_mem_B34C                                IN hl: addr
        assert!(self.call_hook(0xb34c));
        //                                                                                                 bc: cnt/cnt
        //                                                                                              OUT hl
        //         ram:49d9 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:49da b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:49db 20 41           JR         NZ,l_exit
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12);
            self.SetPC(0x4a1e);
            //         ram:4a1e f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:4a1f 37              SCF
            self.instr_hk__SCF();
            //         ram:4a20 c9              RET
            self.assert_pc(0x4a20);
            return true;
        }
        self.increase_cycles(7);

        //         ram:49dd af              XOR        A
        self.instr_hk__XOR_A_A();
        //                              loop_1                                          XREF[1]:     ram:4a10(j)
        loop {
            self.SetPC(0x49de);
            //         ram:49de d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:49df 32 1b c2        LD         (bt_player_idx_c21b),A
            self.instr_hk__LD_iNNNN_A(0xc21b);
            //         ram:49e2 cd 05 47        CALL       sb_read_mem_for_player_4705                      HL <- c349 + player_idx + 3
            assert!(self.call_hook(0x4705));
            //                                                                                              bc <- player_idx
            //         ram:49e5 7e              LD         A,(HL=>DAT_ram_c99a)
            self.instr_hk__LD_A_iHL();
            //         ram:49e6 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:49e7 ca 09 4a        JP         Z,loop_1_chk_cond_exit
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(loop_1_chk_cond_exit);
            } else {
                //         ram:49ea d5              PUSH       DE
                self.instr_hk__PUSH_DE();
                //         ram:49eb cd ea 46        CALL       sb_read_mem_for_player_46ea                      OUT d,e
                assert!(self.call_hook(0x46ea));
                //         ram:49ee d5              PUSH       DE
                self.instr_hk__PUSH_DE();
                //         ram:49ef cd da 47        CALL       sb_read_mem_calc_47DA                            OUT bc
                assert!(self.call_hook(0x47da));
                //                                                                                                  ak
                //         ram:49f2 05              DEC        B
                self.instr_hk__DEC_B();
                //         ram:49f3 0d              DEC        C
                self.instr_hk__DEC_C();
                //         ram:49f4 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:49f5 d1              POP        DE
                self.instr_hk__POP_DE();
                //                              loop_1_a                                        XREF[1]:     ram:4a07(j)
                loop {
                    self.SetPC(0x49f6);
                    //         ram:49f6 c5              PUSH       BC
                    self.instr_hk__PUSH_BC();
                    //         ram:49f7 d5              PUSH       DE
                    self.instr_hk__PUSH_DE();
                    //                              loop_1_a_1                                      XREF[1]:     ram:4a01(j)
                    loop {
                        self.SetPC(0x49f8);
                        //         ram:49f8 7c              LD         A,H
                        self.instr_hk__LD_A_H();
                        //         ram:49f9 ba              CP         D
                        self.instr_hk__CP_D();
                        //         ram:49fa 20 04           JR         NZ,loop_1_a_1_chk_cond
                        self.IncPC(2);
                        if (self.data.F & FLAG_Z) == 0 {
                            self.increase_cycles(12);
                            // JR(loop_1_a_1_chk_cond);
                        } else {
                            self.increase_cycles(7);
                            //         ram:49fc 7d              LD         A,L
                            self.instr_hk__LD_A_L();
                            //         ram:49fd bb              CP         E
                            self.instr_hk__CP_E();
                            //         ram:49fe 28 16           JR         Z,l_exit_x
                            self.IncPC(2);
                            if (self.data.F & FLAG_Z) != 0 {
                                self.increase_cycles(12);
                                // JR(l_exit_x);
                                self.SetPC(0x4a16);
                                //                              l_exit_x                                        XREF[1]:     ram:49fe(j)
                                //         ram:4a16 e1              POP        HL
                                self.instr_hk__POP_HL();
                                //         ram:4a17 e1              POP        HL
                                self.instr_hk__POP_HL();
                                //         ram:4a18 e1              POP        HL
                                self.instr_hk__POP_HL();
                                //         ram:4a19 f1              POP        AF
                                self.instr_hk__POP_AF();
                                //         ram:4a1a 3d              DEC        A
                                self.instr_hk__DEC_A();
                                //         ram:4a1b e6 03           AND        0x3
                                self.instr_hk__AND_NN(0x3);
                                //         ram:4a1d f5              PUSH       AF
                                self.instr_hk__PUSH_AF();
                                //                              l_exit                                          XREF[3]:     ram:49c2(j), ram:49c7(j),
                                //                                                                                           ram:49db(j)
                                //         ram:4a1e f1              POP        AF
                                self.instr_hk__POP_AF();
                                //         ram:4a1f 37              SCF
                                self.instr_hk__SCF();
                                //         ram:4a20 c9              RET
                                self.assert_pc(0x4a20);
                                return true;
                            } else {
                                self.increase_cycles(7);
                            }
                        }

                        //                              loop_1_a_1_chk_cond                             XREF[1]:     ram:49fa(j)
                        //         ram:4a00 14              INC        D
                        self.instr_hk__INC_D();
                        //         ram:4a01 10 f5           DJNZ       loop_1_a_1
                        self.IncPC(2);
                        self.decB();
                        if self.data.B != 0 {
                            self.increase_cycles(13);
                            //JP loop_1_a_1;
                        } else {
                            self.increase_cycles(8);
                            break;
                        }
                    }

                    //         ram:4a03 d1              POP        DE
                    self.instr_hk__POP_DE();
                    //         ram:4a04 1c              INC        E
                    self.instr_hk__INC_E();
                    //         ram:4a05 c1              POP        BC
                    self.instr_hk__POP_BC();
                    //         ram:4a06 0d              DEC        C
                    self.instr_hk__DEC_C();
                    //         ram:4a07 20 ed           JR         NZ,loop_1_a
                    self.IncPC(2);
                    if (self.data.F & FLAG_Z) == 0 {
                        self.increase_cycles(12);
                        // JR(loop_1_a);
                    } else {
                        self.increase_cycles(7);
                        break;
                    }
                }
            }

            self.SetPC(0x4a09);
            //                              loop_1_chk_cond_exit                            XREF[1]:     ram:49e7(j)
            //         ram:4a09 d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:4a0a 3a 1b c2        LD         A,(bt_player_idx_c21b)
            self.instr_hk__LD_A_iNNNN(0xc21b);
            //         ram:4a0d 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:4a0e fe 03           CP         0x3
            self.instr_hk__CP_NN(0x3);
            //         ram:4a10 c2 de 49        JP         NZ,loop_1
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(loop_1);
            } else {
                //         ram:4a13 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:4a14 b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:4a15 c9              RET
                self.assert_pc(0x4a15);
                return true;
            }
        }
    }
    pub(crate) fn hook_4b61(&mut self) -> bool {
        //         ram:4b61 21 08 28        LD         HL,0x2808
        self.instr_hk__LD_HL_NNNN(0x2808);
        //         ram:4b64 01 30 02        LD         BC,0x230
        self.instr_hk__LD_BC_NNNN(0x230);
        //         ram:4b67 cd ba c0        CALL       sb_fill_vram_guess_c0ba                          IN
        assert!(self.call_hook(0xc0ba));
        //         ram:4b6a c9              RET
        self.assert_pc(0x4b6a);
        true
    }

    pub(crate) fn hook_4c17(&mut self) -> bool {
        // println!("hook_4c17");
        //         ram:4c17 d5              PUSH       DE
        self.instr_hk__PUSH_DE();

        //         ram:4c18 05              DEC        B
        self.instr_hk__DEC_B();
        //         ram:4c19 05              DEC        B
        self.instr_hk__DEC_B();
        //         ram:4c1a 0d              DEC        C
        self.instr_hk__DEC_C();
        //         ram:4c1b c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:4c1c 3e  5b           LD         A,'['
        self.instr_hk__LD_A_NN(0x5b);
        //         ram:4c1e cd  d6  89       CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //                              LAB_ram_4c21
        loop {
            self.SetPC(0x4c21);
            //         ram:4c21 3e  26           LD         A,'&'
            self.instr_hk__LD_A_NN(0x26);
            //         ram:4c23 cd  d6  89       CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //         ram:4c26 10  f9           DJNZ       LAB_ram_4c21
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                // JP LAB_ram_4c21;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        self.SetPC(0x4c28);
        //         ram:4c28 3e  5c           LD         A,'\'
        self.instr_hk__LD_A_NN(0x5c);
        //         ram:4c2a cd  d6  89       CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //         ram:4c2d c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:4c2e d1              POP        DE
        self.instr_hk__POP_DE();
        //                              LAB_ram_4c2f
        loop {
            self.SetPC(0x4c2f);
            //         ram:4c2f c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:4c30 1c              INC        E
            self.instr_hk__INC_E();
            //         ram:4c31 d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:4c32 3e  25           LD         A,'%'
            self.instr_hk__LD_A_NN(0x25);
            //         ram:4c34 cd  d6  89       CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //                              LAB_ram_4c37
            loop {
                self.SetPC(0x4c37);
                //         ram:4c37 3e  20           LD         A,' '
                self.instr_hk__LD_A_NN(0x20);
                //         ram:4c39 cd  d6  89       CALL       fn_putchar_xy_89d6
                assert!(self.call_hook(0x89d6));

                //         ram:4c3c 10  f9           DJNZ       LAB_ram_4c37
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    // JP LAB_ram_4c37;
                } else {
                    self.increase_cycles(8);
                    break;
                }
            }
            self.SetPC(0x4c3e);
            //         ram:4c3e 3e  25           LD         A,'%'
            self.instr_hk__LD_A_NN(0x25);
            //         ram:4c40 cd  d6  89       CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //         ram:4c43 d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:4c44 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:4c45 0d              DEC        C
            self.instr_hk__DEC_C();
            //         ram:4c46 c2  2f  4c       JP         NZ,LAB_ram_4c2f
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP LAB_ram_4c2f;
            } else {
                break;
            }
        }
        self.SetPC(0x4c49);
        //         ram:4c49 3e  5d           LD         A,']'
        self.instr_hk__LD_A_NN(0x5d);
        //         ram:4c4b cd  d6  89       CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //                              LAB_ram_4c4e
        self.assert_pc(0x4c4e);
        loop {
            self.SetPC(0x4c4e);
            //         ram:4c4e 3e  26           LD         A,'&'
            self.instr_hk__LD_A_NN(0x26);
            //         ram:4c50 cd  d6  89       CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //         ram:4c53 10  f9           DJNZ       LAB_ram_4c4e
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                // JP LAB_ram_4c4e;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        self.SetPC(0x4c55);
        //         ram:4c55 3e  5e           LD         A,'^'
        self.instr_hk__LD_A_NN(0x5e);
        //         ram:4c57 cd  d6  89       CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //         ram:4c5a c9              RET

        self.assert_pc(0x4c5a);
        true
    }
    // fn hook_4c6e(&mut self) -> bool {
    //     loop {
    //         self.SetPC(0x4c6e);
    //         //         ram:4c6e c5              PUSH       BC
    //         self.instr_hk__PUSH_BC();
    //
    //         //         ram:4c6f f5              PUSH       AF
    //         self.instr_hk__PUSH_AF();
    //         //         ram:4c70 e5              PUSH       HL
    //         self.instr_hk__PUSH_HL();
    //         //         ram:4c71 cd  ed  8c       CALL       sb_read_fonts_to_temp_8CED                       : char
    //         assert!(self.call_hook(0x8CED));
    //         //         ram:4c74 21  40  fc       LD         HL,PATWRK_fc40                                   8       Returned character patte
    //         self.instr_hk__LD_HL_NNNN(0xfc40);
    //         //         ram:4c77 d1              POP        DE
    //         self.instr_hk__POP_DE();
    //         //         ram:4c78 d5              PUSH       DE
    //         self.instr_hk__PUSH_DE();
    //         //         ram:4c79 01  08  00       LD         BC,0x8
    //         self.instr_hk__LD_BC_NNNN(0x8);
    //         //         ram:4c7c f3              DI
    //         self.instr_hk__DI();
    //         //         ram:4c7d cd  85  c0       CALL       sb_blit_ram_to_vram_guess_C085
    //         assert!(self.call_hook(0xc085));
    //
    //
    //         //         ram:4c80 fb              EI
    //         self.instr_hk__EI();
    //         //         ram:4c81 e1              POP        HL
    //         self.instr_hk__POP_HL();
    //         //         ram:4c82 f1              POP        AF
    //         self.instr_hk__POP_AF();
    //         //         ram:4c83 3c              INC        A
    //         self.instr_hk__INC_A();
    //         //         ram:4c84 11  08  00       LD         DE,0x8
    //         self.instr_hk__LD_DE_NNNN(0x8);
    //         //         ram:4c87 19              ADD        HL,DE
    //         self.instr_hk__ADD_HL_DE();
    //         //         ram:4c88 c1              POP        BC
    //         self.instr_hk__POP_BC();
    //         //         ram:4c89 10  e3           DJNZ       sb_write_font_temp_guess_4C6E
    //         self.IncPC(2);
    //         self.decB();
    //         if self.data.B != 0 {
    //             self.increase_cycles(13);
    //             // JP sb_write_font_temp_guess_4C6E;
    //         } else {
    //             self.increase_cycles(8);
    //             break;
    //         }
    //     }
    //     //
    //     //         ram:4c8b c9              RET
    //     //
    //     true
    // }
    pub(crate) fn hook_4c5b(&mut self) -> bool {
        println!("hook_4c5b");
        //         ram:4c5b cd 41 00        CALL       bios_DISSCR_0041                                 Function : Inhibits the screen d
        // assert!(self.call_hook(0x0041));
        //                                                                                              Registers: AF, BC
        self.SetPC(0x4c5e);
        //         ram:4c5e af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:4c5f 21 00 18        LD         HL,SCREEN2_PATTERN_GENERATOR_TABLE_SIZE
        self.instr_hk__LD_HL_NNNN(SCREEN2_PATTERN_GENERATOR_TABLE_SIZE);
        //         ram:4c62 01 00 03        LD         BC,0x300
        self.instr_hk__LD_BC_NNNN(0x300);
        //         ram:4c65 f3              DI
        self.instr_hk__DI();
        //         ram:4c66 cd ba c0        CALL       sb_fill_vram_guess_c0ba                          IN
        assert!(self.call_hook(0xc0ba));
        //                                                                                                hl: vram addr?
        //                                                                                                bc: count
        //                                                                                                a: value
        self.assert_pc(0x4c69);
        //         ram:4c69 fb              EI
        self.instr_hk__EI();
        //         ram:4c6a cd 44 00        CALL       bios_ENASCR_0044                                 Function : Displays the screen
        // assert!(self.call_hook(0x0044));
        //                                                                                              Registers: AF, BC
        //         ram:4c6d c9              RET
        // self.assert_pc(0x4c6d);
        //
        true
    }
    pub(crate) fn hook_4c6e(&mut self) -> bool {
        loop {
            self.SetPC(0x4c6e);
            //         ram:4c6e c5              PUSH       BC                                               IN a
            self.instr_hk__PUSH_BC();
            //                                                                                               b: cnt
            //         ram:4c6f f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:4c70 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:4c71 cd ed 8c        CALL       sb_read_fonts_to_temp_8CED                       IN a: char
            assert!(self.call_hook(0x8ced));
            //         ram:4c74 21 40 fc        LD         HL,PATWRK_fc40                                   8       Returned character patte
            self.instr_hk__LD_HL_NNNN(0xfc40);
            //         ram:4c77 d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:4c78 d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:4c79 01 08 00        LD         BC,0x8
            self.instr_hk__LD_BC_NNNN(0x8);
            //         ram:4c7c f3              DI
            self.instr_hk__DI();
            //         ram:4c7d cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085                   IN bc: count
            assert!(self.call_hook(0xc085));
            //                                                                                                  de: targe vram addr
            //                                                                                                  hl: source addr
            //         ram:4c80 fb              EI
            self.instr_hk__EI();
            //         ram:4c81 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:4c82 f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:4c83 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:4c84 11 08 00        LD         DE,0x8
            self.instr_hk__LD_DE_NNNN(0x8);
            //         ram:4c87 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:4c88 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:4c89 10 e3           DJNZ       sb_write_font_temp_guess_4C6E                    IN a
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP sb_write_font_temp_guess_4C6E;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //                                                                                               b: cnt
        //         ram:4c8b c9              RET
        self.SetPC(0x4c8b);
        // return true;
        //
        true
    }
    pub(crate) fn hook_4fce(&mut self) -> bool {
        //         ram:4fce 06 09           LD         B,0x9                                            from (14,13)  wh (15, 9)
        self.instr_hk__LD_B_NN(0x9);
        //         ram:4fd0 11 0d 0e        LD         DE,0xe0d
        self.instr_hk__LD_DE_NNNN(0xe0d);
        //                              loop
        loop {
            self.SetPC(0x4fd3);
            //         ram:4fd3 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:4fd4 d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:4fd5 06 0f           LD         B,0xf
            self.instr_hk__LD_B_NN(0xf);
            //         ram:4fd7 cd db 60        CALL       sb_print_spaces_60db
            assert!(self.call_hook(0x60db));

            //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
            //         ram:4fda d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:4fdb 1c              INC        E
            self.instr_hk__INC_E();
            //         ram:4fdc c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:4fdd 10 f4           DJNZ       fn_draw_empty_rect_4fce::loop
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP fn_draw_empty_rect_4fce::loop;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        //         ram:4fdf c9              RET

        self.assert_pc(0x4fdf);
        true
    }
    pub(crate) fn hook_513b(&mut self) -> bool {
        //
        //                              **************************************************************
        //                              *                          FUNCTION                          *
        //                              **************************************************************
        //                              undefined fn_psg_only_513b()
        //              undefined         A:1            <RETURN>
        //                              fn_psg_only_513b                                XREF[3]:     fn_psg_wait_xxx_515f:5160(c),
        //                                                                                           sb_psg_mem_7899:78b0(j),
        //                                                                                           ram:7916(c)
        //         ram:513b 1e 0d           LD         E,0xd
        self.instr_hk__LD_E_NN(0xd);
        //         ram:513d 3e 0a           LD         A,0xa
        self.instr_hk__LD_A_NN(0xa);
        //         ram:513f cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        //         ram:5142 3e 04           LD         A,0x4
        self.instr_hk__LD_A_NN(0x4);
        //         ram:5144 1e 55           LD         E,0x55
        self.instr_hk__LD_E_NN(0x55);
        //         ram:5146 cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        //         ram:5149 1e 00           LD         E,0x0
        self.instr_hk__LD_E_NN(0x0);
        //         ram:514b 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:514c cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        //         ram:514f 3e 07           LD         A,0x7
        self.instr_hk__LD_A_NN(0x7);
        //         ram:5151 cd 96 00        CALL       bios_RDPSG_0096                                  Function : Reads value from PSG
        assert!(self.call_hook(0x0096));
        //                                                                                              Input    : A  - PSG register read
        //                                                                                              Output   : A  - Value read
        //         ram:5154 e6 bb           AND        0xbb
        self.instr_hk__AND_NN(0xbb);
        //         ram:5156 f6 a0           OR         0xa0
        self.instr_hk__OR_NN(0xa0);
        //         ram:5158 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:5159 3e 07           LD         A,0x7
        self.instr_hk__LD_A_NN(0x7);
        //         ram:515b cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        //         ram:515e c9              RET
        self.assert_pc(0x515e);

        // return true;
        //
        true
    }
    pub(crate) fn hook_518c(&mut self) -> bool {
        loop {
            self.SetPC(0x518c);
            //         ram:518c 0b              DEC        BC
            self.instr_hk__DEC_BC();
            //         ram:518d e3              EX         (SP=>local_res0),HL
            self.instr_hk__EX_iSP_HL();
            //         ram:518e e3              EX         (SP=>local_res0),HL
            self.instr_hk__EX_iSP_HL();
            //         ram:518f 78              LD         A,B
            self.instr_hk__LD_A_B();
            //         ram:5190 b1              OR         C
            self.instr_hk__OR_A_C();
            //         ram:5191 20 f9           JR         NZ,fn_wait_xxx_518c
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR(fn_wait_xxx_518c);
            } else {
                self.increase_cycles(7);
                break;
            }
        }

        //         ram:5193 c9              RET
        self.assert_pc(0x5193);
        //
        true
    }
    pub(crate) fn hook_5194(&mut self) -> bool {
        //         ram:5194 11  11  02       LD         DE,0x211                                         maybe character command in the i
        self.instr_hk__LD_DE_NNNN(0x211);
        //                                                                                              prints HEALING, LEAVE, STAY, SPE
        //         ram:5197 01  06  09       LD         BC,0x906
        self.instr_hk__LD_BC_NNNN(0x906);
        //         ram:519a cd  17  4c       CALL       fn_draw_border_guess_4c17
        assert!(self.call_hook(0x4c17));

        //         ram:519d 21  ee  56       LD         HL,s_HEALING_ram_56ee                            = "HEALING"
        self.instr_hk__LD_HL_NNNN(0x56ee);
        //         ram:51a0 11  12  03       LD         DE,0x312
        self.instr_hk__LD_DE_NNNN(0x312);
        //         ram:51a3 cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:51a6 21  f6  56       LD         HL,s_LEAVE_ram_56f6                              = "LEAVE"
        self.instr_hk__LD_HL_NNNN(0x56f6);
        //         ram:51a9 11  13  03       LD         DE,0x313
        self.instr_hk__LD_DE_NNNN(0x313);
        //         ram:51ac cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:51af 21  fc  56       LD         HL,s_STAY_ram_56fc                               = "STAY"
        self.instr_hk__LD_HL_NNNN(0x56fc);
        //         ram:51b2 11  14  03       LD         DE,0x314
        self.instr_hk__LD_DE_NNNN(0x314);
        //         ram:51b5 cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:51b8 21  e8  56       LD         HL,s_SPELL_ram_56e8                              = "SPELL"
        self.instr_hk__LD_HL_NNNN(0x56e8);
        //         ram:51bb 11  15  03       LD         DE,0x315
        self.instr_hk__LD_DE_NNNN(0x315);
        //         ram:51be cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:51c1 c9              RET
        self.assert_pc(0x51c1);
        true
    }
    pub(crate) fn hook_53fb(&mut self) -> bool {
        //         ram:53fb cd  f1  47       CALL       sb_get_char_level_47F1
        assert!(self.call_hook(0x47f1));
        //         ram:53fe 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:53ff 11  06  09       LD         DE,0x906
        self.instr_hk__LD_DE_NNNN(0x906);
        //         ram:5402 c6  30           ADD        A,'0'
        self.instr_hk__ADD_A_NN(0x30);
        //         ram:5404 cd  d6  89       CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //         ram:5407 cd  0d  48       CALL       sb_is_dungeon_master_480D
        assert!(self.call_hook(0x480d));
        //         ram:540a 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:540b b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:540c ca  15  54       JP         Z,LAB_ram_5415
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP LAB_ram_5415;
            self.SetPC(0x5415);
            //                              LAB_ram_5415
            //         ram:5415 cd  f5  47       CALL       sb_get_char_class_47F5                           0; "FIGHTER"
            assert!(self.call_hook(0x47f5));
            //                                                                                              1; "CLERIC"
            //                                                                                              2; "THIEF"
            //                                                                                              3; "MAGICIAN"
            //         ram:5418 6e              LD         L,(HL)
            self.instr_hk__LD_L_iHL();
            //         ram:5419 26  00           LD         H,0x0
            self.instr_hk__LD_H_NN(0x0);
            //         ram:541b 11  12  00       LD         DE,0x12
            self.instr_hk__LD_DE_NNNN(0x12);
            //         ram:541e cd  a9  b7       CALL       sb_multiply_guess_B7A9
            assert!(self.call_hook(0xb7a9));
            //         ram:5421 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:5422 cd  f1  47       CALL       sb_get_char_level_47F1
            assert!(self.call_hook(0x47f1));
            //         ram:5425 7e              LD         A,(HL=>DAT_ram_0012 )
            self.instr_hk__LD_A_iHL();
            //         ram:5426 3d              DEC        A
            self.instr_hk__DEC_A();
            //         ram:5427 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:5428 4f              LD         C,A
            self.instr_hk__LD_C_A();
            //         ram:5429 06  00           LD         B,0x0
            self.instr_hk__LD_B_NN(0x0);
            //         ram:542b 21  15  57       LD         HL,rank_names_5715                               = 575Dh
            self.instr_hk__LD_HL_NNNN(0x5715);
            //         ram:542e 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:542f 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:5430 5e              LD         E,(HL)
            self.instr_hk__LD_E_iHL();
            //         ram:5431 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:5432 66              LD         H,(HL)
            self.instr_hk__LD_H_iHL();
            //         ram:5433 6b              LD         L,E
            self.instr_hk__LD_L_E();
        } else {
            self.SetPC(0x540f);
            // ram:540f
            //         ram:540f 21  06  57       LD         HL,s_DUNGEON_MASTER_ram_5706                     = "DUNGEON MASTER"
            self.instr_hk__LD_HL_NNNN(0x5706);
            //         ram:5412 c3  34  54       JP         LAB_ram_5434
            self.IncPC(3);
            self.increase_cycles(10); //JP LAB_ram_5434;
        }
        self.SetPC(0x5434);

        //                              LAB_ram_5434
        //         ram:5434 e5              PUSH       HL=>s_DUNGEON_MASTER_ram_5706                    = "DUNGEON MASTER"
        self.instr_hk__PUSH_HL();
        //         ram:5435 11  08  03       LD         DE,0x308
        self.instr_hk__LD_DE_NNNN(0x308);
        //         ram:5438 06  0e           LD         B,0xe
        self.instr_hk__LD_B_NN(0xe);
        //         ram:543a cd  db  60       CALL       sb_print_spaces_60db
        assert!(self.call_hook(0x60db));

        //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
        //         ram:543d e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:543e 11  08  03       LD         DE,0x308
        self.instr_hk__LD_DE_NNNN(0x308);
        //         ram:5441 cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:5444 c9              RET

        self.assert_pc(0x5444);
        true
    }
    pub(crate) fn hook_5445(&mut self) -> bool {
        //         ram:5445 11  08  15       LD         DE,0x1508
        self.instr_hk__LD_DE_NNNN(0x1508);
        //         ram:5448 06  07           LD         B,0x7
        self.instr_hk__LD_B_NN(0x7);
        //         ram:544a cd  db  60       CALL       sb_print_spaces_60db
        assert!(self.call_hook(0x60db));

        //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
        //         ram:544d cd  f9  47       CALL       sb_get_char_hp_47F9                              ()
        assert!(self.call_hook(0x47f9));
        //         ram:5450 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:5451 6e              LD         L,(HL)
        self.instr_hk__LD_L_iHL();
        //         ram:5452 26  00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:5454 11  b0  c7       LD         DE,bt_buffer_c7b0
        self.instr_hk__LD_DE_NNNN(0xc7b0);
        //         ram:5457 cd  bd  b7       CALL       sb_itoa_guess_B7BD
        assert!(self.call_hook(0xb7bd));

        //         ram:545a 21  b0  c7       LD         HL,bt_buffer_c7b0
        self.instr_hk__LD_HL_NNNN(0xc7b0);
        //         ram:545d 11  08  15       LD         DE,0x1508
        self.instr_hk__LD_DE_NNNN(0x1508);
        //         ram:5460 cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:5463 3e  2f           LD         A,0x2f
        self.instr_hk__LD_A_NN(0x2f);
        //         ram:5465 cd  d6  89       CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //         ram:5468 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:5469 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:546a d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:546b 6e              LD         L,(HL)
        self.instr_hk__LD_L_iHL();
        //         ram:546c 26  00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:546e 11  b0  c7       LD         DE,bt_buffer_c7b0
        self.instr_hk__LD_DE_NNNN(0xc7b0);
        //         ram:5471 cd  bd  b7       CALL       sb_itoa_guess_B7BD
        assert!(self.call_hook(0xb7bd));

        //         ram:5474 21  b0  c7       LD         HL,bt_buffer_c7b0
        self.instr_hk__LD_HL_NNNN(0xc7b0);
        //         ram:5477 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:5478 cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:547b c9              RET

        self.assert_pc(0x547b);
        true
    }
    pub(crate) fn hook_547c(&mut self) -> bool {
        //         ram:547c cd  45  54       CALL       sb_print_hp_max_5445                             ()
        assert!(self.call_hook(0x5445));
        //         ram:547f 11  0d  03       LD         DE,0x30d
        self.instr_hk__LD_DE_NNNN(0x30d);
        //         ram:5482 06  07           LD         B,0x7
        self.instr_hk__LD_B_NN(0x7);
        //         ram:5484 cd  db  60       CALL       sb_print_spaces_60db
        assert!(self.call_hook(0x60db));

        //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
        //         ram:5487 cd  91  54       CALL       sb_get_gold_str_5491
        assert!(self.call_hook(0x5491));
        //         ram:548a 11  0d  03       LD         DE,0x30d
        self.instr_hk__LD_DE_NNNN(0x30d);
        //         ram:548d cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:5490 c9              RET
        self.assert_pc(0x5490);
        true
    }
    pub(crate) fn hook_5491(&mut self) -> bool {
        //         ram:5491 cd 01 48        CALL       sb_get_char_gold_hi_4801
        assert!(self.call_hook(0x4801));
        //         ram:5494 5e              LD         E,(HL)
        self.instr_hk__LD_E_iHL();
        //         ram:5495 16 00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:5497 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:5498 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:5499 5e              LD         E,(HL)
        self.instr_hk__LD_E_iHL();
        //         ram:549a 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:549b 56              LD         D,(HL)
        self.instr_hk__LD_D_iHL();
        //         ram:549c d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:549d 21 b0 c7        LD         HL,bt_buffer_c7b0
        self.instr_hk__LD_HL_NNNN(0xc7b0);
        //         ram:54a0 cd 25 b8        CALL       sb_itoa_3bytes_B825
        assert!(self.call_hook(0xb825));
        //         ram:54a3 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:54a4 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:54a5 21 b0 c7        LD         HL,bt_buffer_c7b0
        self.instr_hk__LD_HL_NNNN(0xc7b0);
        //         ram:54a8 c9              RET
        self.assert_pc(0x54a8);
        true
    }
    pub(crate) fn hook_54a9(&mut self) -> bool {
        //         ram:54a9 3a 4f c2        LD         A,(DAT_ram_c24f)
        self.instr_hk__LD_A_iNNNN(0xc24f);
        //         ram:54ac ee 01           XOR        0x1
        self.instr_hk__XOR_NN(0x1);
        //         ram:54ae 32 4f c2        LD         (DAT_ram_c24f),A
        self.instr_hk__LD_iNNNN_A(0xc24f);
        //         ram:54b1 c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:54b2 3a 4e c2        LD         A,(DAT_ram_c24e)
        self.instr_hk__LD_A_iNNNN(0xc24e);
        //         ram:54b5 3d              DEC        A
        self.instr_hk__DEC_A();
        //         ram:54b6 ca ce 54        JP         Z,LAB_ram_54ce
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP LAB_ram_54ce;

            //                              LAB_ram_54ce
            //         ram:54ce 3e 08           LD         A,0x8
            self.instr_hk__LD_A_NN(0x8);
            //         ram:54d0 32 4e c2        LD         (DAT_ram_c24e),A
            self.instr_hk__LD_iNNNN_A(0xc24e);
            //         ram:54d3 cd 60 47        CALL       sb_read_mem_for_player_4760
            assert!(self.call_hook(0x4760));

            //         ram:54d6 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:54d7 e6 7f           AND        0x7f
            self.instr_hk__AND_NN(0x7f);
            //         ram:54d9 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:54da fe 04           CP         0x4
            self.instr_hk__CP_NN(0x4);
            //         ram:54dc c2 c2 54        JP         NZ,l_exit_show_char_class_sprite
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP l_exit_show_char_class_sprite;
            } else {
                //         ram:54df af              XOR        A
                self.instr_hk__XOR_A_A();
                //         ram:54e0 c3 c2 54        JP         l_exit_show_char_class_sprite
                self.IncPC(3);
                self.increase_cycles(10); //JP l_exit_show_char_class_sprite;
            }
        } else {
            //         ram:54b9
            //         ram:54b9 32 4e c2        LD         (DAT_ram_c24e),A
            self.instr_hk__LD_iNNNN_A(0xc24e);
            //         ram:54bc cd 60 47        CALL       sb_read_mem_for_player_4760
            assert!(self.call_hook(0x4760));

            //         ram:54bf 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:54c0 ee 80           XOR        0x80
            self.instr_hk__XOR_NN(0x80);
        }
        self.SetPC(0x54c2);
        //                              l_exit_show_char_class_sprite
        //         ram:54c2 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:54c3 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:54c4 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:54c5 cd f5 47        CALL       sb_get_char_class_47F5
        assert!(self.call_hook(0x47f5));
        //                                                                                              0; "FIGHTER"
        //                                                                                              1; "CLERIC"
        //                                                                                              2; "THIEF"
        //                                                                                              3; "MAGICIAN"
        //         ram:54c8 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:54c9 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:54ca cd c9 8a        CALL       sb_draw_char_sprite_guess_8AC9
        assert!(self.call_hook(0x8ac9));

        //         ram:54cd c9              RET
        self.assert_pc(0x54cd);
        true
    }
    pub(crate) fn hook_562d(&mut self) -> bool {
        //         ram:562d 3a 47 c2        LD         A,(bt_cmds_idx_c247)
        self.instr_hk__LD_A_iNNNN(0xc247);
        //         ram:5630 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:5631 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:5632 16 00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:5634 21 9a 56        LD         HL,commands_569a                                 = 56EEh
        self.instr_hk__LD_HL_NNNN(0x569a);
        self.assert_pc(0x5637);

        //         ram:5637 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:5638 5e              LD         E,(HL=>commands_569a)                            = 56EEh
        self.instr_hk__LD_E_iHL();
        //         ram:5639 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:563a 66              LD         H,(HL=>commands_569a+1)
        self.instr_hk__LD_H_iHL();
        //         ram:563b 6b              LD         L,E
        self.instr_hk__LD_L_E();
        //         ram:563c cb 3f           SRL        A
        self.instr_hk__SRL_A();
        //         ram:563e c6 12           ADD        A,0x12
        self.instr_hk__ADD_A_NN(0x12);
        //         ram:5640 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:5641 16 03           LD         D,0x3
        self.instr_hk__LD_D_NN(0x3);
        //         ram:5643 cd c7 89        CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:5646 c9              RET

        self.assert_pc(0x5646);
        true
    }
    pub(crate) fn hook_5647(&mut self) -> bool {
        // ram:5647 3a 47 c2        LD         A,(bt_cmds_idx_c247)                             @(3, cmd_idx + 18) ~ 7lines
        self.instr_hk__LD_A_iNNNN(0xc247);
        // ram:564a c6 12           ADD        A,0x12
        self.instr_hk__ADD_A_NN(0x12);
        // ram:564c 5f              LD         E,A
        self.instr_hk__LD_E_A();
        // ram:564d 16 03           LD         D,0x3
        self.instr_hk__LD_D_NN(0x3);
        // ram:564f 06 07           LD         B,0x7
        self.instr_hk__LD_B_NN(0x7);
        // ram:5651 cd db 60        CALL       sb_print_spaces_60db
        assert!(self.call_hook(0x60db));

        // ram:5654 c9              RET

        self.assert_pc(0x5654);
        true
    }
    pub(crate) fn hook_566f(&mut self) -> bool {
        //         ram:566f 3a  47  c2       LD         A,(bt_cmds_idx_c247 )
        self.instr_hk__LD_A_iNNNN(0xc247);
        //         ram:5672 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:5673 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:5674 16  00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:5676 21  a2  56       LD         HL,class_names_56a2                              = 56AAh
        self.instr_hk__LD_HL_NNNN(0x56a2);
        //         ram:5679 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:567a 5e              LD         E,(HL=>class_names_56a2 )                        = 56AAh
        self.instr_hk__LD_E_iHL();
        //         ram:567b 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:567c 66              LD         H,(HL=>class_names_56a2+1 )
        self.instr_hk__LD_H_iHL();
        //         ram:567d 6b              LD         L,E
        self.instr_hk__LD_L_E();
        //         ram:567e c6  0d           ADD        A,0xd
        self.instr_hk__ADD_A_NN(0xd);
        //         ram:5680 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:5681 16  0c           LD         D,0xc
        self.instr_hk__LD_D_NN(0xc);
        //         ram:5683 cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:5686 c9              RET
        self.assert_pc(0x5686);
        true
    }
    pub(crate) fn hook_5687(&mut self) -> bool {
        //         ram:5687 3a  47  c2       LD         A,(bt_cmds_idx_c247 )                            8 spaces from (12, 13 + cmd_idx)
        self.instr_hk__LD_A_iNNNN(0xc247);
        //         ram:568a 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:568b c6  0d           ADD        A,0xd
        self.instr_hk__ADD_A_NN(0xd);
        //         ram:568d 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:568e 16  0c           LD         D,0xc
        self.instr_hk__LD_D_NN(0xc);
        //         ram:5690 06  08           LD         B,0x8
        self.instr_hk__LD_B_NN(0x8);
        //         ram:5692 cd  db  60       CALL       sb_print_spaces_60db
        assert!(self.call_hook(0x60db));
        //         ram:5695 c9              RET
        self.assert_pc(0x5695);
        true
    }
    pub(crate) fn hook_600a(&mut self) -> bool {
        //         ram:600a 3a  1b  c2       LD         A,(bt_player_idx_c21b )
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:600d 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:600e 16  00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:6010 21  19  00       LD         HL,char_19h_size
        self.instr_hk__LD_HL_NNNN(0x0019);
        //         ram:6013 cd  a9  b7       CALL       sb_multiply_guess_B7A9
        assert!(self.call_hook(0xb7a9));
        //         ram:6016 11  00  c1       LD         DE,BYTE_ram_c100
        self.instr_hk__LD_DE_NNNN(0xc100);
        //         ram:6019 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:601a c9              RET

        self.assert_pc(0x601a);
        true
    }
    pub(crate) fn hook_606f(&mut self) -> bool {
        //         ram:606f 21  96  60       LD         HL,s_LOAD_CHARACTER_ram_6096                     prints "LOAD CHARACTER"
        self.instr_hk__LD_HL_NNNN(0x6096);
        //         ram:6072 11  0a  09       LD         DE,0x90a
        self.instr_hk__LD_DE_NNNN(0x90a);
        //         ram:6075 cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:6078 c9              RET
        self.assert_pc(0x6078);
        true
    }
    pub(crate) fn hook_6079(&mut self) -> bool {
        //         ram:6079 21  83  60       LD         HL,s_MAKE_NEW_CHARACTER_ram_6083                 prints "MAKE NEW CHARACTER"
        self.instr_hk__LD_HL_NNNN(0x6083);
        //         ram:607c 11  0c  07       LD         DE,0x70c
        self.instr_hk__LD_DE_NNNN(0x70c);
        //         ram:607f cd  c7  89       CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:6082 c9              RET
        self.assert_pc(0x6082);
        true
    }
    pub(crate) fn hook_60e5(&mut self) -> bool {
        //
        //                              **************************************************************
        //                              *                         SUBROUTINE                         *
        //                              **************************************************************
        //                              SUB_ram_60e5                                    XREF[2]:     ram:61fe(c), ram:6240(c)
        //         ram:60e5 3a 3e c2        LD         A,(DAT_ram_c23e)                                 OUT cf
        self.instr_hk__LD_A_iNNNN(0xc23e);
        //         ram:60e8 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:60e9 ca ff 60        JP         Z,LAB_ram_60ff
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP(LAB_ram_60ff);
            //                              LAB_ram_60ff                                    XREF[1]:     ram:60e9(j)
            //         ram:60ff cd fa 46        CALL       sb_read_mem_for_player_46fa                      OUT d,e
            assert!(self.call_hook(0x46fa));
            //         ram:6102 7a              LD         A,D
            self.instr_hk__LD_A_D();
            //         ram:6103 e6 03           AND        0x3
            self.instr_hk__AND_NN(0x3);
            //         ram:6105 fe 03           CP         0x3
            self.instr_hk__CP_NN(0x3);
            //         ram:6107 ca b1 61        JP         Z,l_exit_set_cf
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(l_exit_set_cf);
                //                              l_exit_set_cf                                   XREF[6]:     ram:60f2(j), ram:60f9(j),
                //                                                                                           ram:6107(j), ram:610f(j),
                //                                                                                           ram:614d(j), ram:617c(j)
                //         ram:61b1 37              SCF
                self.instr_hk__SCF();
                //         ram:61b2 c9              RET
                return true;
            }

            //         ram:610a 7b              LD         A,E
            self.instr_hk__LD_A_E();
            //         ram:610b e6 03           AND        0x3
            self.instr_hk__AND_NN(0x3);
            //         ram:610d fe 03           CP         0x3
            self.instr_hk__CP_NN(0x3);
            //         ram:610f ca b1 61        JP         Z,l_exit_set_cf
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(l_exit_set_cf);
                //                              l_exit_set_cf                                   XREF[6]:     ram:60f2(j), ram:60f9(j),
                //                                                                                           ram:6107(j), ram:610f(j),
                //                                                                                           ram:614d(j), ram:617c(j)
                //         ram:61b1 37              SCF
                self.instr_hk__SCF();
                //         ram:61b2 c9              RET
                return true;
            }

            //         ram:6112 d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:6113 06 03           LD         B,0x3
            self.instr_hk__LD_B_NN(0x3);
            //         ram:6115 21 c0 c1        LD         HL,BYTE_ram_c1c0
            self.instr_hk__LD_HL_NNNN(0xc1c0);
            //                              loop_1                                          XREF[1]:     ram:6130(j)
            loop {
                self.SetPC(0x6118);
                //         ram:6118 7e              LD         A,(HL=>BYTE_ram_c1c0)
                self.instr_hk__LD_A_iHL();
                //         ram:6119 fe 0a           CP         0xa
                self.instr_hk__CP_NN(0xa);
                //         ram:611b 28 12           JR         Z,loop_1_chk_cond
                self.IncPC(2);
                if (self.data.F & FLAG_Z) != 0 {
                    self.increase_cycles(12);
                    // JR(loop_1_chk_cond);
                } else {
                    self.increase_cycles(7);
                    //         ram:611d fe 0b           CP         0xb
                    self.instr_hk__CP_NN(0xb);
                    //         ram:611f 28 0e           JR         Z,loop_1_chk_cond
                    self.IncPC(2);
                    if (self.data.F & FLAG_Z) != 0 {
                        self.increase_cycles(12);
                        // JR(loop_1_chk_cond);
                    } else {
                        self.increase_cycles(7);
                        //         ram:6121 fe 0c           CP         0xc
                        self.instr_hk__CP_NN(0xc);
                        //         ram:6123 28 0a           JR         Z,loop_1_chk_cond
                        self.IncPC(2);
                        if (self.data.F & FLAG_Z) != 0 {
                            self.increase_cycles(12);
                            // JR(loop_1_chk_cond);
                        } else {
                            self.increase_cycles(7);
                            //         ram:6125 fe 06           CP         0x6
                            self.instr_hk__CP_NN(0x6);
                            //         ram:6127 da 2f 61        JP         C,loop_1_chk_cond
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_C) != 0 {
                                // JP(loop_1_chk_cond);
                            } else {
                                //         ram:612a fe 10           CP         0x10
                                self.instr_hk__CP_NN(0x10);
                                //         ram:612c da 35 61        JP         C,LAB_ram_6135
                                self.IncPC(3);
                                self.increase_cycles(10);
                                if (self.data.F & FLAG_C) != 0 {
                                    // JP(LAB_ram_6135);
                                    //                              LAB_ram_6135                                    XREF[1]:     ram:612c(j)
                                    //         ram:6135 3e 03           LD         A,0x3
                                    self.instr_hk__LD_A_NN(0x3);
                                    //         ram:6137 90              SUB        B
                                    self.instr_hk__SUB_A_B();
                                    //         ram:6138 87              ADD        A,A
                                    self.instr_hk__ADD_A_A();
                                    //         ram:6139 87              ADD        A,A
                                    self.instr_hk__ADD_A_A();
                                    //         ram:613a 4f              LD         C,A
                                    self.instr_hk__LD_C_A();
                                    //         ram:613b 06 00           LD         B,0x0
                                    self.instr_hk__LD_B_NN(0x0);
                                    //         ram:613d 21 ec c1        LD         HL,BYTE_ram_c1ec
                                    self.instr_hk__LD_HL_NNNN(0xc1ec);
                                    //         ram:6140 09              ADD        HL,BC
                                    self.instr_hk__ADD_HL_BC();
                                    //         ram:6141 56              LD         D,(HL=>BYTE_ram_c1ec)
                                    self.instr_hk__LD_D_iHL();
                                    //         ram:6142 23              INC        HL
                                    self.instr_hk__INC_HL();
                                    //         ram:6143 5e              LD         E,(HL=>BYTE_ram_c1ed)
                                    self.instr_hk__LD_E_iHL();
                                    //         ram:6144 e1              POP        HL
                                    self.instr_hk__POP_HL();
                                    //         ram:6145 e5              PUSH       HL
                                    self.instr_hk__PUSH_HL();
                                    //         ram:6146 b7              OR         A
                                    self.instr_hk__OR_A_A();
                                    //         ram:6147 ed 52           SBC        HL,DE
                                    self.instr_hk__SBC_HL_DE();
                                    //         ram:6149 ca 50 61        JP         Z,LAB_ram_6150
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    if (self.data.F & FLAG_Z) != 0 {
                                        // JP(LAB_ram_6150);
                                        break;
                                    } else {
                                        //         ram:614c e1              POP        HL
                                        self.instr_hk__POP_HL();
                                        //         ram:614d c3 b1 61        JP         l_exit_set_cf
                                        self.IncPC(3);
                                        self.increase_cycles(10);
                                        // JP(l_exit_set_cf);
                                        //                              l_exit_set_cf                                   XREF[6]:     ram:60f2(j), ram:60f9(j),
                                        //                                                                                           ram:6107(j), ram:610f(j),
                                        //                                                                                           ram:614d(j), ram:617c(j)
                                        //         ram:61b1 37              SCF
                                        self.instr_hk__SCF();
                                        //         ram:61b2 c9              RET
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                //                              loop_1_chk_cond                                 XREF[4]:     ram:611b(j), ram:611f(j),
                //                                                                                           ram:6123(j), ram:6127(j)
                //         ram:612f 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:6130 10 e6           DJNZ       loop_1
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP loop_1;
                } else {
                    self.increase_cycles(8);
                    // break;
                    //         ram:6132 c3 50 61        JP         LAB_ram_6150
                    self.IncPC(3);
                    self.increase_cycles(10);
                    // JP(LAB_ram_6150);
                    break;
                }
            }

            //         ram:6132 c3 50 61        JP         LAB_ram_6150
            // self.IncPC(3);
            // self.increase_cycles(10);
            // JP(LAB_ram_6150);

            //                              LAB_ram_6150                                    XREF[2]:     ram:6132(j), ram:6149(j)
            //         ram:6150 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:6151 cb 3c           SRL        H
            self.instr_hk__SRL_H();
            //         ram:6153 cb 3c           SRL        H
            self.instr_hk__SRL_H();
            //         ram:6155 cb 3d           SRL        L
            self.instr_hk__SRL_L();
            //         ram:6157 cb 3d           SRL        L
            self.instr_hk__SRL_L();
            //         ram:6159 cd bc 89        CALL       fn_calc_voffset_89BC                             hl <- (hl & 0xff) * 20 + (hl >> 8)
            assert!(self.call_hook(0x89bc));
            //                                                                                              de <- (hl >> 8) << 8 + (hl & 0xff)
            //         ram:615c 11 ac c3        LD         DE,DAT_ram_c3ac
            self.instr_hk__LD_DE_NNNN(0xc3ac);
            //         ram:615f 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:6160 3a 87 c3        LD         A,(BYTE_ram_c387)
            self.instr_hk__LD_A_iNNNN(0xc387);
            //         ram:6163 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:6164 ca 84 61        JP         Z,LAB_ram_6184
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(LAB_ram_6184);
                //                              LAB_ram_6184                                    XREF[1]:     ram:6164(j)
                //         ram:6184 7e              LD         A,(HL=>DAT_ram_c3ac)
                self.instr_hk__LD_A_iHL();
                //         ram:6185 fe 03           CP         0x3
                self.instr_hk__CP_NN(0x3);
                //         ram:6187 c2 8f 61        JP         NZ,LAB_ram_618f
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP(LAB_ram_618f);
                    //                              LAB_ram_618f                                    XREF[1]:     ram:6187(j)
                    //         ram:618f fe 05           CP         0x5
                    self.instr_hk__CP_NN(0x5);
                    //         ram:6191 c2 9c 61        JP         NZ,LAB_ram_619c
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_Z) == 0 {
                        // JP(LAB_ram_619c);
                        //                              LAB_ram_619c                                    XREF[1]:     ram:6191(j)
                        //         ram:619c fe 07           CP         0x7
                        self.instr_hk__CP_NN(0x7);
                        //         ram:619e c2 a4 61        JP         NZ,LAB_ram_61a4
                        self.IncPC(3);
                        self.increase_cycles(10);
                        if (self.data.F & FLAG_Z) == 0 {
                            // JP(LAB_ram_61a4);
                            //                              LAB_ram_61a4                                    XREF[1]:     ram:619e(j)
                            //         ram:61a4 fe 09           CP         0x9
                            self.instr_hk__CP_NN(0x9);
                            //         ram:61a6 c2 ac 61        JP         NZ,LAB_ram_61ac
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_Z) == 0 {
                                // JP(LAB_ram_61ac);
                                //                              LAB_ram_61ac                                    XREF[1]:     ram:61a6(j)
                                //         ram:61ac fe 0c           CP         0xc
                                self.instr_hk__CP_NN(0xc);
                                //         ram:61ae d2 75 61        JP         NC,LAB_ram_6175
                                self.IncPC(3);
                                self.increase_cycles(10);
                                if (self.data.F & FLAG_C) == 0 {
                                    // JP(LAB_ram_6175);
                                    //                              LAB_ram_6175                                    XREF[2]:     ram:616d(j), ram:61ae(j)
                                    //         ram:6175 3e 06           LD         A,0x6
                                    self.instr_hk__LD_A_NN(0x6);
                                    //         ram:6177 c3 b3 61        JP         l_exit_chk_a
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    // JP(l_exit_chk_a);
                                    //                              l_exit_chk_a                                    XREF[3]:     ram:6177(j), ram:6181(j),
                                    //                                                                                           ram:618c(j)
                                    //         ram:61b3 b7              OR         A
                                    self.instr_hk__OR_A_A();
                                    //         ram:61b4 c9              RET
                                    //
                                    return true;
                                } else {
                                    // JP(l_exit_set_cf);
                                    //                              l_exit_set_cf                                   XREF[6]:     ram:60f2(j), ram:60f9(j),
                                    //                                                                                           ram:6107(j), ram:610f(j),
                                    //                                                                                           ram:614d(j), ram:617c(j)
                                    //         ram:61b1 37              SCF
                                    self.instr_hk__SCF();
                                    //         ram:61b2 c9              RET
                                    return true;
                                }
                            } else {
                                //         ram:61a9 c3 7f 61        JP         LAB_ram_617f
                                self.IncPC(3);
                                self.increase_cycles(10);
                                // JP(LAB_ram_617f);
                                //                              LAB_ram_617f                                    XREF[4]:     ram:60fc(j), ram:6199(j),
                                //                                                                                           ram:61a1(j), ram:61a9(j)
                                //         ram:617f 3e 0e           LD         A,0xe
                                self.instr_hk__LD_A_NN(0xe);
                                //         ram:6181 c3 b3 61        JP         l_exit_chk_a
                                self.IncPC(3);
                                self.increase_cycles(10);
                                // JP(l_exit_chk_a);
                                //                              l_exit_chk_a                                    XREF[3]:     ram:6177(j), ram:6181(j),
                                //                                                                                           ram:618c(j)
                                //         ram:61b3 b7              OR         A
                                self.instr_hk__OR_A_A();
                                //         ram:61b4 c9              RET
                                //
                                return true;
                            }
                        } else {
                            //         ram:61a1 c3 7f 61        JP         LAB_ram_617f
                            self.IncPC(3);
                            self.increase_cycles(10);
                            // JP(LAB_ram_617f);
                            //                              LAB_ram_617f                                    XREF[4]:     ram:60fc(j), ram:6199(j),
                            //                                                                                           ram:61a1(j), ram:61a9(j)
                            //         ram:617f 3e 0e           LD         A,0xe
                            self.instr_hk__LD_A_NN(0xe);
                            //         ram:6181 c3 b3 61        JP         l_exit_chk_a
                            self.IncPC(3);
                            self.increase_cycles(10);
                            // JP(l_exit_chk_a);
                            //                              l_exit_chk_a                                    XREF[3]:     ram:6177(j), ram:6181(j),
                            //                                                                                           ram:618c(j)
                            //         ram:61b3 b7              OR         A
                            self.instr_hk__OR_A_A();
                            //         ram:61b4 c9              RET
                            //
                            return true;
                        }
                    } else {
                        //         ram:6194 3e 01           LD         A,0x1
                        self.instr_hk__LD_A_NN(0x1);
                        //         ram:6196 32 0b c2        LD         (BYTE_ram_c20b),A
                        self.instr_hk__LD_iNNNN_A(0xc20b);
                        //         ram:6199 c3 7f 61        JP         LAB_ram_617f
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP(LAB_ram_617f);
                        //                              LAB_ram_617f                                    XREF[4]:     ram:60fc(j), ram:6199(j),
                        //                                                                                           ram:61a1(j), ram:61a9(j)
                        //         ram:617f 3e 0e           LD         A,0xe
                        self.instr_hk__LD_A_NN(0xe);
                        //         ram:6181 c3 b3 61        JP         l_exit_chk_a
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP(l_exit_chk_a);
                        //                              l_exit_chk_a                                    XREF[3]:     ram:6177(j), ram:6181(j),
                        //                                                                                           ram:618c(j)
                        //         ram:61b3 b7              OR         A
                        self.instr_hk__OR_A_A();
                        //         ram:61b4 c9              RET
                        //
                        return true;
                    }
                } else {
                    //         ram:618a 3e 08           LD         A,0x8
                    self.instr_hk__LD_A_NN(0x8);
                    //         ram:618c c3 b3 61        JP         l_exit_chk_a
                    self.IncPC(3);
                    self.increase_cycles(10);
                    // JP(l_exit_chk_a);
                    //                              l_exit_chk_a                                    XREF[3]:     ram:6177(j), ram:6181(j),
                    //                                                                                           ram:618c(j)
                    //         ram:61b3 b7              OR         A
                    self.instr_hk__OR_A_A();
                    //         ram:61b4 c9              RET
                    //
                    return true;
                }
            }

            //         ram:6167 7e              LD         A,(HL=>DAT_ram_c3ac)
            self.instr_hk__LD_A_iHL();
            //         ram:6168 fe 0c           CP         0xc
            self.instr_hk__CP_NN(0xc);
            //         ram:616a c2 70 61        JP         NZ,LAB_ram_6170
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(LAB_ram_6170);
                //                              LAB_ram_6170                                    XREF[1]:     ram:616a(j)
                //         ram:6170 fe 0e           CP         0xe
                self.instr_hk__CP_NN(0xe);
                //         ram:6172 c2 7a 61        JP         NZ,LAB_ram_617a
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP(LAB_ram_617a);
                    //                              LAB_ram_617a                                    XREF[1]:     ram:6172(j)
                    //         ram:617a fe 10           CP         0x10
                    self.instr_hk__CP_NN(0x10);
                    //         ram:617c c2 b1 61        JP         NZ,l_exit_set_cf
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_Z) == 0 {
                        // JP(l_exit_set_cf);
                        //                              l_exit_set_cf                                   XREF[6]:     ram:60f2(j), ram:60f9(j),
                        //                                                                                           ram:6107(j), ram:610f(j),
                        //                                                                                           ram:614d(j), ram:617c(j)
                        //         ram:61b1 37              SCF
                        self.instr_hk__SCF();
                        //         ram:61b2 c9              RET
                        return true;
                    } else {
                        //                              LAB_ram_617f                                    XREF[4]:     ram:60fc(j), ram:6199(j),
                        //                                                                                           ram:61a1(j), ram:61a9(j)
                        //         ram:617f 3e 0e           LD         A,0xe
                        self.instr_hk__LD_A_NN(0xe);
                        //         ram:6181 c3 b3 61        JP         l_exit_chk_a
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP(l_exit_chk_a);
                        //                              l_exit_chk_a                                    XREF[3]:     ram:6177(j), ram:6181(j),
                        //                                                                                           ram:618c(j)
                        //         ram:61b3 b7              OR         A
                        self.instr_hk__OR_A_A();
                        //         ram:61b4 c9              RET
                        //
                        return true;
                    }
                }
            } else {
                //         ram:616d c3 75 61        JP         LAB_ram_6175
                self.IncPC(3);
                self.increase_cycles(10);
                // JP(LAB_ram_6175);
            }

            //                              LAB_ram_6175                                    XREF[2]:     ram:616d(j), ram:61ae(j)
            //         ram:6175 3e 06           LD         A,0x6
            self.instr_hk__LD_A_NN(0x6);
            //         ram:6177 c3 b3 61        JP         l_exit_chk_a
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(l_exit_chk_a);
            //                              l_exit_chk_a                                    XREF[3]:     ram:6177(j), ram:6181(j),
            //                                                                                           ram:618c(j)
            //         ram:61b3 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:61b4 c9              RET
            //
            return true;
        }

        //         ram:60ec cd ea 46        CALL       sb_read_mem_for_player_46ea                      OUT d,e
        assert!(self.call_hook(0x46ea));
        //         ram:60ef 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:60f0 fe 12           CP         0x12
        self.instr_hk__CP_NN(0x12);
        //         ram:60f2 c2 b1 61        JP         NZ,l_exit_set_cf
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            // JP(l_exit_set_cf);
            //                              l_exit_set_cf                                   XREF[6]:     ram:60f2(j), ram:60f9(j),
            //                                                                                           ram:6107(j), ram:610f(j),
            //                                                                                           ram:614d(j), ram:617c(j)
            //         ram:61b1 37              SCF
            self.instr_hk__SCF();
            //         ram:61b2 c9              RET
            return true;
        }

        //         ram:60f5 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:60f6 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:60f7 fe 0f           CP         0xf
        self.instr_hk__CP_NN(0xf);
        //         ram:60f9 c2 b1 61        JP         NZ,l_exit_set_cf
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            // JP(l_exit_set_cf);
            //                              l_exit_set_cf                                   XREF[6]:     ram:60f2(j), ram:60f9(j),
            //                                                                                           ram:6107(j), ram:610f(j),
            //                                                                                           ram:614d(j), ram:617c(j)
            //         ram:61b1 37              SCF
            self.instr_hk__SCF();
            //         ram:61b2 c9              RET
            true
        } else {
            //         ram:60fc c3 7f 61        JP         LAB_ram_617f
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(LAB_ram_617f);
            //                              LAB_ram_617f                                    XREF[4]:     ram:60fc(j), ram:6199(j),
            //                                                                                           ram:61a1(j), ram:61a9(j)
            //         ram:617f 3e 0e           LD         A,0xe
            self.instr_hk__LD_A_NN(0xe);
            //         ram:6181 c3 b3 61        JP         l_exit_chk_a
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(l_exit_chk_a);
            //                              l_exit_chk_a                                    XREF[3]:     ram:6177(j), ram:6181(j),
            //                                                                                           ram:618c(j)
            //         ram:61b3 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:61b4 c9              RET
            //
            true
        }
    }
    pub(crate) fn hook_60db(&mut self) -> bool {
        loop {
            self.SetPC(0x60db);
            //         ram:60db c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:60dc 3e  20           LD         A,' '
            self.instr_hk__LD_A_NN(0x20);
            //         ram:60de cd  d6  89       CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));
            //         ram:60e1 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:60e2 10  f7           DJNZ       sb_print_spaces_60db
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                // JP sb_print_spaces_60db;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        self.assert_pc(0x60e4);
        //         ram:60e4 c9              RET
        true
    }
    pub(crate) fn hook_67f7(&mut self) -> bool {
        println!("hook_67f7");
        //         ram:67f7 cd  6b  47       CALL       sb_get_current_char_spell_addr_guess_476b
        assert!(self.call_hook(0x476b));

        //         ram:67fa 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:67fb 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:67fc c9              RET
        self.assert_pc(0x67fc);

        true
    }
    pub(crate) fn hook_69ac(&mut self) -> bool {
        //         ram:69ac 3a 1f c2        LD         A,(BYTE_ram_c21f)
        self.instr_hk__LD_A_iNNNN(0xc21f);
        //         ram:69af b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:69b0 c0              RET        NZ
        self.IncPC(1);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:69b1 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:69b2 32 1f c2        LD         (BYTE_ram_c21f),A
        self.instr_hk__LD_iNNNN_A(0xc21f);
        //         ram:69b5 af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:69b6 32 3e c2        LD         (DAT_ram_c23e),A
        self.instr_hk__LD_iNNNN_A(0xc23e);
        //         ram:69b9 3a 0c c2        LD         A,(BYTE_ram_c20c)
        self.instr_hk__LD_A_iNNNN(0xc20c);
        //         ram:69bc b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:69bd 20 1d           JR         NZ,LAB_ram_69dc
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12); //JR LAB_ram_69dc;
        } else {
            self.increase_cycles(7);
            //         ram:69bf 3a ed c1        LD         A,(BYTE_ram_c1ed)
            self.instr_hk__LD_A_iNNNN(0xc1ed);
            //         ram:69c2 e6 fc           AND        0xfc
            self.instr_hk__AND_NN(0xfc);
            //         ram:69c4 6f              LD         L,A
            self.instr_hk__LD_L_A();
            //         ram:69c5 26 00           LD         H,0x0
            self.instr_hk__LD_H_NN(0x0);
            //         ram:69c7 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:69c8 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:69c9 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:69ca 3a ec c1        LD         A,(BYTE_ram_c1ec)
            self.instr_hk__LD_A_iNNNN(0xc1ec);
            //         ram:69cd cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:69cf cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:69d1 4f              LD         C,A
            self.instr_hk__LD_C_A();
            //         ram:69d2 06 00           LD         B,0x0
            self.instr_hk__LD_B_NN(0x0);
            //         ram:69d4 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:69d5 01 ac c3        LD         BC,DAT_ram_c3ac
            self.instr_hk__LD_BC_NNNN(0xc3ac);
            //         ram:69d8 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:69d9 7e              LD         A,(HL=>DAT_ram_c3ac)
            self.instr_hk__LD_A_iHL();
            //         ram:69da 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:69db 77              LD         (HL),A=>DAT_ram_c3ac
            self.instr_hk__LD_iHL_A(); //=>DAT_ram_c3ac();
        }

        //                              LAB_ram_69dc
        //         ram:69dc af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:69dd 32 0c c2        LD         (BYTE_ram_c20c),A
        self.instr_hk__LD_iNNNN_A(0xc20c);

        self.assert_pc(0x69e0);
        //         ram:69e0 c3 5d 82        JP         FUN_ram_825d                                     ()
        self.IncPC(3);
        self.increase_cycles(10);
        // JP FUN_ram_825d;
        self.internal_825d();
        //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)

        true
    }
    pub(crate) fn hook_69e3(&mut self) -> bool {
        //         ram:69e3 32 1c c2        LD         (BYTE_ram_c21c),A
        self.instr_hk__LD_iNNNN_A(0xc21c);
        //         ram:69e6 ed 53 1d c2     LD         (wd_l_c21d),DE
        self.instr_hk__LD_iNNNN_DE(0xc21d);
        //         ram:69ea eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:69eb cd bc 89        CALL       fn_calc_voffset_89BC
        assert!(self.call_hook(0x89bc));

        //         ram:69ee 11 9a c9        LD         DE,DAT_ram_c99a
        self.instr_hk__LD_DE_NNNN(0xc99a);
        //         ram:69f1 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:69f2 01 00 00        LD         BC,0x0
        self.instr_hk__LD_BC_NNNN(0x0);
        //         ram:69f5 50              LD         D,B
        self.instr_hk__LD_D_B();
        //         ram:69f6 cd 81 6a        CALL       sb_change_bcd_6A81
        assert!(self.call_hook(0x6a81));

        //         ram:69f9 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:69fa cd 81 6a        CALL       sb_change_bcd_6A81
        assert!(self.call_hook(0x6a81));

        //         ram:69fd d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:69fe 11 1f 00        LD         DE,0x1f
        self.instr_hk__LD_DE_NNNN(0x1f);
        //         ram:6a01 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:6a02 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:6a03 cd 81 6a        CALL       sb_change_bcd_6A81
        assert!(self.call_hook(0x6a81));

        //         ram:6a06 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:6a07 cd 81 6a        CALL       sb_change_bcd_6A81
        assert!(self.call_hook(0x6a81));

        //         ram:6a0a 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:6a0b fe 02           CP         0x2
        self.instr_hk__CP_NN(0x2);
        //         ram:6a0d ca 79 6a        JP         Z,l_exit_set_cf_a_1
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP l_exit_set_cf_a_1;
            //                              l_exit_set_cf_a_1
            //         ram:6a79 3e 01           LD         A,0x1
            self.instr_hk__LD_A_NN(0x1);
            //         ram:6a7b 37              SCF
            self.instr_hk__SCF();
            //         ram:6a7c c9              RET
            return true;
        }

        //         ram:6a10 fe 06           CP         0x6
        self.instr_hk__CP_NN(0x6);
        //         ram:6a12 ca 7d 6a        JP         Z,l_exit_set_cf_a_3
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP l_exit_set_cf_a_3;
            //                              l_exit_set_cf_a_3
            //         ram:6a7d 3e 03           LD         A,0x3
            self.instr_hk__LD_A_NN(0x3);
            //         ram:6a7f 37              SCF
            self.instr_hk__SCF();
            //         ram:6a80 c9              RET
        }

        //         ram:6a15 79              LD         A,C
        self.instr_hk__LD_A_C();
        //         ram:6a16 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:6a17 c2 76 6a        JP         NZ,l_exit_set_cf_a_0
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            // JP l_exit_set_cf_a_0;
            //                              l_exit_set_cf_a_0
            //         ram:6a76 af              XOR        A
            self.instr_hk__XOR_A_A();
            //         ram:6a77 37              SCF
            self.instr_hk__SCF();
            //         ram:6a78 c9              RET
            return true;
        }

        //         ram:6a1a 7a              LD         A,D
        self.instr_hk__LD_A_D();
        //         ram:6a1b fe 03           CP         0x3
        self.instr_hk__CP_NN(0x3);
        //         ram:6a1d d2 76 6a        JP         NC,l_exit_set_cf_a_0
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) == 0 {
            // JP l_exit_set_cf_a_0;
            //                              l_exit_set_cf_a_0
            //         ram:6a76 af              XOR        A
            self.instr_hk__XOR_A_A();
            //         ram:6a77 37              SCF
            self.instr_hk__SCF();
            //         ram:6a78 c9              RET
            return true;
        }

        //         ram:6a20 21 ee c1        LD         HL,BYTE_ram_c1ee
        self.instr_hk__LD_HL_NNNN(0xc1ee);
        //         ram:6a23 06 03           LD         B,0x3
        self.instr_hk__LD_B_NN(0x3);
        //                              loop_1
        loop {
            self.SetPC(0x6a25);
            //         ram:6a25 e5              PUSH       HL=>BYTE_ram_c1ee
            self.instr_hk__PUSH_HL();
            //         ram:6a26 3e 03           LD         A,0x3
            self.instr_hk__LD_A_NN(0x3);
            //         ram:6a28 90              SUB        B
            self.instr_hk__SUB_A_B();
            //         ram:6a29 5f              LD         E,A
            self.instr_hk__LD_E_A();
            //         ram:6a2a 3a 1c c2        LD         A,(BYTE_ram_c21c)
            self.instr_hk__LD_A_iNNNN(0xc21c);
            //         ram:6a2d bb              CP         E
            self.instr_hk__CP_E();
            //         ram:6a2e ca 6a 6a        JP         Z,LAB_ram_6a6a
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP LAB_ram_6a6a;
            } else {
                //         ram:6a31 16 00           LD         D,0x0
                self.instr_hk__LD_D_NN(0x0);
                //         ram:6a33 21 4c c3        LD         HL,BYTE_ram_c34c
                self.instr_hk__LD_HL_NNNN(0xc34c);
                //         ram:6a36 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:6a37 7e              LD         A,(HL=>BYTE_ram_c34c)
                self.instr_hk__LD_A_iHL();
                //         ram:6a38 b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:6a39 ca 6a 6a        JP         Z,LAB_ram_6a6a
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) != 0 {
                    // JP LAB_ram_6a6a;
                } else {
                    //         ram:6a3c e1              POP        HL
                    self.instr_hk__POP_HL();
                    //         ram:6a3d 56              LD         D,(HL=>BYTE_ram_c1ee)
                    self.instr_hk__LD_D_iHL();
                    //         ram:6a3e 23              INC        HL
                    self.instr_hk__INC_HL();
                    //         ram:6a3f 5e              LD         E,(HL=>BYTE_ram_c1ef)
                    self.instr_hk__LD_E_iHL();
                    //         ram:6a40 23              INC        HL
                    self.instr_hk__INC_HL();
                    //         ram:6a41 23              INC        HL
                    self.instr_hk__INC_HL();
                    //         ram:6a42 23              INC        HL
                    self.instr_hk__INC_HL();
                    //         ram:6a43 e5              PUSH       HL=>BYTE_ram_c1f2
                    self.instr_hk__PUSH_HL();
                    //         ram:6a44 2a 1d c2        LD         HL,(wd_l_c21d)
                    self.instr_hk__LD_HL_iNNNN(0xc21d);
                    //         ram:6a47 7c              LD         A,H
                    self.instr_hk__LD_A_H();
                    //         ram:6a48 3d              DEC        A
                    self.instr_hk__DEC_A();
                    //         ram:6a49 ba              CP         D
                    self.instr_hk__CP_D();
                    //         ram:6a4a ca 57 6a        JP         Z,LAB_ram_6a57
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_Z) != 0 {
                        // JP LAB_ram_6a57;
                    } else {
                        //         ram:6a4d 3c              INC        A
                        self.instr_hk__INC_A();
                        //         ram:6a4e ba              CP         D
                        self.instr_hk__CP_D();
                        //         ram:6a4f ca 57 6a        JP         Z,LAB_ram_6a57
                        self.IncPC(3);
                        self.increase_cycles(10);
                        if (self.data.F & FLAG_Z) != 0 {
                            // JP LAB_ram_6a57;
                        } else {
                            //         ram:6a52 3c              INC        A
                            self.instr_hk__INC_A();
                            //         ram:6a53 ba              CP         D
                            self.instr_hk__CP_D();
                            //         ram:6a54 c2 70 6a        JP         NZ,loop_1_chk_cond
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_Z) == 0 {
                                // JP loop_1_chk_cond;
                                //         ram:6a70 e1              POP        HL
                                self.instr_hk__POP_HL();
                                //         ram:6a71 10 b2           DJNZ       loop_1
                                self.IncPC(2);
                                self.decB();
                                if self.data.B != 0 {
                                    self.increase_cycles(13);
                                    //JP loop_1;
                                    continue;
                                } else {
                                    self.increase_cycles(8);
                                    break;
                                }
                            }
                        }
                    }

                    //                              LAB_ram_6a57
                    //         ram:6a57 7d              LD         A,L
                    self.instr_hk__LD_A_L();
                    //         ram:6a58 3d              DEC        A
                    self.instr_hk__DEC_A();
                    //         ram:6a59 bb              CP         E
                    self.instr_hk__CP_E();
                    //         ram:6a5a ca 75 6a        JP         Z,ll_exit_set_cf_a_0
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_Z) != 0 {
                        // JP ll_exit_set_cf_a_0;
                        //                              ll_exit_set_cf_a_0

                        //         ram:6a75 e1              POP        HL
                        self.instr_hk__POP_HL();
                        //                              l_exit_set_cf_a_0
                        //         ram:6a76 af              XOR        A
                        self.instr_hk__XOR_A_A();
                        //         ram:6a77 37              SCF
                        self.instr_hk__SCF();
                        //         ram:6a78 c9              RET
                        return true;
                    } else {
                        //         ram:6a5d 3c              INC        A
                        self.instr_hk__INC_A();
                        //         ram:6a5e bb              CP         E
                        self.instr_hk__CP_E();
                        //         ram:6a5f ca 75 6a        JP         Z,ll_exit_set_cf_a_0
                        self.IncPC(3);
                        self.increase_cycles(10);
                        if (self.data.F & FLAG_Z) != 0 {
                            // JP ll_exit_set_cf_a_0;
                            //                              ll_exit_set_cf_a_0

                            //         ram:6a75 e1              POP        HL
                            self.instr_hk__POP_HL();
                            //                              l_exit_set_cf_a_0
                            //         ram:6a76 af              XOR        A
                            self.instr_hk__XOR_A_A();
                            //         ram:6a77 37              SCF
                            self.instr_hk__SCF();
                            //         ram:6a78 c9              RET
                            return true;
                        } else {
                            //         ram:6a62 3c              INC        A
                            self.instr_hk__INC_A();
                            //         ram:6a63 bb              CP         E
                            self.instr_hk__CP_E();
                            //         ram:6a64 ca 75 6a        JP         Z,ll_exit_set_cf_a_0
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_Z) != 0 {
                                // JP ll_exit_set_cf_a_0;
                                //                              ll_exit_set_cf_a_0

                                //         ram:6a75 e1              POP        HL
                                self.instr_hk__POP_HL();
                                //                              l_exit_set_cf_a_0
                                //         ram:6a76 af              XOR        A
                                self.instr_hk__XOR_A_A();
                                //         ram:6a77 37              SCF
                                self.instr_hk__SCF();
                                //         ram:6a78 c9              RET
                                return true;
                            } else {
                                //         ram:6a67 c3 70 6a        JP         loop_1_chk_cond
                                self.IncPC(3);
                                self.increase_cycles(10);
                                // JP loop_1_chk_cond;
                                //         ram:6a70 e1              POP        HL
                                self.instr_hk__POP_HL();
                                //         ram:6a71 10 b2           DJNZ       loop_1
                                self.IncPC(2);
                                self.decB();
                                if self.data.B != 0 {
                                    self.increase_cycles(13);
                                    //JP loop_1;
                                    continue;
                                } else {
                                    self.increase_cycles(8);
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            //                              LAB_ram_6a6a
            //         ram:6a6a e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:6a6b 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:6a6c 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:6a6d 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:6a6e 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:6a6f e5              PUSH       HL=>BYTE_ram_c1f2
            self.instr_hk__PUSH_HL();
            //                              loop_1_chk_cond
            //         ram:6a70 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:6a71 10 b2           DJNZ       loop_1
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_1;
                continue;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        self.SetPC(0x6a73);
        //         ram:6a73 af              XOR        A
        self.instr_hk__XOR_A_A();
        self.assert_pc(0x6a74);
        //         ram:6a74 c9              RET
        true
    }
    pub(crate) fn hook_6a81(&mut self) -> bool {
        //         ram:6a81 3a 87 c3        LD         A,(BYTE_ram_c387)
        self.instr_hk__LD_A_iNNNN(0xc387);

        //         ram:6a84 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:6a85 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:6a86 ca a9 6a        JP         Z,l_exit_x
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            self.SetPC(0x6aa9);
            // JP(l_exit_x);
            //                              l_exit_x
            //         ram:6aa9 fe 40           CP         64
            self.instr_hk__CP_NN(64);
            //         ram:6aab d2 c9 6a        JP         NC,l_exit_inc_d
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP(l_exit_inc_d);
                self.SetPC(0x6ac9);
                //                              l_exit_inc_d
                //         ram:6ac9 14              INC        D
                self.instr_hk__INC_D();
                //         ram:6aca c9              RET

                self.assert_pc(0x6aca);
                true
            } else {
                self.assert_pc(0x6aae);
                //         ram:6aae fe 25           CP         37
                self.instr_hk__CP_NN(37);
                //         ram:6ab0 d0              RET        NC
                self.IncPC(1);
                if (self.data.F & FLAG_C) == 0 {
                    self.increase_cycles(11);
                    true
                } else {
                    self.assert_pc(0x6ab1);
                    self.increase_cycles(5);
                    //         ram:6ab1 fe 21           CP         33
                    self.instr_hk__CP_NN(33);
                    //         ram:6ab3 d2 c4 6a        JP         NC,l_exit_inc_c_add_b_3
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) == 0 {
                        self.SetPC(0x6ac4);
                        // JP(l_exit_inc_c_add_b_3);
                        //                              l_exit_inc_c_add_b_3
                        //         ram:6ac4 04              INC        B
                        self.instr_hk__INC_B();
                        //         ram:6ac5 04              INC        B
                        self.instr_hk__INC_B();
                        //                              l_exit_inc_c_inc_b

                        //         ram:6ac6 04              INC        B
                        self.instr_hk__INC_B();
                        //                              l_exit_inc_c
                        //         ram:6ac7 0c              INC        C
                        self.instr_hk__INC_C();
                        //         ram:6ac8 c9              RET
                        self.assert_pc(0x6ac8);
                        true
                    } else {
                        self.assert_pc(0x6ab6);
                        //         ram:6ab6 fe 15           CP         21
                        self.instr_hk__CP_NN(21);
                        //         ram:6ab8 d0              RET        NC
                        self.IncPC(1);
                        if (self.data.F & FLAG_C) == 0 {
                            self.increase_cycles(11);
                            true
                        } else {
                            self.assert_pc(0x6ab9);
                            self.increase_cycles(5);
                            //         ram:6ab9 fe 05           CP         5
                            self.instr_hk__CP_NN(5);
                            //         ram:6abb d2 c7 6a        JP         NC,l_exit_inc_c
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_C) == 0 {
                                self.SetPC(0x6ac7);
                                // JP(l_exit_inc_c);
                                //                              l_exit_inc_c
                                //         ram:6ac7 0c              INC        C
                                self.instr_hk__INC_C();
                                //         ram:6ac8 c9              RET
                                self.assert_pc(0x6ac8);
                                true
                            } else {
                                self.assert_pc(0x6abe);
                                //         ram:6abe fe 01           CP         1
                                self.instr_hk__CP_NN(1);
                                //         ram:6ac0 d2 c6 6a        JP         NC,l_exit_inc_c_inc_b
                                self.IncPC(3);
                                self.increase_cycles(10);
                                if (self.data.F & FLAG_C) == 0 {
                                    // JP(l_exit_inc_c_inc_b);
                                    //                              l_exit_inc_c_inc_b
                                    self.SetPC(0x6ac6);

                                    //         ram:6ac6 04              INC        B
                                    self.instr_hk__INC_B();
                                    //                              l_exit_inc_c
                                    //         ram:6ac7 0c              INC        C
                                    self.instr_hk__INC_C();
                                    //         ram:6ac8 c9              RET
                                    self.assert_pc(0x6ac8);
                                    true
                                } else {
                                    //         ram:6ac3 c9              RET
                                    self.assert_pc(0x6ac3);
                                    true
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self.assert_pc(0x6a89);
            //         ram:6a89 fe 40           CP         64
            self.instr_hk__CP_NN(64);
            //         ram:6a8b d2 c9 6a        JP         NC,l_exit_inc_d
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP(l_exit_inc_d);
                //                              l_exit_inc_d
                self.SetPC(0x6ac9);
                //         ram:6ac9 14              INC        D
                self.instr_hk__INC_D();
                //         ram:6aca c9              RET

                self.assert_pc(0x6aca);
                true
            } else {
                //         ram:6a8e fe 23           CP         35
                self.instr_hk__CP_NN(35);
                //         ram:6a90 d0              RET        NC
                self.IncPC(1);
                if (self.data.F & FLAG_C) == 0 {
                    self.increase_cycles(11);
                    true
                } else {
                    self.assert_pc(0x6a91);
                    self.increase_cycles(5);
                    //         ram:6a91 fe 1f           CP         31
                    self.instr_hk__CP_NN(31);
                    //         ram:6a93 d2 c6 6a        JP         NC,l_exit_inc_c_inc_b
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) == 0 {
                        // JP(l_exit_inc_c_inc_b);
                        self.SetPC(0x6ac6);
                        //                              l_exit_inc_c_inc_b

                        //         ram:6ac6 04              INC        B
                        self.instr_hk__INC_B();
                        //                              l_exit_inc_c
                        //         ram:6ac7 0c              INC        C
                        self.instr_hk__INC_C();
                        //         ram:6ac8 c9              RET
                        self.assert_pc(0x6ac8);
                        true
                    } else {
                        //         ram:6a96 fe 15           CP         21
                        self.assert_pc(0x6a96);
                        self.instr_hk__CP_NN(21);
                        //         ram:6a98 d0              RET        NC
                        self.IncPC(1);
                        if (self.data.F & FLAG_C) == 0 {
                            self.increase_cycles(11);
                            true
                        } else {
                            self.assert_pc(0x6a99);
                            self.increase_cycles(5);
                            //         ram:6a99 fe 11           CP         17
                            self.instr_hk__CP_NN(17);
                            //         ram:6a9b d2 c4 6a        JP         NC,l_exit_inc_c_add_b_3
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_C) == 0 {
                                // JP(l_exit_inc_c_add_b_3);
                                self.assert_pc(0x6ac4);
                                //                              l_exit_inc_c_add_b_3
                                //         ram:6ac4 04              INC        B
                                self.instr_hk__INC_B();
                                //         ram:6ac5 04              INC        B
                                self.instr_hk__INC_B();
                                //                              l_exit_inc_c_inc_b

                                //         ram:6ac6 04              INC        B
                                self.instr_hk__INC_B();
                                //                              l_exit_inc_c
                                //         ram:6ac7 0c              INC        C
                                self.instr_hk__INC_C();
                                //         ram:6ac8 c9              RET
                                self.assert_pc(0x6ac8);
                                true
                            } else {
                                self.assert_pc(0x6a9e);
                                //         ram:6a9e fe 09           CP         9
                                self.instr_hk__CP_NN(9);
                                //         ram:6aa0 d2 c6 6a        JP         NC,l_exit_inc_c_inc_b
                                self.IncPC(3);
                                self.increase_cycles(10);
                                if (self.data.F & FLAG_C) == 0 {
                                    // JP(l_exit_inc_c_inc_b);
                                    self.SetPC(0x6ac6);
                                    //                              l_exit_inc_c_inc_b

                                    //         ram:6ac6 04              INC        B
                                    self.instr_hk__INC_B();
                                    //                              l_exit_inc_c
                                    //         ram:6ac7 0c              INC        C
                                    self.instr_hk__INC_C();
                                    //         ram:6ac8 c9              RET
                                    self.assert_pc(0x6ac8);
                                    true
                                } else {
                                    self.assert_pc(0x6aa3);
                                    //         ram:6aa3 fe 01           CP         1
                                    self.instr_hk__CP_NN(1);
                                    //         ram:6aa5 d2 c7 6a        JP         NC,l_exit_inc_c
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    if (self.data.F & FLAG_C) == 0 {
                                        // JP(l_exit_inc_c);
                                        self.assert_pc(0x6ac7);
                                        //                              l_exit_inc_c
                                        //         ram:6ac7 0c              INC        C
                                        self.instr_hk__INC_C();
                                        //         ram:6ac8 c9              RET
                                        self.assert_pc(0x6ac8);
                                        true
                                    } else {
                                        //         ram:6aa8 c9              RET
                                        self.assert_pc(0x6aa8);
                                        true
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // fn hook_6acb(&mut self) -> bool {
    //     //
    //     //                              **************************************************************
    //     //                              *                          FUNCTION                          *
    //     //                              **************************************************************
    //     //                              ()
    //     //              undefined         A:1            <RETURN>
    //     //                              FUN_ram_6acb
    //     //         ram:6acb af              XOR        A
    //     self.instr_hk__XOR_A_A();
    //     //         ram:6acc 32 08 c2        LD         (BYTE_ram_c208),A
    //     self.instr_hk__LD_iNNNN_A(0xc208);
    //     //         ram:6acf cd 4a 6b        CALL       FUN_ram_6b4a                                     ()
    //     assert!(self.call_hook(0x6b4a));
    //     //         ram:6ad2 21 6b c3        LD         HL,BYTE_ram_c36b
    //     self.instr_hk__LD_HL_NNNN(0xc36b);
    //     //         ram:6ad5 06 04           LD         B,0x4
    //     self.instr_hk__LD_B_NN(0x4);
    //     //                              loop_1
    //     loop {
    //         //         ram:6ad7 c5              PUSH       BC
    //         self.instr_hk__PUSH_BC();
    //         //         ram:6ad8 e5              PUSH       HL=>BYTE_ram_c36b
    //         self.instr_hk__PUSH_HL();
    //         //         ram:6ad9 7e              LD         A,(HL=>BYTE_ram_c36b)
    //         self.instr_hk__LD_A_iHL();
    //         //         ram:6ada cd 41 6c        CALL       FUN_ram_6c41                                     ()
    //         assert!(self.call_hook(0x6c41));
    //         //         ram:6add e1              POP        HL
    //         self.instr_hk__POP_HL();
    //         //         ram:6ade 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:6adf c1              POP        BC
    //         self.instr_hk__POP_BC();
    //         //         ram:6ae0 10 f5           DJNZ       loop_1
    //         self.IncPC(2);
    //         self.decB();
    //         if self.data.B != 0 {
    //             self.increase_cycles(13);
    //             //JP loop_1;
    //         } else {
    //             self.increase_cycles(8);
    //             break;
    //         }
    //     }

    //     //         ram:6ae2 3a 3e c2        LD         A,(DAT_ram_c23e)
    //     self.instr_hk__LD_A_iNNNN(0xc23e);
    //     //         ram:6ae5 b7              OR         A
    //     self.instr_hk__OR_A_A();
    //     //         ram:6ae6 20 22           JR         NZ,LAB_ram_6b0a
    //     self.IncPC(2);
    //     if (self.data.F & FLAG_Z) == 0 {
    //         self.increase_cycles(12);
    //         // JR(LAB_ram_6b0a);
    //     } else {
    //         self.increase_cycles(7);
    //         //         ram:6ae8 21 da c8        LD         HL,BYTE_ram_c8da
    //         self.instr_hk__LD_HL_NNNN(0xc8da);
    //         //         ram:6aeb af              XOR        A
    //         self.instr_hk__XOR_A_A();
    //         //                              loop_2
    //         loop {
    //             //         ram:6aec 32 b7 c8        LD         (BYTE_ram_c8b7),A
    //             self.instr_hk__LD_iNNNN_A(0xc8b7);
    //             //         ram:6aef 7e              LD         A,(HL=>BYTE_ram_c8da)
    //             self.instr_hk__LD_A_iHL();
    //             //         ram:6af0 3c              INC        A
    //             self.instr_hk__INC_A();
    //             //         ram:6af1 28 0b           JR         Z,LAB_ram_6afe
    //             self.IncPC(2);
    //             if (self.data.F & FLAG_Z) != 0 {
    //                 self.increase_cycles(12);
    //                 // JR(LAB_ram_6afe);
    //             } else {
    //                 self.increase_cycles(7);
    //                 //         ram:6af3 e5              PUSH       HL=>BYTE_ram_c8da
    //                 self.instr_hk__PUSH_HL();
    //                 //         ram:6af4 cd c6 47        CALL       sb_set_addr_base_47C6                            ()
    //                 assert!(self.call_hook(0x47c6));
    //                 //         ram:6af7 2a b8 c8        LD         HL,(BYTE_ram_c8b8)
    //                 self.instr_hk__LD_HL_iNNNN(0xc8b8);
    //                 //         ram:6afa cd 92 8a        CALL       sb_set_vmem_guess_8A92                           ()
    //                 assert!(self.call_hook(0x8A92));
    //                 //         ram:6afd e1              POP        HL
    //                 self.instr_hk__POP_HL();
    //             }

    //             //                              LAB_ram_6afe
    //             //         ram:6afe 23              INC        HL
    //             self.instr_hk__INC_HL();
    //             //         ram:6aff 23              INC        HL
    //             self.instr_hk__INC_HL();
    //             //         ram:6b00 23              INC        HL
    //             self.instr_hk__INC_HL();
    //             //         ram:6b01 23              INC        HL
    //             self.instr_hk__INC_HL();
    //             //         ram:6b02 3a b7 c8        LD         A,(BYTE_ram_c8b7)
    //             self.instr_hk__LD_A_iNNNN(0xc8b7);
    //             //         ram:6b05 3c              INC        A
    //             self.instr_hk__INC_A();
    //             //         ram:6b06 fe 08           CP         0x8
    //             self.instr_hk__CP_NN(0x8);
    //             //         ram:6b08 20 e2           JR         NZ,loop_2
    //             self.IncPC(2);
    //             if (self.data.F & FLAG_Z) == 0 {
    //                 self.increase_cycles(12);
    //                 // JR(loop_2);
    //             } else {
    //                 self.increase_cycles(7);
    //                 break;
    //             }
    //         }
    //     }

    //     //                              LAB_ram_6b0a
    //     //         ram:6b0a cd 24 88        CALL       FUN_ram_8824                                     ()
    //     assert!(self.call_hook(0x8824));
    //     //         ram:6b0d 3a 08 c2        LD         A,(BYTE_ram_c208)
    //     self.instr_hk__LD_A_iNNNN(0xc208);
    //     //         ram:6b10 b7              OR         A
    //     self.instr_hk__OR_A_A();
    //     //         ram:6b11 ca 30 6b        JP         Z,LAB_ram_6b30
    //     self.IncPC(3);
    //     self.increase_cycles(10);
    //     if (self.data.F & FLAG_Z) != 0 {
    //         // JP(LAB_ram_6b30);
    //     } else {
    //         //         ram:6b14 af              XOR        A
    //         self.instr_hk__XOR_A_A();
    //         //                              loop_3
    //         loop {
    //             //         ram:6b15 32 1b c2        LD         (bt_player_idx_c21b),A
    //             self.instr_hk__LD_iNNNN_A(0xc21b);
    //             //         ram:6b18 cd a5 70        CALL       FUN_ram_70a5                                     ()
    //             assert!(self.call_hook(0x70a5));
    //             //         ram:6b1b 3a 1b c2        LD         A,(bt_player_idx_c21b)
    //             self.instr_hk__LD_A_iNNNN(0xc21b);
    //             //         ram:6b1e 3c              INC        A
    //             self.instr_hk__INC_A();
    //             //         ram:6b1f fe 04           CP         0x4
    //             self.instr_hk__CP_NN(0x4);
    //             //         ram:6b21 20 f2           JR         NZ,loop_3
    //             self.IncPC(2);
    //             if (self.data.F & FLAG_Z) == 0 {
    //                 self.increase_cycles(12);
    //                 // JR(loop_3);
    //             } else {
    //                 self.increase_cycles(7);
    //                 break;
    //             }
    //         }

    //         //         ram:6b23 21 fe c1        LD         HL,BYTE_ram_c1fe
    //         self.instr_hk__LD_HL_NNNN(0xc1fe);
    //         //         ram:6b26 11 ff c1        LD         DE,BYTE_ram_c1ff
    //         self.instr_hk__LD_DE_NNNN(0xc1ff);
    //         //         ram:6b29 01 05 00        LD         BC,0x5
    //         self.instr_hk__LD_BC_NNNN(0x5);
    //         //         ram:6b2c 36 00           LD         (HL=>BYTE_ram_c1fe),0x0
    //         self.instr_hk__LD_iHL_NN(0x0);
    //         //         ram:6b2e ed b0           LDIR
    //         self.instr_hk__LDIR();
    //     }

    //     //                              LAB_ram_6b30                                         FUN_ram_6acb:6b11(j)
    //     //         ram:6b30 af              XOR        A
    //     self.instr_hk__XOR_A_A();
    //     //         ram:6b31 21 fe c1        LD         HL,BYTE_ram_c1fe
    //     self.instr_hk__LD_HL_NNNN(0xc1fe);
    //     //                              loop_4
    //     loop {
    //         //         ram:6b34 32 1b c2        LD         (bt_player_idx_c21b),A
    //         self.instr_hk__LD_iNNNN_A(0xc21b);
    //         //         ram:6b37 56              LD         D,(HL=>BYTE_ram_c1fe)
    //         self.instr_hk__LD_D_iHL();
    //         //         ram:6b38 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:6b39 5e              LD         E,(HL=>BYTE_ram_c1ff)
    //         self.instr_hk__LD_E_iHL();
    //         //         ram:6b3a 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:6b3b e5              PUSH       HL
    //         self.instr_hk__PUSH_HL();
    //         //         ram:6b3c cd d6 6e        CALL       FUN_ram_6ed6                                     ()
    //         assert!(self.call_hook(0x6ed6));
    //         //         ram:6b3f e1              POP        HL
    //         self.instr_hk__POP_HL();
    //         //         ram:6b40 3a 1b c2        LD         A,(bt_player_idx_c21b)
    //         self.instr_hk__LD_A_iNNNN(0xc21b);
    //         //         ram:6b43 3c              INC        A
    //         self.instr_hk__INC_A();
    //         //         ram:6b44 fe 03           CP         0x3
    //         self.instr_hk__CP_NN(0x3);
    //         //         ram:6b46 c2 34 6b        JP         NZ,loop_4
    //         self.IncPC(3);
    //         self.increase_cycles(10);
    //         if (self.data.F & FLAG_Z) == 0 {
    //             // JP(loop_4);
    //         } else {
    //             break;
    //         }
    //     }

    //     self.assert_pc(0x6b49);
    //     //         ram:6b49 c9              RET
    //     return true;
    //     //
    //     // true
    // }
    // fn hook_6b23(&mut self) -> bool {
    //     //                              **************************************************************
    //     //                              *                          FUNCTION                          *
    //     //                              **************************************************************
    //     //                              ()
    //     //              undefined         A:1            <RETURN>
    //     //                              FUN_ram_6b23
    //     //         ram:6b23 21 fe c1        LD         HL,BYTE_ram_c1fe
    //     self.instr_hk__LD_HL_NNNN(0xc1fe);
    //     //         ram:6b26 11 ff c1        LD         DE,BYTE_ram_c1ff
    //     self.instr_hk__LD_DE_NNNN(0xc1ff);
    //     //         ram:6b29 01 05 00        LD         BC,0x5
    //     self.instr_hk__LD_BC_NNNN(0x5);
    //     //         ram:6b2c 36 00           LD         (HL=>BYTE_ram_c1fe),0x0
    //     self.instr_hk__LD_iHL_NN(0x0);
    //     //         ram:6b2e ed b0           LDIR
    //     self.instr_hk__LDIR();
    //     //                              LAB_ram_6b30                                         FUN_ram_6acb:6b11(j)
    //     //         ram:6b30 af              XOR        A
    //     self.instr_hk__XOR_A_A();
    //     //         ram:6b31 21 fe c1        LD         HL,BYTE_ram_c1fe
    //     self.instr_hk__LD_HL_NNNN(0xc1fe);
    //     //                              loop_4
    //     loop {
    //         //         ram:6b34 32 1b c2        LD         (bt_player_idx_c21b),A
    //         self.instr_hk__LD_iNNNN_A(0xc21b);
    //         //         ram:6b37 56              LD         D,(HL=>BYTE_ram_c1fe)
    //         self.instr_hk__LD_D_iHL();
    //         //         ram:6b38 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:6b39 5e              LD         E,(HL=>BYTE_ram_c1ff)
    //         self.instr_hk__LD_E_iHL();
    //         //         ram:6b3a 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:6b3b e5              PUSH       HL
    //         self.instr_hk__PUSH_HL();
    //         //         ram:6b3c cd d6 6e        CALL       FUN_ram_6ed6                                     ()
    //         assert!(self.call_hook(0x6ed6));
    //         //         ram:6b3f e1              POP        HL
    //         self.instr_hk__POP_HL();
    //         //         ram:6b40 3a 1b c2        LD         A,(bt_player_idx_c21b)
    //         self.instr_hk__LD_A_iNNNN(0xc21b);
    //         //         ram:6b43 3c              INC        A
    //         self.instr_hk__INC_A();
    //         //         ram:6b44 fe 03           CP         0x3
    //         self.instr_hk__CP_NN(0x3);
    //         //         ram:6b46 c2 34 6b        JP         NZ,loop_4
    //         self.IncPC(3);
    //         self.increase_cycles(10);
    //         if (self.data.F & FLAG_Z) == 0 {
    //             // JP(loop_4);
    //         } else {
    //             break;
    //         }
    //     }

    //     self.assert_pc(0x6b49);
    //     //         ram:6b49 c9              RET
    //     return true;
    //     //
    // }
    pub(crate) fn hook_6b4a(&mut self) -> bool {
        //                              **************************************************************
        //                              *                          FUNCTION                          *
        //                              **************************************************************
        //                              ()
        //              undefined         A:1            <RETURN>
        //                              FUN_ram_6b4a

        //         ram:6b4a af              XOR        A
        self.instr_hk__XOR_A_A();
        //                              loop_1                                               FUN_ram_6be0:6c2e(j)
        loop {
            // loop_1
            self.SetPC(0x6b4b);

            //         ram:6b4b 32 1b c2        LD         (bt_player_idx_c21b),A
            self.instr_hk__LD_iNNNN_A(0xc21b);
            //         ram:6b4e cd 6b 47        CALL       sb_get_current_char_spell_addr_guess_476b        ;
            assert!(self.call_hook(0x476b));

            //         ram:6b51 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:6b52 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:6b53 ca 28 6c        JP         Z,FUN_ram_6be0::loop_1_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(FUN_ram_6be0::loop_1_chk_cond);
                // JP(loop_1_chk_cond);
            } else {
                //         ram:6b56 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:6b57 56              LD         D,(HL)
                self.instr_hk__LD_D_iHL();
                //         ram:6b58 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:6b59 5e              LD         E,(HL)
                self.instr_hk__LD_E_iHL();
                //         ram:6b5a eb              EX         DE,HL
                self.instr_hk__EX_DE_HL();
                //         ram:6b5b 22 10 c2        LD         (WORD_ram_c210),HL
                self.instr_hk__LD_iNNNN_HL(0xc210);
                //         ram:6b5e cd bc 89        CALL       fn_calc_voffset_89BC
                assert!(self.call_hook(0x89bc));

                //         ram:6b61 11 9a c9        LD         DE,DAT_ram_c99a
                self.instr_hk__LD_DE_NNNN(0xc99a);
                //         ram:6b64 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:6b65 22 12 c2        LD         (WORD_ram_c212),HL
                self.instr_hk__LD_iNNNN_HL(0xc212);
                //         ram:6b68 01 00 00        LD         BC,0x0
                self.instr_hk__LD_BC_NNNN(0x0);
                //         ram:6b6b 50              LD         D,B
                self.instr_hk__LD_D_B();
                //         ram:6b6c cd 81 6a        CALL       sb_change_bcd_6A81
                assert!(self.call_hook(0x6a81));

                //         ram:6b6f 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:6b70 cd 81 6a        CALL       sb_change_bcd_6A81
                assert!(self.call_hook(0x6a81));

                //         ram:6b73 d5              PUSH       DE
                self.instr_hk__PUSH_DE();
                //         ram:6b74 11 1f 00        LD         DE,0x1f
                self.instr_hk__LD_DE_NNNN(0x1f);
                //         ram:6b77 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:6b78 d1              POP        DE
                self.instr_hk__POP_DE();
                //         ram:6b79 cd 81 6a        CALL       sb_change_bcd_6A81
                assert!(self.call_hook(0x6a81));

                //         ram:6b7c 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:6b7d cd 81 6a        CALL       sb_change_bcd_6A81
                assert!(self.call_hook(0x6a81));

                //         ram:6b80 7a              LD         A,D
                self.instr_hk__LD_A_D();
                //         ram:6b81 b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:6b82 28 49           JR         Z,LAB_ram_6bcd
                self.IncPC(2);
                if (self.data.F & FLAG_Z) != 0 {
                    self.increase_cycles(12);
                    // JR(LAB_ram_6bcd);

                    //                              LAB_ram_6bcd
                    //         ram:6bcd 79              LD         A,C
                    self.instr_hk__LD_A_C();
                    //         ram:6bce b7              OR         A
                    self.instr_hk__OR_A_A();
                    //         ram:6bcf 28 15           JR         Z,LAB_ram_6be6
                    self.IncPC(2);
                    if (self.data.F & FLAG_Z) != 0 {
                        self.increase_cycles(12);
                        // JR(LAB_ram_6be6);

                        //                              LAB_ram_6be6                                         FUN_ram_6b4a:6bcf(j)
                        //         ram:6be6 3a 1b c2        LD         A,(bt_player_idx_c21b)
                        self.instr_hk__LD_A_iNNNN(0xc21b);
                        //         ram:6be9 fe 03           CP         0x3
                        self.instr_hk__CP_NN(0x3);
                        //         ram:6beb c2 ee 6b        JP         NZ,LAB_ram_6bee
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // if (self.data.F & FLAG_Z) == 0 {
                        //     JP(LAB_ram_6bee);
                        // }

                        //                              LAB_ram_6bee
                        //         ram:6bee cd 6b 47        CALL       sb_get_current_char_spell_addr_guess_476b        ;
                        assert!(self.call_hook(0x476b));

                        //         ram:6bf1 7e              LD         A,(HL)
                        self.instr_hk__LD_A_iHL();
                        //         ram:6bf2 b7              OR         A
                        self.instr_hk__OR_A_A();
                        //         ram:6bf3 20 0a           JR         NZ,LAB_ram_6bff
                        self.IncPC(2);
                        if (self.data.F & FLAG_Z) == 0 {
                            self.increase_cycles(12);
                            // JR(LAB_ram_6bff);

                            //                              LAB_ram_6bff
                            //         ram:6bff fe 01           CP         0x1
                            self.instr_hk__CP_NN(0x1);
                            //         ram:6c01 20 0b           JR         NZ,LAB_ram_6c0e
                            self.IncPC(2);
                            if (self.data.F & FLAG_Z) == 0 {
                                self.increase_cycles(12);
                                // JR(LAB_ram_6c0e);
                                //                              LAB_ram_6c0e
                                //         ram:6c0e fe 02           CP         0x2
                                self.instr_hk__CP_NN(0x2);
                                //         ram:6c10 20 0c           JR         NZ,LAB_ram_6c1e
                                self.IncPC(2);
                                if (self.data.F & FLAG_Z) == 0 {
                                    self.increase_cycles(12);
                                    // JR(LAB_ram_6c1e);

                                    //                              LAB_ram_6c1e
                                    //         ram:6c1e 23              INC        HL
                                    self.instr_hk__INC_HL();
                                    //         ram:6c1f 7e              LD         A,(HL)
                                    self.instr_hk__LD_A_iHL();
                                    //         ram:6c20 3d              DEC        A
                                    self.instr_hk__DEC_A();
                                    //         ram:6c21 ca e0 6b        JP         Z,FUN_ram_6be0
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    if (self.data.F & FLAG_Z) != 0 {
                                        // JP(FUN_ram_6be0);
                                        // 6be0
                                        //         ram:6be0 cd d1 6b        CALL       sb_draw_current_spell_guess_6bd1
                                        assert!(self.call_hook(0x6bd1));
                                        //         ram:6be3 c3 28 6c        JP         loop_1_chk_cond
                                        self.IncPC(3);
                                        self.increase_cycles(10);
                                        // JP(loop_1_chk_cond);
                                    } else {
                                        //         ram:6c24 77              LD         (HL),A
                                        self.instr_hk__LD_iHL_A();
                                        // JR(LAB_ram_6c25);
                                        //                              LAB_ram_6c25

                                        //         ram:6c25 cd a5 70        CALL       FUN_ram_70a5                                     ()
                                        assert!(self.call_hook(0x70a5));
                                        // JR(loop_1_chk_cond);
                                    }
                                    // JP(loop_1_chk_cond);
                                } else {
                                    self.increase_cycles(7);
                                    //         ram:6c12 23              INC        HL
                                    self.instr_hk__INC_HL();
                                    //         ram:6c13 23              INC        HL
                                    self.instr_hk__INC_HL();
                                    //         ram:6c14 7e              LD         A,(HL)
                                    self.instr_hk__LD_A_iHL();
                                    //         ram:6c15 3c              INC        A
                                    self.instr_hk__INC_A();
                                    //         ram:6c16 fe 12           CP         0x12
                                    self.instr_hk__CP_NN(0x12);
                                    //         ram:6c18 ca e0 6b        JP         Z,FUN_ram_6be0
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    if (self.data.F & FLAG_Z) != 0 {
                                        // JP(FUN_ram_6be0);
                                        // 6be0
                                        //         ram:6be0 cd d1 6b        CALL       sb_draw_current_spell_guess_6bd1
                                        assert!(self.call_hook(0x6bd1));
                                        //         ram:6be3 c3 28 6c        JP         loop_1_chk_cond
                                        self.IncPC(3);
                                        self.increase_cycles(10);
                                        // JP(loop_1_chk_cond);
                                    } else {
                                        //         ram:6c1b 77              LD         (HL),A
                                        self.instr_hk__LD_iHL_A();
                                        //         ram:6c1c 18 07           JR         LAB_ram_6c25
                                        self.IncPC(2);
                                        self.increase_cycles(12);
                                        // JR(LAB_ram_6c25);
                                        //                              LAB_ram_6c25

                                        //         ram:6c25 cd a5 70        CALL       FUN_ram_70a5                                     ()
                                        assert!(self.call_hook(0x70a5));
                                        // JR(loop_1_chk_cond);
                                    }
                                    // JR(loop_1_chk_cond);
                                }
                                // JR(loop_1_chk_cond);
                            } else {
                                self.increase_cycles(7);

                                //         ram:6c03 23              INC        HL
                                self.instr_hk__INC_HL();
                                //         ram:6c04 7e              LD         A,(HL)
                                self.instr_hk__LD_A_iHL();
                                //         ram:6c05 3c              INC        A
                                self.instr_hk__INC_A();
                                //         ram:6c06 fe 1e           CP         0x1e
                                self.instr_hk__CP_NN(0x1e);
                                //         ram:6c08 ca e0 6b        JP         Z,FUN_ram_6be0
                                self.IncPC(3);
                                self.increase_cycles(10);
                                if (self.data.F & FLAG_Z) != 0 {
                                    // JP(FUN_ram_6be0);
                                    // 6be0
                                    //         ram:6be0 cd d1 6b        CALL       sb_draw_current_spell_guess_6bd1
                                    assert!(self.call_hook(0x6bd1));
                                    //         ram:6be3 c3 28 6c        JP         loop_1_chk_cond
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    // JP(loop_1_chk_cond);
                                } else {
                                    //         ram:6c0b 77              LD         (HL),A
                                    self.instr_hk__LD_iHL_A();
                                    //         ram:6c0c 18 17           JR         LAB_ram_6c25
                                    self.IncPC(2);
                                    self.increase_cycles(12);
                                    // JR(LAB_ram_6c25);
                                    //                              LAB_ram_6c25

                                    //         ram:6c25 cd a5 70        CALL       FUN_ram_70a5                                     ()
                                    assert!(self.call_hook(0x70a5));
                                    // JR(loop_1_chk_cond);
                                }
                                // JP(loop_1_chk_cond);
                            }
                            // JR(loop_1_chk_cond);
                        } else {
                            self.increase_cycles(7);
                            //         ram:6bf5 23              INC        HL
                            self.instr_hk__INC_HL();
                            //         ram:6bf6 23              INC        HL
                            self.instr_hk__INC_HL();
                            //         ram:6bf7 7e              LD         A,(HL)
                            self.instr_hk__LD_A_iHL();
                            //         ram:6bf8 3d              DEC        A
                            self.instr_hk__DEC_A();
                            //         ram:6bf9 ca e0 6b        JP         Z,FUN_ram_6be0
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_Z) != 0 {
                                // JP(FUN_ram_6be0);
                                // 6be0
                                //         ram:6be0 cd d1 6b        CALL       sb_draw_current_spell_guess_6bd1
                                assert!(self.call_hook(0x6bd1));
                                //         ram:6be3 c3 28 6c        JP         loop_1_chk_cond
                                self.IncPC(3);
                                self.increase_cycles(10);
                                // JP(loop_1_chk_cond);
                            } else {
                                //         ram:6bfc 77              LD         (HL),A
                                self.instr_hk__LD_iHL_A();
                                //         ram:6bfd 18 26           JR         LAB_ram_6c25
                                self.IncPC(2);
                                self.increase_cycles(12);
                                // JR(LAB_ram_6c25);
                                //                              LAB_ram_6c25

                                //         ram:6c25 cd a5 70        CALL       FUN_ram_70a5                                     ()
                                assert!(self.call_hook(0x70a5));
                                // JR(loop_1_chk_cond);
                            }
                            // JP(loop_1_chk_cond);
                        }
                        // JR(loop_1_chk_cond);
                    } else {
                        self.increase_cycles(7);
                        // JR(LAB_ram_6bd1);
                        self.hook_6bd1();
                        return true;
                    }
                } else {
                    self.increase_cycles(7);

                    //         ram:6b84 3a 1b c2        LD         A,(bt_player_idx_c21b)
                    self.instr_hk__LD_A_iNNNN(0xc21b);
                    //         ram:6b87 fe 03           CP         0x3
                    self.instr_hk__CP_NN(0x3);
                    //         ram:6b89 ca e0 6b        JP         Z,FUN_ram_6be0
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_Z) != 0 {
                        // JP(FUN_ram_6be0);
                    } else {
                        //         ram:6b8c 2a 12 c2        LD         HL,(WORD_ram_c212)
                        self.instr_hk__LD_HL_iNNNN(0xc212);
                        //         ram:6b8f 7e              LD         A,(HL)
                        self.instr_hk__LD_A_iHL();
                        //         ram:6b90 d6 40           SUB        0x40
                        self.instr_hk__SUB_NN(0x40);
                        //         ram:6b92 30 13           JR         NC,LAB_ram_6ba7
                        self.IncPC(2);
                        if (self.data.F & FLAG_C) == 0 {
                            self.increase_cycles(12);
                            // JR(LAB_ram_6ba7);
                        } else {
                            self.increase_cycles(7);
                            //         ram:6b94 23              INC        HL
                            self.instr_hk__INC_HL();
                            //         ram:6b95 7e              LD         A,(HL)
                            self.instr_hk__LD_A_iHL();
                            //         ram:6b96 d6 40           SUB        0x40
                            self.instr_hk__SUB_NN(0x40);
                            //         ram:6b98 30 0d           JR         NC,LAB_ram_6ba7
                            self.IncPC(2);
                            if (self.data.F & FLAG_C) == 0 {
                                self.increase_cycles(12);
                                // JR(LAB_ram_6ba7);
                            } else {
                                self.increase_cycles(7);
                                //         ram:6b9a 11 1f 00        LD         DE,0x1f
                                self.instr_hk__LD_DE_NNNN(0x1f);
                                //         ram:6b9d 19              ADD        HL,DE
                                self.instr_hk__ADD_HL_DE();
                                //         ram:6b9e 7e              LD         A,(HL)
                                self.instr_hk__LD_A_iHL();
                                //         ram:6b9f d6 40           SUB        0x40
                                self.instr_hk__SUB_NN(0x40);
                                //         ram:6ba1 30 04           JR         NC,LAB_ram_6ba7
                                self.IncPC(2);
                                if (self.data.F & FLAG_C) == 0 {
                                    self.increase_cycles(12);
                                    // JR(LAB_ram_6ba7);
                                } else {
                                    self.increase_cycles(7);
                                    //         ram:6ba3 23              INC        HL
                                    self.instr_hk__INC_HL();
                                    //         ram:6ba4 7e              LD         A,(HL)
                                    self.instr_hk__LD_A_iHL();
                                    //         ram:6ba5 d6 40           SUB        0x40
                                    self.instr_hk__SUB_NN(0x40);
                                }
                            }
                        }

                        //                              LAB_ram_6ba7

                        //         ram:6ba7 f5              PUSH       AF
                        self.instr_hk__PUSH_AF();
                        //         ram:6ba8 cd ba 6b        CALL       FUN_ram_6bba                                     ()
                        assert!(self.call_hook(0x6bba));
                        //         ram:6bab cd 0a 60        CALL       sb_get_player_addr_600A
                        assert!(self.call_hook(0x600a));
                        //         ram:6bae 22 54 c2        LD         (pt_char_c254),HL
                        self.instr_hk__LD_iNNNN_HL(0xc254);
                        //         ram:6bb1 f1              POP        AF
                        self.instr_hk__POP_AF();
                        //         ram:6bb2 cd 04 44        CALL       FUN_ram_4404                                     ()
                        assert!(self.call_hook(0x4404));
                        //         ram:6bb5 cd ba 6b        CALL       FUN_ram_6bba                                     ()
                        assert!(self.call_hook(0x6bba));
                        //         ram:6bb8 18 26           JR         FUN_ram_6be0                                     ()
                        self.IncPC(2);
                        self.increase_cycles(12);
                        // JR(FUN_ram_6be0);
                    }
                    // JP(FUN_ram_6be0);
                    // 6be0
                    //         ram:6be0 cd d1 6b        CALL       sb_draw_current_spell_guess_6bd1
                    assert!(self.call_hook(0x6bd1));
                    //         ram:6be3 c3 28 6c        JP         loop_1_chk_cond
                    self.IncPC(3);
                    self.increase_cycles(10);
                    // JP(loop_1_chk_cond);
                }
            }

            // // 6be0
            // //         ram:6be0 cd d1 6b        CALL       sb_draw_current_spell_guess_6bd1
            // assert!(self.call_hook(0x6bd1));
            // //         ram:6be3 c3 28 6c        JP         loop_1_chk_cond
            // self.IncPC(3);
            // self.increase_cycles(10);
            // JP(loop_1_chk_cond);

            //                              loop_1_chk_cond                                      FUN_ram_6b4a:6b53(j),

            //         ram:6c28 3a 1b c2        LD         A,(bt_player_idx_c21b)
            self.instr_hk__LD_A_iNNNN(0xc21b);
            //         ram:6c2b 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:6c2c fe 04           CP         0x4
            self.instr_hk__CP_NN(0x4);
            //         ram:6c2e c2 4b 6b        JP         NZ,FUN_ram_6b4a::loop_1
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(FUN_ram_6b4a::loop_1);
            } else {
                break;
            }
            // loop_1
        }

        //         ram:6c31 c9              RET
        // return true;

        true //?
             //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
    }
    pub(crate) fn hook_6bba(&mut self) -> bool {
        //         ram:6bba cd 6b 47        CALL       sb_get_current_char_spell_addr_guess_476b        OUT hl: addr;
        assert!(self.call_hook(0x476b));
        //                                                                                                    [0] -> direction?
        //                                                                                                  b<-0
        //         ram:6bbd 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:6bbe 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:6bbf 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:6bc0 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:6bc1 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:6bc2 d9              EXX
        self.instr_hk__EXX();
        //         ram:6bc3 cd 42 47        CALL       sb_get_power_for_player_4742                     hl <- addr
        assert!(self.call_hook(0x4742));
        //                                                                                              bc <- player_idx
        //         ram:6bc6 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:6bc7 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:6bc8 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:6bc9 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:6bca d9              EXX
        self.instr_hk__EXX();
        //         ram:6bcb 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:6bcc c9              RET
        self.assert_pc(0x6bcc);
        return true;
    }
    pub(crate) fn hook_6bd1(&mut self) -> bool {
        println!("hook_6bd1");
        //         ram:6bd1 cd f7 67        CALL       sb_is_current_char_spell_valid_guess_67F7
        assert!(self.call_hook(0x67f7));

        //         ram:6bd4 c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        }
        self.increase_cycles(5);
        //         ram:6bd5 36 ff           LD         (HL),0xff
        self.instr_hk__LD_iHL_NN(0xff);
        //         ram:6bd7 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:6bda c6 06           ADD        A,0x6
        self.instr_hk__ADD_A_NN(0x6);
        //         ram:6bdc cd 6c 8b        CALL       sb_mem_blit_ram_to_vram_guess_8B6C
        assert!(self.call_hook(0x8b6c));
        //         ram:6bdf c9              RET
        self.assert_pc(0x6bdf);

        true
    }
    pub(crate) fn hook_6ed6(&mut self) -> bool {
        //         ram:6ed6 ed 53 3c c2     LD         (DAT_ram_c23c),DE
        self.instr_hk__LD_iNNNN_DE(0xc23c);
        //         ram:6eda cd 05 47        CALL       sb_read_mem_for_player_4705
        assert!(self.call_hook(0x4705));
        //         ram:6edd 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:6ede b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:6edf c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            self.assert_pc(0x6ee0);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:6ee0 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:6ee3 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:6ee4 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:6ee5 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:6ee6 21 ee c1        LD         HL,BYTE_ram_c1ee
        self.instr_hk__LD_HL_NNNN(0xc1ee);
        //         ram:6ee9 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:6eea e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:6eeb 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:6eec 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:6eed 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:6eee 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:6eef 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:6ef0 3a 3d c2        LD         A,(DAT_ram_c23d)
        self.instr_hk__LD_A_iNNNN(0xc23d);
        //         ram:6ef3 83              ADD        A,E
        self.instr_hk__ADD_A_E();
        //         ram:6ef4 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:6ef5 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:6ef8 0e 01           LD         C,0x1
        self.instr_hk__LD_C_NN(0x1);
        //         ram:6efa d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:6efb f3              DI
        self.instr_hk__DI();
        //         ram:6efc cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21
        assert!(self.call_hook(0x8b21));
        //         ram:6eff d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:6f00 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:6f03 c6 03           ADD        A,0x3
        self.instr_hk__ADD_A_NN(0x3);
        //         ram:6f05 0e 01           LD         C,0x1
        self.instr_hk__LD_C_NN(0x1);
        //         ram:6f07 cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21
        assert!(self.call_hook(0x8b21));
        //         ram:6f0a e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:6f0b 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:6f0c 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:6f0d 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:6f0e 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:6f0f 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:6f10 3d              DEC        A
        self.instr_hk__DEC_A();
        //         ram:6f11 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:6f12 3a 3c c2        LD         A,(DAT_ram_c23c)
        self.instr_hk__LD_A_iNNNN(0xc23c);
        //         ram:6f15 83              ADD        A,E
        self.instr_hk__ADD_A_E();
        //         ram:6f16 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:6f17 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:6f1a 0e 00           LD         C,0x0
        self.instr_hk__LD_C_NN(0x0);
        //         ram:6f1c d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:6f1d cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21
        assert!(self.call_hook(0x8b21));
        //         ram:6f20 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:6f21 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:6f24 c6 03           ADD        A,0x3
        self.instr_hk__ADD_A_NN(0x3);
        //         ram:6f26 0e 00           LD         C,0x0
        self.instr_hk__LD_C_NN(0x0);
        //         ram:6f28 cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21
        assert!(self.call_hook(0x8b21));
        //         ram:6f2b fb              EI
        self.instr_hk__EI();
        //         ram:6f2c c9              RET

        self.assert_pc(0x6f2c);
        true
    }
    pub(crate) fn hook_6f2d(&mut self) -> bool {
        //         ram:6f2d cd  4a  6b       CALL       FUN_ram_6b4a                                     undefined FUN_ram_6b4a()
        assert!(self.call_hook(0x6b4a));
        //         ram:6f30 21  6b  c3       LD         HL,BYTE_ram_c36b
        self.instr_hk__LD_HL_NNNN(0xc36b);
        //         ram:6f33 06  04           LD         B,0x4
        self.instr_hk__LD_B_NN(0x4);
        //                              LAB_ram_6f35                                    XREF[1]:     ram:6f3e (j)
        loop {
            self.SetPC(0x6f35);
            //         ram:6f35 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:6f36 e5              PUSH       HL=>BYTE_ram_c36b
            self.instr_hk__PUSH_HL();
            //         ram:6f37 7e              LD         A,(HL=>BYTE_ram_c36b )
            self.instr_hk__LD_A_iHL();
            //         ram:6f38 cd  48  6f       CALL       FUN_ram_6f48                                     undefined FUN_ram_6f48()
            assert!(self.call_hook(0x6f48));
            //         ram:6f3b e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:6f3c 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:6f3d c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:6f3e 10  f5           DJNZ       LAB_ram_6f35
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_6f35;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:6f40 3a  08  c2       LD         A,(BYTE_ram_c208 )
        self.instr_hk__LD_A_iNNNN(0xc208);
        //         ram:6f43 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:6f44 cc  23  6b       CALL       Z,FUN_ram_6b23                                   undefined FUN_ram_6b23()
        assert!(self.call_hook(0x6b23));
        //         ram:6f47 c9              RET
        self.assert_pc(0x6f47);
        // return true;
        //
        true
    }
    pub(crate) fn hook_6f48(&mut self) -> bool {
        //
        //                              *************************************************************
        //                              *                           FUNCTION
        //                              *************************************************************
        //                              undefined  FUN_ram_6f48 ()
        //              undefined         A:1            <RETURN>
        //                              FUN_ram_6f48                                    XREF[1]:     FUN_ram_6f2d:6f38 (c)
        //         ram:6f48 fe  03           CP         0x3                                              IN a <- player_idx cand
        self.instr_hk__CP_NN(0x3);
        //         ram:6f4a c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:6f4b 32  1b  c2       LD         (bt_player_idx_c21b ),A
        self.instr_hk__LD_iNNNN_A(0xc21b);
        //         ram:6f4e cd  0a  60       CALL       sb_get_player_addr_600A                          hl <- c100 + 19h * player_idx
        assert!(self.call_hook(0x600a));
        //         ram:6f51 22  54  c2       LD         (pt_char_c254 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc254);
        //         ram:6f54 cd  51  47       CALL       sb_get_addr_for_player_4751                      undefined sb_get_addr_for_player
        assert!(self.call_hook(0x4751));
        //         ram:6f57 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:6f58 fe  10           CP         0x10
        self.instr_hk__CP_NN(0x10);
        //         ram:6f5a ca  3e  72       JP         Z,LAB_ram_723e
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            JP(LAB_ram_723e);
        }

        //         ram:6f5d fe  11           CP         0x11
        self.instr_hk__CP_NN(0x11);
        //         ram:6f5f ca  45  71       JP         Z,LAB_ram_7145
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP(LAB_ram_7145);
        } else {
            //         ram:6f62 fe  12           CP         0x12
            self.instr_hk__CP_NN(0x12);
            //         ram:6f64 ca  cd  70       JP         Z,LAB_ram_70cd
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(LAB_ram_70cd);
                //                              LAB_ram_70cd                                    XREF[1]:     ram:6f64 (j)
                //         ram:70cd 3a  a9  ca       LD         A,(DAT_ram_caa9 )
                self.instr_hk__LD_A_iNNNN(0xcaa9);
                //         ram:70d0 b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:70d1 ca  40  71       JP         Z,LAB_ram_7140
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) != 0 {
                } else {
                    //         ram:70d4 af              XOR        A
                    self.instr_hk__XOR_A_A();
                    //         ram:70d5 cd  da  46       CALL       sb_find_empty_item_slot_46da                     OUT CF: set
                    assert!(self.call_hook(0x46da));
                    //                                                                                                  hl: addr for empty pos
                    //                                                                                                  CF: unset if item bag is full
                    //         ram:70d8 d2  40  71       JP         NC,LAB_ram_7140
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) == 0 {
                    } else {
                        //         ram:70db 22  50  c2       LD         (wd_l_c250 ),HL
                        self.instr_hk__LD_iNNNN_HL(0xc250);
                        //         ram:70de cd  4b  85       CALL       sb_get_item_addr_854B                            undefined sb_get_item_addr_854B()
                        assert!(self.call_hook(0x854b));
                        //         ram:70e1 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:70e2 5e              LD         E,(HL)
                        self.instr_hk__LD_E_iHL();
                        //         ram:70e3 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:70e4 56              LD         D,(HL)
                        self.instr_hk__LD_D_iHL();
                        //         ram:70e5 21  0a  00       LD         HL,0xa
                        self.instr_hk__LD_HL_NNNN(0xa);
                        //         ram:70e8 cd  a9  b7       CALL       sb_multiply_guess_B7A9                           hl <- hl * de ?
                        assert!(self.call_hook(0xb7a9));
                        //         ram:70eb 22  52  c2       LD         (DAT_ram_c252 ),HL
                        self.instr_hk__LD_iNNNN_HL(0xc252);
                        //         ram:70ee 2a  54  c2       LD         HL,(pt_char_c254 )
                        self.instr_hk__LD_HL_iNNNN(0xc254);
                        //         ram:70f1 01  0b  00       LD         BC,char_0bh_
                        self.instr_hk__LD_BC_NNNN(0x0b);
                        //         ram:70f4 09              ADD        HL,BC
                        self.instr_hk__ADD_HL_BC();
                        //         ram:70f5 3a  52  c2       LD         A,(DAT_ram_c252 )
                        self.instr_hk__LD_A_iNNNN(0xc252);
                        //         ram:70f8 47              LD         B,A
                        self.instr_hk__LD_B_A();
                        //         ram:70f9 7e              LD         A,(HL)
                        self.instr_hk__LD_A_iHL();
                        //         ram:70fa 90              SUB        B
                        self.instr_hk__SUB_A_B();
                        //         ram:70fb 32  b1  c7       LD         (DAT_ram_c7b1 ),A
                        self.instr_hk__LD_iNNNN_A(0xc7b1);
                        //         ram:70fe 3a  53  c2       LD         A,(DAT_ram_c253 )
                        self.instr_hk__LD_A_iNNNN(0xc253);
                        //         ram:7101 47              LD         B,A
                        self.instr_hk__LD_B_A();
                        //         ram:7102 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:7103 7e              LD         A,(HL)
                        self.instr_hk__LD_A_iHL();
                        //         ram:7104 98              SBC        A,B
                        self.instr_hk__SBC_A_B();
                        //         ram:7105 32  b2  c7       LD         (DAT_ram_c7b2 ),A
                        self.instr_hk__LD_iNNNN_A(0xc7b2);
                        //         ram:7108 2b              DEC        HL
                        self.instr_hk__DEC_HL();
                        //         ram:7109 2b              DEC        HL
                        self.instr_hk__DEC_HL();
                        //         ram:710a 7e              LD         A,(HL)
                        self.instr_hk__LD_A_iHL();
                        //         ram:710b de  00           SBC        A,0x0
                        self.instr_hk__SBC_A_NN(0x0);
                        //         ram:710d da  40  71       JP         C,LAB_ram_7140
                        self.IncPC(3);
                        self.increase_cycles(10);
                        if (self.data.F & FLAG_C) != 0 {
                            // JP(LAB_ram_7140);
                        } else {
                            //         ram:7110 32  b0  c7       LD         (bt_buffer_c7b0 ),A
                            self.instr_hk__LD_iNNNN_A(0xc7b0);
                            //         ram:7113 eb              EX         DE,HL
                            self.instr_hk__EX_DE_HL();
                            //         ram:7114 21  b0  c7       LD         HL,bt_buffer_c7b0
                            self.instr_hk__LD_HL_NNNN(0xc7b0);
                            //         ram:7117 01  03  00       LD         BC,0x3
                            self.instr_hk__LD_BC_NNNN(0x3);
                            //         ram:711a ed  b0           LDIR
                            self.instr_hk__LDIR();
                            //         ram:711c cd  4b  85       CALL       sb_get_item_addr_854B                            undefined sb_get_item_addr_854B()
                            assert!(self.call_hook(0x854b));
                            //         ram:711f 7e              LD         A,(HL=>DAT_ram_c7b1 )
                            self.instr_hk__LD_A_iHL();
                            //         ram:7120 2a  50  c2       LD         HL,(wd_l_c250 )
                            self.instr_hk__LD_HL_iNNNN(0xc250);
                            //         ram:7123 77              LD         (HL),A
                            self.instr_hk__LD_iHL_A();
                            //         ram:7124 cd  d2  75       CALL       FUN_ram_75d2                                     undefined FUN_ram_75d2()
                            assert!(self.call_hook(0x75d2));
                            //         ram:7127 af              XOR        A
                            self.instr_hk__XOR_A_A();
                            //         ram:7128 32  a9  ca       LD         (DAT_ram_caa9 ),A
                            self.instr_hk__LD_iNNNN_A(0xcaa9);
                            //         ram:712b 11  08  0f       LD         DE,0xf08
                            self.instr_hk__LD_DE_NNNN(0xf08);
                            //         ram:712e 06  02           LD         B,0x2
                            self.instr_hk__LD_B_NN(0x2);
                            //         ram:7130 cd  db  60       CALL       sb_print_spaces_60db                             IN b: cnt
                            assert!(self.call_hook(0x60db));
                            //                                                                                                  de: xy
                            //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
                            //         ram:7133 11  09  0f       LD         DE,0xf09
                            self.instr_hk__LD_DE_NNNN(0xf09);
                            //         ram:7136 06  02           LD         B,0x2
                            self.instr_hk__LD_B_NN(0x2);
                            //         ram:7138 cd  db  60       CALL       sb_print_spaces_60db                             IN b: cnt
                            assert!(self.call_hook(0x60db));
                            //                                                                                                  de: xy
                            //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
                            //         ram:713b 16  08           LD         D,0x8
                            self.instr_hk__LD_D_NN(0x8);
                            //         ram:713d cd  37  7b       CALL       fn_psg_7b37                                      undefined fn_psg_7b37(void)
                            assert!(self.call_hook(0x7b37));
                            // JP(LAB_ram_7140);
                        }
                    }
                }
                // JP(LAB_ram_7140);
            } else {
                //         ram:6f67 fe  13           CP         0x13
                self.instr_hk__CP_NN(0x13);
                //         ram:6f69 ca  1f  70       JP         Z,LAB_ram_701f
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) != 0 {
                    // JP(LAB_ram_701f);

                    //                              LAB_ram_701f                                    XREF[1]:     ram:6f69 (j)
                    //         ram:701f 16  09           LD         D,0x9
                    self.instr_hk__LD_D_NN(0x9);
                    //         ram:7021 cd  37  7b       CALL       fn_psg_7b37                                      undefined fn_psg_7b37(void)
                    assert!(self.call_hook(0x7b37));
                    //         ram:7024 cd  4c  47       CALL       sb_read_mem_for_player_474c                      undefined sb_read_mem_for_player
                    assert!(self.call_hook(0x474c));
                    //         ram:7027 7e              LD         A,(HL)
                    self.instr_hk__LD_A_iHL();
                    //         ram:7028 32  0d  c2       LD         (BYTE_ram_c20d ),A
                    self.instr_hk__LD_iNNNN_A(0xc20d);
                    //         ram:702b 3e  08           LD         A,0x8
                    self.instr_hk__LD_A_NN(0x8);
                    //         ram:702d 32  0e  c2       LD         (BYTE_ram_c20e ),A
                    self.instr_hk__LD_iNNNN_A(0xc20e);
                    //         ram:7030 af              XOR        A
                    self.instr_hk__XOR_A_A();
                    //         ram:7031 32  0f  c2       LD         (bt_spell_kind_c20f ),A
                    self.instr_hk__LD_iNNNN_A(0xc20f);
                    //         ram:7034 cd  3a  70       CALL       sb_draw_spell_sprite_703A                        undefined sb_draw_spell_sprite_7
                    assert!(self.call_hook(0x703a));
                    //         ram:7037 c3  40  71       JP         LAB_ram_7140
                    self.IncPC(3);
                    self.increase_cycles(10);
                    // JP(LAB_ram_7140);
                } else {
                    //         ram:6f6c fe  15           CP         0x15
                    self.instr_hk__CP_NN(0x15);
                    //         ram:6f6e ca  1c  70       JP         Z,LAB_ram_701c
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_Z) != 0 {
                        // JP(LAB_ram_701c);
                        //                              LAB_ram_701c                                    XREF[1]:     ram:6f6e (j)
                        //         ram:701c c3  40  71       JP         LAB_ram_7140
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP(LAB_ram_7140);
                    } else {
                        //         ram:6f71 fe  05           CP         0x5
                        self.instr_hk__CP_NN(0x5);
                        //         ram:6f73 d0              RET        NC
                        self.IncPC(1);
                        if (self.data.F & FLAG_C) == 0 {
                            self.increase_cycles(11);
                            return true;
                        } else {
                            self.increase_cycles(5);
                            //         ram:6f74 b7              OR         A
                            self.instr_hk__OR_A_A();
                            //         ram:6f75 c8              RET        Z
                            self.IncPC(1);
                            if (self.data.F & FLAG_Z) != 0 {
                                self.increase_cycles(11);
                                return true;
                            } else {
                                self.increase_cycles(5);
                                //                              LAB_ram_6f76                                    XREF[6]:     ram:7148 (j) , ram:7204 (j) ,
                                //                                                                                           ram:7209 (j) , ram:7241 (j) ,
                                //                                                                                           ram:7292 (j) , ram:7332 (j)
                                //         ram:6f76 cd  ea  46       CALL       sb_read_mem_for_player_46ea                      OUT d,e
                                assert!(self.call_hook(0x46ea));
                                //         ram:6f79 eb              EX         DE,HL
                                self.instr_hk__EX_DE_HL();
                                //         ram:6f7a cd  bc  89       CALL       fn_calc_voffset_89BC                             hl <- (hl & 0xff) * 32 + (hl >>
                                assert!(self.call_hook(0x89bc));
                                //                                                                                              de <- (hl >> 8)
                                //         ram:6f7d 11  9a  c9       LD         DE,DAT_ram_c99a
                                self.instr_hk__LD_DE_NNNN(0xc99a);
                                //         ram:6f80 19              ADD        HL,DE
                                self.instr_hk__ADD_HL_DE();
                                //         ram:6f81 7e              LD         A,(HL=>DAT_ram_ceb8 )
                                self.instr_hk__LD_A_iHL();
                                //         ram:6f82 fe  28           CP         0x28
                                self.instr_hk__CP_NN(0x28);
                                //         ram:6f84 c2  b6  6f       JP         NZ,LAB_ram_6fb6
                                self.IncPC(3);
                                self.increase_cycles(10);
                                if (self.data.F & FLAG_Z) == 0 {
                                    // JP(LAB_ram_6fb6);
                                    //                              LAB_ram_6fb6                                    XREF[1]:     ram:6f84 (j)
                                    //         ram:6fb6 fe  2c           CP         0x2c
                                    self.instr_hk__CP_NN(0x2c);
                                    //         ram:6fb8 c2  c1  6f       JP         NZ,LAB_ram_6fc1
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    if (self.data.F & FLAG_Z) == 0 {
                                        // JP(LAB_ram_6fc1);

                                        //                              LAB_ram_6fc1                                    XREF[1]:     ram:6fb8 (j)
                                        //         ram:6fc1 fe  30           CP         0x30
                                        self.instr_hk__CP_NN(0x30);
                                        //         ram:6fc3 c2  1b  70       JP         NZ,l_exit_701b
                                        self.IncPC(3);
                                        self.increase_cycles(10);
                                        if (self.data.F & FLAG_Z) == 0 {
                                            // JP(l_exit_701b);
                                            //                              l_exit_701b                                     XREF[2]:     ram:6fc3 (j) , ram:6ff2 (j)
                                            //         ram:701b c9              RET
                                            return true;
                                        }

                                        //         ram:6fc6 21  16  c2       LD         HL,BYTE_ram_c216
                                        self.instr_hk__LD_HL_NNNN(0xc216);
                                    } else {
                                        //         ram:6fbb 21  15  c2       LD         HL,BYTE_ram_c215
                                        self.instr_hk__LD_HL_NNNN(0xc215);
                                        //         ram:6fbe c3  c9  6f       JP         LAB_ram_6fc9
                                        self.IncPC(3);
                                        self.increase_cycles(10);
                                    }
                                    // JP(LAB_ram_6fc9);
                                    //                              LAB_ram_6fc9                                    XREF[1]:     ram:6fbe (j)
                                    //         ram:6fc9 7e              LD         A,(HL=>BYTE_ram_c215 )
                                    self.instr_hk__LD_A_iHL();
                                    //         ram:6fca fe  34           CP         0x34
                                    self.instr_hk__CP_NN(0x34);
                                    //         ram:6fcc da  eb  6f       JP         C,LAB_ram_6feb
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    if (self.data.F & FLAG_C) != 0 {
                                        // JP(LAB_ram_6feb);
                                        //                              LAB_ram_6feb                                    XREF[1]:     ram:6fcc (j)
                                        //         ram:6feb e5              PUSH       HL=>BYTE_ram_c215
                                        self.instr_hk__PUSH_HL();
                                        //         ram:6fec af              XOR        A
                                        self.instr_hk__XOR_A_A();
                                        //         ram:6fed cd  da  46       CALL       sb_find_empty_item_slot_46da                     OUT CF: set
                                        assert!(self.call_hook(0x46da));
                                        //                                                                                                  hl: addr for empty pos
                                        //                                                                                                  CF: unset if item bag is full
                                        //         ram:6ff0 eb              EX         DE,HL
                                        self.instr_hk__EX_DE_HL();
                                        //         ram:6ff1 e1              POP        HL
                                        self.instr_hk__POP_HL();
                                        //         ram:6ff2 d2  1b  70       JP         NC,l_exit_701b
                                        self.IncPC(3);
                                        self.increase_cycles(10);
                                        if (self.data.F & FLAG_C) == 0 {
                                            // JP(l_exit_701b);
                                            //                              l_exit_701b                                     XREF[2]:     ram:6fc3 (j) , ram:6ff2 (j)
                                            //         ram:701b c9              RET
                                            return true;
                                        }

                                        //         ram:6ff5 7e              LD         A,(HL=>BYTE_ram_c215 )
                                        self.instr_hk__LD_A_iHL();
                                        //         ram:6ff6 12              LD         (DE=>BYTE_ram_c215 ),A
                                        self.instr_hk__LD_iDE_A();
                                        //         ram:6ff7 36  00           LD         (HL=>BYTE_ram_c215 ),0x0
                                        self.instr_hk__LD_iHL_NN(0x0);
                                        //         ram:6ff9 cd  d2  75       CALL       FUN_ram_75d2                                     undefined FUN_ram_75d2()
                                        assert!(self.call_hook(0x75d2));
                                    } else {
                                        //         ram:6fcf 36  00           LD         (HL=>BYTE_ram_c215 ),0x0
                                        self.instr_hk__LD_iHL_NN(0x0);
                                        //         ram:6fd1 d6  34           SUB        0x34
                                        self.instr_hk__SUB_NN(0x34);
                                        //         ram:6fd3 0e  01           LD         C,0x1
                                        self.instr_hk__LD_C_NN(0x1);
                                        //                              LAB_ram_6fd5                                    XREF[1]:     ram:6fdb (j)
                                        loop {
                                            //         ram:6fd5 ca  de  6f       JP         Z,LAB_ram_6fde
                                            self.IncPC(3);
                                            self.increase_cycles(10);
                                            if (self.data.F & FLAG_Z) != 0 {
                                                // JP(LAB_ram_6fde);
                                                break;
                                            }

                                            //         ram:6fd8 cb  21           SLA        C
                                            //         ram:6fda 3d              DEC        A
                                            self.instr_hk__DEC_A();
                                            //         ram:6fdb c3  d5  6f       JP         LAB_ram_6fd5
                                            self.IncPC(3);
                                            self.increase_cycles(10);
                                            // JP(LAB_ram_6fd5);
                                        }

                                        //                              LAB_ram_6fde                                    XREF[1]:     ram:6fd5 (j)
                                        //         ram:6fde 79              LD         A,C
                                        self.instr_hk__LD_A_C();
                                        //         ram:6fdf 2a  54  c2       LD         HL,(pt_char_c254 )
                                        self.instr_hk__LD_HL_iNNNN(0xc254);
                                        //         ram:6fe2 11  10  00       LD         DE,char_10h_flag
                                        self.instr_hk__LD_DE_NNNN(0x10);
                                        //         ram:6fe5 19              ADD        HL,DE
                                        self.instr_hk__ADD_HL_DE();
                                        //         ram:6fe6 b6              OR         (HL)
                                        self.instr_hk__OR_A_iHL();
                                        //         ram:6fe7 77              LD         (HL),A
                                        self.instr_hk__LD_iHL_A();
                                        //         ram:6fe8 c3  fc  6f       JP         LAB_ram_6ffc
                                        self.IncPC(3);
                                        self.increase_cycles(10);
                                    }
                                    // JP(LAB_ram_6ffc);
                                } else {
                                    //         ram:6f87 3a  85  c3       LD         A,(BYTE_ram_c385 )
                                    self.instr_hk__LD_A_iNNNN(0xc385);
                                    //         ram:6f8a 3c              INC        A
                                    self.instr_hk__INC_A();
                                    //         ram:6f8b 87              ADD        A,A
                                    self.instr_hk__ADD_A_A();
                                    //         ram:6f8c 06  05           LD         B,0x5
                                    self.instr_hk__LD_B_NN(0x5);
                                    //         ram:6f8e cd  34  b6       CALL       sb_rand_calc_B634                                IN a:
                                    assert!(self.call_hook(0xb634));
                                    //                                                                                                 b: cnt
                                    //                                                                                              OUT hl
                                    //         ram:6f91 eb              EX         DE,HL
                                    self.instr_hk__EX_DE_HL();
                                    //         ram:6f92 2a  54  c2       LD         HL,(pt_char_c254 )
                                    self.instr_hk__LD_HL_iNNNN(0xc254);
                                    //         ram:6f95 01  0b  00       LD         BC,char_0bh_
                                    self.instr_hk__LD_BC_NNNN(0x0b);
                                    //         ram:6f98 09              ADD        HL,BC
                                    self.instr_hk__ADD_HL_BC();
                                    //         ram:6f99 7e              LD         A,(HL)
                                    self.instr_hk__LD_A_iHL();
                                    //         ram:6f9a 83              ADD        A,E
                                    self.instr_hk__ADD_A_E();
                                    //         ram:6f9b 77              LD         (HL),A
                                    self.instr_hk__LD_iHL_A();
                                    //         ram:6f9c 23              INC        HL
                                    self.instr_hk__INC_HL();
                                    //         ram:6f9d 7e              LD         A,(HL)
                                    self.instr_hk__LD_A_iHL();
                                    //         ram:6f9e 8a              ADC        A,D
                                    self.instr_hk__ADC_A_D();
                                    //         ram:6f9f 77              LD         (HL),A
                                    self.instr_hk__LD_iHL_A();
                                    //         ram:6fa0 2b              DEC        HL
                                    self.instr_hk__DEC_HL();
                                    //         ram:6fa1 2b              DEC        HL
                                    self.instr_hk__DEC_HL();
                                    //         ram:6fa2 7e              LD         A,(HL)
                                    self.instr_hk__LD_A_iHL();
                                    //         ram:6fa3 ce  00           ADC        A,0x0
                                    self.instr_hk__ADC_A_NN(0x0);
                                    //         ram:6fa5 77              LD         (HL),A
                                    self.instr_hk__LD_iHL_A();
                                    //         ram:6fa6 cb  7f           BIT        0x7 ,A
                                    self.instr_hk__BIT_7_A();
                                    //         ram:6fa8 ca  fc  6f       JP         Z,LAB_ram_6ffc
                                    self.IncPC(3);
                                    self.increase_cycles(10);
                                    if (self.data.F & FLAG_Z) == 0 {
                                        //         ram:6fab 36  7f           LD         (HL),0x7f
                                        self.instr_hk__LD_iHL_NN(0x7f);
                                        //         ram:6fad 3e  ff           LD         A,0xff
                                        self.instr_hk__LD_A_NN(0xff);
                                        //         ram:6faf 23              INC        HL
                                        self.instr_hk__INC_HL();
                                        //         ram:6fb0 77              LD         (HL),A
                                        self.instr_hk__LD_iHL_A();
                                        //         ram:6fb1 23              INC        HL
                                        self.instr_hk__INC_HL();
                                        //         ram:6fb2 77              LD         (HL),A
                                        self.instr_hk__LD_iHL_A();
                                        //         ram:6fb3 c3  fc  6f       JP         LAB_ram_6ffc
                                        self.IncPC(3);
                                        self.increase_cycles(10);
                                    }
                                    // JP(LAB_ram_6ffc);
                                }

                                //                              LAB_ram_6ffc                                    XREF[3]:     ram:6fa8 (j) , ram:6fb3 (j) ,
                                //                                                                                           ram:6fe8 (j)
                                //         ram:6ffc 16  08           LD         D,0x8
                                self.instr_hk__LD_D_NN(0x8);
                                //         ram:6ffe cd  37  7b       CALL       fn_psg_7b37                                      undefined fn_psg_7b37(void)
                                assert!(self.call_hook(0x7b37));
                                //         ram:7001 cd  ea  46       CALL       sb_read_mem_for_player_46ea                      OUT d,e
                                assert!(self.call_hook(0x46ea));
                                //         ram:7004 eb              EX         DE,HL
                                self.instr_hk__EX_DE_HL();
                                //         ram:7005 e5              PUSH       HL
                                self.instr_hk__PUSH_HL();
                                //         ram:7006 af              XOR        A
                                self.instr_hk__XOR_A_A();
                                //         ram:7007 01  02  02       LD         BC,0x202
                                self.instr_hk__LD_BC_NNNN(0x202);
                                //         ram:700a cd  9e  8a       CALL       sb_set_vmem_guess_8a9e                           IN a: val0?
                                assert!(self.call_hook(0x8a9e));
                                //                                                                                                hl: x0/y0?
                                //                                                                                                bc: w/h?
                                //         ram:700d d1              POP        DE
                                self.instr_hk__POP_DE();
                                //         ram:700e d5              PUSH       DE
                                self.instr_hk__PUSH_DE();
                                //         ram:700f 06  02           LD         B,0x2
                                self.instr_hk__LD_B_NN(0x2);
                                //         ram:7011 cd  db  60       CALL       sb_print_spaces_60db                             IN b: cnt
                                assert!(self.call_hook(0x60db));
                                //                                                                                                  de: xy
                                //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
                                //         ram:7014 d1              POP        DE
                                self.instr_hk__POP_DE();
                                //         ram:7015 1c              INC        E
                                self.instr_hk__INC_E();
                                //         ram:7016 06  02           LD         B,0x2
                                self.instr_hk__LD_B_NN(0x2);
                                //         ram:7018 cd  db  60       CALL       sb_print_spaces_60db                             IN b: cnt
                                assert!(self.call_hook(0x60db));
                                //                                                                                                  de: xy
                                //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
                                //                              l_exit_701b                                     XREF[2]:     ram:6fc3 (j) , ram:6ff2 (j)
                                //         ram:701b c9              RET
                                return true;
                            }
                        }
                    }
                }
            }

            //                              LAB_ram_7140                                    XREF[5]:     ram:701c (j) , ram:7037 (j) ,
            //                                                                                           ram:70d1 (j) , ram:70d8 (j) ,
            //                                                                                           ram:710d (j)
            //         ram:7140 cd  51  47       CALL       sb_get_addr_for_player_4751                      undefined sb_get_addr_for_player
            assert!(self.call_hook(0x4751));
            //         ram:7143 70              LD         (HL),B
            self.instr_hk__LD_iHL_B();
            //         ram:7144 c9              RET
            return true;
        }

        //                              LAB_ram_7145                                    XREF[1]:     ram:6f5f (j)
        //         ram:7145 cd  0c  72       CALL       FUN_ram_720c                                     undefined FUN_ram_720c()
        assert!(self.call_hook(0x720c));
        //         ram:7148 c2  76  6f       JP         NZ,LAB_ram_6f76
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            JP(LAB_ram_6f76);
        }

        //         ram:714b 06  02           LD         B,0x2
        self.instr_hk__LD_B_NN(0x2);
        //         ram:714d 21  15  c2       LD         HL,BYTE_ram_c215
        self.instr_hk__LD_HL_NNNN(0xc215);
        //                              LAB_ram_7150                                    XREF[1]:     ram:7156 (j)
        loop {
            //         ram:7150 7e              LD         A,(HL=>BYTE_ram_c215 )
            self.instr_hk__LD_A_iHL();
            //         ram:7151 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:7152 ca  5c  71       JP         Z,LAB_ram_715c
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(LAB_ram_715c);
                //                              LAB_ram_715c                                    XREF[1]:     ram:7152 (j)
                //         ram:715c 3e  03           LD         A,0x3
                self.instr_hk__LD_A_NN(0x3);
                //         ram:715e 90              SUB        B
                self.instr_hk__SUB_A_B();
                // JP(LAB_ram_715f);
                break;
            }

            //         ram:7155 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7156 10  f8           DJNZ       LAB_ram_7150
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_7150;
            } else {
                self.increase_cycles(8);
                //         ram:7158 af              XOR        A
                self.instr_hk__XOR_A_A();
                //         ram:7159 c3  5f  71       JP         LAB_ram_715f
                self.IncPC(3);
                self.increase_cycles(10);
                // JP(LAB_ram_715f);
                break;
            }
        }

        //                              LAB_ram_715f                                    XREF[1]:     ram:7159 (j)
        //         ram:715f f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:7160 cd  60  47       CALL       sb_read_mem_for_player_4760                      undefined sb_read_mem_for_player
        assert!(self.call_hook(0x4760));
        //         ram:7163 7e              LD         A,(HL=>BYTE_ram_c216 )
        self.instr_hk__LD_A_iHL();
        //         ram:7164 e6  03           AND        0x3
        self.instr_hk__AND_NN(0x3);
        //         ram:7166 c2  6f  71       JP         NZ,LAB_ram_716f
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            //         ram:7169 11  c0  ff       LD         DE,0xffc0
            self.instr_hk__LD_DE_NNNN(0xffc0);
            //         ram:716c c3  88  71       JP         LAB_ram_7188
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(LAB_ram_7188);
        } else {
            // JP(LAB_ram_716f);
            //                              LAB_ram_716f                                    XREF[1]:     ram:7166 (j)
            //         ram:716f fe  01           CP         0x1
            self.instr_hk__CP_NN(0x1);
            //         ram:7171 c2  7a  71       JP         NZ,LAB_ram_717a
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(LAB_ram_717a);
                //                              LAB_ram_717a                                    XREF[1]:     ram:7171 (j)
                //         ram:717a fe  02           CP         0x2
                self.instr_hk__CP_NN(0x2);
                //         ram:717c c2  85  71       JP         NZ,LAB_ram_7185
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP(LAB_ram_7185);
                    //                              LAB_ram_7185                                    XREF[1]:     ram:717c (j)
                    //         ram:7185 11  fe  ff       LD         DE,0xfffe
                    self.instr_hk__LD_DE_NNNN(0xfffe);
                    // JP(LAB_ram_7188);
                } else {
                    //         ram:717f 11  40  00       LD         DE,0x40
                    self.instr_hk__LD_DE_NNNN(0x40);
                    //         ram:7182 c3  88  71       JP         LAB_ram_7188
                    self.IncPC(3);
                    self.increase_cycles(10);
                    // JP(LAB_ram_7188);
                }
            } else {
                //         ram:7174 11  02  00       LD         DE,0x2
                self.instr_hk__LD_DE_NNNN(0x2);
                //         ram:7177 c3  88  71       JP         LAB_ram_7188
                self.IncPC(3);
                self.increase_cycles(10);
                // JP(LAB_ram_7188);
            }
        }

        //                              LAB_ram_7188                                    XREF[3]:     ram:716c (j) , ram:7177 (j) ,
        //                                                                                           ram:7182 (j)
        //         ram:7188 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:7189 cd  ea  46       CALL       sb_read_mem_for_player_46ea                      OUT d,e
        assert!(self.call_hook(0x46ea));
        //         ram:718c eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:718d cd  bc  89       CALL       fn_calc_voffset_89BC                             hl <- (hl & 0xff) * 32 + (hl >>
        assert!(self.call_hook(0x89bc));
        //                                                                                              de <- (hl >> 8)
        //         ram:7190 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:7191 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:7192 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:7193 11  9a  c9       LD         DE,DAT_ram_c99a
        self.instr_hk__LD_DE_NNNN(0xc99a);
        //         ram:7196 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:7197 6e              LD         L=>DAT_ram_c91a ,(HL)
        self.instr_hk__LD_L_iHL();
        //         ram:7198 3a  87  c3       LD         A,(BYTE_ram_c387 )
        self.instr_hk__LD_A_iNNNN(0xc387);
        //         ram:719b b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:719c 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:719d 20  07           JR         NZ,LAB_ram_71a6
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_71a6);
            //                              LAB_ram_71a6                                    XREF[1]:     ram:719d (j)
            //         ram:71a6 fe  11           CP         0x11
            self.instr_hk__CP_NN(0x11);
            //         ram:71a8 c2  07  72       JP         NZ,LAB_ram_7207
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(LAB_ram_7207);
                //                              LAB_ram_7207                                    XREF[2]:     ram:71a1 (j) , ram:71a8 (j)
                //         ram:7207 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:7208 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:7209 c3  76  6f       JP         LAB_ram_6f76
                self.IncPC(3);
                self.increase_cycles(10);
                JP(LAB_ram_6f76);
            }
            // JR(LAB_ram_71ab);
        } else {
            self.increase_cycles(7);
            //         ram:719f fe  21           CP         0x21
            self.instr_hk__CP_NN(0x21);
            //         ram:71a1 c2  07  72       JP         NZ,LAB_ram_7207
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(LAB_ram_7207);
                //                              LAB_ram_7207                                    XREF[2]:     ram:71a1 (j) , ram:71a8 (j)
                //         ram:7207 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:7208 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:7209 c3  76  6f       JP         LAB_ram_6f76
                self.IncPC(3);
                self.increase_cycles(10);
                JP(LAB_ram_6f76);
            } else {
                //         ram:71a4 18  05           JR         LAB_ram_71ab
                self.IncPC(2);
                self.increase_cycles(12);
                // JR(LAB_ram_71ab);
            }
        }

        //                              LAB_ram_71ab                                    XREF[1]:     ram:71a4 (j)
        //         ram:71ab cd  0e  b6       CALL       sb_rand_guess_B60E                               OUT hl, a
        assert!(self.call_hook(0xb60e));
        //         ram:71ae 7c              LD         A,H
        self.instr_hk__LD_A_H();
        //         ram:71af e6  1f           AND        0x1f
        self.instr_hk__AND_NN(0x1f);
        //         ram:71b1 67              LD         H,A
        self.instr_hk__LD_H_A();
        //         ram:71b2 06  2d           LD         B,0x2d
        self.instr_hk__LD_B_NN(0x2d);
        //         ram:71b4 11  11  ad       LD         DE,DAT_ram_ad11                                  = FFh
        self.instr_hk__LD_DE_NNNN(0xad11);
        //                              LAB_ram_71b7                                    XREF[1]:     ram:71c5 (j)
        loop {
            //         ram:71b7 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:71b8 1a              LD         A,(DE=>DAT_ram_ad11 )                            = FFh
            self.instr_hk__LD_A_iDE();
            //                                                                                              = 40h    @
            //         ram:71b9 4f              LD         C,A
            self.instr_hk__LD_C_A();
            //         ram:71ba 06  00           LD         B,0x0
            self.instr_hk__LD_B_NN(0x0);
            //         ram:71bc b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:71bd ed  42           SBC        HL,BC
            self.instr_hk__SBC_HL_BC();
            //         ram:71bf c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:71c0 da  d1  71       JP         C,LAB_ram_71d1
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                // JP(LAB_ram_71d1);
                //                              LAB_ram_71d1                                    XREF[1]:     ram:71c0 (j)
                //         ram:71d1 13              INC        DE
                self.instr_hk__INC_DE();
                //         ram:71d2 1a              LD         A,(DE=>DAT_ram_ad12 )
                self.instr_hk__LD_A_iDE();
                //         ram:71d3 4f              LD         C,A
                self.instr_hk__LD_C_A();
                //         ram:71d4 3a  85  c3       LD         A,(BYTE_ram_c385 )
                self.instr_hk__LD_A_iNNNN(0xc385);
                //         ram:71d7 91              SUB        C
                self.instr_hk__SUB_A_C();
                //         ram:71d8 38  ed           JR         C,LAB_ram_71c7
                self.IncPC(2);
                if (self.data.F & FLAG_C) != 0 {
                    self.increase_cycles(12);
                    // JR(LAB_ram_71c7);
                    //                              LAB_ram_71c7                                    XREF[2]:     ram:71d8 (j) , ram:71de (j)
                    //         ram:71c7 e1              POP        HL
                    self.instr_hk__POP_HL();
                    //         ram:71c8 f1              POP        AF
                    self.instr_hk__POP_AF();
                    //         ram:71c9 af              XOR        A
                    self.instr_hk__XOR_A_A();
                    //         ram:71ca f5              PUSH       AF
                    self.instr_hk__PUSH_AF();
                    //         ram:71cb e5              PUSH       HL
                    self.instr_hk__PUSH_HL();
                    //         ram:71cc 3e  ff           LD         A,0xff
                    self.instr_hk__LD_A_NN(0xff);
                    //         ram:71ce c3  e3  71       JP         LAB_ram_71e3
                    self.IncPC(3);
                    self.increase_cycles(10);
                    // JP(LAB_ram_71e3);
                } else {
                    self.increase_cycles(7);
                    //         ram:71da 3a  b5  c8       LD         A,(BYTE_ram_c8b5 )
                    self.instr_hk__LD_A_iNNNN(0xc8b5);
                    //         ram:71dd b7              OR         A
                    self.instr_hk__OR_A_A();
                    //         ram:71de 20  e7           JR         NZ,LAB_ram_71c7
                    self.IncPC(2);
                    if (self.data.F & FLAG_Z) == 0 {
                        self.increase_cycles(12);
                        // JR(LAB_ram_71c7);
                        //                              LAB_ram_71c7                                    XREF[2]:     ram:71d8 (j) , ram:71de (j)
                        //         ram:71c7 e1              POP        HL
                        self.instr_hk__POP_HL();
                        //         ram:71c8 f1              POP        AF
                        self.instr_hk__POP_AF();
                        //         ram:71c9 af              XOR        A
                        self.instr_hk__XOR_A_A();
                        //         ram:71ca f5              PUSH       AF
                        self.instr_hk__PUSH_AF();
                        //         ram:71cb e5              PUSH       HL
                        self.instr_hk__PUSH_HL();
                        //         ram:71cc 3e  ff           LD         A,0xff
                        self.instr_hk__LD_A_NN(0xff);
                        //         ram:71ce c3  e3  71       JP         LAB_ram_71e3
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP(LAB_ram_71e3);
                    } else {
                        self.increase_cycles(7);
                        //         ram:71e0 3e  2e           LD         A,0x2e
                        self.instr_hk__LD_A_NN(0x2e);
                        //         ram:71e2 90              SUB        B
                        self.instr_hk__SUB_A_B();
                        // JP(LAB_ram_71e3);
                    }
                }
                break;
            }

            //         ram:71c3 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:71c4 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:71c5 10  f0           DJNZ       LAB_ram_71b7
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_71b7;
            } else {
                self.increase_cycles(8);
                // JR(LAB_ram_71c7);
                //                              LAB_ram_71c7                                    XREF[2]:     ram:71d8 (j) , ram:71de (j)
                //         ram:71c7 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:71c8 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:71c9 af              XOR        A
                self.instr_hk__XOR_A_A();
                //         ram:71ca f5              PUSH       AF
                self.instr_hk__PUSH_AF();
                //         ram:71cb e5              PUSH       HL
                self.instr_hk__PUSH_HL();
                //         ram:71cc 3e  ff           LD         A,0xff
                self.instr_hk__LD_A_NN(0xff);
                //         ram:71ce c3  e3  71       JP         LAB_ram_71e3
                self.IncPC(3);
                self.increase_cycles(10);
                // JP(LAB_ram_71e3);
                break;
            }
        }

        // //                              LAB_ram_71c7                                    XREF[2]:     ram:71d8 (j) , ram:71de (j)
        // //         ram:71c7 e1              POP        HL
        // self.instr_hk__POP_HL();
        // //         ram:71c8 f1              POP        AF
        // self.instr_hk__POP_AF();
        // //         ram:71c9 af              XOR        A
        // self.instr_hk__XOR_A_A();
        // //         ram:71ca f5              PUSH       AF
        // self.instr_hk__PUSH_AF();
        // //         ram:71cb e5              PUSH       HL
        // self.instr_hk__PUSH_HL();
        // //         ram:71cc 3e  ff           LD         A,0xff
        // self.instr_hk__LD_A_NN(0xff);
        // //         ram:71ce c3  e3  71       JP         LAB_ram_71e3
        // self.IncPC(3);
        // self.increase_cycles(10);
        // JP(LAB_ram_71e3);

        //                              LAB_ram_71e3                                    XREF[1]:     ram:71ce (j)
        //         ram:71e3 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:71e4 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:71e5 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:71e6 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:71e7 c2  ec  71       JP         NZ,LAB_ram_71ec
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            //         ram:71ea 06  ff           LD         B,0xff
            self.instr_hk__LD_B_NN(0xff);
        }

        //         ram:71ec e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:71ed 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:71ee 16  00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:71f0 21  14  c2       LD         HL,BYTE_ram_c214
        self.instr_hk__LD_HL_NNNN(0xc214);
        //         ram:71f3 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:71f4 70              LD         (HL),B=>BYTE_ram_c214
        self.instr_hk__LD_iHL_B();
        //         ram:71f5 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:71f6 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:71f7 c6  28           ADD        A,0x28
        self.instr_hk__ADD_A_NN(0x28);
        //         ram:71f9 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:71fa 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:71fb e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:71fc cd  77  86       CALL       sb_draw_sprite_guess_8677                        undefined sb_draw_sprite_guess_8
        assert!(self.call_hook(0x8677));
        //         ram:71ff 16  05           LD         D,0x5
        self.instr_hk__LD_D_NN(0x5);
        //         ram:7201 cd  37  7b       CALL       fn_psg_7b37                                      undefined fn_psg_7b37(void)
        assert!(self.call_hook(0x7b37));
        //         ram:7204 c3  76  6f       JP         LAB_ram_6f76
        self.IncPC(3);
        self.increase_cycles(10);
        JP(LAB_ram_6f76);

        //
        true
    }
    pub(crate) fn hook_7335(&mut self) -> bool {
        //
        //                              **************************************************************
        //                              *                          FUNCTION                          *
        //                              **************************************************************
        //                              undefined FUN_ram_7335()
        //              undefined         A:1            <RETURN>
        //                              FUN_ram_7335                                    XREF[1]:     ram:7244(c)
        //         ram:7335 cd 60 47        CALL       sb_read_mem_for_player_4760                      hl <- addr
        assert!(self.call_hook(0x4760));
        //                                                                                              bc <- player_idx
        //         ram:7338 56              LD         D,(HL)
        self.instr_hk__LD_D_iHL();
        //         ram:7339 60              LD         H,B
        self.instr_hk__LD_H_B();
        //         ram:733a 69              LD         L,C
        self.instr_hk__LD_L_C();
        //         ram:733b 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:733c 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:733d 01 ec c1        LD         BC,BYTE_ram_c1ec
        self.instr_hk__LD_BC_NNNN(0xc1ec);
        //         ram:7340 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:7341 7a              LD         A,D
        self.instr_hk__LD_A_D();
        //         ram:7342 e6 03           AND        0x3
        self.instr_hk__AND_NN(0x3);
        //         ram:7344 c2 5e 73        JP         NZ,LAB_ram_735e
        self.IncPC(3);
        self.increase_cycles(10);

        let mut LAB_ram_73ad_enabled = false;

        if (self.data.F & FLAG_Z) == 0 {
            // JP(LAB_ram_735e);
            //                              LAB_ram_735e                                    XREF[1]:     ram:7344(j)
            //         ram:735e fe 01           CP         0x1
            self.instr_hk__CP_NN(0x1);
            //         ram:7360 c2 7a 73        JP         NZ,LAB_ram_737a
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(LAB_ram_737a);
                //                              LAB_ram_737a                                    XREF[1]:     ram:7360(j)
                //         ram:737a fe 02           CP         0x2
                self.instr_hk__CP_NN(0x2);
                //         ram:737c c2 98 73        JP         NZ,LAB_ram_7398
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP(LAB_ram_7398);
                    //                              LAB_ram_7398                                    XREF[1]:     ram:737c(j)
                    //         ram:7398 7e              LD         A,(HL=>BYTE_ram_c1ec)
                    self.instr_hk__LD_A_iHL();
                    //         ram:7399 e6 03           AND        0x3
                    self.instr_hk__AND_NN(0x3);
                    //         ram:739b c2 ad 73        JP         NZ,LAB_ram_73ad
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_Z) == 0 {
                        // JP(LAB_ram_73ad);
                        LAB_ram_73ad_enabled = true;
                    } else {
                        //         ram:739e 56              LD         D,(HL=>BYTE_ram_c1ec)
                        self.instr_hk__LD_D_iHL();
                        //         ram:739f cb 3a           SRL        D
                        self.instr_hk__SRL_D();
                        //         ram:73a1 cb 3a           SRL        D
                        self.instr_hk__SRL_D();
                        //         ram:73a3 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:73a4 5e              LD         E,(HL=>BYTE_ram_c1ed)
                        self.instr_hk__LD_E_iHL();
                        //         ram:73a5 cb 3b           SRL        E
                        self.instr_hk__SRL_E();
                        //         ram:73a7 cb 3b           SRL        E
                        self.instr_hk__SRL_E();
                        //         ram:73a9 15              DEC        D
                        self.instr_hk__DEC_D();
                        //         ram:73aa c3 b8 73        JP         l_exit_x
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP(l_exit_x);
                    }
                } else {
                    //         ram:737f 23              INC        HL
                    self.instr_hk__INC_HL();
                    //         ram:7380 7e              LD         A,(HL=>BYTE_ram_c1ed)
                    self.instr_hk__LD_A_iHL();
                    //         ram:7381 2b              DEC        HL
                    self.instr_hk__DEC_HL();
                    //         ram:7382 e6 03           AND        0x3
                    self.instr_hk__AND_NN(0x3);
                    //         ram:7384 fe 02           CP         0x2
                    self.instr_hk__CP_NN(0x2);
                    //         ram:7386 da ad 73        JP         C,LAB_ram_73ad
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) != 0 {
                        // JP(LAB_ram_73ad);
                        LAB_ram_73ad_enabled = true;
                    } else {
                        //         ram:7389 56              LD         D,(HL=>BYTE_ram_c1ec)
                        self.instr_hk__LD_D_iHL();
                        //         ram:738a cb 3a           SRL        D
                        self.instr_hk__SRL_D();
                        //         ram:738c cb 3a           SRL        D
                        self.instr_hk__SRL_D();
                        //         ram:738e 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:738f 5e              LD         E,(HL=>BYTE_ram_c1ed)
                        self.instr_hk__LD_E_iHL();
                        //         ram:7390 cb 3b           SRL        E
                        self.instr_hk__SRL_E();
                        //         ram:7392 cb 3b           SRL        E
                        self.instr_hk__SRL_E();
                        //         ram:7394 1c              INC        E
                        self.instr_hk__INC_E();
                        //         ram:7395 c3 b8 73        JP         l_exit_x
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP(l_exit_x);
                    }
                }
            } else {
                //         ram:7363 7e              LD         A,(HL=>BYTE_ram_c1ec)
                self.instr_hk__LD_A_iHL();
                //         ram:7364 e6 03           AND        0x3
                self.instr_hk__AND_NN(0x3);
                //         ram:7366 fe 02           CP         0x2
                self.instr_hk__CP_NN(0x2);
                //         ram:7368 da ad 73        JP         C,LAB_ram_73ad
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) != 0 {
                    // JP(LAB_ram_73ad);
                    LAB_ram_73ad_enabled = true;
                } else {
                    //         ram:736b 56              LD         D,(HL=>BYTE_ram_c1ec)
                    self.instr_hk__LD_D_iHL();
                    //         ram:736c cb 3a           SRL        D
                    self.instr_hk__SRL_D();
                    //         ram:736e cb 3a           SRL        D
                    self.instr_hk__SRL_D();
                    //         ram:7370 23              INC        HL
                    self.instr_hk__INC_HL();
                    //         ram:7371 5e              LD         E,(HL=>BYTE_ram_c1ed)
                    self.instr_hk__LD_E_iHL();
                    //         ram:7372 cb 3b           SRL        E
                    self.instr_hk__SRL_E();
                    //         ram:7374 cb 3b           SRL        E
                    self.instr_hk__SRL_E();
                    //         ram:7376 14              INC        D
                    self.instr_hk__INC_D();
                    //         ram:7377 c3 b8 73        JP         l_exit_x
                    self.IncPC(3);
                    self.increase_cycles(10);
                    // JP(l_exit_x);
                }
            }
        } else {
            //         ram:7347 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7348 7e              LD         A,(HL=>BYTE_ram_c1ed)
            self.instr_hk__LD_A_iHL();
            //         ram:7349 2b              DEC        HL
            self.instr_hk__DEC_HL();
            //         ram:734a e6 03           AND        0x3
            self.instr_hk__AND_NN(0x3);
            //         ram:734c c2 ad 73        JP         NZ,LAB_ram_73ad
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(LAB_ram_73ad);
                LAB_ram_73ad_enabled = true;
            } else {
                //         ram:734f 56              LD         D,(HL=>BYTE_ram_c1ec)
                self.instr_hk__LD_D_iHL();
                //         ram:7350 cb 3a           SRL        D
                self.instr_hk__SRL_D();
                //         ram:7352 cb 3a           SRL        D
                self.instr_hk__SRL_D();
                //         ram:7354 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:7355 5e              LD         E,(HL=>BYTE_ram_c1ed)
                self.instr_hk__LD_E_iHL();
                //         ram:7356 cb 3b           SRL        E
                self.instr_hk__SRL_E();
                //         ram:7358 cb 3b           SRL        E
                self.instr_hk__SRL_E();
                //         ram:735a 1d              DEC        E
                self.instr_hk__DEC_E();
                //         ram:735b c3 b8 73        JP         l_exit_x
                self.IncPC(3);
                self.increase_cycles(10);
                // JP(l_exit_x);
            }
        }

        if LAB_ram_73ad_enabled {
            //                              LAB_ram_73ad                                    XREF[4]:     ram:734c(j), ram:7368(j),
            //                                                                                           ram:7386(j), ram:739b(j)
            //         ram:73ad 56              LD         D,(HL=>BYTE_ram_c1ec)
            self.instr_hk__LD_D_iHL();
            //         ram:73ae cb 3a           SRL        D
            self.instr_hk__SRL_D();
            //         ram:73b0 cb 3a           SRL        D
            self.instr_hk__SRL_D();
            //         ram:73b2 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:73b3 5e              LD         E,(HL=>BYTE_ram_c1ed)
            self.instr_hk__LD_E_iHL();
            //         ram:73b4 cb 3b           SRL        E
            self.instr_hk__SRL_E();
            //         ram:73b6 cb 3b           SRL        E
            self.instr_hk__SRL_E();
        }

        self.SetPC(0x73b8);
        //                              l_exit_x                                        XREF[4]:     ram:735b(j), ram:7377(j),
        //                                                                                           ram:7395(j), ram:73aa(j)
        //         ram:73b8 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:73b9 6b              LD         L,E
        self.instr_hk__LD_L_E();
        //         ram:73ba 62              LD         H,D
        self.instr_hk__LD_H_D();
        //         ram:73bb cd bc 89        CALL       fn_calc_voffset_89BC                             hl <- (hl & 0xff) * 20 + (hl >> 8)
        assert!(self.call_hook(0x89bc));
        //                                                                                              de <- (hl >> 8) << 8 + (hl & 0xff)
        //         ram:73be 11 ac c3        LD         DE,DAT_ram_c3ac
        self.instr_hk__LD_DE_NNNN(0xc3ac);
        //         ram:73c1 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:73c2 7e              LD         A,(HL=>DAT_ram_c3ac)
        self.instr_hk__LD_A_iHL();
        //         ram:73c3 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:73c4 c9              RET
        self.assert_pc(0x73c4);
        //
        true
    }
    pub(crate) fn hook_747f(&mut self) -> bool {
        //         ram:747f 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:7480 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:7481 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:7482 c6  02           ADD        A,0x2
        self.instr_hk__ADD_A_NN(0x2);
        //         ram:7484 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:7485 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:7486 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:7487 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:7488 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:7489 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:748a 57              LD         D,A
        self.instr_hk__LD_D_A();
        //         ram:748b 21  ec  c1       LD         HL,BYTE_ram_c1ec
        self.instr_hk__LD_HL_NNNN(0xc1ec);
        //         ram:748e 06  03           LD         B,0x3
        self.instr_hk__LD_B_NN(0x3);
        //                              LAB_ram_7490                                    XREF[1]:     ram:7496 (j)
        loop {
            self.SetPC(0x7490);
            //         ram:7490 72              LD         (HL=>BYTE_ram_c1ec ),D
            self.instr_hk__LD_iHL_D();
            //         ram:7491 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7492 73              LD         (HL=>BYTE_ram_c1ed ),E
            self.instr_hk__LD_iHL_E();
            //         ram:7493 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7494 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7495 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7496 10  f8           DJNZ       LAB_ram_7490
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_7490;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:7498 cd  9c  74       CALL       FUN_ram_749c                                     undefined FUN_ram_749c()
        assert!(self.call_hook(0x749c));
        //         ram:749b c9              RET
        self.assert_pc(0x749b);
        // return true;
        //
        true
    }
    pub(crate) fn hook_749c(&mut self) -> bool {
        //         ram:749c 21  ec  c1       LD         HL,BYTE_ram_c1ec
        self.instr_hk__LD_HL_NNNN(0xc1ec);
        //         ram:749f 11  c0  c1       LD         DE,BYTE_ram_c1c0
        self.instr_hk__LD_DE_NNNN(0xc1c0);
        //                              LAB_ram_74a2                                    XREF[1]:     ram:74ac (j)
        loop {
            self.SetPC(0x74a2);
            //         ram:74a2 1a              LD         A,(DE=>BYTE_ram_c1c0 )
            self.instr_hk__LD_A_iDE();
            //         ram:74a3 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:74a4 c2  af  74       JP         NZ,LAB_ram_74af
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(LAB_ram_74af);
                break;
            }

            //         ram:74a7 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:74a8 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:74a9 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:74aa 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:74ab 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:74ac c3  a2  74       JP         LAB_ram_74a2
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(LAB_ram_74a2);
        }

        //                              LAB_ram_74af                                    XREF[1]:     ram:74a4 (j)
        //         ram:74af 7e              LD         A,(HL=>BYTE_ram_c1ec )
        self.instr_hk__LD_A_iHL();
        //         ram:74b0 fe  0f           CP         0xf
        self.instr_hk__CP_NN(0xf);
        //         ram:74b2 da  cb  74       JP         C,LAB_ram_74cb
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) != 0 {
            // JP(LAB_ram_74cb);
            //                              LAB_ram_74cb                                    XREF[1]:     ram:74b2 (j)
            //         ram:74cb af              XOR        A
            self.instr_hk__XOR_A_A();
            //         ram:74cc c3  bc  74       JP         LAB_ram_74bc
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(LAB_ram_74bc);
        } else {
            //         ram:74b5 fe  6c           CP         0x6c
            self.instr_hk__CP_NN(0x6c);
            //         ram:74b7 d2  cf  74       JP         NC,LAB_ram_74cf
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP(LAB_ram_74cf);
                //                              LAB_ram_74cf                                    XREF[1]:     ram:74b7 (j)
                //         ram:74cf 3e  5c           LD         A,0x5c
                self.instr_hk__LD_A_NN(0x5c);
                //         ram:74d1 c3  bc  74       JP         LAB_ram_74bc
                self.IncPC(3);
                self.increase_cycles(10);
                // JP(LAB_ram_74bc);
            } else {
                //         ram:74ba d6  0f           SUB        0xf
                self.instr_hk__SUB_NN(0xf);
            }
            // JP(LAB_ram_74bc);
        }

        //                              LAB_ram_74bc                                    XREF[2]:     ram:74cc (j) , ram:74d1 (j)
        //         ram:74bc 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:74bd 3a  e6  c1       LD         A,(BYTE_ram_c1e6 )
        self.instr_hk__LD_A_iNNNN(0xc1e6);
        //         ram:74c0 91              SUB        C
        self.instr_hk__SUB_A_C();
        //         ram:74c1 32  bc  c8       LD         (BYTE_ram_c8bc ),A
        self.instr_hk__LD_iNNNN_A(0xc8bc);
        //         ram:74c4 79              LD         A,C
        self.instr_hk__LD_A_C();
        //         ram:74c5 32  e6  c1       LD         (BYTE_ram_c1e6 ),A
        self.instr_hk__LD_iNNNN_A(0xc1e6);
        //         ram:74c8 c3  d4  74       JP         LAB_ram_74d4
        self.IncPC(3);
        self.increase_cycles(10);
        // JP(LAB_ram_74d4);

        //                              LAB_ram_74d4                                    XREF[1]:     ram:74c8 (j)
        //         ram:74d4 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:74d5 7e              LD         A,(HL=>BYTE_ram_c1ed )
        self.instr_hk__LD_A_iHL();
        //         ram:74d6 fe  09           CP         0x9
        self.instr_hk__CP_NN(0x9);
        //         ram:74d8 da  f1  74       JP         C,LAB_ram_74f1
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) != 0 {
            // JP(LAB_ram_74f1);
            //                              LAB_ram_74f1                                    XREF[1]:     ram:74d8 (j)
            //         ram:74f1 af              XOR        A
            self.instr_hk__XOR_A_A();
            //         ram:74f2 c3  e2  74       JP         LAB_ram_74e2
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(LAB_ram_74e2);
        } else {
            //         ram:74db fe  72           CP         0x72
            self.instr_hk__CP_NN(0x72);
            //         ram:74dd d2  f5  74       JP         NC,LAB_ram_74f5
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP(LAB_ram_74f5);
                //                              LAB_ram_74f5                                    XREF[1]:     ram:74dd (j)
                //         ram:74f5 3e  68           LD         A,0x68
                self.instr_hk__LD_A_NN(0x68);
                //         ram:74f7 c3  e2  74       JP         LAB_ram_74e2
                self.IncPC(3);
                self.increase_cycles(10);
                // JP(LAB_ram_74e2);
            } else {
                //         ram:74e0 d6  09           SUB        0x9
                self.instr_hk__SUB_NN(0x9);
            }
        }

        //                              LAB_ram_74e2                                    XREF[2]:     ram:74f2 (j) , ram:74f7 (j)
        //         ram:74e2 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:74e3 3a  e7  c1       LD         A,(BYTE_ram_c1e7 )
        self.instr_hk__LD_A_iNNNN(0xc1e7);
        //         ram:74e6 91              SUB        C
        self.instr_hk__SUB_A_C();
        //         ram:74e7 32  bd  c8       LD         (BYTE_ram_c8bd ),A
        self.instr_hk__LD_iNNNN_A(0xc8bd);
        //         ram:74ea 79              LD         A,C
        self.instr_hk__LD_A_C();
        //         ram:74eb 32  e7  c1       LD         (BYTE_ram_c1e7 ),A
        self.instr_hk__LD_iNNNN_A(0xc1e7);
        //         ram:74ee c3  fa  74       JP         LAB_ram_74fa
        self.IncPC(3);
        self.increase_cycles(10);
        // JP(LAB_ram_74fa);

        //                              LAB_ram_74fa                                    XREF[1]:     ram:74ee (j)
        //         ram:74fa 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:74fb 3a  e6  c1       LD         A,(BYTE_ram_c1e6 )
        self.instr_hk__LD_A_iNNNN(0xc1e6);
        //         ram:74fe 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:74ff 7e              LD         A,(HL=>BYTE_ram_c1ec )
        self.instr_hk__LD_A_iHL();
        //         ram:7500 91              SUB        C
        self.instr_hk__SUB_A_C();
        //         ram:7501 32  ee  c1       LD         (BYTE_ram_c1ee ),A
        self.instr_hk__LD_iNNNN_A(0xc1ee);
        //         ram:7504 32  f2  c1       LD         (BYTE_ram_c1f2 ),A
        self.instr_hk__LD_iNNNN_A(0xc1f2);
        //         ram:7507 32  f6  c1       LD         (BYTE_ram_c1f6 ),A
        self.instr_hk__LD_iNNNN_A(0xc1f6);
        //         ram:750a 3a  e7  c1       LD         A,(BYTE_ram_c1e7 )
        self.instr_hk__LD_A_iNNNN(0xc1e7);
        //         ram:750d 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:750e 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:750f 7e              LD         A,(HL=>BYTE_ram_c1ed )
        self.instr_hk__LD_A_iHL();
        //         ram:7510 91              SUB        C
        self.instr_hk__SUB_A_C();
        //         ram:7511 32  ef  c1       LD         (BYTE_ram_c1ef ),A
        self.instr_hk__LD_iNNNN_A(0xc1ef);
        //         ram:7514 32  f3  c1       LD         (BYTE_ram_c1f3 ),A
        self.instr_hk__LD_iNNNN_A(0xc1f3);
        //         ram:7517 32  f7  c1       LD         (BYTE_ram_c1f7 ),A
        self.instr_hk__LD_iNNNN_A(0xc1f7);
        //         ram:751a 06  08           LD         B,0x8
        self.instr_hk__LD_B_NN(0x8);
        //         ram:751c 21  db  c8       LD         HL,BYTE_ram_c8db
        self.instr_hk__LD_HL_NNNN(0xc8db);
        //         ram:751f 3a  bc  c8       LD         A,(BYTE_ram_c8bc )
        self.instr_hk__LD_A_iNNNN(0xc8bc);
        //         ram:7522 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //                              LAB_ram_7523                                    XREF[1]:     ram:7526 (j)
        loop {
            self.SetPC(0x7523);
            //         ram:7523 cd  37  75       CALL       FUN_ram_7537                                     undefined FUN_ram_7537()
            assert!(self.call_hook(0x7537));
            //         ram:7526 10  fb           DJNZ       LAB_ram_7523
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_7523;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:7528 06  08           LD         B,0x8
        self.instr_hk__LD_B_NN(0x8);
        //         ram:752a 21  dc  c8       LD         HL,BYTE_ram_c8dc
        self.instr_hk__LD_HL_NNNN(0xc8dc);
        //         ram:752d 3a  bd  c8       LD         A,(BYTE_ram_c8bd )
        self.instr_hk__LD_A_iNNNN(0xc8bd);
        //         ram:7530 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //                              LAB_ram_7531                                    XREF[1]:     ram:7534 (j)
        loop {
            self.SetPC(0x7531);
            //         ram:7531 cd  37  75       CALL       FUN_ram_7537                                     undefined FUN_ram_7537()
            assert!(self.call_hook(0x7537));
            //         ram:7534 10  fb           DJNZ       LAB_ram_7531
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_7531;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:7536 c9              RET
        return true;
        // true
    }
    pub(crate) fn hook_7537(&mut self) -> bool {
        //         ram:7537 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:7538 81              ADD        A,C
        self.instr_hk__ADD_A_C();
        //         ram:7539 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:753a 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:753b 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:753c 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:753d 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:753e c9              RET
        self.assert_pc(0x753e);
        // return true;
        //
        true
    }
    pub(crate) fn hook_7587(&mut self) -> bool {
        //         ram:7587 06  04           LD         B,0x4
        self.instr_hk__LD_B_NN(0x4);
        //         ram:7589 21  d0  c1       LD         HL,bt_char_spell_guess_c1d0                      = FFh
        self.instr_hk__LD_HL_NNNN(0xc1d0);
        //                              LAB_ram_758c                                    XREF[1]:     ram:75a0 (j)
        loop {
            self.SetPC(0x758c);
            //         ram:758c 7e              LD         A,(HL=>bt_char_spell_guess_c1d0 )                = FFh
            self.instr_hk__LD_A_iHL();
            //         ram:758d 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:758e 28  0c           JR         Z,LAB_ram_759c
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_759c);
            } else {
                self.increase_cycles(7);
                //         ram:7590 36  ff           LD         (HL=>bt_char_spell_guess_c1d0 ),0xff             = FFh
                self.instr_hk__LD_iHL_NN(0xff);
                //         ram:7592 e5              PUSH       HL=>bt_char_spell_guess_c1d0                     = FFh
                self.instr_hk__PUSH_HL();
                //         ram:7593 c5              PUSH       BC
                self.instr_hk__PUSH_BC();
                //         ram:7594 3e  0a           LD         A,0xa
                self.instr_hk__LD_A_NN(0xa);
                //         ram:7596 90              SUB        B
                self.instr_hk__SUB_A_B();
                //         ram:7597 cd  6c  8b       CALL       sb_mem_blit_ram_to_vram_guess_8B6C               undefined sb_mem_blit_ram_to_vra
                assert!(self.call_hook(0x8b6c));
                //         ram:759a c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:759b e1              POP        HL
                self.instr_hk__POP_HL();
            }

            //                              LAB_ram_759c                                    XREF[1]:     ram:758e (j)
            //         ram:759c 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:759d 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:759e 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:759f 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:75a0 10  ea           DJNZ       LAB_ram_758c
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_758c;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:75a2 16  ff           LD         D,0xff
        self.instr_hk__LD_D_NN(0xff);
        //         ram:75a4 cd  37  7b       CALL       fn_psg_7b37                                      undefined fn_psg_7b37(void)
        assert!(self.call_hook(0x7b37));
        //         ram:75a7 af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:75a8 32  b4  c8       LD         (BYTE_ram_c8b4 ),A
        self.instr_hk__LD_iNNNN_A(0xc8b4);
        //         ram:75ab c9              RET
        self.assert_pc(0x75ab);
        // return true;
        //
        true
    }
    pub(crate) fn hook_75b5(&mut self) -> bool {
        //         ram:75b5 06  03           LD         B,0x3
        self.instr_hk__LD_B_NN(0x3);
        //         ram:75b7 21  4c  c3       LD         HL,BYTE_ram_c34c
        self.instr_hk__LD_HL_NNNN(0xc34c);
        //         ram:75ba af              XOR        A
        self.instr_hk__XOR_A_A();
        //                              LAB_ram_75bb                                    XREF[1]:     ram:75bd (j)
        loop {
            self.SetPC(0x75bb);
            //         ram:75bb b6              OR         (HL=>BYTE_ram_c34c )
            self.instr_hk__OR_A_iHL();
            //         ram:75bc 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:75bd 10  fc           DJNZ       LAB_ram_75bb
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_75bb;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:75bf b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:75c0 c9              RET
        self.assert_pc(0x75c0);

        // return true;
        //
        true
    }
    pub(crate) fn hook_75c1(&mut self) -> bool {
        //         ram:75c1 21  c0  c1       LD         HL,BYTE_ram_c1c0                                 IN c: check
        self.instr_hk__LD_HL_NNNN(0xc1c0);
        //                                                                                              OUT cf: found check between [C1C
        //                                                                                                  nc: not found
        //         ram:75c4 06  03           LD         B,0x3
        self.instr_hk__LD_B_NN(0x3);
        //                              LAB_ram_75c6                                    XREF[1]:     ram:75cc (j)
        loop {
            self.SetPC(0x75c6);
            //         ram:75c6 7e              LD         A,(HL=>BYTE_ram_c1c0 )
            self.instr_hk__LD_A_iHL();
            //         ram:75c7 b9              CP         C
            self.instr_hk__CP_C();
            //         ram:75c8 ca  d0  75       JP         Z,LAB_ram_75d0
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(LAB_ram_75d0);
                //                              LAB_ram_75d0                                    XREF[1]:     ram:75c8 (j)
                //         ram:75d0 37              SCF
                self.instr_hk__SCF();
                //         ram:75d1 c9              RET
                return true;
            }

            //         ram:75cb 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:75cc 10  f8           DJNZ       LAB_ram_75c6
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_75c6;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:75ce b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:75cf c9              RET
        // return true;
        //
        true
    }
    pub(crate) fn hook_75d2(&mut self) -> bool {
        //         ram:75d2 cd 0a 60        CALL       sb_get_player_addr_600A
        assert!(self.call_hook(0x600a));
        //         ram:75d5 22 54 c2        LD         (pt_char_c254),HL
        self.instr_hk__LD_iNNNN_HL(0xc254);
        //         ram:75d8 11 11 00        LD         DE,char_11_items
        self.instr_hk__LD_DE_NNNN(0x11);
        //         ram:75db 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:75dc 22 24 c2        LD         (wd_l_char_11_c224),HL
        self.instr_hk__LD_iNNNN_HL(0xc224);
        //         ram:75df 06 05           LD         B,0x5
        self.instr_hk__LD_B_NN(0x5);
        //                              loop_1
        loop {
            //         ram:75e1 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:75e2 54              LD         D,H
            self.instr_hk__LD_D_H();
            //         ram:75e3 5d              LD         E,L
            self.instr_hk__LD_E_L();
            //         ram:75e4 13              INC        DE
            self.instr_hk__INC_DE();
            //                              loop_1_a
            loop {
                //         ram:75e5 1a              LD         A,(DE)
                self.instr_hk__LD_A_iDE();
                //         ram:75e6 be              CP         (HL)
                self.instr_hk__CP_iHL();
                //         ram:75e7 da ee 75        JP         C,loop_1_a_chk_cond
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) != 0 {
                    // JP loop_1_a_chk_cond;
                } else {
                    //         ram:75ea 4e              LD         C,(HL)
                    self.instr_hk__LD_C_iHL();
                    //         ram:75eb 77              LD         (HL),A
                    self.instr_hk__LD_iHL_A();
                    //         ram:75ec 79              LD         A,C
                    self.instr_hk__LD_A_C();
                    //         ram:75ed 12              LD         (DE),A
                    self.instr_hk__LD_iDE_A();
                }

                //                              loop_1_a_chk_cond
                //         ram:75ee 13              INC        DE
                self.instr_hk__INC_DE();
                //         ram:75ef 10 f4           DJNZ       loop_1_a
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP loop_1_a;
                } else {
                    self.increase_cycles(8);
                    break;
                }
            }

            //         ram:75f1 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:75f2 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:75f3 10 ec           DJNZ       loop_1
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_1;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:75f5 cd 3d 47        CALL       sb_get_addr_ac_for_player_guess_473d
        assert!(self.call_hook(0x473d));

        //         ram:75f8 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:75f9 22 26 c2        LD         (wd_l_c226),HL
        self.instr_hk__LD_iNNNN_HL(0xc226);
        //         ram:75fc 21 99 c1        LD         HL,BYTE_ram_c199
        self.instr_hk__LD_HL_NNNN(0xc199);
        //         ram:75ff 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:7600 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:7601 22 28 c2        LD         (wd_l_c228),HL
        self.instr_hk__LD_iNNNN_HL(0xc228);
        //         ram:7604 21 ba c1        LD         HL,bt_c1ba
        self.instr_hk__LD_HL_NNNN(0xc1ba);
        //         ram:7607 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:7608 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:7609 22 2a c2        LD         (wd_l_c22a),HL
        self.instr_hk__LD_iNNNN_HL(0xc22a);
        //         ram:760c cd 78 47        CALL       sb_read_mem_for_player_4778
        assert!(self.call_hook(0x4778));
        //         ram:760f 22 2c c2        LD         (wd_l_c22c),HL
        self.instr_hk__LD_iNNNN_HL(0xc22c);
        //         ram:7612 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:7613 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:7614 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:7615 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:7616 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:7617 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:7618 70              LD         (HL),B=>by_player_controller_c1bd
        self.instr_hk__LD_iHL_B();
        //         ram:7619 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:761a 70              LD         (HL),B=>BYTE_ram_c1be
        self.instr_hk__LD_iHL_B();
        //         ram:761b 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:761c 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:761d 2a 24 c2        LD         HL,(wd_l_char_11_c224)
        self.instr_hk__LD_HL_iNNNN(0xc224);
        //         ram:7620 06 06           LD         B,0x6
        self.instr_hk__LD_B_NN(0x6);
        //                              loop_2
        loop {
            //         ram:7622 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:7623 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:7624 ca 2f 76        JP         Z,LAB_ram_762f
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP LAB_ram_762f;
                //                              LAB_ram_762f
                //         ram:762f 2a 28 c2        LD         HL,(wd_l_c228)
                self.instr_hk__LD_HL_iNNNN(0xc228);
                //         ram:7632 36 01           LD         (HL),0x1
                self.instr_hk__LD_iHL_NN(0x1);
                //         ram:7634 c3 41 76        JP         LAB_ram_7641
                self.IncPC(3);
                self.increase_cycles(10);
                break; //JP LAB_ram_7641;
            } else {
                //         ram:7627 fe 11           CP         0x11
                self.instr_hk__CP_NN(0x11);
                //         ram:7629 da 37 76        JP         C,LAB_ram_7637
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) != 0 {
                    // JP LAB_ram_7637;
                    //                              LAB_ram_7637
                    //         ram:7637 cd 13 77        CALL       sb_is_usable_guess_7713
                    assert!(self.call_hook(0x7713));

                    //         ram:763a da 2c 76        JP         C,loop_2_chk_cond
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) != 0 {
                        // JP loop_2_chk_cond;
                    } else {
                        // jp ram:763d
                        //         ram:763d 2a 28 c2        LD         HL,(wd_l_c228)
                        self.instr_hk__LD_HL_iNNNN(0xc228);
                        //         ram:7640 77              LD         (HL),A
                        self.instr_hk__LD_iHL_A();
                        break; //JP LAB_ram_7641;
                    }
                }
            }

            //                              loop_2_chk_cond
            //         ram:762c 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:762d 10 f3           DJNZ       loop_2
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_2;
            } else {
                self.increase_cycles(8);
                // break;
                //                              LAB_ram_762f
                //         ram:762f 2a 28 c2        LD         HL,(wd_l_c228)
                self.instr_hk__LD_HL_iNNNN(0xc228);
                //         ram:7632 36 01           LD         (HL),0x1
                self.instr_hk__LD_iHL_NN(0x1);
                //         ram:7634 c3 41 76        JP         LAB_ram_7641
                self.IncPC(3);
                self.increase_cycles(10);
                break; //JP LAB_ram_7641;
            }
        }

        // //                              LAB_ram_762f
        // //         ram:762f 2a 28 c2        LD         HL,(wd_l_c228)
        // self.instr_hk__LD_HL_iNNNN(0xc228);
        // //         ram:7632 36 01           LD         (HL),0x1
        // self.instr_hk__LD_iHL_NN(0x1);
        // //         ram:7634 c3 41 76        JP         LAB_ram_7641
        // self.IncPC(3);self.increase_cycles(10);
        // JP LAB_ram_7641;

        //                              LAB_ram_7641
        //         ram:7641 2a 24 c2        LD         HL,(wd_l_char_11_c224)
        self.instr_hk__LD_HL_iNNNN(0xc224);
        //         ram:7644 06 06           LD         B,0x6
        self.instr_hk__LD_B_NN(0x6);
        //                              loop_3
        loop {
            //         ram:7646 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:7647 fe 11           CP         0x11
            self.instr_hk__CP_NN(0x11);
            //         ram:7649 da 51 76        JP         C,loop_3_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                // JP loop_3_chk_cond;
            } else {
                //         ram:764c fe 13           CP         0x13
                self.instr_hk__CP_NN(0x13);
                //         ram:764e da 57 76        JP         C,LAB_ram_7657
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) != 0 {
                    // JP LAB_ram_7657;
                    //                              LAB_ram_7657
                    //         ram:7657 cd 13 77        CALL       sb_is_usable_guess_7713
                    assert!(self.call_hook(0x7713));

                    //         ram:765a da 51 76        JP         C,loop_3_chk_cond
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) != 0 {
                        // JP loop_3_chk_cond;
                    } else {
                        // jp ram:765d

                        //         ram:765d 2a 2a c2        LD         HL,(wd_l_c22a)
                        self.instr_hk__LD_HL_iNNNN(0xc22a);
                        //         ram:7660 4f              LD         C,A
                        self.instr_hk__LD_C_A();
                        //         ram:7661 7e              LD         A,(HL)
                        self.instr_hk__LD_A_iHL();
                        //         ram:7662 b9              CP         C
                        self.instr_hk__CP_C();
                        //         ram:7663 30 01           JR         NC,LAB_ram_7666
                        self.IncPC(2);
                        if (self.data.F & FLAG_C) == 0 {
                            self.increase_cycles(12); //JR LAB_ram_7666;
                        } else {
                            self.increase_cycles(7);
                            //         ram:7665 71              LD         (HL),C
                            self.instr_hk__LD_iHL_C();
                        }
                        break; //jp LAB_ram_7666
                    }
                }
            }

            //                              loop_3_chk_cond
            //         ram:7651 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7652 10 f2           DJNZ       loop_3
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_3;
            } else {
                self.increase_cycles(8);
                // break;
                //         ram:7654 c3 66 76        JP         LAB_ram_7666
                self.IncPC(3);
                self.increase_cycles(10);
                break; // JP LAB_ram_7666;
            }
        }

        //                              LAB_ram_7666
        //         ram:7666 2a 24 c2        LD         HL,(wd_l_char_11_c224)
        self.instr_hk__LD_HL_iNNNN(0xc224);
        //         ram:7669 06 06           LD         B,0x6
        self.instr_hk__LD_B_NN(0x6);
        //                              loop_4
        loop {
            //         ram:766b 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:766c fe 13           CP         0x13
            self.instr_hk__CP_NN(0x13);
            //         ram:766e da 76 76        JP         C,loop_4_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                // JP loop_4_chk_cond;
            } else {
                //         ram:7671 fe 1b           CP         0x1b
                self.instr_hk__CP_NN(0x1b);
                //         ram:7673 da 7c 76        JP         C,LAB_ram_767c
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) != 0 {
                    // JP LAB_ram_767c;
                    //                              LAB_ram_767c
                    //         ram:767c cd 13 77        CALL       sb_is_usable_guess_7713
                    assert!(self.call_hook(0x7713));

                    //         ram:767f da 76 76        JP         C,loop_4_chk_cond
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) != 0 {
                        // JP loop_4_chk_cond;
                    } else {
                        // jp ram:7682
                        //         ram:7682 2a 26 c2        LD         HL,(wd_l_c226)
                        self.instr_hk__LD_HL_iNNNN(0xc226);
                        //         ram:7685 77              LD         (HL),A
                        self.instr_hk__LD_iHL_A();
                        break; //jp LAB_ram_7686
                    }
                }
            }

            //                              loop_4_chk_cond
            //         ram:7676 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7677 10 f2           DJNZ       loop_4
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_4;
            } else {
                self.increase_cycles(8);
                // break;
                //         ram:7679 c3 86 76        JP         LAB_ram_7686
                self.IncPC(3);
                self.increase_cycles(10);
                break; // JP LAB_ram_7686;
            }
        }

        //         ram:7679 c3 86 76        JP         LAB_ram_7686
        // self.IncPC(3);self.increase_cycles(10);JP LAB_ram_7686;

        // //         ram:7682 2a 26 c2        LD         HL,(wd_l_c226)
        // self.instr_hk__LD_HL_iNNNN(0xc226);
        // //         ram:7685 77              LD         (HL),A
        // self.instr_hk__LD_iHL_A();
        // jp LAB_ram_7686

        //                              LAB_ram_7686
        //         ram:7686 2a 24 c2        LD         HL,(wd_l_char_11_c224)
        self.instr_hk__LD_HL_iNNNN(0xc224);
        //         ram:7689 06 06           LD         B,0x6
        self.instr_hk__LD_B_NN(0x6);
        //                              loop_5
        loop {
            //         ram:768b 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:768c fe 1b           CP         0x1b
            self.instr_hk__CP_NN(0x1b);
            //         ram:768e da 96 76        JP         C,loop_5_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                // JP loop_5_chk_cond;
            } else {
                //         ram:7691 fe 21           CP         0x21
                self.instr_hk__CP_NN(0x21);
                //         ram:7693 da 9c 76        JP         C,LAB_ram_769c
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) != 0 {
                    // JP LAB_ram_769c;
                    //                              LAB_ram_769c
                    //         ram:769c cd 13 77        CALL       sb_is_usable_guess_7713
                    assert!(self.call_hook(0x7713));

                    //         ram:769f da 96 76        JP         C,loop_5_chk_cond
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) != 0 {
                        // JP loop_5_chk_cond;
                    } else {
                        // jp ram:76a2

                        //         ram:76a2 2a 26 c2        LD         HL,(wd_l_c226)
                        self.instr_hk__LD_HL_iNNNN(0xc226);
                        //         ram:76a5 86              ADD        A,(HL)
                        self.instr_hk__ADD_A_iHL();
                        //         ram:76a6 77              LD         (HL),A
                        self.instr_hk__LD_iHL_A();
                        //         ram:76a7 cd 56 47        CALL       sb_get_addr_ac_for_player_guess_4756
                        assert!(self.call_hook(0x4756));

                        //         ram:76aa 7e              LD         A,(HL)
                        self.instr_hk__LD_A_iHL();
                        //         ram:76ab 2a 26 c2        LD         HL,(wd_l_c226)
                        self.instr_hk__LD_HL_iNNNN(0xc226);
                        //         ram:76ae 86              ADD        A,(HL)
                        self.instr_hk__ADD_A_iHL();
                        //         ram:76af 77              LD         (HL),A
                        self.instr_hk__LD_iHL_A();
                        // to LAB_ram_76b0
                        break;
                    }
                } else {
                    //                              loop_5_chk_cond
                }
            }

            //                              loop_5_chk_cond
            //         ram:7696 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7697 10 f2           DJNZ       loop_5
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_5;
            } else {
                self.increase_cycles(8);
                // break;
                //         ram:7699 c3 b0 76        JP         LAB_ram_76b0
                self.IncPC(3);
                self.increase_cycles(10);
                break; //JP LAB_ram_76b0;
            }
        }

        //                              LAB_ram_76b0
        //         ram:76b0 2a 24 c2        LD         HL,(wd_l_char_11_c224)
        self.instr_hk__LD_HL_iNNNN(0xc224);
        //         ram:76b3 06 06           LD         B,0x6
        self.instr_hk__LD_B_NN(0x6);
        //                              loop_6
        loop {
            //         ram:76b5 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:76b6 fe 21           CP         0x21
            self.instr_hk__CP_NN(0x21);
            //         ram:76b8 da c0 76        JP         C,loop_6_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                // JP loop_6_chk_cond;
            } else {
                //         ram:76bb fe 27           CP         0x27
                self.instr_hk__CP_NN(0x27);
                //         ram:76bd da fb 76        JP         C,LAB_ram_76fb
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) != 0 {
                    // JP LAB_ram_76fb;
                    //                              LAB_ram_76fb
                    //         ram:76fb cd 13 77        CALL       sb_is_usable_guess_7713
                    assert!(self.call_hook(0x7713));

                    //         ram:76fe da c0 76        JP         C,loop_6_chk_cond
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_C) != 0 {
                        // JP loop_6_chk_cond;
                    } else {
                        //         ram:7701 e5              PUSH       HL
                        self.instr_hk__PUSH_HL();
                        //         ram:7702 2a 2c c2        LD         HL,(wd_l_c22c)
                        self.instr_hk__LD_HL_iNNNN(0xc22c);
                        //         ram:7705 7b              LD         A,E
                        self.instr_hk__LD_A_E();
                        //         ram:7706 d6 21           SUB        0x21
                        self.instr_hk__SUB_NN(0x21);
                        //         ram:7708 5f              LD         E,A
                        self.instr_hk__LD_E_A();
                        //         ram:7709 2a 2c c2        LD         HL,(wd_l_c22c)
                        self.instr_hk__LD_HL_iNNNN(0xc22c);
                        //         ram:770c 19              ADD        HL,DE
                        self.instr_hk__ADD_HL_DE();
                        //         ram:770d 36 01           LD         (HL),0x1
                        self.instr_hk__LD_iHL_NN(0x1);
                        //         ram:770f e1              POP        HL
                        self.instr_hk__POP_HL();
                        //         ram:7710 c3 c0 76        JP         loop_6_chk_cond
                        self.IncPC(3);
                        self.increase_cycles(10); //JP loop_6_chk_cond;
                    }
                }
            }

            //                              loop_6_chk_cond

            //         ram:76c0 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:76c1 10 f2           DJNZ       loop_6
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_6;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:76c3 2a 2c c2        LD         HL,(wd_l_c22c)
        self.instr_hk__LD_HL_iNNNN(0xc22c);
        //         ram:76c6 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:76c7 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:76c8 ca d8 76        JP         Z,LAB_ram_76d8
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP LAB_ram_76d8;
        } else {
            //         ram:76cb 2a 26 c2        LD         HL,(wd_l_c226)
            self.instr_hk__LD_HL_iNNNN(0xc226);
            //         ram:76ce 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:76cf c6 03           ADD        A,0x3
            self.instr_hk__ADD_A_NN(0x3);
            //         ram:76d1 fe 13           CP         0x13
            self.instr_hk__CP_NN(0x13);
            //         ram:76d3 38 02           JR         C,LAB_ram_76d7
            self.IncPC(2);
            if (self.data.F & FLAG_C) != 0 {
                self.increase_cycles(12); //JR LAB_ram_76d7;
            } else {
                self.increase_cycles(7);
                //         ram:76d5 3e 12           LD         A,0x12
                self.instr_hk__LD_A_NN(0x12);
            }

            //                              LAB_ram_76d7
            //         ram:76d7 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
        }

        //                              LAB_ram_76d8
        //         ram:76d8 2a 2c c2        LD         HL,(wd_l_c22c)
        self.instr_hk__LD_HL_iNNNN(0xc22c);
        //         ram:76db 01 05 00        LD         BC,0x5
        self.instr_hk__LD_BC_NNNN(0x5);
        //         ram:76de 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:76df 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:76e0 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:76e1 c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:76e2 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:76e3 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:76e4 36 01           LD         (HL),offset by_player_controller_c1bd
        self.instr_hk__LD_iHL_NN(0x1);
        //         ram:76e6 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:76e7 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:76ea 01 07 00        LD         BC,char_07h_class
        self.instr_hk__LD_BC_NNNN(0x07);
        //         ram:76ed 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:76ee 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:76ef fe 01           CP         0x1
        self.instr_hk__CP_NN(0x1);
        //         ram:76f1 28 03           JR         Z,l_exit_x
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12); //JR l_exit_x;
        } else {
            self.increase_cycles(7);
            //         ram:76f3 fe 03           CP         0x3
            self.instr_hk__CP_NN(0x3);
            //         ram:76f5 c0              RET        NZ
            self.IncPC(1);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(11);
                return true;
            } else {
                self.increase_cycles(5);
            }
        }
        //                              l_exit_x
        //         ram:76f6 13              INC        DE
        self.instr_hk__INC_DE();
        //         ram:76f7 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:76f8 36 01           LD         (HL),offset BYTE_ram_c1be
        self.instr_hk__LD_iHL_NN(0x1);
        //         ram:76fa c9              RET

        self.assert_pc(0x76fa);
        true
    }
    // called from interrupt
    //
    // pub(crate) fn hook_77a2(&mut self) -> bool {
    //     println!("hook_77a2");
    //     //         ram:77a2 cd 99 78        CALL       sb_psg_mem_7899                                  undefined sb_psg_mem_7899()
    //     assert!(self.call_hook(0x7899));
    //     //         ram:77a5 3a 81 c3        LD         A,(DAT_ram_c381)
    //     self.instr_hk__LD_A_iNNNN(0xc381);
    //     //         ram:77a8 b7              OR         A
    //     self.instr_hk__OR_A_A();
    //     //         ram:77a9 20 05           JR         NZ,LAB_ram_77b0
    //     self.IncPC(2);
    //     if (self.data.F & FLAG_Z) == 0 {
    //         self.increase_cycles(12);
    //         // JR(LAB_ram_77b0);
    //         //                              LAB_ram_77b0                                    XREF[1]:     ram:77a9(j)
    //         //         ram:77b0 4f              LD         C,A
    //         self.instr_hk__LD_C_A();
    //         //         ram:77b1 3a 82 c3        LD         A,(DAT_ram_c382)
    //         self.instr_hk__LD_A_iNNNN(0xc382);
    //         //         ram:77b4 3c              INC        A
    //         self.instr_hk__INC_A();
    //         //         ram:77b5 32 82 c3        LD         (DAT_ram_c382),A
    //         self.instr_hk__LD_iNNNN_A(0xc382);
    //         //         ram:77b8 b9              CP         C
    //         self.instr_hk__CP_C();
    //         //         ram:77b9 38 25           JR         C,l_exit
    //         self.IncPC(2);
    //         if (self.data.F & FLAG_C) != 0 {
    //             self.increase_cycles(12);
    //             // JR(l_exit);
    //         } else {
    //             self.increase_cycles(7);
    //             //         ram:77bb af              XOR        A
    //             self.instr_hk__XOR_A_A();
    //             //         ram:77bc 32 82 c3        LD         (DAT_ram_c382),A
    //             self.instr_hk__LD_iNNNN_A(0xc382);
    //             //         ram:77bf dd 21 75 c3     LD         IX,DAT_ram_c375
    //             self.instr_hk__LD_IX_NNNN(0xc375);
    //             //         ram:77c3 0e 00           LD         C,0x0
    //             self.instr_hk__LD_C_NN(0x0);
    //             //         ram:77c5 cd e1 77        CALL       fn_psg_mem_ix_77e1                               undefined fn_psg_mem_ix_77e1(void)
    //             println!("ram:77c5");
    //             assert!(self.call_hook(0x77e1));
    //             println!("ram:77c8");
    //             //         ram:77c8 dd 21 79 c3     LD         IX,DAT_ram_c379
    //             self.instr_hk__LD_IX_NNNN(0xc379);
    //             //         ram:77cc 0e 02           LD         C,0x2
    //             self.instr_hk__LD_C_NN(0x2);
    //             //         ram:77ce cd e1 77        CALL       fn_psg_mem_ix_77e1                               undefined fn_psg_mem_ix_77e1(void)
    //             println!("ram:77ce");
    //             assert!(self.call_hook(0x77e1));
    //             println!("ram:77d1");
    //             //         ram:77d1 3a 73 c3        LD         A,(DAT_ram_c373)
    //             self.instr_hk__LD_A_iNNNN(0xc373);
    //             //         ram:77d4 b7              OR         A
    //             self.instr_hk__OR_A_A();
    //             //         ram:77d5 28 09           JR         Z,l_exit
    //             self.IncPC(2);
    //             if (self.data.F & FLAG_Z) != 0 {
    //                 self.increase_cycles(12);
    //                 // JR(l_exit);
    //             } else {
    //                 self.increase_cycles(7);
    //                 //         ram:77d7 dd 21 7d c3     LD         IX,DAT_ram_c37d
    //                 self.instr_hk__LD_IX_NNNN(0xc37d);
    //                 //         ram:77db 0e 04           LD         C,0x4
    //                 self.instr_hk__LD_C_NN(0x4);
    //                 //         ram:77dd cd e1 77        CALL       fn_psg_mem_ix_77e1                               undefined fn_psg_mem_ix_77e1(void)
    //                 println!("ram:77dd");
    //                 assert!(self.call_hook(0x77e1));
    //                 println!("-ram:77dd");
    //             }
    //         }
    //     } else {
    //         self.increase_cycles(7);
    //         //         ram:77ab cd 7a 78        CALL       fn_psg_mem_787a                                  undefined fn_psg_mem_787a(void)
    //         assert!(self.call_hook(0x787a));
    //         //         ram:77ae 18 30           JR         l_exit
    //         self.IncPC(2);
    //         self.increase_cycles(12);
    //         // JR(l_exit);
    //     }

    //     //                              l_exit                                          XREF[3]:     ram:77ae(j), ram:77b9(j),
    //     //                                                                                           ram:77d5(j)
    //     //         ram:77e0 c9              RET
    //     self.assert_pc(0x77e0);
    //     //
    //     true
    // }
    pub(crate) fn hook_77e1(&mut self) -> bool {
        // println!("hook_77e1");
        // self.peek_stack(5);
        //         ram:77e1 dd 46 00        LD         B,(IX+0x0)
        self.instr_hk__LD_B_iIXpDD(0x0);
        //         ram:77e4 dd 7e 01        LD         A,(IX+0x1)
        self.instr_hk__LD_A_iIXpDD(0x1);
        //         ram:77e7 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:77e8 dd 77 01        LD         (IX+0x1),A
        self.instr_hk__LD_iIXpDD_A(0x1);
        //         ram:77eb b8              CP         B
        self.instr_hk__CP_B();
        //         ram:77ec d8              RET        C
        self.IncPC(1);
        if (self.data.F & FLAG_C) != 0 {
            self.increase_cycles(11);
            return true;
        }
        self.increase_cycles(5);

        //         ram:77ed af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:77ee dd 77 01        LD         (IX+0x1),A
        self.instr_hk__LD_iIXpDD_A(0x1);
        //                              loop_1                                          XREF[1]:     ram:780e(j)
        loop {
            self.SetPC(0x77f1);
            //         ram:77f1 dd 6e 02        LD         L,(IX+0x2)
            self.instr_hk__LD_L_iIXpDD(0x2);
            //         ram:77f4 dd 66 03        LD         H,(IX+0x3)
            self.instr_hk__LD_H_iIXpDD(0x3);
            //         ram:77f7 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:77f8 fe ff           CP         0xff
            self.instr_hk__CP_NN(0xff);
            //         ram:77fa 20 04           JR         NZ,loop_1_chk_cond
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(7);
                //         ram:77fc cd 7a 78        CALL       fn_psg_mem_787a                                  undefined fn_psg_mem_787a(void)
                assert!(self.call_hook(0x787a));
                //         ram:77ff c9              RET
                return true;
            }
            self.increase_cycles(12);
            // JR(loop_1_chk_cond);

            //                              loop_1_chk_cond                                 XREF[2]:     ram:77fa(j),
            //                                                                                           sb_psg_mem_7899:78b0(j)
            //         ram:7800 fe fe           CP         0xfe
            self.instr_hk__CP_NN(0xfe);
            //         ram:7802 20 0c           JR         NZ,LAB_ram_7810
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_7810);
                break;
            } else {
                self.increase_cycles(7);
            }

            //         ram:7804 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7805 5e              LD         E,(HL)
            self.instr_hk__LD_E_iHL();
            //         ram:7806 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7807 56              LD         D,(HL)
            self.instr_hk__LD_D_iHL();
            //         ram:7808 dd 73 02        LD         (IX+0x2),E
            self.instr_hk__LD_iIXpDD_E(0x2);
            //         ram:780b dd 72 03        LD         (IX+0x3),D
            self.instr_hk__LD_iIXpDD_D(0x3);
            //         ram:780e 18 e1           JR         loop_1
            self.IncPC(2);
            self.increase_cycles(12);
            // JR(loop_1);
        }

        //                              LAB_ram_7810                                    XREF[1]:     ram:7802(j)
        //         ram:7810 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:7811 cb 78           BIT        0x7,B
        self.instr_hk__BIT_7_B();
        //         ram:7813 28 07           JR         Z,l_exit_x
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(l_exit_x);
        } else {
            self.increase_cycles(7);
            //         ram:7815 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7816 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:7817 dd 77 00        LD         (IX+0x0),A
            self.instr_hk__LD_iIXpDD_A(0x0);
            //         ram:781a cb b8           RES        0x7,B
            self.instr_hk__RES_7_B();
        }
        self.SetPC(0x781c);

        //                              l_exit_x                                        XREF[1]:     ram:7813(j)
        //         ram:781c 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:781d dd 75 02        LD         (IX+0x2),L
        self.instr_hk__LD_iIXpDD_L(0x2);
        //         ram:7820 dd 74 03        LD         (IX+0x3),H
        self.instr_hk__LD_iIXpDD_H(0x3);
        //         ram:7823 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:7824 cd 28 78        CALL       fn_psg_mem_7828                                  undefined fn_psg_mem_7828(void)
        assert!(self.call_hook(0x7828));
        //         ram:7827 c9              RET
        self.assert_pc(0x7827);
        // return true;
        // println!("~hook_77e1");
        //
        true
    }
    pub(crate) fn hook_7828(&mut self) -> bool {
        // println!("hook_7828");
        // self.peek_stack(5);
        //         ram:7828 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:7829 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:782a c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:782b fe 60           CP         0x60
        self.instr_hk__CP_NN(0x60);
        //         ram:782d 28 25           JR         Z,LAB_ram_7854
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_7854);
            //                              LAB_ram_7854                                    XREF[1]:     ram:782d(j)
            //         ram:7854 cb 39           SRL        C
            self.instr_hk__SRL_C();
            //         ram:7856 3e 08           LD         A,0x8
            self.instr_hk__LD_A_NN(0x8);
            //         ram:7858 81              ADD        A,C
            self.instr_hk__ADD_A_C();
            //         ram:7859 1e 00           LD         E,0x0
            self.instr_hk__LD_E_NN(0x0);
            //         ram:785b cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
            assert!(self.call_hook(0x0093));
        } else {
            self.increase_cycles(7);
            //         ram:782f 6f              LD         L,A
            self.instr_hk__LD_L_A();
            //         ram:7830 26 00           LD         H,0x0
            self.instr_hk__LD_H_NN(0x0);
            //         ram:7832 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:7833 11 82 7b        LD         DE,WORD_ram_7b82                                 = D5Dh
            self.instr_hk__LD_DE_NNNN(0x7b82);
            //         ram:7836 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:7837 5e              LD         E=>WORD_ram_7b82,(HL)                            = D5Dh
            self.instr_hk__LD_E_iHL();
            //         ram:7838 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:7839 56              LD         D,(HL=>WORD_ram_7b82+1)
            self.instr_hk__LD_D_iHL();
            //         ram:783a af              XOR        A
            self.instr_hk__XOR_A_A();
            //         ram:783b 81              ADD        A,C
            self.instr_hk__ADD_A_C();
            //         ram:783c cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
            assert!(self.call_hook(0x0093));
            //                                                                                              Input    : A  - PSG register num
            //                                                                                                         E  - Data write
            //         ram:783f 5a              LD         E,D
            self.instr_hk__LD_E_D();
            //         ram:7840 3e 01           LD         A,0x1
            self.instr_hk__LD_A_NN(0x1);
            //         ram:7842 81              ADD        A,C
            self.instr_hk__ADD_A_C();
            //         ram:7843 cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
            assert!(self.call_hook(0x0093));
            //                                                                                              Input    : A  - PSG register num
            //                                                                                                         E  - Data write
            //         ram:7846 cb 39           SRL        C
            self.instr_hk__SRL_C();
            //         ram:7848 3a 74 c3        LD         A,(DAT_ram_c374)
            self.instr_hk__LD_A_iNNNN(0xc374);
            //         ram:784b 5f              LD         E,A
            self.instr_hk__LD_E_A();
            //         ram:784c 3e 08           LD         A,0x8
            self.instr_hk__LD_A_NN(0x8);
            //         ram:784e 81              ADD        A,C
            self.instr_hk__ADD_A_C();
            //         ram:784f cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
            assert!(self.call_hook(0x0093));
            //                                                                                              Input    : A  - PSG register num
            //                                                                                                         E  - Data write
            //         ram:7852 18 0a           JR         LAB_ram_785e
            self.IncPC(2);
            self.increase_cycles(12);
            // JR(LAB_ram_785e);
        }

        self.SetPC(0x785e);
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        //                              LAB_ram_785e                                    XREF[1]:     ram:7852(j)
        //         ram:785e 1e b8           LD         E,0xb8
        self.instr_hk__LD_E_NN(0xb8);
        //         ram:7860 3a 73 c3        LD         A,(DAT_ram_c373)
        self.instr_hk__LD_A_iNNNN(0xc373);
        //         ram:7863 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:7864 20 02           JR         NZ,LAB_ram_7868
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_7868);
        } else {
            self.increase_cycles(7);

            //         ram:7866 1e bc           LD         E,0xbc
            self.instr_hk__LD_E_NN(0xbc);
        }
        self.SetPC(0x7868);
        //                              LAB_ram_7868                                    XREF[1]:     ram:7864(j)
        //         ram:7868 3e 07           LD         A,0x7
        self.instr_hk__LD_A_NN(0x7);
        //         ram:786a cd 96 00        CALL       bios_RDPSG_0096                                  Function : Reads value from PSG
        assert!(self.call_hook(0x0096));
        //                                                                                              Input    : A  - PSG register read
        //                                                                                              Output   : A  - Value read
        //         ram:786d a3              AND        E
        self.instr_hk__AND_A_E();
        //         ram:786e f6 18           OR         0x18
        self.instr_hk__OR_NN(0x18);
        //         ram:7870 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:7871 3e 07           LD         A,0x7
        self.instr_hk__LD_A_NN(0x7);
        //         ram:7873 cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        //         ram:7876 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:7877 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:7878 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:7879 c9              RET
        self.assert_pc(0x7879);
        // return true;
        // println!("~hook_7828");
        //
        true
    }
    pub(crate) fn hook_787a(&mut self) -> bool {
        println!("hook_787a");
        self.peek_stack(5);
        //         ram:787a cd 82 78        CALL       fn_psg_mem_7882                                  undefined fn_psg_mem_7882(void)
        assert!(self.call_hook(0x7882));
        //         ram:787d af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:787e 32 81 c3        LD         (DAT_ram_c381),A
        self.instr_hk__LD_iNNNN_A(0xc381);
        //         ram:7881 c9              RET
        self.assert_pc(0x7881);
        // return true;
        //
        true
    }
    pub(crate) fn hook_7882(&mut self) -> bool {
        println!("hook_7882");
        self.peek_stack(5);
        //         ram:7882 1e 00           LD         E,0x0
        self.instr_hk__LD_E_NN(0x0);
        //         ram:7884 3e 08           LD         A,0x8
        self.instr_hk__LD_A_NN(0x8);
        //         ram:7886 cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        //         ram:7889 3e 09           LD         A,0x9
        self.instr_hk__LD_A_NN(0x9);
        //         ram:788b cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        //         ram:788e 3a 73 c3        LD         A,(DAT_ram_c373)
        self.instr_hk__LD_A_iNNNN(0xc373);
        //         ram:7891 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:7892 c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:7893 3e 0a           LD         A,0xa
        self.instr_hk__LD_A_NN(0xa);
        //         ram:7895 cd 93 00        CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //                                                                                              Input    : A  - PSG register num
        //                                                                                                         E  - Data write
        self.assert_pc(0x7898);
        //         ram:7898 c9              RET
        // return true;
        println!("~hook_7882");
        //
        true
    }
    pub(crate) fn hook_7899(&mut self) -> bool {
        //         ram:7899 3a  83  c3       LD         A,(BYTE_ram_c383 )
        self.instr_hk__LD_A_iNNNN(0xc383);
        //         ram:789c 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:789d 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:789e 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:789f 26  00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:78a1 01  b1  78       LD         BC,switchD_ram:78b0::switchdataD_ram:78b1        = 79DCh
        self.instr_hk__LD_BC_NNNN(0x78b1);
        //         ram:78a4 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:78a5 5e              LD         E,(HL=>switchD_ram:78b0::switchdataD_ram:78b1 ) = 79DCh
        self.instr_hk__LD_E_iHL();
        //         ram:78a6 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:78a7 56              LD         D,(HL=>switchdataD_ram:78b1+1 )
        self.instr_hk__LD_D_iHL();
        //         ram:78a8 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:78a9 01  0a  0d       LD         BC,0xd0a
        self.instr_hk__LD_BC_NNNN(0xd0a);
        //         ram:78ac 3a  84  c3       LD         A,(BYTE_ram_c384 )
        self.instr_hk__LD_A_iNNNN(0xc384);
        //         ram:78af b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:78b0 e9              JP         (HL)                                             Function : Writes data to PSG re
        self.instr_hk__JP_HL();
        //                                                                                              Input    : A  - PSG register num
        true
    }
    pub(crate) fn hook_78eb(&mut self) -> bool {
        //         ram:78eb 1e  00           LD         E,0x0
        self.instr_hk__LD_E_NN(0x0);
        //         ram:78ed 3e  0a           LD         A,0xa
        self.instr_hk__LD_A_NN(0xa);
        //         ram:78ef cd  93  00       CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //         ram:78f2 3e  07           LD         A,0x7
        self.instr_hk__LD_A_NN(0x7);
        //         ram:78f4 cd  96  00       CALL       bios_RDPSG_0096                                  Function : Reads value from PSG
        assert!(self.call_hook(0x0096));
        //         ram:78f7 f6  a4           OR         0xa4
        self.instr_hk__OR_NN(0xa4);
        //         ram:78f9 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:78fa 3e  07           LD         A,0x7
        self.instr_hk__LD_A_NN(0x7);
        //         ram:78fc cd  93  00       CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
        assert!(self.call_hook(0x0093));
        //         ram:78ff af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:7900 32  84  c3       LD         (BYTE_ram_c384 ),A
        self.instr_hk__LD_iNNNN_A(0xc384);
        //         ram:7903 32  83  c3       LD         (BYTE_ram_c383 ),A
        self.instr_hk__LD_iNNNN_A(0xc383);
        //         ram:7906 c9              RET
        self.assert_pc(0x7906);
        // return true;
        //
        true
    }
    pub(crate) fn hook_7b37(&mut self) -> bool {
        //         ram:7b37 f3              DI
        self.instr_hk__DI();
        //         ram:7b38 3a  83  c3       LD         A,(BYTE_ram_c383 )
        self.instr_hk__LD_A_iNNNN(0xc383);
        //         ram:7b3b fe  0d           CP         0xd
        self.instr_hk__CP_NN(0xd);
        //         ram:7b3d 28  1a           JR         Z,LAB_ram_7b59
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_7b59);
        } else {
            self.increase_cycles(7);
            //         ram:7b3f b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:7b40 28  06           JR         Z,LAB_ram_7b48
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_7b48);
                self.SetPC(0x7b48);
                //                              LAB_ram_7b48                                    XREF[1]:     ram:7b40 (j)
                //         ram:7b48 7a              LD         A,D
                self.instr_hk__LD_A_D();
                //         ram:7b49 32  83  c3       LD         (BYTE_ram_c383 ),A
                self.instr_hk__LD_iNNNN_A(0xc383);
                //         ram:7b4c 1e  00           LD         E,0x0
                self.instr_hk__LD_E_NN(0x0);
                //         ram:7b4e 3e  0b           LD         A,0xb
                self.instr_hk__LD_A_NN(0xb);
                //         ram:7b50 cd  93  00       CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
                assert!(self.call_hook(0x0093));
                //         ram:7b53 1e  35           LD         E,0x35
                self.instr_hk__LD_E_NN(0x35);
                //         ram:7b55 3c              INC        A
                self.instr_hk__INC_A();
                //         ram:7b56 cd  93  00       CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
                assert!(self.call_hook(0x0093));
                //                              LAB_ram_7b59                                    XREF[2]:     ram:7b3d (j) , ram:7b43 (j)
                //         ram:7b59 fb              EI
                self.instr_hk__EI();
                //         ram:7b5a c9              RET
                self.assert_pc(0x7b5a);
                // return true;
                return true;
            } else {
                self.increase_cycles(7);
            }

            //         ram:7b42 ba              CP         D
            self.instr_hk__CP_D();
            //         ram:7b43 30  14           JR         NC,LAB_ram_7b59
            self.IncPC(2);
            if (self.data.F & FLAG_C) == 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_7b59);
            } else {
                self.increase_cycles(7);
                //         ram:7b45 cd  eb  78       CALL       fn_psg_mem_78eb                                  undefined fn_psg_mem_78eb(void)
                assert!(self.call_hook(0x78eb));
                //                              LAB_ram_7b48                                    XREF[1]:     ram:7b40 (j)
                //         ram:7b48 7a              LD         A,D
                self.instr_hk__LD_A_D();
                //         ram:7b49 32  83  c3       LD         (BYTE_ram_c383 ),A
                self.instr_hk__LD_iNNNN_A(0xc383);
                //         ram:7b4c 1e  00           LD         E,0x0
                self.instr_hk__LD_E_NN(0x0);
                //         ram:7b4e 3e  0b           LD         A,0xb
                self.instr_hk__LD_A_NN(0xb);
                //         ram:7b50 cd  93  00       CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
                assert!(self.call_hook(0x0093));
                //         ram:7b53 1e  35           LD         E,0x35
                self.instr_hk__LD_E_NN(0x35);
                //         ram:7b55 3c              INC        A
                self.instr_hk__INC_A();
                //         ram:7b56 cd  93  00       CALL       bios_WRTPSG_0093                                 Function : Writes data to PSG re
                assert!(self.call_hook(0x0093));
            }
        }

        self.SetPC(0x7b59);
        //                              LAB_ram_7b59                                    XREF[2]:     ram:7b3d (j) , ram:7b43 (j)
        //         ram:7b59 fb              EI
        self.instr_hk__EI();
        //         ram:7b5a c9              RET
        self.assert_pc(0x7b5a);
        // return true;
        //
        true
    }
    pub(crate) fn hook_7b5b(&mut self) -> bool {
        println!("hook_7b5b");
        self.peek_stack(5);
        //         ram:7b5b f3              DI
        self.instr_hk__DI();
        //         ram:7b5c 32 81 c3        LD         (DAT_ram_c381),A
        self.instr_hk__LD_iNNNN_A(0xc381);
        //         ram:7b5f 22 77 c3        LD         (DAT_ram_c377),HL
        self.instr_hk__LD_iNNNN_HL(0xc377);
        //         ram:7b62 ed 53 7b c3     LD         (DAT_ram_c37b),DE
        self.instr_hk__LD_iNNNN_DE(0xc37b);
        //         ram:7b66 ed 43 7f c3     LD         (DAT_ram_c37f),BC
        self.instr_hk__LD_iNNNN_BC(0xc37f);
        //         ram:7b6a cd 82 78        CALL       fn_psg_mem_7882                                  undefined fn_psg_mem_7882(void)
        assert!(self.call_hook(0x7882));
        //         ram:7b6d af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:7b6e 32 75 c3        LD         (DAT_ram_c375),A
        self.instr_hk__LD_iNNNN_A(0xc375);
        //         ram:7b71 32 79 c3        LD         (DAT_ram_c379),A
        self.instr_hk__LD_iNNNN_A(0xc379);
        //         ram:7b74 32 7d c3        LD         (DAT_ram_c37d),A
        self.instr_hk__LD_iNNNN_A(0xc37d);
        //         ram:7b77 32 76 c3        LD         (DAT_ram_c376),A
        self.instr_hk__LD_iNNNN_A(0xc376);
        //         ram:7b7a 32 7a c3        LD         (DAT_ram_c37a),A
        self.instr_hk__LD_iNNNN_A(0xc37a);
        //         ram:7b7d 32 7e c3        LD         (DAT_ram_c37e),A
        self.instr_hk__LD_iNNNN_A(0xc37e);
        //         ram:7b80 fb              EI
        self.instr_hk__EI();
        //         ram:7b81 c9              RET
        self.assert_pc(0x7b81);
        // return true;
        println!("~hook_7b5b");
        //
        true
    }
    pub(crate) fn hook_7f80(&mut self) -> bool {
        println!("hook_7f80");

        //                              *************************************************************
        //                              *                           FUNCTION
        //                              *************************************************************
        //                              undefined  FUN_ram_7f80 ()
        //              undefined         A:1            <RETURN>
        //                              FUN_ram_7f80                                          ,
        //                                                                                           FUN_ram_8018:8049 (c) ,
        //
        loop {
            self.SetPC(0x7f80);
            //         ram:7f80 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:7f81 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:7f82 cd  bc  89       CALL       fn_calc_voffset_89BC
            assert!(self.call_hook(0x89bc));

            //         ram:7f85 11  ac  c3       LD         DE,DAT_ram_c3ac
            self.instr_hk__LD_DE_NNNN(0xc3ac);
            //         ram:7f88 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:7f89 7e              LD         A,(HL=>DAT_ram_c3ac )
            self.instr_hk__LD_A_iHL();
            //         ram:7f8a e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:7f8b c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:7f8c b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:7f8d 28  0e           JR         Z,LAB_ram_7f9d
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_7f9d);
                //                              LAB_ram_7f9d
                //         ram:7f9d e5              PUSH       HL
                self.instr_hk__PUSH_HL();
                //         ram:7f9e c5              PUSH       BC
                self.instr_hk__PUSH_BC();
                //         ram:7f9f cd  0e  b6       CALL       sb_rand_guess_B60E
                assert!(self.call_hook(0xb60e));
                //         ram:7fa2 c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:7fa3 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:7fa4 e6  3f           AND        0x3f
                self.instr_hk__AND_NN(0x3f);
                //         ram:7fa6 20  e7           JR         NZ,LAB_ram_7f8f
                self.IncPC(2);
                if (self.data.F & FLAG_Z) == 0 {
                    self.increase_cycles(12);
                    // JR(LAB_ram_7f8f);
                } else {
                    self.increase_cycles(7);
                    //         ram:7fa8 3a  e6  c1       LD         A,(BYTE_ram_c1e6 )
                    self.instr_hk__LD_A_iNNNN(0xc1e6);
                    //         ram:7fab cb  3f           SRL        A
                    self.instr_hk__SRL_A();
                    //         ram:7fad cb  3f           SRL        A
                    self.instr_hk__SRL_A();
                    //         ram:7faf 47              LD         B,A
                    self.instr_hk__LD_B_A();
                    //         ram:7fb0 7c              LD         A,H
                    self.instr_hk__LD_A_H();
                    //         ram:7fb1 90              SUB        B
                    self.instr_hk__SUB_A_B();
                    //         ram:7fb2 87              ADD        A,A
                    self.instr_hk__ADD_A_A();
                    //         ram:7fb3 87              ADD        A,A
                    self.instr_hk__ADD_A_A();
                    //         ram:7fb4 67              LD         H,A
                    self.instr_hk__LD_H_A();
                    //         ram:7fb5 3a  e6  c1       LD         A,(BYTE_ram_c1e6 )
                    self.instr_hk__LD_A_iNNNN(0xc1e6);
                    //         ram:7fb8 e6  03           AND        0x3
                    self.instr_hk__AND_NN(0x3);
                    //         ram:7fba ed  44           NEG
                    self.instr_hk__NEG();
                    //         ram:7fbc 84              ADD        A,H
                    self.instr_hk__ADD_A_H();
                    //         ram:7fbd 32  bc  c8       LD         (BYTE_ram_c8bc ),A
                    self.instr_hk__LD_iNNNN_A(0xc8bc);
                    //         ram:7fc0 3a  e7  c1       LD         A,(BYTE_ram_c1e7 )
                    self.instr_hk__LD_A_iNNNN(0xc1e7);
                    //         ram:7fc3 cb  3f           SRL        A
                    self.instr_hk__SRL_A();
                    //         ram:7fc5 cb  3f           SRL        A
                    self.instr_hk__SRL_A();
                    //         ram:7fc7 4f              LD         C,A
                    self.instr_hk__LD_C_A();
                    //         ram:7fc8 7d              LD         A,L
                    self.instr_hk__LD_A_L();
                    //         ram:7fc9 91              SUB        C
                    self.instr_hk__SUB_A_C();
                    //         ram:7fca 87              ADD        A,A
                    self.instr_hk__ADD_A_A();
                    //         ram:7fcb 87              ADD        A,A
                    self.instr_hk__ADD_A_A();
                    //         ram:7fcc 6f              LD         L,A
                    self.instr_hk__LD_L_A();
                    //         ram:7fcd 3a  e7  c1       LD         A,(BYTE_ram_c1e7 )
                    self.instr_hk__LD_A_iNNNN(0xc1e7);
                    //         ram:7fd0 e6  03           AND        0x3
                    self.instr_hk__AND_NN(0x3);
                    //         ram:7fd2 ed  44           NEG
                    self.instr_hk__NEG();
                    //         ram:7fd4 85              ADD        A,L
                    self.instr_hk__ADD_A_L();
                    //         ram:7fd5 32  bd  c8       LD         (BYTE_ram_c8bd ),A
                    self.instr_hk__LD_iNNNN_A(0xc8bd);
                    //         ram:7fd8 af              XOR        A
                    self.instr_hk__XOR_A_A();
                    //         ram:7fd9 21  da  c8       LD         HL,BYTE_ram_c8da
                    self.instr_hk__LD_HL_NNNN(0xc8da);
                    //                              LAB_ram_7fdc
                    loop {
                        self.SetPC(0x7fdc);
                        //         ram:7fdc 4f              LD         C,A
                        self.instr_hk__LD_C_A();
                        //         ram:7fdd 3a  b7  c8       LD         A,(BYTE_ram_c8b7 )
                        self.instr_hk__LD_A_iNNNN(0xc8b7);
                        //         ram:7fe0 b9              CP         C
                        self.instr_hk__CP_C();
                        //         ram:7fe1 ca  0c  80       JP         Z,LAB_ram_800c
                        self.IncPC(3);
                        self.increase_cycles(10);
                        if (self.data.F & FLAG_Z) != 0 {
                            // JP(LAB_ram_800c);
                        } else {
                            //         ram:7fe4 79              LD         A,C
                            self.instr_hk__LD_A_C();
                            //         ram:7fe5 f5              PUSH       AF
                            self.instr_hk__PUSH_AF();
                            //         ram:7fe6 7e              LD         A,(HL=>BYTE_ram_c8da )
                            self.instr_hk__LD_A_iHL();
                            //         ram:7fe7 3c              INC        A
                            self.instr_hk__INC_A();
                            //         ram:7fe8 28  21           JR         Z,LAB_ram_800b
                            self.IncPC(2);
                            if (self.data.F & FLAG_Z) != 0 {
                                self.increase_cycles(12);
                            } else {
                                self.increase_cycles(7);
                                //         ram:7fea e5              PUSH       HL=>BYTE_ram_c8da
                                self.instr_hk__PUSH_HL();
                                //         ram:7feb 23              INC        HL
                                self.instr_hk__INC_HL();
                                //         ram:7fec 56              LD         D,(HL=>BYTE_ram_c8db )
                                self.instr_hk__LD_D_iHL();
                                //         ram:7fed 23              INC        HL
                                self.instr_hk__INC_HL();
                                //         ram:7fee 5e              LD         E,(HL=>BYTE_ram_c8dc )
                                self.instr_hk__LD_E_iHL();
                                //         ram:7fef e1              POP        HL
                                self.instr_hk__POP_HL();
                                //         ram:7ff0 3a  bc  c8       LD         A,(BYTE_ram_c8bc )
                                self.instr_hk__LD_A_iNNNN(0xc8bc);
                                //         ram:7ff3 92              SUB        D
                                self.instr_hk__SUB_A_D();
                                //         ram:7ff4 30  02           JR         NC,LAB_ram_7ff8
                                self.IncPC(2);
                                if (self.data.F & FLAG_C) == 0 {
                                    self.increase_cycles(12);
                                    // JR(LAB_ram_7ff8);
                                } else {
                                    self.increase_cycles(7);
                                    //         ram:7ff6 ed  44           NEG
                                    self.instr_hk__NEG();
                                }

                                //                              LAB_ram_7ff8
                                //         ram:7ff8 fe  04           CP         0x4
                                self.instr_hk__CP_NN(0x4);
                                //         ram:7ffa 30  0f           JR         NC,LAB_ram_800b
                                self.IncPC(2);
                                if (self.data.F & FLAG_C) == 0 {
                                    self.increase_cycles(12);
                                } else {
                                    self.increase_cycles(7);
                                    //         ram:7ffc 3a  bd  c8       LD         A,(BYTE_ram_c8bd )
                                    self.instr_hk__LD_A_iNNNN(0xc8bd);
                                    //         ram:7fff 93              SUB        E
                                    self.instr_hk__SUB_A_E();
                                    //         ram:8000 30  02           JR         NC,LAB_ram_8004
                                    self.IncPC(2);
                                    if (self.data.F & FLAG_C) == 0 {
                                        self.increase_cycles(12);
                                        // JR(LAB_ram_8004);
                                    } else {
                                        self.increase_cycles(7);
                                        //         ram:8002 ed  44           NEG
                                        self.instr_hk__NEG();
                                    }

                                    //                              LAB_ram_8004
                                    //         ram:8004 fe  04           CP         0x4
                                    self.instr_hk__CP_NN(0x4);
                                    //         ram:8006 30  03           JR         NC,LAB_ram_800b
                                    self.IncPC(2);
                                    if (self.data.F & FLAG_C) != 0 {
                                        self.increase_cycles(7);
                                        //         ram:8008 f1              POP        AF
                                        self.instr_hk__POP_AF();
                                        //         ram:8009 37              SCF
                                        self.instr_hk__SCF();
                                        //         ram:800a c9              RET
                                        return true;
                                    }
                                    self.increase_cycles(12);
                                }
                            }

                            //                              LAB_ram_800b                                          ,  ,
                            //
                            //         ram:800b f1              POP        AF
                            self.instr_hk__POP_AF();
                        }

                        //                              LAB_ram_800c
                        //         ram:800c 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:800d 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:800e 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:800f 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:8010 3c              INC        A
                        self.instr_hk__INC_A();
                        //         ram:8011 fe  08           CP         0x8
                        self.instr_hk__CP_NN(0x8);
                        //         ram:8013 c2  dc  7f       JP         NZ,LAB_ram_7fdc
                        self.IncPC(3);
                        self.increase_cycles(10);
                        if (self.data.F & FLAG_Z) == 0 {
                            // JP(LAB_ram_7fdc);
                        } else {
                            //         ram:8016 b7              OR         A
                            self.instr_hk__OR_A_A();
                            //         ram:8017 c9              RET

                            return true;
                        }
                    }
                }
            } else {
                self.increase_cycles(7);
            }

            //                              LAB_ram_7f8f
            //         ram:7f8f d9              EXX
            self.instr_hk__EXX();
            //         ram:7f90 7c              LD         A,H
            self.instr_hk__LD_A_H();
            //         ram:7f91 d9              EXX
            self.instr_hk__EXX();
            //         ram:7f92 84              ADD        A,H
            self.instr_hk__ADD_A_H();
            //         ram:7f93 67              LD         H,A
            self.instr_hk__LD_H_A();
            //         ram:7f94 d9              EXX
            self.instr_hk__EXX();
            //         ram:7f95 7d              LD         A,L
            self.instr_hk__LD_A_L();
            //         ram:7f96 d9              EXX
            self.instr_hk__EXX();
            //         ram:7f97 85              ADD        A,L
            self.instr_hk__ADD_A_L();
            //         ram:7f98 6f              LD         L,A
            self.instr_hk__LD_L_A();
            //         ram:7f99 10  e5           DJNZ       FUN_ram_7f80
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP FUN_ram_7f80;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:7f9b 37              SCF
        self.instr_hk__SCF();
        //         ram:7f9c c9              RET
        true
    }
    pub(crate) fn hook_8018(&mut self) -> bool {
        println!("hook_8018");

        //                              **************************************************************
        //                              *                          FUNCTION                          *
        //                              **************************************************************
        //                              ()
        //                                                                                           hl

        //              undefined         A:1            <RETURN>
        //                              FUN_ram_8018

        //         ram:8018 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //                                                                                                 hl

        //         ram:8019 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:801a f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:801b 3e 08           LD         A,0x8
        self.instr_hk__LD_A_NN(0x8);
        //         ram:801d 90              SUB        B
        self.instr_hk__SUB_A_B();
        //         ram:801e 32 b7 c8        LD         (BYTE_ram_c8b7),A
        self.instr_hk__LD_iNNNN_A(0xc8b7);
        //         ram:8021 cd 82 ae        CALL       sb_calc_mem_set_AE82                             ()
        assert!(self.call_hook(0xae82));
        //         ram:8024 cd c6 47        CALL       sb_set_addr_base_47C6                            ()
        assert!(self.call_hook(0x47c6));
        //         ram:8027 3a e6 c1        LD         A,(BYTE_ram_c1e6)
        self.instr_hk__LD_A_iNNNN(0xc1e6);
        //         ram:802a cb 3f           SRL        A
        self.instr_hk__SRL_A();
        //         ram:802c cb 3f           SRL        A
        self.instr_hk__SRL_A();
        //         ram:802e 67              LD         H,A
        self.instr_hk__LD_H_A();
        //         ram:802f 3a e7 c1        LD         A,(BYTE_ram_c1e7)
        self.instr_hk__LD_A_iNNNN(0xc1e7);
        //         ram:8032 cb 3f           SRL        A
        self.instr_hk__SRL_A();
        //         ram:8034 cb 3f           SRL        A
        self.instr_hk__SRL_A();
        //         ram:8036 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:8037 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:8038 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:8039 c2 55 80        JP         NZ,LAB_ram_8055
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            // JP(LAB_ram_8055);
            //                              LAB_ram_8055                                         FUN_ram_8018:8039(j)
            //         ram:8055 fe 01           CP         0x1
            self.instr_hk__CP_NN(0x1);
            //         ram:8057 c2 65 80        JP         NZ,loop_1_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP(loop_1_chk_cond);
                //                              loop_1_chk_cond
                //         ram:8065 fe 02           CP         0x2
                self.instr_hk__CP_NN(0x2);
                //         ram:8067 c2 75 80        JP         NZ,LAB_ram_8075
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP(LAB_ram_8075);
                    //                              LAB_ram_8075
                    //         ram:8075 7c              LD         A,H
                    self.instr_hk__LD_A_H();
                    //         ram:8076 b7              OR         A
                    self.instr_hk__OR_A_A();
                    //         ram:8077 ca 8c 80        JP         Z,l_exit_x_ff
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_Z) != 0 {
                        // JP(l_exit_x_ff);
                    } else {
                        //         ram:807a 25              DEC        H
                        self.instr_hk__DEC_H();
                        // JP(LAB_ram_807b);
                        // break;
                        //                              LAB_ram_807b
                        //         ram:807b 06 05           LD         B,0x5
                        self.instr_hk__LD_B_NN(0x5);
                        //         ram:807d d9              EXX
                        self.instr_hk__EXX();
                        //         ram:807e 21 01 00        LD         HL,0x1
                        self.instr_hk__LD_HL_NNNN(0x1);
                        //         ram:8081 d9              EXX
                        self.instr_hk__EXX();
                        //         ram:8082 cd 80 7f        CALL       FUN_ram_7f80                                     ()
                        assert!(self.call_hook(0x7f80));
                        //         ram:8085 38 05           JR         C,l_exit_x_ff
                        self.IncPC(2);
                        if (self.data.F & FLAG_C) != 0 {
                            self.increase_cycles(12);
                            // JR(l_exit_x_ff);
                        } else {
                            self.increase_cycles(7);
                            //         ram:8087 21 bc c8        LD         HL,BYTE_ram_c8bc
                            self.instr_hk__LD_HL_NNNN(0xc8bc);
                            //         ram:808a 18 05           JR         l_exit_x
                            self.IncPC(2);
                            self.increase_cycles(12);
                            // JR(l_exit_x);
                            //                              l_exit_x                                             FUN_ram_8018:8052(j),

                            //         ram:8091 cd 5d b3        CALL       sb_set_mem_fill_vram_guess_B35D
                            assert!(self.call_hook(0xb35d));
                            //         ram:8094 c1              POP        BC
                            self.instr_hk__POP_BC();
                            //         ram:8095 e1              POP        HL
                            self.instr_hk__POP_HL();
                            //         ram:8096 c9              RET

                            println!("~hook_8018");
                            return true;
                        }
                    }
                    // JP(l_exit_x_ff);
                    //                              l_exit_x_ff                                          FUN_ram_8018:803e(j),
                    //         ram:808c c1              POP        BC
                    self.instr_hk__POP_BC();
                    //         ram:808d e1              POP        HL
                    self.instr_hk__POP_HL();
                    //         ram:808e 36 ff           LD         (HL),0xff
                    self.instr_hk__LD_iHL_NN(0xff);
                    //         ram:8090 c9              RET
                    println!("~hook_8018_1");
                    return true;
                }

                //         ram:806a 7d              LD         A,L
                self.instr_hk__LD_A_L();
                //         ram:806b fe 18           CP         0x18
                self.instr_hk__CP_NN(0x18);
                //         ram:806d d2 8c 80        JP         NC,l_exit_x_ff
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) == 0 {
                    // JP(l_exit_x_ff);
                    //                              l_exit_x_ff                                          FUN_ram_8018:803e(j),
                    //         ram:808c c1              POP        BC
                    self.instr_hk__POP_BC();
                    //         ram:808d e1              POP        HL
                    self.instr_hk__POP_HL();
                    //         ram:808e 36 ff           LD         (HL),0xff
                    self.instr_hk__LD_iHL_NN(0xff);
                    //         ram:8090 c9              RET
                    println!("~hook_8018_2");
                    return true;
                }

                //         ram:8070 c6 06           ADD        A,0x6
                self.instr_hk__ADD_A_NN(0x6);
                //         ram:8072 6f              LD         L,A
                self.instr_hk__LD_L_A();
                //         ram:8073 18 cd           JR         FUN_ram_8018::loop_1
                self.IncPC(2);
                self.increase_cycles(12);
                // JR(FUN_ram_8018::loop_1);
                // continue;
            } else {
                //         ram:805a 7c              LD         A,H
                self.instr_hk__LD_A_H();
                //         ram:805b fe 15           CP         0x15
                self.instr_hk__CP_NN(0x15);
                //         ram:805d d2 8c 80        JP         NC,l_exit_x_ff
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_C) == 0 {
                    // JP(l_exit_x_ff);
                } else {
                    //         ram:8060 c6 09           ADD        A,0x9
                    self.instr_hk__ADD_A_NN(0x9);
                    //         ram:8062 67              LD         H,A
                    self.instr_hk__LD_H_A();
                    //         ram:8063 18 16           JR         LAB_ram_807b
                    self.IncPC(2);
                    self.increase_cycles(12);
                    // JR(LAB_ram_807b);
                    // break;
                    //                              LAB_ram_807b
                    //         ram:807b 06 05           LD         B,0x5
                    self.instr_hk__LD_B_NN(0x5);
                    //         ram:807d d9              EXX
                    self.instr_hk__EXX();
                    //         ram:807e 21 01 00        LD         HL,0x1
                    self.instr_hk__LD_HL_NNNN(0x1);
                    //         ram:8081 d9              EXX
                    self.instr_hk__EXX();
                    //         ram:8082 cd 80 7f        CALL       FUN_ram_7f80                                     ()
                    assert!(self.call_hook(0x7f80));
                    //         ram:8085 38 05           JR         C,l_exit_x_ff
                    self.IncPC(2);
                    if (self.data.F & FLAG_C) != 0 {
                        self.increase_cycles(12);
                        // JR(l_exit_x_ff);
                    } else {
                        self.increase_cycles(7);
                        //         ram:8087 21 bc c8        LD         HL,BYTE_ram_c8bc
                        self.instr_hk__LD_HL_NNNN(0xc8bc);
                        //         ram:808a 18 05           JR         l_exit_x
                        self.IncPC(2);
                        self.increase_cycles(12);
                        // JR(l_exit_x);
                        //                              l_exit_x                                             FUN_ram_8018:8052(j),

                        //         ram:8091 cd 5d b3        CALL       sb_set_mem_fill_vram_guess_B35D
                        assert!(self.call_hook(0xb35d));
                        //         ram:8094 c1              POP        BC
                        self.instr_hk__POP_BC();
                        //         ram:8095 e1              POP        HL
                        self.instr_hk__POP_HL();
                        //         ram:8096 c9              RET

                        println!("~hook_8018_3");
                        return true;
                    }
                }
                //                              l_exit_x_ff                                          FUN_ram_8018:803e(j),
                //         ram:808c c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:808d e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:808e 36 ff           LD         (HL),0xff
                self.instr_hk__LD_iHL_NN(0xff);
                //         ram:8090 c9              RET
                println!("~hook_8018_4");
                return true;
            }
        } else {
            //         ram:803c 7d              LD         A,L
            self.instr_hk__LD_A_L();
            //         ram:803d b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:803e ca 8c 80        JP         Z,l_exit_x_ff
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(l_exit_x_ff);
                //                              l_exit_x_ff                                          FUN_ram_8018:803e(j),
                //         ram:808c c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:808d e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:808e 36 ff           LD         (HL),0xff
                self.instr_hk__LD_iHL_NN(0xff);
                //         ram:8090 c9              RET
                println!("~hook_8018_5");
                return true;
            }

            //         ram:8041 2d              DEC        L
            self.instr_hk__DEC_L();
        }

        //                              loop_1
        // loop {
        //         ram:8042 06 08           LD         B,0x8
        self.instr_hk__LD_B_NN(0x8);
        //         ram:8044 d9              EXX
        self.instr_hk__EXX();
        //         ram:8045 21 00 01        LD         HL,0x100
        self.instr_hk__LD_HL_NNNN(0x100);
        //         ram:8048 d9              EXX
        self.instr_hk__EXX();
        //         ram:8049 cd 80 7f        CALL       FUN_ram_7f80                                     ()
        assert!(self.call_hook(0x7f80));
        //         ram:804c da 8c 80        JP         C,l_exit_x_ff
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) != 0 {
            // JP(l_exit_x_ff);
            //                              l_exit_x_ff                                          FUN_ram_8018:803e(j),
            //         ram:808c c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:808d e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:808e 36 ff           LD         (HL),0xff
            self.instr_hk__LD_iHL_NN(0xff);
            //         ram:8090 c9              RET
            println!("~hook_8018_6");
            true
        } else {
            //         ram:804f 21 bd c8        LD         HL,BYTE_ram_c8bd
            self.instr_hk__LD_HL_NNNN(0xc8bd);
            //         ram:8052 c3 91 80        JP         l_exit_x
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(l_exit_x);
            //                              l_exit_x                                             FUN_ram_8018:8052(j),

            //         ram:8091 cd 5d b3        CALL       sb_set_mem_fill_vram_guess_B35D
            assert!(self.call_hook(0xb35d));
            //         ram:8094 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:8095 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:8096 c9              RET

            println!("~hook_8018_7");
            true
        }
        // }
    }
    pub(crate) fn hook_8097(&mut self) -> bool {
        //         ram:8097 21 da c8        LD         HL,BYTE_ram_c8da
        self.instr_hk__LD_HL_NNNN(0xc8da);
        //         ram:809a 06 08           LD         B,0x8
        self.instr_hk__LD_B_NN(0x8);
        //                              loop_1                                          XREF[1]:     ram:80a8(j)
        loop {
            self.SetPC(0x809c);
            //         ram:809c 7e              LD         A,(HL=>BYTE_ram_c8da)
            self.instr_hk__LD_A_iHL();
            //         ram:809d 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:809e 20 03           JR         NZ,loop_1_chk_cond
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR(loop_1_chk_cond);
            } else {
                self.increase_cycles(7);
                //         ram:80a0 cd 18 80        CALL       FUN_ram_8018                                     IN a
                assert!(self.call_hook(0x8018));
                //                                                                                                 hl
                //                                                                                              OUT hl: addr
            }

            //                              loop_1_chk_cond                                 XREF[1]:     ram:809e(j)
            //         ram:80a3 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80a4 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80a5 34              INC        (HL=>BYTE_ram_c8dc)
            self.instr_hk__INC_iHL();
            //         ram:80a6 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80a7 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80a8 10 f2           DJNZ       loop_1
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_1;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:80aa 21 d2 c1        LD         HL,BYTE_ram_c1d2                                 = FFh
        self.instr_hk__LD_HL_NNNN(0xc1d2);
        //         ram:80ad 06 04           LD         B,0x4
        self.instr_hk__LD_B_NN(0x4);
        //                              loop_2                                          XREF[1]:     ram:80b4(j)
        loop {
            self.SetPC(0x80af);
            //         ram:80af 34              INC        (HL=>BYTE_ram_c1d2)                              = FFh
            self.instr_hk__INC_iHL();
            //         ram:80b0 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80b1 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80b2 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80b3 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80b4 10 f9           DJNZ       loop_2
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_2;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:80b6 21 e7 c1        LD         HL,BYTE_ram_c1e7
        self.instr_hk__LD_HL_NNNN(0xc1e7);
        //         ram:80b9 35              DEC        (HL=>BYTE_ram_c1e7)
        self.instr_hk__DEC_iHL();
        //         ram:80ba 3e 02           LD         A,0x2
        self.instr_hk__LD_A_NN(0x2);
        //         ram:80bc cd 40 81        CALL       sb_calc_change_mem_8140                          IN a
        assert!(self.call_hook(0x8140));
        //         ram:80bf c9              RET
        self.assert_pc(0x80bf);
        // return true;
        //
        true
    }
    pub(crate) fn hook_80c0(&mut self) -> bool {
        println!("hook_80c0");

        //                              *************************************************************
        //                              *                           FUNCTION
        //                              *************************************************************
        //                              undefined  FUN_ram_80c0 ()
        //              undefined         A:1            <RETURN>
        //                              FUN_ram_80c0
        //         ram:80c0 21  da  c8       LD         HL,BYTE_ram_c8da
        self.instr_hk__LD_HL_NNNN(0xc8da);
        //         ram:80c3 06  08           LD         B,0x8
        self.instr_hk__LD_B_NN(0x8);
        //                              LAB_ram_80c5
        loop {
            //         ram:80c5 7e              LD         A,(HL=>BYTE_ram_c8da )
            self.instr_hk__LD_A_iHL();
            //         ram:80c6 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:80c7 20  05           JR         NZ,LAB_ram_80ce
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_80ce);
            } else {
                self.increase_cycles(7);
                //         ram:80c9 3e  02           LD         A,0x2
                self.instr_hk__LD_A_NN(0x2);
                //         ram:80cb cd  18  80       CALL       FUN_ram_8018
                assert!(self.call_hook(0x8018));
            }

            //                              LAB_ram_80ce
            //         ram:80ce 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80cf 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80d0 35              DEC        (HL=>BYTE_ram_c8dc )
            self.instr_hk__DEC_iHL();
            //         ram:80d1 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80d2 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80d3 10  f0           DJNZ       LAB_ram_80c5
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_80c5;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:80d5 21  d2  c1       LD         HL,BYTE_ram_c1d2
        self.instr_hk__LD_HL_NNNN(0xc1d2);
        //         ram:80d8 06  04           LD         B,0x4
        self.instr_hk__LD_B_NN(0x4);
        //                              LAB_ram_80da
        loop {
            self.SetPC(0x80da);
            //         ram:80da 35              DEC        (HL=>BYTE_ram_c1d2 )
            self.instr_hk__DEC_iHL();
            //         ram:80db 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80dc 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80dd 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80de 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80df 10  f9           DJNZ       LAB_ram_80da
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_80da;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        self.SetPC(0x80e1);

        //         ram:80e1 21  e7  c1       LD         HL,BYTE_ram_c1e7
        self.instr_hk__LD_HL_NNNN(0xc1e7);
        //         ram:80e4 34              INC        (HL=>BYTE_ram_c1e7 )
        self.instr_hk__INC_iHL();
        //         ram:80e5 af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:80e6 cd  40  81       CALL       sb_calc_change_mem_8140
        assert!(self.call_hook(0x8140));
        //         ram:80e9 c9              RET
        self.assert_pc(0x80e9);

        true
    }
    pub(crate) fn hook_80ea(&mut self) -> bool {
        //         ram:80ea 21 da c8        LD         HL,BYTE_ram_c8da
        self.instr_hk__LD_HL_NNNN(0xc8da);
        //         ram:80ed 06 08           LD         B,0x8
        self.instr_hk__LD_B_NN(0x8);
        //                              loop_1                                          XREF[1]:     ram:80fd(j)
        loop {
            //         ram:80ef 7e              LD         A,(HL=>BYTE_ram_c8da)
            self.instr_hk__LD_A_iHL();
            //         ram:80f0 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:80f1 20 05           JR         NZ,loop_1_chk_cond
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR(loop_1_chk_cond);
            } else {
                self.increase_cycles(7);
                //         ram:80f3 3e 01           LD         A,0x1
                self.instr_hk__LD_A_NN(0x1);
                //         ram:80f5 cd 18 80        CALL       FUN_ram_8018                                     IN a
                assert!(self.call_hook(0x8018));
                //                                                                                                 hl
                //                                                                                              OUT hl: addr
            }

            //                              loop_1_chk_cond                                 XREF[1]:     ram:80f1(j)
            //         ram:80f8 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80f9 35              DEC        (HL=>BYTE_ram_c8db)
            self.instr_hk__DEC_iHL();
            //         ram:80fa 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80fb 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80fc 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:80fd 10 f0           DJNZ       loop_1
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_1;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:80ff 21 d1 c1        LD         HL,BYTE_ram_c1d1                                 = FFh
        self.instr_hk__LD_HL_NNNN(0xc1d1);
        //         ram:8102 06 04           LD         B,0x4
        self.instr_hk__LD_B_NN(0x4);
        //                              loop_2                                          XREF[1]:     ram:8109(j)
        loop {
            self.SetPC(0x8104);
            //         ram:8104 35              DEC        (HL=>BYTE_ram_c1d1)                              = FFh
            self.instr_hk__DEC_iHL();
            //         ram:8105 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8106 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8107 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8108 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8109 10 f9           DJNZ       loop_2
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_2;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        self.SetPC(0x810b);

        //         ram:810b 21 e6 c1        LD         HL,BYTE_ram_c1e6
        self.instr_hk__LD_HL_NNNN(0xc1e6);
        //         ram:810e 34              INC        (HL=>BYTE_ram_c1e6)
        self.instr_hk__INC_iHL();
        //         ram:810f 3e 03           LD         A,0x3
        self.instr_hk__LD_A_NN(0x3);
        //         ram:8111 cd 40 81        CALL       sb_calc_change_mem_8140                          IN a
        assert!(self.call_hook(0x8140));
        //         ram:8114 c9              RET
        self.assert_pc(0x8114);
        //
        true
    }
    pub(crate) fn hook_8115(&mut self) -> bool {
        //
        //                              *************************************************************
        //                              *                           FUNCTION
        //                              *************************************************************
        //                              undefined  FUN_ram_8115 ()
        //              undefined         A:1            <RETURN>
        //                              FUN_ram_8115
        //         ram:8115 21  da  c8       LD         HL,BYTE_ram_c8da
        self.instr_hk__LD_HL_NNNN(0xc8da);
        //         ram:8118 06  08           LD         B,0x8
        self.instr_hk__LD_B_NN(0x8);
        //                              LAB_ram_811a
        loop {
            //         ram:811a 7e              LD         A,(HL=>BYTE_ram_c8da )
            self.instr_hk__LD_A_iHL();
            //         ram:811b 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:811c 20  05           JR         NZ,LAB_ram_8123
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_8123);
            } else {
                self.increase_cycles(7);
                //         ram:811e 3e  03           LD         A,0x3
                self.instr_hk__LD_A_NN(0x3);
                //         ram:8120 cd  18  80       CALL       FUN_ram_8018
                assert!(self.call_hook(0x8018));
                //
            }

            //                              LAB_ram_8123
            //         ram:8123 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8124 34              INC        (HL=>BYTE_ram_c8db )
            self.instr_hk__INC_iHL();
            //         ram:8125 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8126 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8127 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8128 10  f0           DJNZ       LAB_ram_811a
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_811a;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:812a 21  d1  c1       LD         HL,BYTE_ram_c1d1
        self.instr_hk__LD_HL_NNNN(0xc1d1);
        //         ram:812d 06  04           LD         B,0x4
        self.instr_hk__LD_B_NN(0x4);
        //                              LAB_ram_812f
        loop {
            //         ram:812f 34              INC        (HL=>BYTE_ram_c1d1 )
            self.instr_hk__INC_iHL();
            //         ram:8130 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8131 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8132 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8133 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8134 10  f9           DJNZ       LAB_ram_812f
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_812f;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        self.SetPC(0x8136);

        //         ram:8136 21  e6  c1       LD         HL,BYTE_ram_c1e6
        self.instr_hk__LD_HL_NNNN(0xc1e6);
        //         ram:8139 35              DEC        (HL=>BYTE_ram_c1e6 )
        self.instr_hk__DEC_iHL();
        //         ram:813a 3e  01           LD         A,0x1
        self.instr_hk__LD_A_NN(0x1);
        //         ram:813c cd  40  81       CALL       sb_calc_change_mem_8140                          IN a
        assert!(self.call_hook(0x8140));
        //         ram:813f c9              RET
        self.assert_pc(0x813f);
        //
        true
    }
    pub(crate) fn hook_8140(&mut self) -> bool {
        println!("hook_8140");

        //                              *************************************************************
        //                              *                           FUNCTION
        //                              *************************************************************
        //                              undefined  sb_calc_change_mem_8140 ()
        //              undefined         A:1            <RETURN>
        //                              sb_calc_change_mem_8140                              FUN_ram_8097:80bc (c) ,
        //                                                                                           FUN_ram_80c0:80e6 (c) ,
        //                                                                                           FUN_ram_80ea:8111 (c) ,
        //                                                                                           FUN_ram_8115:813c (c)
        //         ram:8140 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:8141 c2  64  81       JP         NZ,LAB_ram_8164
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            //         ram:8144 21  ba  c9       LD         HL,DAT_ram_c9ba
            self.instr_hk__LD_HL_NNNN(0xc9ba);
            //         ram:8147 11  9a  c9       LD         DE,DAT_ram_c99a
            self.instr_hk__LD_DE_NNNN(0xc99a);
            //         ram:814a 01  60  02       LD         BC,0x260
            self.instr_hk__LD_BC_NNNN(0x260);
            //         ram:814d ed  b0           LDIR
            self.instr_hk__LDIR();
            //         ram:814f 3a  e6  c1       LD         A,(BYTE_ram_c1e6 )
            self.instr_hk__LD_A_iNNNN(0xc1e6);
            //         ram:8152 32  e8  c1       LD         (BYTE_ram_c1e8 ),A
            self.instr_hk__LD_iNNNN_A(0xc1e8);
            //         ram:8155 3a  e7  c1       LD         A,(BYTE_ram_c1e7 )
            self.instr_hk__LD_A_iNNNN(0xc1e7);
            //         ram:8158 c6  13           ADD        A,0x13
            self.instr_hk__ADD_A_NN(0x13);
            //         ram:815a 32  e9  c1       LD         (BYTE_ram_c1e9 ),A
            self.instr_hk__LD_iNNNN_A(0xc1e9);
            //         ram:815d 21  fa  cb       LD         HL,DAT_ram_cbfa
            self.instr_hk__LD_HL_NNNN(0xcbfa);
            //         ram:8160 cd  3d  82       CALL       sb_calc_change_mem_823D
            assert!(self.call_hook(0x823d));
            //         ram:8163 c9              RET
            self.assert_pc(0x8163);
            return true;
        }
        // JP(LAB_ram_8164);
        //                              LAB_ram_8164
        //         ram:8164 fe  01           CP         0x1
        self.instr_hk__CP_NN(0x1);
        //         ram:8166 c2  99  81       JP         NZ,LAB_ram_8199
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            // JP(LAB_ram_8199);
            self.SetPC(0x8199);
            //                              LAB_ram_8199
            //         ram:8199 fe  02           CP         0x2
            self.instr_hk__CP_NN(0x2);
            //         ram:819b c2  b6  81       JP         NZ,LAB_ram_81b6
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                self.assert_pc(0x819e);
                //         ram:819e 21  f9  cb       LD         HL,DAT_ram_cbf9
                self.instr_hk__LD_HL_NNNN(0xcbf9);
                //         ram:81a1 11  19  cc       LD         DE,DAT_ram_cc19
                self.instr_hk__LD_DE_NNNN(0xcc19);
                //         ram:81a4 01  60  02       LD         BC,0x260
                self.instr_hk__LD_BC_NNNN(0x260);
                //         ram:81a7 ed  b8           LDDR
                self.instr_hk__LDDR();
                self.assert_pc(0x81a9);
                //         ram:81a9 2a  e6  c1       LD         HL,(BYTE_ram_c1e6 )
                self.instr_hk__LD_HL_iNNNN(0xc1e6);
                //         ram:81ac 22  e8  c1       LD         (BYTE_ram_c1e8 ),HL
                self.instr_hk__LD_iNNNN_HL(0xc1e8);
                //         ram:81af 21  9a  c9       LD         HL,DAT_ram_c99a
                self.instr_hk__LD_HL_NNNN(0xc99a);
                //         ram:81b2 cd  3d  82       CALL       sb_calc_change_mem_823D
                assert!(self.call_hook(0x823d));
                //         ram:81b5 c9              RET
                self.assert_pc(0x81b5);
                return true;
            }
            // JP(LAB_ram_81b6);

            //                              LAB_ram_81b6
            //         ram:81b6 3a  e6  c1       LD         A,(BYTE_ram_c1e6 )
            self.instr_hk__LD_A_iNNNN(0xc1e6);
            //         ram:81b9 c6  1f           ADD        A,0x1f
            self.instr_hk__ADD_A_NN(0x1f);
            //         ram:81bb 32  e8  c1       LD         (BYTE_ram_c1e8 ),A
            self.instr_hk__LD_iNNNN_A(0xc1e8);
            //         ram:81be 3a  e7  c1       LD         A,(BYTE_ram_c1e7 )
            self.instr_hk__LD_A_iNNNN(0xc1e7);
            //         ram:81c1 32  e9  c1       LD         (BYTE_ram_c1e9 ),A
            self.instr_hk__LD_iNNNN_A(0xc1e9);
            //         ram:81c4 21  9b  c9       LD         HL,DAT_ram_c99b
            self.instr_hk__LD_HL_NNNN(0xc99b);
            //         ram:81c7 06  14           LD         B,0x14
            self.instr_hk__LD_B_NN(0x14);
            //                              LAB_ram_81c9
            loop {
                self.SetPC(0x81c9);
                //         ram:81c9 c5              PUSH       BC
                self.instr_hk__PUSH_BC();
                //         ram:81ca e5              PUSH       HL=>DAT_ram_c99b
                self.instr_hk__PUSH_HL();
                //         ram:81cb 54              LD         D,H
                self.instr_hk__LD_D_H();
                //         ram:81cc 5d              LD         E,L
                self.instr_hk__LD_E_L();
                //         ram:81cd 1b              DEC        DE
                self.instr_hk__DEC_DE();
                //         ram:81ce 01  1f  00       LD         BC,0x1f
                self.instr_hk__LD_BC_NNNN(0x1f);
                //         ram:81d1 ed  b0           LDIR
                self.instr_hk__LDIR();
                //         ram:81d3 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:81d4 e5              PUSH       HL=>DAT_ram_c99b
                self.instr_hk__PUSH_HL();
                //         ram:81d5 11  1e  00       LD         DE,0x1e
                self.instr_hk__LD_DE_NNNN(0x1e);
                //         ram:81d8 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:81d9 cd  ec  81       CALL       sb_calc_change_mem_81EC
                assert!(self.call_hook(0x81ec));
                //         ram:81dc 3a  e9  c1       LD         A,(BYTE_ram_c1e9 )
                self.instr_hk__LD_A_iNNNN(0xc1e9);
                //         ram:81df 3c              INC        A
                self.instr_hk__INC_A();
                //         ram:81e0 32  e9  c1       LD         (BYTE_ram_c1e9 ),A
                self.instr_hk__LD_iNNNN_A(0xc1e9);
                //         ram:81e3 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:81e4 11  20  00       LD         DE,0x20
                self.instr_hk__LD_DE_NNNN(0x20);
                //         ram:81e7 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:81e8 c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:81e9 10  de           DJNZ       LAB_ram_81c9
                self.IncPC(2);
                self.decB();
                if self.data.B == 0 {
                    self.increase_cycles(8);
                    // break;
                    //         ram:81eb c9              RET

                    self.assert_pc(0x81eb);
                    return true;
                }
                self.increase_cycles(13);
                //JP LAB_ram_81c9;
            }
        }

        //         ram:8169 2a  e6  c1       LD         HL,(BYTE_ram_c1e6 )
        self.instr_hk__LD_HL_iNNNN(0xc1e6);
        //         ram:816c 22  e8  c1       LD         (BYTE_ram_c1e8 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc1e8);
        //         ram:816f 21  b8  c9       LD         HL,DAT_ram_c9b8
        self.instr_hk__LD_HL_NNNN(0xc9b8);
        //         ram:8172 06  14           LD         B,0x14
        self.instr_hk__LD_B_NN(0x14);
        //                              LAB_ram_8174
        loop {
            self.SetPC(0x8174);
            //         ram:8174 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:8175 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:8176 54              LD         D,H
            self.instr_hk__LD_D_H();
            //         ram:8177 5d              LD         E,L
            self.instr_hk__LD_E_L();
            //         ram:8178 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:8179 01  1f  00       LD         BC,0x1f
            self.instr_hk__LD_BC_NNNN(0x1f);
            //         ram:817c ed  b8           LDDR
            self.instr_hk__LDDR();
            //         ram:817e e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:817f e5              PUSH       HL=>DAT_ram_c9b8
            self.instr_hk__PUSH_HL();
            //         ram:8180 11  1e  00       LD         DE,0x1e
            self.instr_hk__LD_DE_NNNN(0x1e);
            //         ram:8183 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:8184 ed  52           SBC        HL,DE
            self.instr_hk__SBC_HL_DE();
            //         ram:8186 cd  ec  81       CALL       sb_calc_change_mem_81EC
            assert!(self.call_hook(0x81ec));
            //         ram:8189 3a  e9  c1       LD         A,(BYTE_ram_c1e9 )
            self.instr_hk__LD_A_iNNNN(0xc1e9);
            //         ram:818c 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:818d 32  e9  c1       LD         (BYTE_ram_c1e9 ),A
            self.instr_hk__LD_iNNNN_A(0xc1e9);
            //         ram:8190 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:8191 11  20  00       LD         DE,0x20
            self.instr_hk__LD_DE_NNNN(0x20);
            //         ram:8194 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:8195 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:8196 10  dc           DJNZ       LAB_ram_8174
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_8174;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        self.assert_pc(0x8198);
        //         ram:8198 c9              RET
        true
    }
    pub(crate) fn hook_81ec(&mut self) -> bool {
        //         ram:81ec e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:81ed 3a e9 c1        LD         A,(BYTE_ram_c1e9)
        self.instr_hk__LD_A_iNNNN(0xc1e9);
        //         ram:81f0 06 04           LD         B,0x4
        self.instr_hk__LD_B_NN(0x4);
        //         ram:81f2 cd 95 b6        CALL       sb_calc_B695
        assert!(self.call_hook(0xb695));

        //         ram:81f5 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:81f6 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:81f8 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:81f9 32 eb c1        LD         (BYTE_ram_c1eb),A
        self.instr_hk__LD_iNNNN_A(0xc1eb);
        //         ram:81fc 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:81fd 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:81fe 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:81ff 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8200 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8201 3a e8 c1        LD         A,(BYTE_ram_c1e8)
        self.instr_hk__LD_A_iNNNN(0xc1e8);
        //         ram:8204 06 04           LD         B,0x4
        self.instr_hk__LD_B_NN(0x4);
        //         ram:8206 cd 95 b6        CALL       sb_calc_B695
        assert!(self.call_hook(0xb695));

        //         ram:8209 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:820a 16 00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:820c 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:820d 32 ea c1        LD         (BYTE_ram_c1ea),A
        self.instr_hk__LD_iNNNN_A(0xc1ea);
        //         ram:8210 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8211 11 ac c3        LD         DE,DAT_ram_c3ac
        self.instr_hk__LD_DE_NNNN(0xc3ac);
        //         ram:8214 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8215 6e              LD         L,(HL=>DAT_ram_c3ac)
        self.instr_hk__LD_L_iHL();
        //         ram:8216 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:8218 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8219 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:821a 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:821b 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:821c 3a 85 c3        LD         A,(BYTE_ram_c385)
        self.instr_hk__LD_A_iNNNN(0xc385);
        //         ram:821f 11 9b ab        LD         DE,DAT_ram_ab9b
        self.instr_hk__LD_DE_NNNN(0xab9b);
        //         ram:8222 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:8223 c2 29 82        JP         NZ,LAB_ram_8229
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            // JP LAB_ram_8229;
        } else {
            //         ram:8226 11 cb aa        LD         DE,DAT_ram_aacb
            self.instr_hk__LD_DE_NNNN(0xaacb);
        }

        //                              LAB_ram_8229
        //         ram:8229 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:822a 3a eb c1        LD         A,(BYTE_ram_c1eb)
        self.instr_hk__LD_A_iNNNN(0xc1eb);
        //         ram:822d 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:822e 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:822f 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:8230 3a ea c1        LD         A,(BYTE_ram_c1ea)
        self.instr_hk__LD_A_iNNNN(0xc1ea);
        //         ram:8233 83              ADD        A,E
        self.instr_hk__ADD_A_E();
        //         ram:8234 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:8235 16 00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:8237 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8238 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8239 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:823a 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:823b 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:823c c9              RET
        self.assert_pc(0x823c);

        true
    }
    pub(crate) fn hook_823d(&mut self) -> bool {
        //         ram:823d 06 20           LD         B,0x20
        self.instr_hk__LD_B_NN(0x20);
        //         ram:823f 3a e8 c1        LD         A,(BYTE_ram_c1e8)
        self.instr_hk__LD_A_iNNNN(0xc1e8);
        //         ram:8242 f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //                              loop
        loop {
            self.SetPC(0x8243);
            //         ram:8243 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:8244 cd ec 81        CALL       sb_calc_change_mem_81EC
            assert!(self.call_hook(0x81ec));
            //         ram:8247 3a e8 c1        LD         A,(BYTE_ram_c1e8)
            self.instr_hk__LD_A_iNNNN(0xc1e8);
            //         ram:824a 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:824b 32 e8 c1        LD         (BYTE_ram_c1e8),A
            self.instr_hk__LD_iNNNN_A(0xc1e8);
            //         ram:824e c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:824f 10 f2           DJNZ       loop
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:8251 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:8252 32 e8 c1        LD         (BYTE_ram_c1e8),A
        self.instr_hk__LD_iNNNN_A(0xc1e8);
        //         ram:8255 3a e9 c1        LD         A,(BYTE_ram_c1e9)
        self.instr_hk__LD_A_iNNNN(0xc1e9);
        //         ram:8258 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:8259 32 e9 c1        LD         (BYTE_ram_c1e9),A
        self.instr_hk__LD_iNNNN_A(0xc1e9);
        //         ram:825c c9              RET
        self.assert_pc(0x825c);

        true
    }
    fn internal_825d(&mut self) {
        //         ram:825d 2a e6 c1        LD         HL,(BYTE_ram_c1e6)
        self.instr_hk__LD_HL_iNNNN(0xc1e6);
        //         ram:8260 22 e8 c1        LD         (BYTE_ram_c1e8),HL
        self.instr_hk__LD_iNNNN_HL(0xc1e8);
        //         ram:8263 21 9a c9        LD         HL,DAT_ram_c99a
        self.instr_hk__LD_HL_NNNN(0xc99a);
        //         ram:8266 0e 14           LD         C,0x14
        self.instr_hk__LD_C_NN(0x14);
        //                              loop_1
        loop {
            //         ram:8268 cd 3d 82        CALL       sb_calc_change_mem_823D
            assert!(self.call_hook(0x823d));
            //         ram:826b 0d              DEC        C
            self.instr_hk__DEC_C();
            //         ram:826c c2 68 82        JP         NZ,loop_1
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop_1;
            } else {
                break;
            }
        }

        //         ram:826f af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:8270 21 da c8        LD         HL,BYTE_ram_c8da
        self.instr_hk__LD_HL_NNNN(0xc8da);
        //                              loop_2
        loop {
            self.SetPC(0x8273);
            //         ram:8273 32 b7 c8        LD         (BYTE_ram_c8b7),A
            self.instr_hk__LD_iNNNN_A(0xc8b7);
            //         ram:8276 7e              LD         A,(HL=>BYTE_ram_c8da)
            self.instr_hk__LD_A_iHL();
            //         ram:8277 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:8278 28 08           JR         Z,l_exit_x
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(12);
                //JR l_exit_x;
            } else {
                self.increase_cycles(7);
                //         ram:827a e5              PUSH       HL=>BYTE_ram_c8da
                self.instr_hk__PUSH_HL();
                //         ram:827b cd c6 47        CALL       sb_set_addr_base_47C6                            ()
                assert!(self.call_hook(0x47c6));
                //         ram:827e cd 6f 8a        CALL       sb_fill_vram_guess_8A6F
                assert!(self.call_hook(0x8a6f));
                //         ram:8281 e1              POP        HL
                self.instr_hk__POP_HL();
            }

            self.SetPC(0x8282);
            //                              l_exit_x
            //         ram:8282 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8283 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8284 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8285 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8286 3a b7 c8        LD         A,(BYTE_ram_c8b7)
            self.instr_hk__LD_A_iNNNN(0xc8b7);
            //         ram:8289 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:828a fe 08           CP         0x8
            self.instr_hk__CP_NN(0x8);
            //         ram:828c 20 e5           JR         NZ,loop_2
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR loop_2;
            } else {
                self.increase_cycles(7);
                break;
            }
        }

        //         ram:828e cd 24 88        CALL       FUN_ram_8824                                     ()
        assert!(self.call_hook(0x8824));
        //         ram:8291 c9              RET
        self.assert_pc(0x8291);

        // true
    }
    pub(crate) fn hook_825d(&mut self) -> bool {
        self.internal_825d();
        true
    }
    pub(crate) fn hook_82d7(&mut self) -> bool {
        //         ram:82d7 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:82da 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:82db 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:82dc 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:82dd 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:82de 81              ADD        A,C
        self.instr_hk__ADD_A_C();
        //         ram:82df 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:82e0 57              LD         D,A
        self.instr_hk__LD_D_A();
        //         ram:82e1 c9              RET
        self.assert_pc(0x82e1);

        true
    }
    pub(crate) fn hook_8559(&mut self) -> bool {
        //         ram:8559 cd 0a 60        CALL       sb_get_player_addr_600A
        assert!(self.call_hook(0x600a));
        //         ram:855c 22 54 c2        LD         (pt_char_c254),HL
        self.instr_hk__LD_iNNNN_HL(0xc254);
        //         ram:855f cd d7 82        CALL       sb_calc_btl_wdw_x0_82D7
        assert!(self.call_hook(0x82d7));
        //         ram:8562 1e 14           LD         E,0x14
        self.instr_hk__LD_E_NN(0x14);
        //         ram:8564 ed 53 5e c2     LD         (wd_x0_y0_c25e),DE
        self.instr_hk__LD_iNNNN_DE(0xc25e);
        //         ram:8568 01 04 0a        LD         BC,0xa04
        self.instr_hk__LD_BC_NNNN(0xa04);
        //         ram:856b cd 17 4c        CALL       fn_draw_border_guess_4c17
        assert!(self.call_hook(0x4c17));

        //         ram:856e 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:8571 ed 5b 5e c2     LD         DE,(wd_x0_y0_c25e)
        self.instr_hk__LD_DE_iNNNN(0xc25e);
        //         ram:8575 14              INC        D
        self.instr_hk__INC_D();
        //         ram:8576 14              INC        D
        self.instr_hk__INC_D();
        //         ram:8577 06 06           LD         B,0x6
        self.instr_hk__LD_B_NN(0x6);
        //                              loop_print_name
        loop {
            self.SetPC(0x8579);
            //         ram:8579 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:857a 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:857b 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:857c cd d6 89        CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //         ram:857f c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:8580 10 f7           DJNZ       loop_print_name
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_print_name;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:8582 cd 51 47        CALL       sb_get_addr_status_for_player_4751
        assert!(self.call_hook(0x4751));

        //         ram:8585 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8586 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:8587 c2 97 85        JP         NZ,l_print_hp_mp
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            self.SetPC(0x8597);
            // JP l_print_hp_mp;
            //                              l_print_hp_mp
            //         ram:8597 cd 9e 85        CALL       sb_btl_print_hp_859E                             (v
            assert!(self.call_hook(0x859e));
            //         ram:859a cd c8 85        CALL       sb_btl_print_mp_85C8                             (v
            assert!(self.call_hook(0x85c8));
            //         ram:859d c9              RET
            self.assert_pc(0x859d);

            true
        } else {
            self.SetPC(0x858a);
            //         ram:858a ed 5b 5e c2     LD         DE,(wd_x0_y0_c25e)
            self.instr_hk__LD_DE_iNNNN(0xc25e);
            //         ram:858e 14              INC        D
            self.instr_hk__INC_D();
            //         ram:858f 1c              INC        E
            self.instr_hk__INC_E();
            //         ram:8590 01 02 08        LD         BC,0x802
            self.instr_hk__LD_BC_NNNN(0x802);
            //         ram:8593 cd 17 4c        CALL       fn_draw_border_guess_4c17
            assert!(self.call_hook(0x4c17));

            //         ram:8596 c9              RET
            self.assert_pc(0x8596);
            true
        }
    }
    pub(crate) fn hook_859e(&mut self) -> bool {
        //         ram:859e ed 5b 5e c2     LD         DE,(wd_x0_y0_c25e)
        self.instr_hk__LD_DE_iNNNN(0xc25e);
        //         ram:85a2 14              INC        D
        self.instr_hk__INC_D();
        //         ram:85a3 1c              INC        E
        self.instr_hk__INC_E();
        //         ram:85a4 3e 48           LD         A,'H'
        self.instr_hk__LD_A_NN('H' as u32 as u8);
        //         ram:85a6 cd d6 89        CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //         ram:85a9 3e 50           LD         A,'P'
        self.instr_hk__LD_A_NN('P' as u32 as u8);
        //         ram:85ab cd d6 89        CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //         ram:85ae 14              INC        D
        self.instr_hk__INC_D();
        //         ram:85af d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:85b0 11 b0 c7        LD         DE,bt_buffer_c7b0
        self.instr_hk__LD_DE_NNNN(0xc7b0);
        //         ram:85b3 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:85b6 01 08 00        LD         BC,char_08h_hp
        self.instr_hk__LD_BC_NNNN(0x08);
        //         ram:85b9 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:85ba 6e              LD         L,(HL)
        self.instr_hk__LD_L_iHL();
        //         ram:85bb 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:85bd cd bd b7        CALL       sb_itoa_guess_B7BD
        assert!(self.call_hook(0xb7bd));

        //         ram:85c0 21 b0 c7        LD         HL,bt_buffer_c7b0
        self.instr_hk__LD_HL_NNNN(0xc7b0);
        //         ram:85c3 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:85c4 cd c7 89        CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:85c7 c9              RET

        self.assert_pc(0x85c7);

        true
    }
    pub(crate) fn hook_85c8(&mut self) -> bool {
        //         ram:85c8 ed 5b 5e c2     LD         DE,(wd_x0_y0_c25e)
        self.instr_hk__LD_DE_iNNNN(0xc25e);
        //         ram:85cc 14              INC        D
        self.instr_hk__INC_D();
        //         ram:85cd 1c              INC        E
        self.instr_hk__INC_E();
        //         ram:85ce 1c              INC        E
        self.instr_hk__INC_E();
        //         ram:85cf 3e 4d           LD         A,'M'
        self.instr_hk__LD_A_NN('M' as u32 as u8);
        //         ram:85d1 cd d6 89        CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //         ram:85d4 3e 50           LD         A,'P'
        self.instr_hk__LD_A_NN('P' as u32 as u8);
        //         ram:85d6 cd d6 89        CALL       fn_putchar_xy_89d6
        assert!(self.call_hook(0x89d6));

        //         ram:85d9 14              INC        D
        self.instr_hk__INC_D();
        //         ram:85da d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:85db cd 47 47        CALL       sb_get_addr_mp_for_player_4747
        assert!(self.call_hook(0x4747));

        //         ram:85de 6e              LD         L,(HL)
        self.instr_hk__LD_L_iHL();
        //         ram:85df 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:85e1 11 b0 c7        LD         DE,bt_buffer_c7b0
        self.instr_hk__LD_DE_NNNN(0xc7b0);
        //         ram:85e4 cd bd b7        CALL       sb_itoa_guess_B7BD
        assert!(self.call_hook(0xb7bd));

        //         ram:85e7 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:85e8 21 b0 c7        LD         HL,bt_buffer_c7b0
        self.instr_hk__LD_HL_NNNN(0xc7b0);
        //         ram:85eb cd c7 89        CALL       fn_print_xy_89c7
        assert!(self.call_hook(0x89c7));

        //         ram:85ee c9              RET
        self.assert_pc(0x85ee);

        true
    }
    pub(crate) fn hook_8824(&mut self) -> bool {
        //         ram:8824 3a 3e c2        LD         A,(DAT_ram_c23e)
        self.instr_hk__LD_A_iNNNN(0xc23e);
        //         ram:8827 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:8828 28 05           JR         Z,sb_di_vdp_prepare_fill_write_882F
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR sb_di_vdp_prepare_fill_write_882F;
        } else {
            self.increase_cycles(7);
            //         ram:882a 3a 0b c2        LD         A,(BYTE_ram_c20b)
            self.instr_hk__LD_A_iNNNN(0xc20b);
            //         ram:882d b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:882e c0              RET        NZ
            self.IncPC(1);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(11);
                return true;
            } else {
                self.increase_cycles(5);
            }
        }
        self.internal_882f();
        true
    }
    fn internal_882f(&mut self) {
        //         ram:882f f3              DI
        self.instr_hk__DI();
        //         ram:8830 11 9a c9        LD         DE,DAT_ram_c99a
        self.instr_hk__LD_DE_NNNN(0xc99a);
        //         ram:8833 21 00 18        LD         HL,SCREEN2_PATTERN_GENERATOR_TABLE_SIZE
        self.instr_hk__LD_HL_NNNN(SCREEN2_PATTERN_GENERATOR_TABLE_SIZE);
        //         ram:8836 cd 9e c0        CALL       fn_vdp_set_vaddr_to_write_c09e
        assert!(self.call_hook(0xc09e));
        //                              -- Flow Override: CALL_RETURN (CALL_TERMINATOR)
        //         ram:8839 06 14           LD         B,0x14
        self.instr_hk__LD_B_NN(0x14);
        //         ram:883b cd 00 c0        CALL       fn_vdp_write_c000
        assert!(self.call_hook(0xc000));

        //         ram:883e fb              EI
        self.instr_hk__EI();
        //         ram:883f c9              RET
    }
    pub(crate) fn hook_882f(&mut self) -> bool {
        self.internal_882f();
        true
    }
    pub(crate) fn hook_8840(&mut self) -> bool {
        // println!("hook_8840");
        //         ram:8840 f5              PUSH       AF                                               IN
        self.instr_hk__PUSH_AF();
        //         ram:8841 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:8842 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:8843 f3              DI
        self.instr_hk__DI();
        //         ram:8844 cd ba c0        CALL       sb_fill_vram_guess_c0ba                          IN
        assert!(self.call_hook(0xc0ba));
        //         ram:8847 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:8848 01 00 08        LD         BC,0x800
        self.instr_hk__LD_BC_NNNN(0x800);
        //         ram:884b 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:884c c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:884d f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:884e f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:884f c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:8850 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:8851 cd ba c0        CALL       sb_fill_vram_guess_c0ba                          IN
        assert!(self.call_hook(0xc0ba));
        //         ram:8854 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:8855 01 00 08        LD         BC,0x800
        self.instr_hk__LD_BC_NNNN(0x800);
        //         ram:8858 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8859 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:885a f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:885b cd ba c0        CALL       sb_fill_vram_guess_c0ba                          IN
        assert!(self.call_hook(0xc0ba));
        //         ram:885e fb              EI
        self.instr_hk__EI();
        //         ram:885f c9              RET

        self.assert_pc(0x885f);
        true
    }
    pub(crate) fn hook_8860(&mut self) -> bool {
        //         ram:8860 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8861 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8862 22 54 c2        LD         (pt_char_c254),HL
        self.instr_hk__LD_iNNNN_HL(0xc254);
        //         ram:8865 ed 43 56 c2     LD         (BYTE_ram_c256),BC
        self.instr_hk__LD_iNNNN_BC(0xc256);
        //         ram:8869 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:886a 22 58 c2        LD         (tmp_var_c258),HL
        self.instr_hk__LD_iNNNN_HL(0xc258);
        //         ram:886d 2a 56 c2        LD         HL,(BYTE_ram_c256)
        self.instr_hk__LD_HL_iNNNN(0xc256);
        //         ram:8870 11 04 00        LD         DE,0x4
        self.instr_hk__LD_DE_NNNN(0x4);
        //         ram:8873 cd ac b6        CALL       sb_calc_b6ac
        assert!(self.call_hook(0xb6ac));

        //         ram:8876 22 5a c2        LD         (WORD_ram_c25a),HL
        self.instr_hk__LD_iNNNN_HL(0xc25a);
        //         ram:8879 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:887c ed 5b 58 c2     LD         DE,(tmp_var_c258)
        self.instr_hk__LD_DE_iNNNN(0xc258);
        //         ram:8880 ed 4b 5a c2     LD         BC,(WORD_ram_c25a)
        self.instr_hk__LD_BC_iNNNN(0xc25a);
        //         ram:8884 cd ba 88        CALL       sb_calc_88BA                                     IN
        assert!(self.call_hook(0x88ba));

        //         ram:8887 ed 4b 5a c2     LD         BC,(WORD_ram_c25a)
        self.instr_hk__LD_BC_iNNNN(0xc25a);
        //         ram:888b 2a 58 c2        LD         HL,(tmp_var_c258)
        self.instr_hk__LD_HL_iNNNN(0xc258);
        //         ram:888e 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:888f 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8890 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8891 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8892 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:8895 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8896 cd ba 88        CALL       sb_calc_88BA                                     IN
        assert!(self.call_hook(0x88ba));

        //         ram:8899 ed 4b 5a c2     LD         BC,(WORD_ram_c25a)
        self.instr_hk__LD_BC_iNNNN(0xc25a);
        //         ram:889d 2a 58 c2        LD         HL,(tmp_var_c258)
        self.instr_hk__LD_HL_iNNNN(0xc258);
        //         ram:88a0 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:88a1 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:88a2 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:88a3 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:88a6 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:88a7 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:88a8 cd ba 88        CALL       sb_calc_88BA                                     IN
        assert!(self.call_hook(0x88ba));

        //         ram:88ab ed 4b 5a c2     LD         BC,(WORD_ram_c25a)
        self.instr_hk__LD_BC_iNNNN(0xc25a);
        //         ram:88af 2a 58 c2        LD         HL,(tmp_var_c258)
        self.instr_hk__LD_HL_iNNNN(0xc258);
        //         ram:88b2 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:88b3 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:88b4 2a 54 c2        LD         HL,(pt_char_c254)
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:88b7 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:88b8 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:88b9 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();

        self.internal_88ba();
        true
    }
    fn internal_88ba(&mut self) {
        assert!(self.hook_88ba());
    }
    pub(crate) fn hook_88ba(&mut self) -> bool {
        //         ram:88ba d5              PUSH       DE                                               IN
        self.instr_hk__PUSH_DE();
        //         ram:88bb eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //                              loop_1
        loop {
            self.SetPC(0x88bc);
            //         ram:88bc c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:88bd 06 08           LD         B,0x8
            self.instr_hk__LD_B_NN(0x8);
            //         ram:88bf 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //                              loop_1_a
            loop {
                self.SetPC(0x88c0);
                //         ram:88c0 87              ADD        A,A
                self.instr_hk__ADD_A_A();
                //         ram:88c1 cb 19           RR         C
                self.instr_hk__RR_C();
                //         ram:88c3 10 fb           DJNZ       loop_1_a
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP loop_1_a;
                } else {
                    self.increase_cycles(8);
                    break;
                }
            }

            //         ram:88c5 71              LD         (HL),C
            self.instr_hk__LD_iHL_C();
            //         ram:88c6 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:88c7 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:88c8 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:88c9 0b              DEC        BC
            self.instr_hk__DEC_BC();
            //         ram:88ca 78              LD         A,B
            self.instr_hk__LD_A_B();
            //         ram:88cb b1              OR         C
            self.instr_hk__OR_A_C();
            //         ram:88cc 20 ee           JR         NZ,loop_1
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12); //JR loop_1;
            } else {
                self.increase_cycles(7);
                break;
            }
        }
        self.SetPC(0x88ce);

        //         ram:88ce 2a 56 c2        LD         HL,(BYTE_ram_c256)
        self.instr_hk__LD_HL_iNNNN(0xc256);
        //         ram:88d1 01 80 00        LD         BC,0x80
        self.instr_hk__LD_BC_NNNN(0x80);
        //         ram:88d4 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:88d5 ed 42           SBC        HL,BC
        self.instr_hk__SBC_HL_BC();
        //         ram:88d7 20 16           JR         NZ,LAB_ram_88ef
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12); //JR LAB_ram_88ef;

            //                              LAB_ram_88ef
            //         ram:88ef 01 a0 00        LD         BC,0xa0
            self.instr_hk__LD_BC_NNNN(0xa0);
            //         ram:88f2 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:88f3 ed 42           SBC        HL,BC
            self.instr_hk__SBC_HL_BC();
            //         ram:88f5 20 12           JR         NZ,LAB_ram_8909
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12); //JR LAB_ram_8909;

                //                              LAB_ram_8909
                //         ram:8909 01 60 00        LD         BC,0x60
                self.instr_hk__LD_BC_NNNN(0x60);
                //         ram:890c b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:890d ed 42           SBC        HL,BC
                self.instr_hk__SBC_HL_BC();
                //         ram:890f 20 15           JR         NZ,LAB_ram_8926
                self.IncPC(2);
                if (self.data.F & FLAG_Z) == 0 {
                    self.increase_cycles(12); //JR LAB_ram_8926;

                    //                              LAB_ram_8926
                    //         ram:8926 e1              POP        HL
                    self.instr_hk__POP_HL();
                    //         ram:8927 06 04           LD         B,0x4
                    self.instr_hk__LD_B_NN(0x4);
                    //                              loop_2
                    loop {
                        self.SetPC(0x8929);
                        //         ram:8929 c5              PUSH       BC
                        self.instr_hk__PUSH_BC();
                        //         ram:892a e5              PUSH       HL
                        self.instr_hk__PUSH_HL();
                        //         ram:892b 54              LD         D,H
                        self.instr_hk__LD_D_H();
                        //         ram:892c 5d              LD         E,L
                        self.instr_hk__LD_E_L();
                        //         ram:892d 01 18 00        LD         BC,0x18
                        self.instr_hk__LD_BC_NNNN(0x18);
                        //         ram:8930 09              ADD        HL,BC
                        self.instr_hk__ADD_HL_BC();
                        //         ram:8931 eb              EX         DE,HL
                        self.instr_hk__EX_DE_HL();
                        //         ram:8932 cd 4c 89        CALL       sb_exchange_8bytes_894C                          IN
                        assert!(self.call_hook(0x894c));

                        //         ram:8935 e1              POP        HL
                        self.instr_hk__POP_HL();
                        //         ram:8936 e5              PUSH       HL
                        self.instr_hk__PUSH_HL();
                        //         ram:8937 54              LD         D,H
                        self.instr_hk__LD_D_H();
                        //         ram:8938 5d              LD         E,L
                        self.instr_hk__LD_E_L();
                        //         ram:8939 01 08 00        LD         BC,0x8
                        self.instr_hk__LD_BC_NNNN(0x8);
                        //         ram:893c 09              ADD        HL,BC
                        self.instr_hk__ADD_HL_BC();
                        //         ram:893d 09              ADD        HL,BC
                        self.instr_hk__ADD_HL_BC();
                        //         ram:893e eb              EX         DE,HL
                        self.instr_hk__EX_DE_HL();
                        //         ram:893f 09              ADD        HL,BC
                        self.instr_hk__ADD_HL_BC();
                        //         ram:8940 cd 4c 89        CALL       sb_exchange_8bytes_894C                          IN
                        assert!(self.call_hook(0x894c));

                        //         ram:8943 e1              POP        HL
                        self.instr_hk__POP_HL();
                        //         ram:8944 01 20 00        LD         BC,0x20
                        self.instr_hk__LD_BC_NNNN(0x20);
                        //         ram:8947 09              ADD        HL,BC
                        self.instr_hk__ADD_HL_BC();
                        //         ram:8948 c1              POP        BC
                        self.instr_hk__POP_BC();
                        //         ram:8949 10 de           DJNZ       loop_2
                        self.IncPC(2);
                        self.decB();
                        if self.data.B != 0 {
                            self.increase_cycles(13);
                            //JP loop_2;
                        } else {
                            self.increase_cycles(8);
                            break;
                        }
                    }

                    //         ram:894b c9              RET
                    self.assert_pc(0x894b);
                    true
                } else {
                    self.increase_cycles(7);
                    self.SetPC(0x8911);
                    //         ram:8911 e1              POP        HL
                    self.instr_hk__POP_HL();
                    //         ram:8912 54              LD         D,H
                    self.instr_hk__LD_D_H();
                    //         ram:8913 5d              LD         E,L
                    self.instr_hk__LD_E_L();
                    //         ram:8914 01 10 00        LD         BC,0x10
                    self.instr_hk__LD_BC_NNNN(0x10);
                    //         ram:8917 09              ADD        HL,BC
                    self.instr_hk__ADD_HL_BC();
                    //         ram:8918 eb              EX         DE,HL
                    self.instr_hk__EX_DE_HL();
                    //         ram:8919 cd 4c 89        CALL       sb_exchange_8bytes_894C                          IN
                    assert!(self.call_hook(0x894c));

                    //         ram:891c cd 59 89        CALL       sb_exchange_8bytes_ex_8959                       IN
                    assert!(self.call_hook(0x8959));

                    //         ram:891f cd 59 89        CALL       sb_exchange_8bytes_ex_8959                       IN
                    assert!(self.call_hook(0x8959));

                    //         ram:8922 cd 59 89        CALL       sb_exchange_8bytes_ex_8959                       IN
                    assert!(self.call_hook(0x8959));

                    //         ram:8925 c9              RET
                    self.assert_pc(0x8925);
                    true
                }
            } else {
                self.increase_cycles(7);
                self.SetPC(0x88f7);
                //         ram:88f7 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:88f8 54              LD         D,H
                self.instr_hk__LD_D_H();
                //         ram:88f9 5d              LD         E,L
                self.instr_hk__LD_E_L();
                //         ram:88fa 01 10 00        LD         BC,0x10
                self.instr_hk__LD_BC_NNNN(0x10);
                //         ram:88fd 09              ADD        HL,BC
                self.instr_hk__ADD_HL_BC();
                //         ram:88fe eb              EX         DE,HL
                self.instr_hk__EX_DE_HL();
                //         ram:88ff cd 4c 89        CALL       sb_exchange_8bytes_894C                          IN
                assert!(self.call_hook(0x894c));

                self.assert_pc(0x8902);
                //         ram:8902 cd 59 89        CALL       sb_exchange_8bytes_ex_8959                       IN
                assert!(self.call_hook(0x8959));

                //         ram:8905 cd 59 89        CALL       sb_exchange_8bytes_ex_8959                       IN
                assert!(self.call_hook(0x8959));

                //         ram:8908 c9              RET
                self.assert_pc(0x8908);
                true
            }
        } else {
            self.increase_cycles(7);
            self.SetPC(0x88d9);
            //         ram:88d9 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:88da 54              LD         D,H
            self.instr_hk__LD_D_H();
            //         ram:88db 5d              LD         E,L
            self.instr_hk__LD_E_L();
            //         ram:88dc 01 08 00        LD         BC,0x8
            self.instr_hk__LD_BC_NNNN(0x8);
            //         ram:88df 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:88e0 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:88e1 cd 4c 89        CALL       sb_exchange_8bytes_894C                          IN
            assert!(self.call_hook(0x894c));

            //         ram:88e4 01 08 00        LD         BC,0x8
            self.instr_hk__LD_BC_NNNN(0x8);
            //         ram:88e7 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:88e8 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:88e9 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:88ea eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:88eb cd 4c 89        CALL       sb_exchange_8bytes_894C                          IN
            assert!(self.call_hook(0x894c));

            //         ram:88ee c9              RET
            self.assert_pc(0x88ee);
            true
        }
    }
    pub(crate) fn hook_894c(&mut self) -> bool {
        //         ram:894c 06 08           LD         B,0x8                                            IN
        self.instr_hk__LD_B_NN(0x8);
        //                              loop
        loop {
            self.SetPC(0x894e);
            //         ram:894e 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:894f 08              EX         AF,AF_
            self.instr_hk__EX_AF_AF_();
            //         ram:8950 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:8951 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:8952 08              EX         AF,AF_
            self.instr_hk__EX_AF_AF_();
            //         ram:8953 12              LD         (DE),A
            self.instr_hk__LD_iDE_A();
            //         ram:8954 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8955 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:8956 10 f6           DJNZ       loop
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        self.assert_pc(0x8958);
        //         ram:8958 c9              RET
        true
    }
    pub(crate) fn hook_8959(&mut self) -> bool {
        //         ram:8959 01 10 00        LD         BC,0x10                                          IN
        self.instr_hk__LD_BC_NNNN(0x10);
        //         ram:895c 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:895d eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:895e 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:895f eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8960 cd 4c 89        CALL       sb_exchange_8bytes_894C                          IN
        assert!(self.call_hook(0x894c));
        //         ram:8963 c9              RET

        true
    }
    pub(crate) fn hook_8964(&mut self) -> bool {
        //         ram:8964 e5              PUSH       HL                                               IN
        self.instr_hk__PUSH_HL();
        //         ram:8965 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:8966 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:8967 cd 84 89        CALL       sb_blit_ram_to_vram_guess_8984                   IN
        assert!(self.call_hook(0x8984));
        //         ram:896a c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:896b d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:896c 21 00 08        LD         HL,0x800
        self.instr_hk__LD_HL_NNNN(0x800);
        //         ram:896f 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8970 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8971 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:8972 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:8973 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:8974 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:8975 cd 84 89        CALL       sb_blit_ram_to_vram_guess_8984                   IN
        assert!(self.call_hook(0x8984));
        //         ram:8978 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:8979 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:897a 21 00 08        LD         HL,0x800
        self.instr_hk__LD_HL_NNNN(0x800);
        //         ram:897d 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:897e eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:897f e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:8980 cd 84 89        CALL       sb_blit_ram_to_vram_guess_8984                   IN
        assert!(self.call_hook(0x8984));
        //         ram:8983 c9              RET
        self.assert_pc(0x8983);
        true
    }
    pub(crate) fn hook_8984(&mut self) -> bool {
        loop {
            self.SetPC(0x8984);
            //         ram:8984 c5              PUSH       BC                                               IN
            self.instr_hk__PUSH_BC();
            //         ram:8985 cd 9a 89        CALL       sb_blit_ram_to_vram_guess_899a                   IN
            assert!(self.call_hook(0x899a));
            //         ram:8988 01 10 00        LD         BC,0x10
            self.instr_hk__LD_BC_NNNN(0x10);
            //         ram:898b 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:898c eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:898d 01 08 00        LD         BC,0x8
            self.instr_hk__LD_BC_NNNN(0x8);
            //         ram:8990 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:8991 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:8992 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:8993 0b              DEC        BC
            self.instr_hk__DEC_BC();
            //         ram:8994 78              LD         A,B
            self.instr_hk__LD_A_B();
            //         ram:8995 b1              OR         C
            self.instr_hk__OR_A_C();
            //         ram:8996 c2 84 89        JP         NZ,sb_blit_ram_to_vram_guess_8984                IN
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP sb_blit_ram_to_vram_guess_8984;
            } else {
                break;
            }
        }
        //         ram:8999 c9              RET
        self.assert_pc(0x8999);
        true
    }
    pub(crate) fn hook_899a(&mut self) -> bool {
        //         ram:899a e5              PUSH       HL                                               IN
        self.instr_hk__PUSH_HL();
        //         ram:899b d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:899c e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:899d d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:899e 01 08 00        LD         BC,0x8
        self.instr_hk__LD_BC_NNNN(0x8);
        //         ram:89a1 f3              DI
        self.instr_hk__DI();
        //         ram:89a2 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085
        assert!(self.call_hook(0xc085));
        //         ram:89a5 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:89a6 21 00 20        LD         HL,0x2000
        self.instr_hk__LD_HL_NNNN(0x2000);
        //         ram:89a9 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:89aa eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:89ab e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:89ac d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:89ad 11 08 00        LD         DE,0x8
        self.instr_hk__LD_DE_NNNN(0x8);
        //         ram:89b0 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:89b1 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:89b2 01 08 00        LD         BC,0x8
        self.instr_hk__LD_BC_NNNN(0x8);
        //         ram:89b5 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085
        assert!(self.call_hook(0xc085));
        //         ram:89b8 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:89b9 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:89ba fb              EI
        self.instr_hk__EI();
        //         ram:89bb c9              RET
        self.assert_pc(0x89bb);
        true
    }
    pub(crate) fn hook_89bc(&mut self) -> bool {
        //         ram:89bc 5c              LD         E,H
        self.instr_hk__LD_E_H();
        //         ram:89bd 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:89bf 54              LD         D,H
        self.instr_hk__LD_D_H();
        //         ram:89c0 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:89c1 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:89c2 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:89c3 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:89c4 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:89c5 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:89c6 c9              RET

        self.assert_pc(0x89c6);
        true
    }
    pub(crate) fn hook_89c7(&mut self) -> bool {
        //         ram:89c7 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //                              loop
        loop {
            self.SetPC(0x89c8);
            //         ram:89c8 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:89c9 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:89ca b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:89cb ca d4 89        JP         Z,l_exit
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP l_exit;
                break;
            }
            //         ram:89ce cd d6 89        CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));
            //         ram:89d1 c3 c8 89        JP         loop
            self.IncPC(3);
            self.increase_cycles(10); //JP loop;
        }
        //                              l_exit
        self.SetPC(0x89d4);
        //         ram:89d4 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:89d5 c9              RET
        self.assert_pc(0x89d5);
        true
    }
    pub(crate) fn hook_89d6(&mut self) -> bool {
        //         ram:89d6 fe a0           CP         0xa0
        self.instr_hk__CP_NN(0xa0);
        //         ram:89d8 d2 e0 89        JP         NC,LAB_ram_89e0
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) == 0 {
            // JP LAB_ram_89e0;
            self.SetPC(0x89e0);
            //                              LAB_ram_89e0
            //         ram:89e0 c6 60           ADD        A,0x60
            self.instr_hk__ADD_A_NN(0x60);
        } else {
            // ram:89db
            //         ram:89db c6 a0           ADD        A,0xa0
            self.instr_hk__ADD_A_NN(0xa0);
            //         ram:89dd c3 e2 89        JP         LAB_ram_89e2
            self.IncPC(3);
            self.increase_cycles(10); //JP LAB_ram_89e2;
        }
        self.SetPC(0x89e2);
        //                              LAB_ram_89e2
        //         ram:89e2 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:89e3 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:89e4 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:89e5 cd bc 89        CALL       fn_calc_voffset_89BC
        assert!(self.call_hook(0x89bc));
        //         ram:89e8 11 00 18        LD         DE,SCREEN2_PATTERN_GENERATOR_TABLE_SIZE
        self.instr_hk__LD_DE_NNNN(SCREEN2_PATTERN_GENERATOR_TABLE_SIZE);
        //         ram:89eb 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:89ec f3              DI
        self.instr_hk__DI();
        //         ram:89ed cd 94 c0        CALL       sb_vram_write_1_byte_C094
        assert!(self.call_hook(0xc094));
        //         ram:89f0 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:89f1 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:89f2 14              INC        D
        self.instr_hk__INC_D();
        //         ram:89f3 fb              EI
        self.instr_hk__EI();
        //         ram:89f4 c9              RET
        self.assert_pc(0x89f4);
        true
    }
    pub(crate) fn hook_89f5(&mut self) -> bool {
        //
        //                              **************************************************************
        //                              *                          FUNCTION                          *
        //                              **************************************************************
        //                              undefined sb_check_mem_fill_vram_89F5()
        //              undefined         A:1            <RETURN>
        //                              sb_check_mem_fill_vram_89F5                     XREF[2]:     FUN_ram_481e:4991(c),
        //                                                                                           sb_set_mem_fill_vram_guess_B35D:
        //         ram:89f5 2a b8 c8        LD         HL,(BYTE_ram_c8b8)
        self.instr_hk__LD_HL_iNNNN(0xc8b8);
        //         ram:89f8 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:89f9 fe ff           CP         0xff
        self.instr_hk__CP_NN(0xff);
        //         ram:89fb c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            return true;
        } else {
            self.increase_cycles(5);
        }
        //         ram:89fc cd 87 b3        CALL       sb_calc_vaddr_B387                               hl <- (*c8b7) >> 1
        assert!(self.call_hook(0xb387));
        //         ram:89ff 11 00 02        LD         DE,0x200
        self.instr_hk__LD_DE_NNNN(0x200);
        //         ram:8a02 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8a03 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:8a04 2a ba c8        LD         HL,(wd_addr_c8ba)
        self.instr_hk__LD_HL_iNNNN(0xc8ba);
        //         ram:8a07 5e              LD         E,(HL)
        self.instr_hk__LD_E_iHL();
        //         ram:8a08 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a09 56              LD         D,(HL)
        self.instr_hk__LD_D_iHL();
        //         ram:8a0a 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a0b 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8a0c 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8a0d 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8a0e 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8a0f 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:8a10 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a11 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a12 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a13 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a14 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8a15 fe ff           CP         0xff
        self.instr_hk__CP_NN(0xff);
        //         ram:8a17 28 17           JR         Z,LAB_ram_8a30
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_8a30);
        } else {
            self.increase_cycles(7);
            //         ram:8a19 e6 02           AND        0x2
            self.instr_hk__AND_NN(0x2);
            //         ram:8a1b 28 13           JR         Z,LAB_ram_8a30
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_8a30);
            } else {
                self.increase_cycles(7);
                //         ram:8a1d c5              PUSH       BC
                self.instr_hk__PUSH_BC();
                //         ram:8a1e cd 97 47        CALL       sb_read_mem_4797                                 hl <- c8ca + *c8b7 if *c23e == 0
                assert!(self.call_hook(0x4797));
                //                                                                                                    c8d2 + *c8b7 otherwise
                //         ram:8a21 c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:8a22 7e              LD         A,(HL)
                self.instr_hk__LD_A_iHL();
                //         ram:8a23 e6 03           AND        0x3
                self.instr_hk__AND_NN(0x3);
                //         ram:8a25 06 00           LD         B,0x0
                self.instr_hk__LD_B_NN(0x0);
                //         ram:8a27 eb              EX         DE,HL
                self.instr_hk__EX_DE_HL();
                //         ram:8a28 b7              OR         A
                self.instr_hk__OR_A_A();
                //                              loop_1                                          XREF[1]:     ram:8a2d(j)
                loop {
                    self.SetPC(0x8a29);
                    //         ram:8a29 28 04           JR         Z,LAB_ram_8a2f
                    self.IncPC(2);
                    if (self.data.F & FLAG_Z) != 0 {
                        self.increase_cycles(12);
                        // JR(LAB_ram_8a2f);
                        break;
                    } else {
                        self.increase_cycles(7);
                    }

                    //         ram:8a2b 09              ADD        HL,BC
                    self.instr_hk__ADD_HL_BC();
                    //         ram:8a2c 3d              DEC        A
                    self.instr_hk__DEC_A();
                    //         ram:8a2d 18 fa           JR         loop_1
                    self.IncPC(2);
                    self.increase_cycles(12);
                    // JR(loop_1);
                }

                //                              LAB_ram_8a2f                                    XREF[1]:     ram:8a29(j)
                //         ram:8a2f eb              EX         DE,HL
                self.instr_hk__EX_DE_HL();
            }
        }

        //                              LAB_ram_8a30                                    XREF[2]:     sb_check_mem_fill_vram_89F5:8a17
        //                                                                                           sb_check_mem_fill_vram_89F5:8a1b
        //         ram:8a30 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:8a31 cd 97 47        CALL       sb_read_mem_4797                                 hl <- c8ca + *c8b7 if *c23e == 0
        assert!(self.call_hook(0x4797));
        //                                                                                                    c8d2 + *c8b7 otherwise
        //         ram:8a34 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:8a35 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8a36 e6 04           AND        0x4
        self.instr_hk__AND_NN(0x4);
        //         ram:8a38 28 18           JR         Z,LAB_ram_8a52
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_8a52);
        } else {
            self.increase_cycles(7);
            //         ram:8a3a 2a ba c8        LD         HL,(wd_addr_c8ba)
            self.instr_hk__LD_HL_iNNNN(0xc8ba);
            //         ram:8a3d 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8a3e 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8a3f 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8a40 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8a41 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8a42 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8a43 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:8a44 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:8a45 fe ff           CP         0xff
            self.instr_hk__CP_NN(0xff);
            //         ram:8a47 28 07           JR         Z,LAB_ram_8a50
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_8a50);
            } else {
                self.increase_cycles(7);
                //         ram:8a49 e6 02           AND        0x2
                self.instr_hk__AND_NN(0x2);
                //         ram:8a4b 28 03           JR         Z,LAB_ram_8a50
                self.IncPC(2);
                if (self.data.F & FLAG_Z) != 0 {
                    self.increase_cycles(12);
                    // JR(LAB_ram_8a50);
                } else {
                    self.increase_cycles(7);
                    //         ram:8a4d 09              ADD        HL,BC
                    self.instr_hk__ADD_HL_BC();
                    //         ram:8a4e 09              ADD        HL,BC
                    self.instr_hk__ADD_HL_BC();
                    //         ram:8a4f 09              ADD        HL,BC
                    self.instr_hk__ADD_HL_BC();
                }
            }

            //                              LAB_ram_8a50                                    XREF[2]:     ram:8a47(j), ram:8a4b(j)
            //         ram:8a50 09              ADD        HL,BC
            self.instr_hk__ADD_HL_BC();
            //         ram:8a51 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
        }
        self.SetPC(0x8a52);

        //                              LAB_ram_8a52                                    XREF[1]:     ram:8a38(j)
        //         ram:8a52 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8a53 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:8a54 f3              DI
        self.instr_hk__DI();
        //         ram:8a55 cd 60 8a        CALL       sb_blit_ram_to_vram_guess_8A60                   IN bc: count
        assert!(self.call_hook(0x8a60));
        //                                                                                                  de: targe vram addr
        //                                                                                                  hl: font or sprite addr
        //                                                                                              OUT de <- de + 0x800
        //         ram:8a58 cd 60 8a        CALL       sb_blit_ram_to_vram_guess_8A60                   IN bc: count
        assert!(self.call_hook(0x8a60));
        //                                                                                                  de: targe vram addr
        //                                                                                                  hl: font or sprite addr
        //                                                                                              OUT de <- de + 0x800
        //         ram:8a5b cd 60 8a        CALL       sb_blit_ram_to_vram_guess_8A60                   IN bc: count
        assert!(self.call_hook(0x8a60));
        //                                                                                                  de: targe vram addr
        //                                                                                                  hl: font or sprite addr
        //                                                                                              OUT de <- de + 0x800
        //         ram:8a5e fb              EI
        self.instr_hk__EI();
        //         ram:8a5f c9              RET
        // return true;
        self.assert_pc(0x8a5f);
        //
        true
    }
    // fn hook_8a21(&mut self) -> bool {
    //     //
    //     //                              *************************************************************
    //     //                              *                           FUNCTION
    //     //                              *************************************************************
    //     //                              undefined  FUN_ram_8a21 ()
    //     //              undefined         A:1            <RETURN>
    //     //                              FUN_ram_8a21
    //     //         ram:8a21 c1              POP        BC
    //     self.instr_hk__POP_BC();
    //     //         ram:8a22 7e              LD         A,(HL)
    //     self.instr_hk__LD_A_iHL();
    //     //         ram:8a23 e6  03           AND        0x3
    //     self.instr_hk__AND_NN(0x3);
    //     //         ram:8a25 06  00           LD         B,0x0
    //     self.instr_hk__LD_B_NN(0x0);
    //     //         ram:8a27 eb              EX         DE,HL
    //     self.instr_hk__EX_DE_HL();
    //     //         ram:8a28 b7              OR         A
    //     self.instr_hk__OR_A_A();
    //     //                              LAB_ram_8a29                                    XREF[1]:     ram:8a2d (j)
    //     loop {
    //         //         ram:8a29 28  04           JR         Z,LAB_ram_8a2f
    //         self.IncPC(2);
    //         if (self.data.F & FLAG_Z) != 0 {
    //             self.increase_cycles(12);
    //             // JR(LAB_ram_8a2f);
    //             break;
    //         } else {
    //             self.increase_cycles(7);
    //         }

    //         //         ram:8a2b 09              ADD        HL,BC
    //         self.instr_hk__ADD_HL_BC();
    //         //         ram:8a2c 3d              DEC        A
    //         self.instr_hk__DEC_A();
    //         //         ram:8a2d 18  fa           JR         LAB_ram_8a29
    //         self.IncPC(2);
    //         self.increase_cycles(12);
    //         // JR(LAB_ram_8a29);
    //     }

    //     //                              LAB_ram_8a2f                                    XREF[1]:     ram:8a29 (j)
    //     //         ram:8a2f eb              EX         DE,HL
    //     self.instr_hk__EX_DE_HL();
    //     //                              LAB_ram_8a30                                    XREF[2]:     sb_check_mem_fill_vram_89F5:8a17
    //     //                                                                                           sb_check_mem_fill_vram_89F5:8a1b
    //     //         ram:8a30 c5              PUSH       BC
    //     self.instr_hk__PUSH_BC();
    //     //         ram:8a31 cd  97  47       CALL       sb_read_mem_4797                                 hl <- c8ca + *c8b7 if *c23e == 0
    //     assert!(self.call_hook(0x4797));
    //     //                                                                                                    c8d2 + *c8b7 otherwise
    //     //         ram:8a34 c1              POP        BC
    //     self.instr_hk__POP_BC();
    //     //         ram:8a35 7e              LD         A,(HL)
    //     self.instr_hk__LD_A_iHL();
    //     //         ram:8a36 e6  04           AND        0x4
    //     self.instr_hk__AND_NN(0x4);
    //     //         ram:8a38 28  18           JR         Z,LAB_ram_8a52
    //     self.IncPC(2);
    //     if (self.data.F & FLAG_Z) != 0 {
    //         self.increase_cycles(12);
    //         // JR(LAB_ram_8a52);
    //     } else {
    //         self.increase_cycles(7);
    //         //         ram:8a3a 2a  ba  c8       LD         HL,(BYTE_ram_c8ba )
    //         self.instr_hk__LD_HL_iNNNN(0xc8ba);
    //         //         ram:8a3d 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:8a3e 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:8a3f 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:8a40 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:8a41 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:8a42 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:8a43 7e              LD         A,(HL)
    //         self.instr_hk__LD_A_iHL();
    //         //         ram:8a44 eb              EX         DE,HL
    //         self.instr_hk__EX_DE_HL();
    //         //         ram:8a45 fe  ff           CP         0xff
    //         self.instr_hk__CP_NN(0xff);
    //         //         ram:8a47 28  07           JR         Z,LAB_ram_8a50
    //         self.IncPC(2);
    //         if (self.data.F & FLAG_Z) != 0 {
    //             self.increase_cycles(12);
    //             // JR(LAB_ram_8a50);
    //         } else {
    //             self.increase_cycles(7);
    //             //         ram:8a49 e6  02           AND        0x2
    //             self.instr_hk__AND_NN(0x2);
    //             //         ram:8a4b 28  03           JR         Z,LAB_ram_8a50
    //             self.IncPC(2);
    //             if (self.data.F & FLAG_Z) != 0 {
    //                 self.increase_cycles(12);
    //                 // JR(LAB_ram_8a50);
    //             } else {
    //                 self.increase_cycles(7);
    //                 //         ram:8a4d 09              ADD        HL,BC
    //                 self.instr_hk__ADD_HL_BC();
    //                 //         ram:8a4e 09              ADD        HL,BC
    //                 self.instr_hk__ADD_HL_BC();
    //                 //         ram:8a4f 09              ADD        HL,BC
    //                 self.instr_hk__ADD_HL_BC();
    //             }
    //         }

    //         //                              LAB_ram_8a50                                    XREF[2]:     ram:8a47 (j) , ram:8a4b (j)
    //         //         ram:8a50 09              ADD        HL,BC
    //         self.instr_hk__ADD_HL_BC();
    //         //         ram:8a51 eb              EX         DE,HL
    //         self.instr_hk__EX_DE_HL();
    //     }

    //     //                              LAB_ram_8a52                                    XREF[1]:     ram:8a38 (j)
    //     //         ram:8a52 eb              EX         DE,HL
    //     self.instr_hk__EX_DE_HL();
    //     //         ram:8a53 d1              POP        DE
    //     self.instr_hk__POP_DE();
    //     //         ram:8a54 f3              DI
    //     self.instr_hk__DI();
    //     //         ram:8a55 cd  60  8a       CALL       sb_blit_ram_to_vram_guess_8A60                   IN bc: count
    //     assert!(self.call_hook(0x8a60));
    //     //                                                                                                  de: targe vram addr
    //     //                                                                                                  hl: source addr
    //     //                                                                                              OUT de <- de + 0x800
    //     //         ram:8a58 cd  60  8a       CALL       sb_blit_ram_to_vram_guess_8A60                   IN bc: count
    //     assert!(self.call_hook(0x8a60));
    //     //                                                                                                  de: targe vram addr
    //     //                                                                                                  hl: source addr
    //     //                                                                                              OUT de <- de + 0x800
    //     //         ram:8a5b cd  60  8a       CALL       sb_blit_ram_to_vram_guess_8A60                   IN bc: count
    //     assert!(self.call_hook(0x8a60));
    //     //                                                                                                  de: targe vram addr
    //     //                                                                                                  hl: source addr
    //     //                                                                                              OUT de <- de + 0x800
    //     //         ram:8a5e fb              EI
    //     self.instr_hk__EI();
    //     //         ram:8a5f c9              RET
    //     self.assert_pc(0x8a5f);
    //     //
    //     true
    // }
    pub(crate) fn hook_8a60(&mut self) -> bool {
        //
        //                              *************************************************************
        //                              *                          SUBROUTINE
        //                              *************************************************************
        //                              sb_blit_ram_to_vram_guess_8A60                  XREF[3]:     FUN_ram_8a21:8a55 (c) ,
        //                                                                                           FUN_ram_8a21:8a58 (c) ,
        //                                                                                           FUN_ram_8a21:8a5b (c)
        //         ram:8a60 c5              PUSH       BC                                               IN bc: count
        self.instr_hk__PUSH_BC();
        //                                                                                                  de: targe vram addr
        //                                                                                                  hl: source addr
        //                                                                                              OUT de <- de + 0x800
        //         ram:8a61 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:8a62 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:8a63 cd  85  c0       CALL       sb_blit_ram_to_vram_guess_C085                   IN bc: count
        assert!(self.call_hook(0xc085));
        //                                                                                                  de: targe vram addr
        //                                                                                                  hl: source addr
        //         ram:8a66 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:8a67 21  00  08       LD         HL,0x800
        self.instr_hk__LD_HL_NNNN(0x800);
        //         ram:8a6a 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8a6b eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8a6c e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:8a6d c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:8a6e c9              RET
        self.assert_pc(0x8a6e);
        //
        true
    }
    pub(crate) fn hook_8a6f(&mut self) -> bool {
        //         ram:8a6f 2a ba c8        LD         HL,(wd_addr_c8ba)
        self.instr_hk__LD_HL_iNNNN(0xc8ba);
        //         ram:8a72 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a73 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a74 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a75 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8a76 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:8a77 cd 87 b3        CALL       sb_calc_vaddr_B387                               hl <- (*c8b7) >> 1
        assert!(self.call_hook(0xb387));
        //         ram:8a7a 11 00 22        LD         DE,0x2200
        self.instr_hk__LD_DE_NNNN(0x2200);
        //         ram:8a7d 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8a7e 01 80 00        LD         BC,0x80
        self.instr_hk__LD_BC_NNNN(0x80);
        //         ram:8a81 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:8a82 cd 40 88        CALL       sb_fill_vram_guess_8840                          IN
        assert!(self.call_hook(0x8840));
        //                                                                                                hl: vram addr?
        //                                                                                                bc: count
        //                                                                                                a: value
        //         ram:8a85 c9              RET
        self.assert_pc(0x8a85);
        //
        true
    }
    pub(crate) fn hook_8a86(&mut self) -> bool {
        //         ram:8a86 cd da 47        CALL       sb_read_mem_calc_47DA                            OUT bc, hl
        assert!(self.call_hook(0x47da));
        //         ram:8a89 2a b8 c8        LD         HL,(BYTE_ram_c8b8)
        self.instr_hk__LD_HL_iNNNN(0xc8b8);
        //         ram:8a8c 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a8d 56              LD         D,(HL)
        self.instr_hk__LD_D_iHL();
        //         ram:8a8e 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8a8f 5e              LD         E,(HL)
        self.instr_hk__LD_E_iHL();
        //         ram:8a90 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8a91 c9              RET
        self.assert_pc(0x8a91);
        //
        true
    }
    pub(crate) fn hook_8a92(&mut self) -> bool {
        //         ram:8a92 cd 86 8a        CALL       sb_read_calc_mem_8A86                            OUT bc, hl
        assert!(self.call_hook(0x8a86));
        //         ram:8a95 3a b7 c8        LD         A,(BYTE_ram_c8b7)
        self.instr_hk__LD_A_iNNNN(0xc8b7);
        //         ram:8a98 c6 04           ADD        A,0x4
        self.instr_hk__ADD_A_NN(0x4);
        //         ram:8a9a 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8a9b 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8a9c 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8a9d 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        self.internal_8a9e();
        true
    }
    fn internal_8a9e(&mut self) {
        //                              **************************************************************
        //                              *                          FUNCTION                          *
        //                              **************************************************************
        //                              undefined sb_set_vmem_guess_8a9e()                         IN a: val0?
        //                                                                                          hl: x0/y0?
        //                                                                                          bc: w/h?
        //              undefined         A:1            <RETURN>
        //                              sb_set_vmem_guess_8a9e                          XREF[4]:     ram:4535(c),
        //                                                                                           FUN_ram_481e:484e(c),
        //                                                                                           ram:700a(c),
        //                                                                                           sb_shop_guess_b3b2:b411(c)
        //         ram:8a9e 08              EX         AF,AF_                                           IN a: val0?
        self.instr_hk__EX_AF_AF_();
        //                                                                                                hl: x0/y0?
        //                                                                                                bc: w/h?
        //                              loop_1                                          XREF[1]:     ram:8ac6(j)
        loop {
            self.SetPC(0x8a9f);
            //         ram:8a9f e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:8aa0 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //                              loop_1_a                                        XREF[1]:     ram:8ac0(j)
            loop {
                self.SetPC(0x8aa1);
                //         ram:8aa1 c5              PUSH       BC
                self.instr_hk__PUSH_BC();
                //         ram:8aa2 7c              LD         A,H
                self.instr_hk__LD_A_H();
                //         ram:8aa3 fe 20           CP         0x20
                self.instr_hk__CP_NN(0x20);
                //         ram:8aa5 30 11           JR         NC,LAB_ram_8ab8
                self.IncPC(2);
                if (self.data.F & FLAG_C) == 0 {
                    self.increase_cycles(12);
                    // JR(LAB_ram_8ab8);
                } else {
                    self.increase_cycles(7);
                    //         ram:8aa7 7d              LD         A,L
                    self.instr_hk__LD_A_L();
                    //         ram:8aa8 fe 14           CP         0x14
                    self.instr_hk__CP_NN(0x14);
                    //         ram:8aaa 30 0c           JR         NC,LAB_ram_8ab8
                    self.IncPC(2);
                    if (self.data.F & FLAG_C) == 0 {
                        self.increase_cycles(12);
                        // JR(LAB_ram_8ab8);
                    } else {
                        self.increase_cycles(7);
                        //         ram:8aac e5              PUSH       HL
                        self.instr_hk__PUSH_HL();
                        //         ram:8aad cd bc 89        CALL       fn_calc_voffset_89BC                             hl <- (hl & 0xff) * 20 + (hl >> 8)
                        assert!(self.call_hook(0x89bc));
                        //                                                                                              de <- (hl >> 8) << 8 + (hl & 0xff)
                        //         ram:8ab0 11 9a c9        LD         DE,DAT_ram_c99a
                        self.instr_hk__LD_DE_NNNN(0xc99a);
                        //         ram:8ab3 19              ADD        HL,DE
                        self.instr_hk__ADD_HL_DE();
                        //         ram:8ab4 08              EX         AF,AF_
                        self.instr_hk__EX_AF_AF_();
                        //         ram:8ab5 77              LD         (HL),A
                        self.instr_hk__LD_iHL_A();
                        //         ram:8ab6 08              EX         AF,AF_
                        self.instr_hk__EX_AF_AF_();
                        //         ram:8ab7 e1              POP        HL
                        self.instr_hk__POP_HL();
                    }
                }

                //                              LAB_ram_8ab8                                    XREF[2]:     ram:8aa5(j), ram:8aaa(j)
                //         ram:8ab8 08              EX         AF,AF_
                self.instr_hk__EX_AF_AF_();
                //         ram:8ab9 b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:8aba 28 01           JR         Z,loop_1_a_chk_cond
                self.IncPC(2);
                if (self.data.F & FLAG_Z) != 0 {
                    self.increase_cycles(12);
                    // JR(loop_1_a_chk_cond);
                } else {
                    self.increase_cycles(7);
                    //         ram:8abc 3c              INC        A
                    self.instr_hk__INC_A();
                }

                //                              loop_1_a_chk_cond                               XREF[1]:     ram:8aba(j)
                //         ram:8abd 08              EX         AF,AF_
                self.instr_hk__EX_AF_AF_();
                //         ram:8abe 24              INC        H
                self.instr_hk__INC_H();
                //         ram:8abf c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:8ac0 10 df           DJNZ       loop_1_a
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP loop_1_a;
                } else {
                    self.increase_cycles(8);
                    break;
                }
            }

            //         ram:8ac2 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:8ac3 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:8ac4 2c              INC        L
            self.instr_hk__INC_L();
            //         ram:8ac5 0d              DEC        C
            self.instr_hk__DEC_C();
            //         ram:8ac6 20 d7           JR         NZ,loop_1
            self.IncPC(2);
            if (self.data.F & FLAG_Z) == 0 {
                self.increase_cycles(12);
                // JR(loop_1);
            } else {
                self.increase_cycles(7);
                break;
            }
        }

        // self.assert_pc(0x8ac8);
        //         ram:8ac8 c9              RET
        // return true;
        //
        // true
    }
    pub(crate) fn hook_8a9e(&mut self) -> bool {
        self.internal_8a9e();
        true
    }
    pub(crate) fn hook_8ac9(&mut self) -> bool {
        // println!("hook_8ac9");
        //         ram:8ac9 21 1a cc        LD         HL,font_cc1a
        self.instr_hk__LD_HL_NNNN(0xcc1a);
        //         ram:8acc cb 78           BIT        0x7,B
        self.instr_hk__BIT_7_B();
        //         ram:8ace ca d5 8a        JP         Z,LAB_ram_8ad5
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP LAB_ram_8ad5;
        } else {
            // ram:8ad1
            //         ram:8ad1 11 80 00        LD         DE,0x80
            self.instr_hk__LD_DE_NNNN(0x80);
            //         ram:8ad4 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
        }
        //                              LAB_ram_8ad5
        //         ram:8ad5 11 00 02        LD         DE,0x200
        self.instr_hk__LD_DE_NNNN(0x200);
        //         ram:8ad8 cb b8           RES        0x7,B
        self.instr_hk__RES_7_B();
        //         ram:8ada b7              OR         A
        self.instr_hk__OR_A_A();
        //                              loop_1
        loop {
            self.SetPC(0x8adb);
            //         ram:8adb ca e3 8a        JP         Z,LAB_ram_8ae3
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP LAB_ram_8ae3;
                break;
            } else {
                //         ram:8ade 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:8adf 3d              DEC        A
                self.instr_hk__DEC_A();
                //         ram:8ae0 c3 db 8a        JP         loop_1
                self.IncPC(3);
                self.increase_cycles(10); //JP loop_1;
            }
        }

        //                              LAB_ram_8ae3
        //         ram:8ae3 11 20 00        LD         DE,0x20
        self.instr_hk__LD_DE_NNNN(0x20);
        //         ram:8ae6 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:8ae7 b7              OR         A
        self.instr_hk__OR_A_A();
        //                              loop_2
        loop {
            self.SetPC(0x8ae8);
            //         ram:8ae8 ca f0 8a        JP         Z,LAB_ram_8af0
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP LAB_ram_8af0;
                break;
            } else {
                //         ram:8aeb 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:8aec 3d              DEC        A
                self.instr_hk__DEC_A();
                //         ram:8aed c3 e8 8a        JP         loop_2
                self.IncPC(3);
                self.increase_cycles(10); //JP loop_2;
            }
        }
        self.SetPC(0x8af0);
        //                              LAB_ram_8af0
        //         ram:8af0 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:8af1 69              LD         L,C
        self.instr_hk__LD_L_C();
        //         ram:8af2 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:8af4 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8af5 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8af6 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8af7 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8af8 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8af9 11 00 38        LD         DE,0x3800
        self.instr_hk__LD_DE_NNNN(0x3800);
        //         ram:8afc 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8afd eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8afe e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:8aff e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:8b00 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:8b01 01 20 00        LD         BC,0x20
        self.instr_hk__LD_BC_NNNN(0x20);
        //         ram:8b04 f3              DI
        self.instr_hk__DI();
        //         ram:8b05 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085
        assert!(self.call_hook(0xc085));

        //         ram:8b08 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:8b09 11 60 00        LD         DE,0x60
        self.instr_hk__LD_DE_NNNN(0x60);
        //         ram:8b0c 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:8b0d d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:8b0e eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8b0f 01 00 01        LD         BC,0x100
        self.instr_hk__LD_BC_NNNN(0x100);
        //         ram:8b12 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8b13 01 20 00        LD         BC,0x20
        self.instr_hk__LD_BC_NNNN(0x20);
        //         ram:8b16 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085
        assert!(self.call_hook(0xc085));
        //         ram:8b19 fb              EI
        self.assert_pc(0x8b19);
        self.instr_hk__EI();
        //         ram:8b1a c9              RET
        self.assert_pc(0x8b1a);
        true
    }
    pub(crate) fn hook_8b1b(&mut self) -> bool {
        //         ram:8b1b f3              DI
        self.instr_hk__DI();
        //         ram:8b1c cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21
        assert!(self.call_hook(0x8b21));
        //         ram:8b1f fb              EI
        self.instr_hk__EI();
        //         ram:8b20 c9              RET
        self.assert_pc(0x8b20);
        true
    }
    pub(crate) fn hook_8b21(&mut self) -> bool {
        //         ram:8b21 06 00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:8b23 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:8b24 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:8b25 60              LD         H,B
        self.instr_hk__LD_H_B();
        //         ram:8b26 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8b27 29              ADD        HL,HL
        self.instr_hk__ADD_HL_HL();
        //         ram:8b28 01 a9 c2        LD         BC,sprite_c2a9
        self.instr_hk__LD_BC_NNNN(0xc2a9);
        //         ram:8b2b 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8b2c c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:8b2d e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:8b2e 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8b2f 73              LD         (HL),E
        self.instr_hk__LD_iHL_E();
        //         ram:8b30 21 29 c3        LD         HL,BYTE_ram_c329
        self.instr_hk__LD_HL_NNNN(0xc329);
        //         ram:8b33 0e 00           LD         C,0x0
        self.instr_hk__LD_C_NN(0x0);
        //                              loop
        loop {
            self.SetPC(0x8b35);
            //         ram:8b35 be              CP         (HL=>BYTE_ram_c329)
            self.instr_hk__CP_iHL();
            //         ram:8b36 ca 3e 8b        JP         Z,l_exit_x
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP l_exit_x;
                break;
            }
            //         ram:8b39 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8b3a 0c              INC        C
            self.instr_hk__INC_C();
            //         ram:8b3b c3 35 8b        JP         loop
            self.IncPC(3);
            self.increase_cycles(10); //JP loop;
        }
        //                              l_exit_x
        self.SetPC(0x8b3e);
        //         ram:8b3e 79              LD         A,C
        self.instr_hk__LD_A_C();
        //         ram:8b3f 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8b40 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8b41 21 00 1b        LD         HL,SCREEN_2_VRAM_SPRITE_TABLE_BEGIN
        self.instr_hk__LD_HL_NNNN(SCREEN_2_VRAM_SPRITE_TABLE_BEGIN);
        //         ram:8b44 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:8b45 06 00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:8b47 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8b48 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:8b49 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:8b4a 01 04 00        LD         BC,0x4
        self.instr_hk__LD_BC_NNNN(0x4);
        //         ram:8b4d cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085
        assert!(self.call_hook(0xc085));
        //         ram:8b50 c9              RET
        self.assert_pc(0x8b50);
        true
    }
    pub(crate) fn hook_8b6c(&mut self) -> bool {
        //         ram:8b6c f3              DI
        self.instr_hk__DI();
        //         ram:8b6d cd 72 8b        CALL       sb_mem_blit_ram_to_vram_guess_8b72
        assert!(self.call_hook(0x8b72));
        //         ram:8b70 fb              EI
        self.instr_hk__EI();
        //         ram:8b71 c9              RET
        self.assert_pc(0x8b71);

        true
    }
    pub(crate) fn hook_8b72(&mut self) -> bool {
        //         ram:8b72 f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:8b73 06  00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:8b75 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:8b76 21  49  c3       LD         HL,BYTE_ram_c349
        self.instr_hk__LD_HL_NNNN(0xc349);
        //         ram:8b79 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:8b7a 70              LD         (HL),B=>BYTE_ram_c349
        self.instr_hk__LD_iHL_B();
        //         ram:8b7b 48              LD         C,B
        self.instr_hk__LD_C_B();
        //         ram:8b7c 1e  d1           LD         E,0xd1
        self.instr_hk__LD_E_NN(0xd1);
        //         ram:8b7e cd  21  8b       CALL       sb_blit_ram_to_vram_guess_8b21
        assert!(self.call_hook(0x8b21));

        //         ram:8b81 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:8b82 f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:8b83 3a  a8  c2       LD         A,(DAT_ram_c2a8 )
        self.instr_hk__LD_A_iNNNN(0xc2a8);
        //         ram:8b86 3d              DEC        A
        self.instr_hk__DEC_A();
        //         ram:8b87 32  a8  c2       LD         (DAT_ram_c2a8 ),A
        self.instr_hk__LD_iNNNN_A(0xc2a8);
        //         ram:8b8a 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:8b8b f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:8b8c 21  2c  c3       LD         HL,BYTE_ram_c32c                                 = 1Fh
        self.instr_hk__LD_HL_NNNN(0xc32c);
        //                              LAB_ram_8b8f
        loop {
            self.SetPC(0x8b8f);
            //         ram:8b8f be              CP         (HL=>BYTE_ram_c32c )                             = 1Fh
            self.instr_hk__CP_iHL();
            //         ram:8b90 ca  98  8b       JP         Z,LAB_ram_8b98
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP LAB_ram_8b98;
                break;
            }
            //         ram:8b93 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8b94 05              DEC        B
            self.instr_hk__DEC_B();
            //         ram:8b95 c3  8f  8b       JP         LAB_ram_8b8f
            self.IncPC(3);
            self.increase_cycles(10); //JP LAB_ram_8b8f;
        }
        self.SetPC(0x8b98);
        //                              LAB_ram_8b98
        //         ram:8b98 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:8b99 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:8b9a ca  a4  8b       JP         Z,LAB_ram_8ba4
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP LAB_ram_8ba4;
        } else {
            // LAB_ram_8b9d
            //                              LAB_ram_8b9d
            loop {
                self.SetPC(0x8b9d);
                //         ram:8b9d 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:8b9e 7e              LD         A,(HL=>BYTE_ram_c32d )                           = 1Fh
                self.instr_hk__LD_A_iHL();
                //         ram:8b9f 2b              DEC        HL
                self.instr_hk__DEC_HL();
                //         ram:8ba0 77              LD         (HL),A
                self.instr_hk__LD_iHL_A();
                //         ram:8ba1 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:8ba2 10  f9           DJNZ       LAB_ram_8b9d
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP LAB_ram_8b9d;
                } else {
                    self.increase_cycles(8);
                    break;
                }
            }
        }
        self.SetPC(0x8ba4);
        //                              LAB_ram_8ba4                                         sb_mem_blit_ram_to_vram_guess_8b
        //         ram:8ba4 3e  1f           LD         A,0x1f
        self.instr_hk__LD_A_NN(0x1f);
        //         ram:8ba6 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:8ba7 0e  00           LD         C,0x0
        self.instr_hk__LD_C_NN(0x0);
        //         ram:8ba9 1e  d1           LD         E,0xd1
        self.instr_hk__LD_E_NN(0xd1);
        //         ram:8bab cd  21  8b       CALL       sb_blit_ram_to_vram_guess_8b21
        assert!(self.call_hook(0x8b21));

        //         ram:8bae c9              RET

        self.assert_pc(0x8bae);
        true
    }
    pub(crate) fn hook_8baf(&mut self) -> bool {
        //         ram:8baf f3              DI
        self.instr_hk__DI();
        //         ram:8bb0 3a  1b  c2       LD         A,(bt_player_idx_c21b )
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:8bb3 0e  00           LD         C,0x0
        self.instr_hk__LD_C_NN(0x0);
        //         ram:8bb5 1e  d1           LD         E,0xd1
        self.instr_hk__LD_E_NN(0xd1);
        //         ram:8bb7 cd  21  8b       CALL       sb_blit_ram_to_vram_guess_8b21
        assert!(self.call_hook(0x8b21));

        //         ram:8bba 3a  1b  c2       LD         A,(bt_player_idx_c21b )
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:8bbd c6  03           ADD        A,0x3
        self.instr_hk__ADD_A_NN(0x3);
        //         ram:8bbf cd  72  8b       CALL       sb_mem_blit_ram_to_vram_guess_8b72
        assert!(self.call_hook(0x8b72));
        //         ram:8bc2 fb              EI
        self.instr_hk__EI();
        //         ram:8bc3 c9              RET

        self.assert_pc(0x8bc3);
        true
    }
    pub(crate) fn hook_8bc4(&mut self) -> bool {
        //         ram:8bc4 cd  ca  8b       CALL       sb_read_mem_for_player_8BCA
        assert!(self.call_hook(0x8bca));
        //         ram:8bc7 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8bc8 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:8bc9 c9              RET
        self.assert_pc(0x8bc9);
        true
    }
    pub(crate) fn hook_8bca(&mut self) -> bool {
        //         ram:8bca 21  90  c2       LD         HL,DAT_ram_c290
        self.instr_hk__LD_HL_NNNN(0xc290);
        //         ram:8bcd cd  63  47       CALL       fn_add_player_idx_to_addr_4763
        assert!(self.call_hook(0x4763));
        //         ram:8bd0 c9              RET
        self.assert_pc(0x8bd0);
        true
    }
    pub(crate) fn hook_8bd1(&mut self) -> bool {
        // log::info!("hook_8bd1");
        //         ram:8bd1 21  a2  c2       LD         HL,0xc2a2
        self.instr_hk__LD_HL_NNNN(0xc2a2);
        //         ram:8bd4 cd  63  47       CALL       fn_add_player_idx_to_addr_4763
        assert!(self.call_hook(0x4763));
        //         ram:8bd7 c9              RET
        self.assert_pc(0x8bd7);
        true
    }
    pub(crate) fn hook_8be4(&mut self) -> bool {
        // log::info!("hook_8be4");
        //         ram:8be4 cd  ea  8b       CALL       sb_read_mem_for_player_8bea
        assert!(self.call_hook(0x8bea));
        //         ram:8be7 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8be8 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:8be9 c9              RET
        self.assert_pc(0x8be9);
        true
    }
    pub(crate) fn hook_8bea(&mut self) -> bool {
        // log::info!("hook_8bea");
        //         ram:8bea 21  93  c2       LD         HL,DAT_ram_c293
        self.instr_hk__LD_HL_NNNN(0xc293);
        //         ram:8bed cd  63  47       CALL       fn_add_player_idx_to_addr_4763
        assert!(self.call_hook(0x4763));
        //         ram:8bf0 c9              RET
        self.assert_pc(0x8bf0);
        true
    }
    pub(crate) fn hook_8bf1(&mut self) -> bool {
        // log::info!("hook_8bf1");
        //         ram:8bf1 21  a5  c2       LD         HL,DAT_ram_c2a5
        self.instr_hk__LD_HL_NNNN(0xc2a5);
        //         ram:8bf4 cd  63  47       CALL       fn_add_player_idx_to_addr_4763
        assert!(self.call_hook(0x4763));
        //         ram:8bf7 c9              RET
        true
    }
    // pub(crate) fn hook_8c58(&mut self) -> bool {
    //     log::info!("hook_8c58");
    //     //
    //     //                              **************************************************************
    //     //                              *                          FUNCTION                          *
    //     //                              **************************************************************
    //     //                              undefined sb_on_tick_guess_8C58()
    //     //              undefined         A:1            <RETURN>
    //     //                              sb_on_tick_guess_8C58                           XREF[1]:     ram:421f(*)
    //     //         ram:8c58 f3              DI
    //     self.instr_hk__DI();
    //     //         ram:8c59 3a e0 f3        LD         A,(RG1SAV_f3e0)                                  1
    //     self.instr_hk__LD_A_iNNNN(0xf3e0);
    //     //                                                                                                      System saves here the by
    //     //         ram:8c5c e6 df           AND        0xdf
    //     self.instr_hk__AND_NN(0xdf);
    //     //         ram:8c5e 0e 01           LD         C,0x1
    //     self.instr_hk__LD_C_NN(0x1);
    //     //         ram:8c60 47              LD         B,A
    //     self.instr_hk__LD_B_A();
    //     //         ram:8c61 cd ca c0        CALL       sb_vdp_write_port_C0CA                           IN b,c
    //     assert!(self.call_hook(0xc0ca));
    //     //         ram:8c64 cd 3e 01        CALL       bios_RDVDP_013e                                  Function : Reads VDP status regi
    //     assert!(self.call_hook(0x013e));
    //     //                                                                                              Output   : A  - Value which was
    //     //                                                                                              Registers: A
    //     //         ram:8c67 3a 1b c2        LD         A,(bt_player_idx_c21b)
    //     self.instr_hk__LD_A_iNNNN(0xc21b);
    //     //         ram:8c6a f5              PUSH       AF
    //     self.instr_hk__PUSH_AF();
    //     //         ram:8c6b af              XOR        A
    //     self.instr_hk__XOR_A_A();
    //     //                              loop_1                                          XREF[1]:     ram:8c81(j)
    //     loop {
    //         self.SetPC(0x8c6c);
    //         //         ram:8c6c 32 1b c2        LD         (bt_player_idx_c21b),A
    //         self.instr_hk__LD_iNNNN_A(0xc21b);
    //         //         ram:8c6f cd 1c 47        CALL       sb_get_controller_for_player_471c                OUT a
    //         assert!(self.call_hook(0x471c));
    //         //                                                                                              bc <- player_idx
    //         //         ram:8c72 3c              INC        A
    //         self.instr_hk__INC_A();
    //         //         ram:8c73 28 06           JR         Z,loop_1_chk_cond
    //         self.IncPC(2);
    //         if (self.data.F & FLAG_Z) != 0 {
    //             self.increase_cycles(12);
    //             // JR(loop_1_chk_cond);
    //         } else {
    //             self.increase_cycles(7);
    //             //         ram:8c75 cd 02 8c        CALL       sb_write_controller_btn_status_mem_for_player_   undefined sb_write_controller_bt
    //             assert!(self.call_hook(0x8c02));
    //             // WRONG CALL sb_write_controller_btn_status_mem_for_player_
    //             //         ram:8c78 cd 2d 8c        CALL       sb_write_controller_stk_status_mem_for_player_   undefined sb_write_controller_st
    //             assert!(self.call_hook(0x8c2d));
    //             // WRONG CALL sb_write_controller_stk_status_mem_for_player_
    //         }

    //         //                              loop_1_chk_cond                                 XREF[1]:     ram:8c73(j)
    //         //         ram:8c7b 3a 1b c2        LD         A,(bt_player_idx_c21b)
    //         self.instr_hk__LD_A_iNNNN(0xc21b);
    //         //         ram:8c7e 3c              INC        A
    //         self.instr_hk__INC_A();
    //         //         ram:8c7f fe 03           CP         0x3
    //         self.instr_hk__CP_NN(0x3);
    //         //         ram:8c81 c2 6c 8c        JP         NZ,loop_1
    //         self.IncPC(3);
    //         self.increase_cycles(10);
    //         if (self.data.F & FLAG_Z) == 0 {
    //             // JP(loop_1);
    //         } else {
    //             break;
    //         }
    //     }

    //     //         ram:8c84 f1              POP        AF
    //     self.instr_hk__POP_AF();
    //     //         ram:8c85 32 1b c2        LD         (bt_player_idx_c21b),A
    //     self.instr_hk__LD_iNNNN_A(0xc21b);
    //     //         ram:8c88 cd a2 77        CALL       sb_psg_mem_77A2                                  undefined sb_psg_mem_77A2()
    //     assert!(self.call_hook(0x77a2));
    //     //         ram:8c8b 21 69 c3        LD         HL,BYTE_ram_c369
    //     self.instr_hk__LD_HL_NNNN(0xc369);
    //     //         ram:8c8e 34              INC        (HL=>BYTE_ram_c369)
    //     self.instr_hk__INC_iHL();
    //     //         ram:8c8f 3a a8 c2        LD         A,(DAT_ram_c2a8)
    //     self.instr_hk__LD_A_iNNNN(0xc2a8);
    //     //         ram:8c92 fe 02           CP         0x2
    //     self.instr_hk__CP_NN(0x2);
    //     //         ram:8c94 da b3 8c        JP         C,LAB_ram_8cb3
    //     self.IncPC(3);
    //     self.increase_cycles(10);
    //     if (self.data.F & FLAG_C) != 0 {
    //         // JP(LAB_ram_8cb3);
    //     } else {
    //         //         ram:8c97 5f              LD         E,A
    //         self.instr_hk__LD_E_A();
    //         //         ram:8c98 16 00           LD         D,0x0
    //         self.instr_hk__LD_D_NN(0x0);
    //         //         ram:8c9a 3a 2c c3        LD         A,(BYTE_ram_c32c)                                = 1Fh
    //         self.instr_hk__LD_A_iNNNN(0xc32c);
    //         //         ram:8c9d f5              PUSH       AF
    //         self.instr_hk__PUSH_AF();
    //         //         ram:8c9e 21 2c c3        LD         HL,BYTE_ram_c32c                                 = 1Fh
    //         self.instr_hk__LD_HL_NNNN(0xc32c);
    //         //         ram:8ca1 7b              LD         A,E
    //         self.instr_hk__LD_A_E();
    //         //         ram:8ca2 3d              DEC        A
    //         self.instr_hk__DEC_A();
    //         //                              loop_2                                          XREF[1]:     ram:8cae(j)
    //         loop {
    //             self.SetPC(0x8ca3);
    //             //         ram:8ca3 ca b1 8c        JP         Z,LAB_ram_8cb1
    //             self.IncPC(3);
    //             self.increase_cycles(10);
    //             if (self.data.F & FLAG_Z) != 0 {
    //                 // JP(LAB_ram_8cb1);
    //                 break;
    //             }

    //             //         ram:8ca6 08              EX         AF,AF_
    //             self.instr_hk__EX_AF_AF_();
    //             //         ram:8ca7 23              INC        HL
    //             self.instr_hk__INC_HL();
    //             //         ram:8ca8 7e              LD         A,(HL=>BYTE_ram_c32d)                            = 1Fh
    //             self.instr_hk__LD_A_iHL();
    //             //         ram:8ca9 2b              DEC        HL
    //             self.instr_hk__DEC_HL();
    //             //         ram:8caa 77              LD         (HL=>BYTE_ram_c32c),A                            = 1Fh
    //             self.instr_hk__LD_iHL_A();
    //             //         ram:8cab 23              INC        HL
    //             self.instr_hk__INC_HL();
    //             //         ram:8cac 08              EX         AF,AF_
    //             self.instr_hk__EX_AF_AF_();
    //             //         ram:8cad 3d              DEC        A
    //             self.instr_hk__DEC_A();
    //             //         ram:8cae c3 a3 8c        JP         loop_2
    //             self.IncPC(3);
    //             self.increase_cycles(10);
    //             // JP(loop_2);
    //         }

    //         //                              LAB_ram_8cb1                                    XREF[1]:     ram:8ca3(j)
    //         //         ram:8cb1 f1              POP        AF
    //         self.instr_hk__POP_AF();
    //         //         ram:8cb2 77              LD         (HL=>BYTE_ram_c32c),A                            = 1Fh
    //         self.instr_hk__LD_iHL_A();
    //     }
    //     self.SetPC(0x8cb3);

    //     //                              LAB_ram_8cb3                                    XREF[1]:     ram:8c94(j)
    //     //         ram:8cb3 3a a8 c2        LD         A,(DAT_ram_c2a8)
    //     self.instr_hk__LD_A_iNNNN(0xc2a8);
    //     //         ram:8cb6 11 0c 1b        LD         DE,0x1b0c
    //     self.instr_hk__LD_DE_NNNN(0x1b0c);
    //     //         ram:8cb9 21 2c c3        LD         HL,BYTE_ram_c32c                                 = 1Fh
    //     self.instr_hk__LD_HL_NNNN(0xc32c);
    //     //         ram:8cbc 47              LD         B,A
    //     self.instr_hk__LD_B_A();
    //     //         ram:8cbd b7              OR         A
    //     self.instr_hk__OR_A_A();
    //     //                              loop_3                                          XREF[1]:     ram:8cde(j)
    //     loop {
    //         self.SetPC(0x8cbe);
    //         //         ram:8cbe ca e1 8c        JP         Z,l_exit_x
    //         self.IncPC(3);
    //         self.increase_cycles(10);
    //         if (self.data.F & FLAG_Z) != 0 {
    //             // JP(l_exit_x);
    //             break;
    //         }

    //         //         ram:8cc1 c5              PUSH       BC
    //         self.instr_hk__PUSH_BC();
    //         //         ram:8cc2 7e              LD         A,(HL=>BYTE_ram_c32c)                            = 1Fh
    //         self.instr_hk__LD_A_iHL();
    //         //         ram:8cc3 87              ADD        A,A
    //         self.instr_hk__ADD_A_A();
    //         //         ram:8cc4 87              ADD        A,A
    //         self.instr_hk__ADD_A_A();
    //         //         ram:8cc5 e5              PUSH       HL=>BYTE_ram_c32c                                = 1Fh
    //         self.instr_hk__PUSH_HL();
    //         //         ram:8cc6 4f              LD         C,A
    //         self.instr_hk__LD_C_A();
    //         //         ram:8cc7 06 00           LD         B,0x0
    //         self.instr_hk__LD_B_NN(0x0);
    //         //         ram:8cc9 21 a9 c2        LD         HL,sprite_c2a9
    //         self.instr_hk__LD_HL_NNNN(0xc2a9);
    //         //         ram:8ccc 09              ADD        HL,BC
    //         self.instr_hk__ADD_HL_BC();
    //         //         ram:8ccd d5              PUSH       DE
    //         self.instr_hk__PUSH_DE();
    //         //         ram:8cce 01 04 00        LD         BC,0x4
    //         self.instr_hk__LD_BC_NNNN(0x4);
    //         //         ram:8cd1 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085                   IN bc: count
    //         assert!(self.call_hook(0xc085));
    //         //                                                                                                  de: targe vram addr
    //         //                                                                                                  hl: source addr
    //         //         ram:8cd4 d1              POP        DE
    //         self.instr_hk__POP_DE();
    //         //         ram:8cd5 21 04 00        LD         HL,0x4
    //         self.instr_hk__LD_HL_NNNN(0x4);
    //         //         ram:8cd8 19              ADD        HL,DE
    //         self.instr_hk__ADD_HL_DE();
    //         //         ram:8cd9 eb              EX         DE,HL
    //         self.instr_hk__EX_DE_HL();
    //         //         ram:8cda e1              POP        HL
    //         self.instr_hk__POP_HL();
    //         //         ram:8cdb 23              INC        HL
    //         self.instr_hk__INC_HL();
    //         //         ram:8cdc c1              POP        BC
    //         self.instr_hk__POP_BC();
    //         //         ram:8cdd 05              DEC        B
    //         self.instr_hk__DEC_B();
    //         //         ram:8cde c3 be 8c        JP         loop_3
    //         self.IncPC(3);
    //         self.increase_cycles(10);
    //         // JP(loop_3);
    //     }
    //     self.SetPC(0x8ce1);

    //     //                              l_exit_x                                        XREF[1]:     ram:8cbe(j)
    //     //         ram:8ce1 3a e0 f3        LD         A,(RG1SAV_f3e0)                                  1
    //     self.instr_hk__LD_A_iNNNN(0xf3e0);
    //     //                                                                                                      System saves here the by
    //     //         ram:8ce4 f6 20           OR         0x20
    //     self.instr_hk__OR_NN(0x20);
    //     //         ram:8ce6 47              LD         B,A
    //     self.instr_hk__LD_B_A();
    //     //         ram:8ce7 0e 01           LD         C,0x1
    //     self.instr_hk__LD_C_NN(0x1);
    //     //         ram:8ce9 cd ca c0        CALL       sb_vdp_write_port_C0CA                           IN b,c
    //     assert!(self.call_hook(0xc0ca));
    //     //         ram:8cec c9              RET
    //     self.assert_pc(0x8cec);
    //     // return true;
    //     //
    //     log::info!("~hook_8c58");
    //     true
    // }
    pub(crate) fn hook_8d12(&mut self) -> bool {
        println!("hook_8d12");
        //
        //                              *************************************************************
        //                              *                           FUNCTION
        //                              *************************************************************
        //                              undefined  sb_mem_8D12 ()
        //              undefined         A:1            <RETURN>
        //                              sb_mem_8D12                                     XREF[2]:     ram:417d (c) , ram:4af9 (c)
        //         ram:8d12 21  1a  cc       LD         HL,DAT_ram_cc1a
        self.instr_hk__LD_HL_NNNN(0xcc1a);
        //         ram:8d15 11  1b  cc       LD         DE,DAT_ram_cc1b
        self.instr_hk__LD_DE_NNNN(0xcc1b);
        //         ram:8d18 ed  4b  58  c2    LD         BC,(BYTE_ram_c258 )
        self.instr_hk__LD_BC_iNNNN(0xc258);
        //         ram:8d1c 36  00           LD         (HL=>DAT_ram_cc1a ),0x0
        self.instr_hk__LD_iHL_NN(0x0);
        //         ram:8d1e ed  b0           LDIR
        self.instr_hk__LDIR();
        //         ram:8d20 21  1a  cc       LD         HL,DAT_ram_cc1a
        self.instr_hk__LD_HL_NNNN(0xcc1a);
        //         ram:8d23 22  56  c2       LD         (BYTE_ram_c256 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc256);
        //         ram:8d26 2a  58  c2       LD         HL,(BYTE_ram_c258 )
        self.instr_hk__LD_HL_iNNNN(0xc258);
        //         ram:8d29 22  5a  c2       LD         (WORD_ram_c25a ),HL
        self.instr_hk__LD_iNNNN_HL(0xc25a);
        //                              LAB_ram_8d2c                                    XREF[1]:     ram:8d32 (j)
        loop {
            self.SetPC(0x8d2c);
            //         ram:8d2c cd  6a  8d       CALL       sb_mem_8d6a                                      undefined sb_mem_8d6a()
            assert!(self.call_hook(0x8d6a));
            //         ram:8d2f cd  4b  8d       CALL       sb_mem_8d4b                                      IN a: val
            assert!(self.call_hook(0x8d4b));
            //                                                                                                  bc: cnt
            //                                                                                              EFFECTS
            //                                                                                                 *c256, *c25a
            //         ram:8d32 d2  2c  8d       JP         NC,LAB_ram_8d2c
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                break;
            } else {
                // JP(LAB_ram_8d2c);
            }
        }
        self.assert_pc(0x8d35);
        // self.SetPC(0x8d35);
        //         ram:8d35 21  1a  cc       LD         HL,DAT_ram_cc1a
        self.instr_hk__LD_HL_NNNN(0xcc1a);
        //         ram:8d38 22  56  c2       LD         (BYTE_ram_c256 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc256);
        //         ram:8d3b 2a  58  c2       LD         HL,(BYTE_ram_c258 )
        self.instr_hk__LD_HL_iNNNN(0xc258);
        //         ram:8d3e 22  5a  c2       LD         (WORD_ram_c25a ),HL
        self.instr_hk__LD_iNNNN_HL(0xc25a);
        self.assert_pc(0x8d41);
        //                              LAB_ram_8d41                                    XREF[1]:     ram:8d47 (j)
        loop {
            self.SetPC(0x8d41);
            //         ram:8d41 cd  6a  8d       CALL       sb_mem_8d6a                                      undefined sb_mem_8d6a()
            assert!(self.call_hook(0x8d6a));
            //         ram:8d44 cd  4f  8d       CALL       sb_mem_8d4f                                      IN a: val
            assert!(self.call_hook(0x8d4f));
            //                                                                                                  bc: cnt
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                break;
                // } else {
                //     // JP(LAB_ram_8d41);
            } //                                                                                              EFFECTS
        }
        //                                                                                                 *c256, *c25a
        self.assert_pc(0x8d4a);
        //
        true
    }
    pub(crate) fn hook_8d4b(&mut self) -> bool {
        println!("hook_8d4b");
        //         ram:8d4b 87              ADD        A,A                                              IN a: val
        self.instr_hk__ADD_A_A();
        //         ram:8d4c 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8d4d 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:8d4e 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        self.assert_pc(0x8d4f);
        self.internal_8d4f();
        //
        true
    }
    pub(crate) fn internal_8d4f(&mut self) {
        //         ram:8d4f 2a  56  c2       LD         HL,(BYTE_ram_c256 )                              IN a: val
        self.instr_hk__LD_HL_iNNNN(0xc256);
        //         ram:8d52 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //                              LAB_ram_8d53                                    XREF[1]:     ram:8d58 (j)
        loop {
            self.SetPC(0x8d53);
            //         ram:8d53 4f              LD         C,A
            self.instr_hk__LD_C_A();
            //         ram:8d54 b6              OR         (HL)
            self.instr_hk__OR_A_iHL();
            //         ram:8d55 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:8d56 79              LD         A,C
            self.instr_hk__LD_A_C();
            //         ram:8d57 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8d58 10  f9           DJNZ       LAB_ram_8d53
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_8d53;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:8d5a 22  56  c2       LD         (BYTE_ram_c256 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc256);
        //         ram:8d5d c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:8d5e 2a  5a  c2       LD         HL,(WORD_ram_c25a )
        self.instr_hk__LD_HL_iNNNN(0xc25a);
        //         ram:8d61 48              LD         C,B
        self.instr_hk__LD_C_B();
        //         ram:8d62 af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:8d63 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:8d64 ed  42           SBC        HL,BC
        self.instr_hk__SBC_HL_BC();
        //         ram:8d66 22  5a  c2       LD         (WORD_ram_c25a ),HL
        self.instr_hk__LD_iNNNN_HL(0xc25a);
        //         ram:8d69 c9              RET
        self.assert_pc(0x8d69);
    }
    pub(crate) fn hook_8d4f(&mut self) -> bool {
        println!("hook_8d4f");
        self.internal_8d4f();
        true
    }
    pub(crate) fn hook_8d6a(&mut self) -> bool {
        println!("hook_8d6a");
        //         ram:8d6a 2a  54  c2       LD         HL,(pt_char_c254 )
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:8d6d 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8d6e 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:8d6f 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:8d70 e6  f0           AND        0xf0
        self.instr_hk__AND_NN(0xf0);
        //         ram:8d72 28  10           JR         Z,LAB_ram_8d84
        self.IncPC(2);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_8d84);
            //                              LAB_ram_8d84                                    XREF[1]:     ram:8d72 (j)
            //         ram:8d84 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:8d85 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:8d86 18  f4           JR         LAB_ram_8d7c
            self.IncPC(2);
            self.increase_cycles(12);
            // JR(LAB_ram_8d7c);
        } else {
            self.increase_cycles(7);
            //         ram:8d74 cb  3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:8d76 cb  3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:8d78 cb  3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:8d7a cb  3f           SRL        A
            self.instr_hk__SRL_A();
        }
        self.SetPC(0x8d7c);
        //                              LAB_ram_8d7c                                    XREF[1]:     ram:8d86 (j)
        //         ram:8d7c 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:8d7d 79              LD         A,C
        self.instr_hk__LD_A_C();
        //         ram:8d7e e6  0f           AND        0xf
        self.instr_hk__AND_NN(0xf);
        //         ram:8d80 22  54  c2       LD         (pt_char_c254 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc254);
        //         ram:8d83 c9              RET
        self.assert_pc(0x8d83);
        // return true;
        //
        true
    }
    pub(crate) fn hook_ae82(&mut self) -> bool {
        println!("hook_ae82");
        //         ram:ae82 cd  0e  b6       CALL       sb_rand_guess_B60E                               OUT hl, a
        assert!(self.call_hook(0xb60e));
        //         ram:ae85 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:ae86 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:ae87 20  07           JR         NZ,LAB_ram_ae90
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_ae90);
            self.SetPC(0xae90);
            //                              LAB_ram_ae90                                    XREF[1]:     ram:ae87 (j)
            //         ram:ae90 06  03           LD         B,0x3
            self.instr_hk__LD_B_NN(0x3);
            //         ram:ae92 7c              LD         A,H
            self.instr_hk__LD_A_H();
            //                              LAB_ram_ae93                                    XREF[1]:     ram:ae97 (j)
            //         ram:ae93 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:ae94 38  05           JR         C,LAB_ram_ae9b
            self.IncPC(2);
            if (self.data.F & FLAG_C) != 0 {
                self.increase_cycles(12);
                self.SetPC(0xae9b);
                // JR(LAB_ram_ae9b);
                //                              LAB_ram_ae9b                                    XREF[1]:     ram:ae94 (j)
                //         ram:ae9b 87              ADD        A,A
                self.instr_hk__ADD_A_A();
                //         ram:ae9c 30  f9           JR         NC,LAB_ram_ae97
                self.IncPC(2);
                if (self.data.F & FLAG_C) == 0 {
                    self.increase_cycles(12);
                    self.SetPC(0xae97);
                    // JR(LAB_ram_ae97);
                    //                              LAB_ram_ae97                                    XREF[1]:     ram:ae9c (j)
                    //         ram:ae97 10  fa           DJNZ       LAB_ram_ae93
                    self.IncPC(2);
                    self.decB();
                    if self.data.B != 0 {
                        self.increase_cycles(13);
                        //JP LAB_ram_ae93;
                    } else {
                        self.increase_cycles(8);
                        // break;
                        //JP ae99;
                    }

                    //         ram:ae99 18  03           JR         LAB_ram_ae9e
                    self.IncPC(2);
                    self.increase_cycles(12);
                // JR(LAB_ram_ae9e);
                } else {
                    self.increase_cycles(7);
                    // JR(LAB_ram_ae9e);
                }
            } else {
                self.increase_cycles(7);
                self.SetPC(0xae96);
                //         ram:ae96 87              ADD        A,A
                self.instr_hk__ADD_A_A();
                //                              LAB_ram_ae97                                    XREF[1]:     ram:ae9c (j)
                //         ram:ae97 10  fa           DJNZ       LAB_ram_ae93
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP LAB_ram_ae93;
                } else {
                    self.increase_cycles(8);
                    // break;
                    //JP ae99;
                }

                //         ram:ae99 18  03           JR         LAB_ram_ae9e
                self.IncPC(2);
                self.increase_cycles(12);
                // JR(LAB_ram_ae9e);
            }
            self.SetPC(0xae9e);

            //                              LAB_ram_ae9e                                    XREF[1]:     ram:ae99 (j)
            //         ram:ae9e c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:ae9f 3a  85  c3       LD         A,(BYTE_ram_c385 )
            self.instr_hk__LD_A_iNNNN(0xc385);
            //         ram:aea2 47              LD         B,A
            self.instr_hk__LD_B_A();
            //         ram:aea3 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:aea4 80              ADD        A,B
            self.instr_hk__ADD_A_B();
            //         ram:aea5 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:aea6 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:aea7 90              SUB        B
            self.instr_hk__SUB_A_B();
            //         ram:aea8 30  01           JR         NC,LAB_ram_aeab
            self.IncPC(2);
            if (self.data.F & FLAG_C) == 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_aeab);
            } else {
                self.increase_cycles(7);
                //         ram:aeaa af              XOR        A
                self.instr_hk__XOR_A_A();
            }
            self.SetPC(0xaeab);

            //                              LAB_ram_aeab                                    XREF[1]:     ram:aea8 (j)
            //         ram:aeab fe  22           CP         0x22
            self.instr_hk__CP_NN(0x22);
            //         ram:aead 38  02           JR         C,LAB_ram_aeb1
            self.IncPC(2);
            if (self.data.F & FLAG_C) != 0 {
                self.increase_cycles(12);
                // JR(LAB_ram_aeb1);
            } else {
                self.increase_cycles(7);
                //         ram:aeaf 3e  21           LD         A,0x21
                self.instr_hk__LD_A_NN(0x21);
            }
        } else {
            self.SetPC(0xae89);
            self.increase_cycles(7);
            //         ram:ae89 7d              LD         A,L
            self.instr_hk__LD_A_L();
            //         ram:ae8a e6  0f           AND        0xf
            self.instr_hk__AND_NN(0xf);
            //         ram:ae8c c6  05           ADD        A,0x5
            self.instr_hk__ADD_A_NN(0x5);
            //         ram:ae8e 18  21           JR         LAB_ram_aeb1
            self.IncPC(2);
            self.increase_cycles(12);
            // JR(LAB_ram_aeb1);
        }
        self.SetPC(0xaeb1);

        //                              LAB_ram_aeb1                                    XREF[2]:     ram:ae8e (j) , ram:aead (j)
        //         ram:aeb1 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:aeb2 3a  3e  c2       LD         A,(DAT_ram_c23e )
        self.instr_hk__LD_A_iNNNN(0xc23e);
        //         ram:aeb5 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:aeb6 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:aeb7 20  04           JR         NZ,LAB_ram_aebd
        self.IncPC(2);
        if (self.data.F & FLAG_Z) == 0 {
            self.increase_cycles(12);
        } else {
            self.increase_cycles(7);
            //         ram:aeb9 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:aeba 28  01           JR         Z,LAB_ram_aebd
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(12);
            } else {
                self.increase_cycles(7);
                //         ram:aebc 3d              DEC        A
                self.instr_hk__DEC_A();
            }
        }
        // JR(LAB_ram_aebd);
        self.SetPC(0xaebd);

        //                              LAB_ram_aebd                                    XREF[2]:     ram:aeb7 (j) , ram:aeba (j)
        //         ram:aebd 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:aebe cd  b0  47       CALL       sb_read_mem_47b0                                 OUT hl <- c8da + 4 * (*c8b7) if
        assert!(self.call_hook(0x47b0));
        //                                                                                                  hl <- c8fa + 4 * (*c8b7) oth
        //         ram:aec1 08              EX         AF,AF_
        self.instr_hk__EX_AF_AF_();
        //         ram:aec2 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:aec3 c9              RET
        self.assert_pc(0xaec3);
        true
    }
    pub(crate) fn hook_aec4(&mut self) -> bool {
        //         ram:aec4 21 9a c9        LD         HL,DAT_ram_c99a
        self.instr_hk__LD_HL_NNNN(0xc99a);

        //         ram:aec7 11 9b c9        LD         DE,DAT_ram_c99b
        self.instr_hk__LD_DE_NNNN(0xc99b);
        self.assert_pc(0xaeca);
        //         ram:aeca 01 7f 02        LD         BC,0x27f
        self.instr_hk__LD_BC_NNNN(0x27f);
        //         ram:aecd 77              LD         (HL=>DAT_ram_c99a),A
        self.assert_pc(0xaecd);
        self.instr_hk__LD_iHL_A();
        //         ram:aece ed b0           LDIR
        self.assert_pc(0xaece);
        self.instr_hk__LDIR();
        //         ram:aed0 c9              RET
        self.assert_pc(0xaed0);

        true
    }
    pub(crate) fn hook_aef5(&mut self) -> bool {
        //         ram:aef5 af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:aef6 cd c4 ae        CALL       sb_mem_fill_xxx_AEC4
        assert!(self.call_hook(0xaec4));

        //         ram:aef9 cd 24 88        CALL       FUN_ram_8824                                     ()
        assert!(self.call_hook(0x8824));
        //         ram:aefc 3a 87 c3        LD         A,(BYTE_ram_c387)
        self.instr_hk__LD_A_iNNNN(0xc387);
        //         ram:aeff b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:af00 ca 25 af        JP         Z,LAB_ram_af25
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP LAB_ram_af25;
            //                              LAB_ram_af25
            //         ram:af25 21 fa d5        LD         HL,font_sprite_d5fa
            self.instr_hk__LD_HL_NNNN(0xd5fa);
            //         ram:af28 11 08 00        LD         DE,0x8
            self.instr_hk__LD_DE_NNNN(0x8);
            //         ram:af2b 01 14 00        LD         BC,0x14
            self.instr_hk__LD_BC_NNNN(0x14);
            //         ram:af2e cd 64 89        CALL       sb_blit_ram_to_vram_guess_8964                   IN
            assert!(self.call_hook(0x8964));

            //         ram:af31 21 9a d5        LD         HL,font_sprite_d58a
            self.instr_hk__LD_HL_NNNN(0xd58a);
            //         ram:af34 11 c8 00        LD         DE,0xc8
            self.instr_hk__LD_DE_NNNN(0xc8);
            //         ram:af37 01 04 00        LD         BC,0x4
            self.instr_hk__LD_BC_NNNN(0x4);
            //         ram:af3a cd 64 89        CALL       sb_blit_ram_to_vram_guess_8964                   IN
            assert!(self.call_hook(0x8964));

            //         ram:af3d 21 9a d4        LD         HL,font_sprite_d49a
            self.instr_hk__LD_HL_NNNN(0xd49a);
            //         ram:af40 11 e8 00        LD         DE,0xe8
            self.instr_hk__LD_DE_NNNN(0xe8);
            //         ram:af43 01 04 00        LD         BC,0x4
            self.instr_hk__LD_BC_NNNN(0x4);
            //         ram:af46 cd 64 89        CALL       sb_blit_ram_to_vram_guess_8964                   IN
            assert!(self.call_hook(0x8964));

            //         ram:af49 21 1a d5        LD         HL,font_sprite_d51a
            self.instr_hk__LD_HL_NNNN(0xd51a);
            //         ram:af4c 11 08 01        LD         DE,0x108
            self.instr_hk__LD_DE_NNNN(0x108);
            //         ram:af4f 01 04 00        LD         BC,0x4
            self.instr_hk__LD_BC_NNNN(0x4);
            //         ram:af52 cd 64 89        CALL       sb_blit_ram_to_vram_guess_8964                   IN
            assert!(self.call_hook(0x8964));
            // jp LAB_ram_af55
        } else {
            //         ram:af03 3a 86 c3        LD         A,(BYTE_ram_c386)
            self.instr_hk__LD_A_iNNNN(0xc386);
            //         ram:af06 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:af07 c2 55 af        JP         NZ,LAB_ram_af55
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP LAB_ram_af55;
            } else {
                //         ram:af0a 21 1a d4        LD         HL,font_sprite_d41a
                self.instr_hk__LD_HL_NNNN(0xd41a);
                //         ram:af0d 11 08 00        LD         DE,0x8
                self.instr_hk__LD_DE_NNNN(0x8);
                //         ram:af10 01 22 00        LD         BC,0x22
                self.instr_hk__LD_BC_NNNN(0x22);
                //         ram:af13 cd 64 89        CALL       sb_blit_ram_to_vram_guess_8964                   IN
                assert!(self.call_hook(0x8964));

                //         ram:af16 21 3a d7        LD         HL,font_sprite_d73a
                self.instr_hk__LD_HL_NNNN(0xd73a);
                //         ram:af19 11 18 01        LD         DE,0x118
                self.instr_hk__LD_DE_NNNN(0x118);
                //         ram:af1c 01 04 00        LD         BC,0x4
                self.instr_hk__LD_BC_NNNN(0x4);
                //         ram:af1f cd 64 89        CALL       sb_blit_ram_to_vram_guess_8964                   IN
                assert!(self.call_hook(0x8964));

                //         ram:af22 c3 55 af        JP         LAB_ram_af55
                self.IncPC(3);
                self.increase_cycles(10);
                // JP LAB_ram_af55;
            }
        }

        //                              LAB_ram_af55
        //         ram:af55 3a 87 c3        LD         A,(BYTE_ram_c387)
        self.instr_hk__LD_A_iNNNN(0xc387);
        //         ram:af58 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:af59 c2 80 af        JP         NZ,LAB_ram_af80
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) == 0 {
            // JP LAB_ram_af80;
            //                              LAB_ram_af80
            //         ram:af80 cd d1 ae        CALL       FUN_ram_aed1                                     ()
            assert!(self.call_hook(0xaed1));
            //         ram:af83 cd 24 88        CALL       FUN_ram_8824                                     ()
            assert!(self.call_hook(0x8824));
            //         ram:af86 11 08 08        LD         DE,0x808
            self.instr_hk__LD_DE_NNNN(0x808);
            //         ram:af89 01 04 10        LD         BC,0x1004
            self.instr_hk__LD_BC_NNNN(0x1004);
            //         ram:af8c cd 17 4c        CALL       fn_draw_border_guess_4c17
            assert!(self.call_hook(0x4c17));

            //         ram:af8f 11 08 0c        LD         DE,0xc08
            self.instr_hk__LD_DE_NNNN(0xc08);
            //         ram:af92 21 07 b0        LD         HL,s_DUNGEON_ram_b007                            = "DUNGEON"
            self.instr_hk__LD_HL_NNNN(0xb007);
            //         ram:af95 cd c7 89        CALL       fn_print_xy_89c7
            assert!(self.call_hook(0x89c7));

            //         ram:af98 11 09 0c        LD         DE,0xc09
            self.instr_hk__LD_DE_NNNN(0xc09);
            //         ram:af9b 21 ce 56        LD         HL,s_LEVEL_ram_56ce                              = "LEVEL"
            self.instr_hk__LD_HL_NNNN(0x56ce);
            //         ram:af9e cd c7 89        CALL       fn_print_xy_89c7
            assert!(self.call_hook(0x89c7));

            //         ram:afa1 14              INC        D
            self.instr_hk__INC_D();
            //         ram:afa2 2a 8a c3        LD         HL,(wd_addr_c38a)
            self.instr_hk__LD_HL_iNNNN(0xc38a);
            //         ram:afa5 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:afa6 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:afa7 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:afa8 fe 0a           CP         0xa
            self.instr_hk__CP_NN(0xa);
            //         ram:afaa da c4 af        JP         C,LAB_ram_afc4
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                // JP LAB_ram_afc4;
                //                              LAB_ram_afc4
                //         ram:afc4 c6 30           ADD        A,'0'
                self.instr_hk__ADD_A_NN('0' as u32 as u8);
                //         ram:afc6 cd d6 89        CALL       fn_putchar_xy_89d6
                assert!(self.call_hook(0x89d6));

                // JP LAB_ram_afc9;
            } else {
                //         ram:afad d5              PUSH       DE
                self.instr_hk__PUSH_DE();
                //         ram:afae 06 0a           LD         B,0xa
                self.instr_hk__LD_B_NN(0xa);
                //         ram:afb0 cd 95 b6        CALL       sb_calc_B695
                assert!(self.call_hook(0xb695));

                //         ram:afb3 d1              POP        DE
                self.instr_hk__POP_DE();
                //         ram:afb4 c5              PUSH       BC
                self.instr_hk__PUSH_BC();
                //         ram:afb5 c6 30           ADD        A,'0'
                self.instr_hk__ADD_A_NN('0' as u32 as u8);
                //         ram:afb7 cd d6 89        CALL       fn_putchar_xy_89d6
                assert!(self.call_hook(0x89d6));

                //         ram:afba c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:afbb 78              LD         A,B
                self.instr_hk__LD_A_B();
                //         ram:afbc c6 30           ADD        A,'0'
                self.instr_hk__ADD_A_NN('0' as u32 as u8);
                //         ram:afbe cd d6 89        CALL       fn_putchar_xy_89d6
                assert!(self.call_hook(0x89d6));

                //         ram:afc1 c3 c9 af        JP         LAB_ram_afc9
                self.IncPC(3);
                self.increase_cycles(10);
                // JP LAB_ram_afc9;
            }

            //                              LAB_ram_afc9
            //         ram:afc9 11 0a 09        LD         DE,0x90a
            self.instr_hk__LD_DE_NNNN(0x90a);
            //         ram:afcc 21 1d b0        LD         HL,s_FLOOR_OF_ram_b01d                           = "FLOOR OF  "
            self.instr_hk__LD_HL_NNNN(0xb01d);
            //         ram:afcf cd c7 89        CALL       fn_print_xy_89c7
            assert!(self.call_hook(0x89c7));

            //         ram:afd2 2a 8a c3        LD         HL,(wd_addr_c38a)
            self.instr_hk__LD_HL_iNNNN(0xc38a);
            //         ram:afd5 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:afd6 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:afd7 f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:afd8 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:afda cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:afdc cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:afde cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:afe0 c6 4b           ADD        A,'K'
            self.instr_hk__ADD_A_NN('K' as u32 as u8);
            //         ram:afe2 cd d6 89        CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //         ram:afe5 f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:afe6 e6 0f           AND        0xf
            self.instr_hk__AND_NN(0xf);
            //         ram:afe8 c6 41           ADD        A,'A'
            self.instr_hk__ADD_A_NN('A' as u32 as u8);
            //         ram:afea cd d6 89        CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //         ram:afed e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:afee 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:afef 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:aff0 f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:aff1 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:aff3 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:aff5 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:aff7 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:aff9 c6 4b           ADD        A,'K'
            self.instr_hk__ADD_A_NN('K' as u32 as u8);
            //         ram:affb cd d6 89        CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //         ram:affe f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:afff e6 0f           AND        0xf
            self.instr_hk__AND_NN(0xf);
            //         ram:b001 c6 41           ADD        A,'A'
            self.instr_hk__ADD_A_NN('A' as u32 as u8);
            //         ram:b003 cd d6 89        CALL       fn_putchar_xy_89d6
            assert!(self.call_hook(0x89d6));

            //         ram:b006 c9              RET
            self.assert_pc(0xb006);

            true
        } else {
            self.SetPC(0xaf5c);
            //         ram:af5c 3e 09           LD         A,0x9
            self.instr_hk__LD_A_NN(0x9);
            //         ram:af5e cd c4 ae        CALL       sb_mem_fill_xxx_AEC4
            assert!(self.call_hook(0xaec4));

            //         ram:af61 cd 24 88        CALL       FUN_ram_8824                                     ()
            assert!(self.call_hook(0x8824));
            //         ram:af64 11 08 07        LD         DE,0x708
            self.instr_hk__LD_DE_NNNN(0x708);
            //         ram:af67 01 05 11        LD         BC,0x1105
            self.instr_hk__LD_BC_NNNN(0x1105);
            //         ram:af6a cd 17 4c        CALL       fn_draw_border_guess_4c17
            assert!(self.call_hook(0x4c17));

            //         ram:af6d 11 09 08        LD         DE,0x809
            self.instr_hk__LD_DE_NNNN(0x809);
            //         ram:af70 01 03 0f        LD         BC,0xf03
            self.instr_hk__LD_BC_NNNN(0xf03);
            //         ram:af73 cd 17 4c        CALL       fn_draw_border_guess_4c17
            assert!(self.call_hook(0x4c17));

            //         ram:af76 21 0f b0        LD         HL,s_CITY_OF_GHOST_ram_b00f                      = "CITY OF GHOST"
            self.instr_hk__LD_HL_NNNN(0xb00f);
            //         ram:af79 11 0a 09        LD         DE,0x90a
            self.instr_hk__LD_DE_NNNN(0x90a);
            //         ram:af7c cd c7 89        CALL       fn_print_xy_89c7
            assert!(self.call_hook(0x89c7));

            //         ram:af7f c9              RET
            self.assert_pc(0xaf7f);
            true
        }
    }
    pub(crate) fn hook_b181(&mut self) -> bool {
        //         ram:b181 06 14           LD         B,0x14
        self.instr_hk__LD_B_NN(0x14);
        //                              loop
        loop {
            self.SetPC(0xb183);
            //         ram:b183 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:b184 cd 4c b6        CALL       FUN_ram_b64c                                     ()
            assert!(self.call_hook(0xb64c));
            //         ram:b187 7d              LD         A,L
            self.instr_hk__LD_A_L();
            //         ram:b188 e6 03           AND        0x3
            self.instr_hk__AND_NN(0x3);
            //         ram:b18a cc 91 b1        CALL       Z,FUN_ram_b191                                   ()
            assert!(self.call_hook(0xb191));
            //         ram:b18d c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:b18e 10 f3           DJNZ       loop
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:b190 c9              RET
        self.assert_pc(0xb190);

        true
    }
    pub(crate) fn hook_b191(&mut self) -> bool {
        //         ram:b191 cd 4c b6        CALL       FUN_ram_b64c                                     ()
        assert!(self.call_hook(0xb64c));
        //         ram:b194 11 0e 00        LD         DE,0xe
        self.instr_hk__LD_DE_NNNN(0xe);
        //         ram:b197 cd ac b6        CALL       sb_calc_b6ac
        assert!(self.call_hook(0xb6ac));

        //         ram:b19a 7b              LD         A,E
        self.instr_hk__LD_A_E();
        //         ram:b19b 32 b0 c8        LD         (BYTE_ram_c8b0),A
        self.instr_hk__LD_iNNNN_A(0xc8b0);
        //         ram:b19e cd 4c b6        CALL       FUN_ram_b64c                                     ()
        assert!(self.call_hook(0xb64c));
        //         ram:b1a1 11 0e 00        LD         DE,0xe
        self.instr_hk__LD_DE_NNNN(0xe);
        //         ram:b1a4 cd ac b6        CALL       sb_calc_b6ac
        assert!(self.call_hook(0xb6ac));

        //         ram:b1a7 7b              LD         A,E
        self.instr_hk__LD_A_E();
        //         ram:b1a8 32 b1 c8        LD         (BYTE_ram_c8b1),A
        self.instr_hk__LD_iNNNN_A(0xc8b1);
        //         ram:b1ab cd 4c b6        CALL       FUN_ram_b64c                                     ()
        assert!(self.call_hook(0xb64c));
        //         ram:b1ae 3a b0 c8        LD         A,(BYTE_ram_c8b0)
        self.instr_hk__LD_A_iNNNN(0xc8b0);
        //         ram:b1b1 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:b1b2 3e 0e           LD         A,0xe
        self.instr_hk__LD_A_NN(0xe);
        //         ram:b1b4 90              SUB        B
        self.instr_hk__SUB_A_B();
        //         ram:b1b5 fe 03           CP         0x3
        self.instr_hk__CP_NN(0x3);
        //         ram:b1b7 da bc b1        JP         C,LAB_ram_b1bc
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) != 0 {
            // JP LAB_ram_b1bc;
        } else {
            //         ram:b1ba 3e 03           LD         A,0x3
            self.instr_hk__LD_A_NN(0x3);
        }

        //                              LAB_ram_b1bc
        //         ram:b1bc 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:b1bd 16 00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:b1bf cd ac b6        CALL       sb_calc_b6ac
        assert!(self.call_hook(0xb6ac));

        //         ram:b1c2 7b              LD         A,E
        self.instr_hk__LD_A_E();
        //         ram:b1c3 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:b1c4 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:b1c5 32 b2 c8        LD         (BYTE_ram_c8b2),A
        self.instr_hk__LD_iNNNN_A(0xc8b2);
        //         ram:b1c8 cd 4c b6        CALL       FUN_ram_b64c                                     ()
        assert!(self.call_hook(0xb64c));
        //         ram:b1cb 3a b1 c8        LD         A,(BYTE_ram_c8b1)
        self.instr_hk__LD_A_iNNNN(0xc8b1);
        //         ram:b1ce 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:b1cf 3e 0e           LD         A,0xe
        self.instr_hk__LD_A_NN(0xe);
        //         ram:b1d1 90              SUB        B
        self.instr_hk__SUB_A_B();
        //         ram:b1d2 fe 03           CP         0x3
        self.instr_hk__CP_NN(0x3);
        //         ram:b1d4 da d9 b1        JP         C,LAB_ram_b1d9
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) != 0 {
            // JP LAB_ram_b1d9;
        } else {
            //         ram:b1d7 3e 03           LD         A,0x3
            self.instr_hk__LD_A_NN(0x3);
        }

        //                              LAB_ram_b1d9
        //         ram:b1d9 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:b1da 16 00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:b1dc cd ac b6        CALL       sb_calc_b6ac
        assert!(self.call_hook(0xb6ac));

        //         ram:b1df 7b              LD         A,E
        self.instr_hk__LD_A_E();
        //         ram:b1e0 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:b1e1 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:b1e2 32 b3 c8        LD         (BYTE_ram_c8b3),A
        self.instr_hk__LD_iNNNN_A(0xc8b3);
        //         ram:b1e5 3a b1 c8        LD         A,(BYTE_ram_c8b1)
        self.instr_hk__LD_A_iNNNN(0xc8b1);
        //         ram:b1e8 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:b1e9 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:b1eb 11 40 00        LD         DE,0x40
        self.instr_hk__LD_DE_NNNN(0x40);
        //         ram:b1ee cd a9 b7        CALL       sb_multiply_guess_B7A9
        assert!(self.call_hook(0xb7a9));
        //         ram:b1f1 3a b0 c8        LD         A,(BYTE_ram_c8b0)
        self.instr_hk__LD_A_iNNNN(0xc8b0);
        //         ram:b1f4 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:b1f5 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //         ram:b1f6 16 00           LD         D,0x0
        self.instr_hk__LD_D_NN(0x0);
        //         ram:b1f8 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:b1f9 11 ac c3        LD         DE,DAT_ram_c3ac
        self.instr_hk__LD_DE_NNNN(0xc3ac);
        //         ram:b1fc 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:b1fd 11 20 00        LD         DE,0x20
        self.instr_hk__LD_DE_NNNN(0x20);
        //         ram:b200 3a b2 c8        LD         A,(BYTE_ram_c8b2)
        self.instr_hk__LD_A_iNNNN(0xc8b2);
        //         ram:b203 47              LD         B,A
        self.instr_hk__LD_B_A();
        //                              loop_1
        loop {
            //         ram:b204 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b205 7e              LD         A,(HL=>DAT_ram_c3d7)
            self.instr_hk__LD_A_iHL();
            //         ram:b206 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:b207 c2 0d b2        JP         NZ,loop_1_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop_1_chk_cond;
            } else {
                //         ram:b20a 3e 02           LD         A,0x2
                self.instr_hk__LD_A_NN(0x2);
                //         ram:b20c 77              LD         (HL=>DAT_ram_c3d7),A
                self.instr_hk__LD_iHL_A();
            }

            //                              loop_1_chk_cond
            //         ram:b20d 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b20e 10 f4           DJNZ       loop_1
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_1;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:b210 3a b3 c8        LD         A,(BYTE_ram_c8b3)
        self.instr_hk__LD_A_iNNNN(0xc8b3);
        //         ram:b213 47              LD         B,A
        self.instr_hk__LD_B_A();
        //                              loop_2
        loop {
            //         ram:b214 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:b215 7e              LD         A,(HL=>DAT_ram_c3f8)
            self.instr_hk__LD_A_iHL();
            //         ram:b216 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:b217 c2 1d b2        JP         NZ,loop_2_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop_2_chk_cond;
            } else {
                //         ram:b21a 3e 03           LD         A,0x3
                self.instr_hk__LD_A_NN(0x3);
                //         ram:b21c 77              LD         (HL=>DAT_ram_c3f8),A
                self.instr_hk__LD_iHL_A();
            }

            //                              loop_2_chk_cond
            //         ram:b21d 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:b21e 10 f4           DJNZ       loop_2
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_2;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:b220 3a b2 c8        LD         A,(BYTE_ram_c8b2)
        self.instr_hk__LD_A_iNNNN(0xc8b2);
        //         ram:b223 47              LD         B,A
        self.instr_hk__LD_B_A();
        //                              loop_3
        loop {
            //         ram:b224 2b              DEC        HL
            self.instr_hk__DEC_HL();
            //         ram:b225 7e              LD         A,(HL=>DAT_ram_c417)
            self.instr_hk__LD_A_iHL();
            //         ram:b226 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:b227 c2 2d b2        JP         NZ,loop_3_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop_3_chk_cond;
            } else {
                //         ram:b22a 3e 02           LD         A,0x2
                self.instr_hk__LD_A_NN(0x2);
                //         ram:b22c 77              LD         (HL=>DAT_ram_c417),A
                self.instr_hk__LD_iHL_A();
            }

            //                              loop_3_chk_cond
            //         ram:b22d 2b              DEC        HL
            self.instr_hk__DEC_HL();
            //         ram:b22e 10 f4           DJNZ       loop_3
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_3;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:b230 3a b3 c8        LD         A,(BYTE_ram_c8b3)
        self.instr_hk__LD_A_iNNNN(0xc8b3);
        //         ram:b233 47              LD         B,A
        self.instr_hk__LD_B_A();
        //                              loop_4
        loop {
            //         ram:b234 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:b235 ed 52           SBC        HL,DE
            self.instr_hk__SBC_HL_DE();
            //         ram:b237 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:b238 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:b239 c2 3f b2        JP         NZ,loop_4_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop_4_chk_cond;
            } else {
                //         ram:b23c 3e 03           LD         A,0x3
                self.instr_hk__LD_A_NN(0x3);
                //         ram:b23e 77              LD         (HL),A
                self.instr_hk__LD_iHL_A();
            }

            //                              loop_4_chk_cond
            //         ram:b23f b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:b240 ed 52           SBC        HL,DE
            self.instr_hk__SBC_HL_DE();
            //         ram:b242 10 f0           DJNZ       loop_4
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP loop_4;
            } else {
                self.increase_cycles(8);
                break;
            }
        }

        //         ram:b244 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b245 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:b246 3a b3 c8        LD         A,(BYTE_ram_c8b3)
        self.instr_hk__LD_A_iNNNN(0xc8b3);
        //         ram:b249 87              ADD        A,A
        self.instr_hk__ADD_A_A();
        //         ram:b24a 3d              DEC        A
        self.instr_hk__DEC_A();
        //         ram:b24b 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //                              loop_5
        loop {
            self.SetPC(0xb24c);
            //         ram:b24c 3a b2 c8        LD         A,(BYTE_ram_c8b2)
            self.instr_hk__LD_A_iNNNN(0xc8b2);
            //         ram:b24f 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:b250 3d              DEC        A
            self.instr_hk__DEC_A();
            //         ram:b251 47              LD         B,A
            self.instr_hk__LD_B_A();
            //         ram:b252 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b253 3e 04           LD         A,0x4
            self.instr_hk__LD_A_NN(0x4);
            //                              loop_5_a
            loop {
                self.SetPC(0xb255);
                //         ram:b255 77              LD         (HL),A
                self.instr_hk__LD_iHL_A();
                //         ram:b256 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b257 10 fc           DJNZ       loop_5_a
                self.IncPC(2);
                self.decB();
                if self.data.B != 0 {
                    self.increase_cycles(13);
                    //JP loop_5_a;
                } else {
                    self.increase_cycles(8);
                    break;
                }
            }

            //         ram:b259 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:b25a 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:b25b 0d              DEC        C
            self.instr_hk__DEC_C();
            //         ram:b25c c2 4c b2        JP         NZ,loop_5
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop_5;
            } else {
                break;
            }
        }

        //         ram:b25f c9              RET

        self.assert_pc(0xb25f);

        true
    }
    pub(crate) fn hook_b260(&mut self) -> bool {
        //         ram:b260 01 e0 00        LD         BC,0xe0
        self.instr_hk__LD_BC_NNNN(0xe0);
        //                              loop_1
        loop {
            self.SetPC(0xb263);
            //         ram:b263 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //                              loop_1_a
            loop {
                self.SetPC(0xb264);
                //         ram:b264 0e 00           LD         C,0x0
                self.instr_hk__LD_C_NN(0x0);
                //         ram:b266 2a ac c7        LD         HL,(DAT_ram_c7ac)
                self.instr_hk__LD_HL_iNNNN(0xc7ac);
                //         ram:b269 2b              DEC        HL
                self.instr_hk__DEC_HL();
                //         ram:b26a 2b              DEC        HL
                self.instr_hk__DEC_HL();
                //         ram:b26b 7e              LD         A,(HL=>DAT_ram_c3cb)
                self.instr_hk__LD_A_iHL();
                //         ram:b26c 3d              DEC        A
                self.instr_hk__DEC_A();
                //         ram:b26d c2 71 b2        JP         NZ,LAB_ram_b271
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP LAB_ram_b271;
                } else {
                    //         ram:b270 0c              INC        C
                    self.instr_hk__INC_C();
                }

                //                              LAB_ram_b271
                //         ram:b271 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b272 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b273 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b274 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b275 7e              LD         A,(HL)
                self.instr_hk__LD_A_iHL();
                //         ram:b276 3d              DEC        A
                self.instr_hk__DEC_A();
                //         ram:b277 c2 7b b2        JP         NZ,LAB_ram_b27b
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP LAB_ram_b27b;
                } else {
                    //         ram:b27a 0c              INC        C
                    self.instr_hk__INC_C();
                }

                //                              LAB_ram_b27b
                //         ram:b27b 11 40 00        LD         DE,0x40
                self.instr_hk__LD_DE_NNNN(0x40);
                //         ram:b27e 2b              DEC        HL
                self.instr_hk__DEC_HL();
                //         ram:b27f 2b              DEC        HL
                self.instr_hk__DEC_HL();
                //         ram:b280 b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:b281 ed 52           SBC        HL,DE
                self.instr_hk__SBC_HL_DE();
                //         ram:b283 7e              LD         A,(HL)
                self.instr_hk__LD_A_iHL();
                //         ram:b284 3d              DEC        A
                self.instr_hk__DEC_A();
                //         ram:b285 c2 89 b2        JP         NZ,LAB_ram_b289
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP LAB_ram_b289;
                } else {
                    //         ram:b288 0c              INC        C
                    self.instr_hk__INC_C();
                }

                //                              LAB_ram_b289
                //         ram:b289 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:b28a 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:b28b 7e              LD         A,(HL)
                self.instr_hk__LD_A_iHL();
                //         ram:b28c 3d              DEC        A
                self.instr_hk__DEC_A();
                //         ram:b28d c2 91 b2        JP         NZ,LAB_ram_b291
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP LAB_ram_b291;
                } else {
                    //         ram:b290 0c              INC        C
                    self.instr_hk__INC_C();
                }

                //                              LAB_ram_b291
                //         ram:b291 0d              DEC        C
                self.instr_hk__DEC_C();
                //         ram:b292 ca f3 b2        JP         Z,LAB_ram_b2f3
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) != 0 {
                    // JP LAB_ram_b2f3;
                    break;
                } else {
                    //         ram:b295 fa a6 b2        JP         M,LAB_ram_b2a6
                    self.IncPC(3);
                    self.increase_cycles(10);
                    if (self.data.F & FLAG_S) != 0 {
                        // JP LAB_ram_b2a6;

                        //                              LAB_ram_b2a6
                        //         ram:b2a6 2a ae c7        LD         HL,(DAT_ram_c7ae)
                        self.instr_hk__LD_HL_iNNNN(0xc7ae);
                        //         ram:b2a9 7e              LD         A,(HL)
                        self.instr_hk__LD_A_iHL();
                        //         ram:b2aa 32 88 c3        LD         (BYTE_ram_c388),A
                        self.instr_hk__LD_iNNNN_A(0xc388);
                        //         ram:b2ad e6 0f           AND        0xf
                        self.instr_hk__AND_NN(0xf);
                        //         ram:b2af 21 00 00        LD         HL,0x0
                        self.instr_hk__LD_HL_NNNN(0x0);
                        //         ram:b2b2 11 40 00        LD         DE,0x40
                        self.instr_hk__LD_DE_NNNN(0x40);
                        //                              loop_1_a_1
                        loop {
                            //         ram:b2b5 ca bd b2        JP         Z,LAB_ram_b2bd
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_Z) != 0 {
                                // JP LAB_ram_b2bd;
                                break;
                            }

                            //         ram:b2b8 19              ADD        HL,DE
                            self.instr_hk__ADD_HL_DE();
                            //         ram:b2b9 3d              DEC        A
                            self.instr_hk__DEC_A();
                            //         ram:b2ba c3 b5 b2        JP         loop_1_a_1
                            self.IncPC(3);
                            self.increase_cycles(10);
                            // JP loop_1_a_1;
                        }
                        //                              LAB_ram_b2bd
                        //         ram:b2bd 3a 88 c3        LD         A,(BYTE_ram_c388)
                        self.instr_hk__LD_A_iNNNN(0xc388);
                        //         ram:b2c0 cb 3f           SRL        A
                        self.instr_hk__SRL_A();
                        //         ram:b2c2 cb 3f           SRL        A
                        self.instr_hk__SRL_A();
                        //         ram:b2c4 cb 3f           SRL        A
                        self.instr_hk__SRL_A();
                        //         ram:b2c6 cb 3f           SRL        A
                        self.instr_hk__SRL_A();
                        //         ram:b2c8 b7              OR         A
                        self.instr_hk__OR_A_A();
                        //                              loop_1_a_2
                        loop {
                            self.SetPC(0xb2c9);
                            //         ram:b2c9 ca d2 b2        JP         Z,loop_1_a_chk_cond
                            self.IncPC(3);
                            self.increase_cycles(10);
                            if (self.data.F & FLAG_Z) != 0 {
                                // JP loop_1_a_chk_cond;
                                break;
                            }

                            //         ram:b2cc 23              INC        HL
                            self.instr_hk__INC_HL();
                            //         ram:b2cd 23              INC        HL
                            self.instr_hk__INC_HL();
                            //         ram:b2ce 3d              DEC        A
                            self.instr_hk__DEC_A();
                            //         ram:b2cf c3 c9 b2        JP         loop_1_a_2
                            self.IncPC(3);
                            self.increase_cycles(10); //JP loop_1_a_2;
                        }

                        //                              loop_1_a_chk_cond
                        //         ram:b2d2 11 cd c3        LD         DE,DAT_ram_c3cd
                        self.instr_hk__LD_DE_NNNN(0xc3cd);
                        //         ram:b2d5 19              ADD        HL,DE
                        self.instr_hk__ADD_HL_DE();
                        //         ram:b2d6 22 ac c7        LD         (DAT_ram_c7ac),HL
                        self.instr_hk__LD_iNNNN_HL(0xc7ac);
                        //         ram:b2d9 2a ae c7        LD         HL,(DAT_ram_c7ae)
                        self.instr_hk__LD_HL_iNNNN(0xc7ae);
                        //         ram:b2dc 2b              DEC        HL
                        self.instr_hk__DEC_HL();
                        //         ram:b2dd 22 ae c7        LD         (DAT_ram_c7ae),HL
                        self.instr_hk__LD_iNNNN_HL(0xc7ae);
                        //         ram:b2e0 c3 64 b2        JP         loop_1_a
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP loop_1_a;
                    } else {
                        //         ram:b298 3a 88 c3        LD         A,(BYTE_ram_c388)
                        self.instr_hk__LD_A_iNNNN(0xc388);
                        //         ram:b29b 2a ae c7        LD         HL,(DAT_ram_c7ae)
                        self.instr_hk__LD_HL_iNNNN(0xc7ae);
                        //         ram:b29e 23              INC        HL
                        self.instr_hk__INC_HL();
                        //         ram:b29f 77              LD         (HL),A
                        self.instr_hk__LD_iHL_A();
                        //         ram:b2a0 22 ae c7        LD         (DAT_ram_c7ae),HL
                        self.instr_hk__LD_iNNNN_HL(0xc7ae);
                        //         ram:b2a3 c3 f3 b2        JP         LAB_ram_b2f3
                        self.IncPC(3);
                        self.increase_cycles(10);
                        // JP LAB_ram_b2f3;
                        break;
                    }
                }
            }

            //                              WORD_ram_b2e3                                        FUN_ram_b260:b308(*)
            //         ram:b2e3 e0 ff           dw         FFE0h
            //         ram:b2e5 ff 00           dw         FFh
            //         ram:b2e7 01 00           dw         1h
            //         ram:b2e9 00 01           dw         100h
            //         ram:b2eb 20 00           dw         20h
            //         ram:b2ed 01 00           dw         1h
            //         ram:b2ef ff ff           dw         FFFFh
            //         ram:b2f1 00 ff           dw         FF00h
            //                              LAB_ram_b2f3
            loop {
                self.SetPC(0xb2f3);
                //         ram:b2f3 cd 4c b6        CALL       FUN_ram_b64c                                     ()
                assert!(self.call_hook(0xb64c));
                //                              LAB_ram_b2f6
                //         ram:b2f6 11 04 00        LD         DE,0x4
                self.instr_hk__LD_DE_NNNN(0x4);
                //         ram:b2f9 cd ac b6        CALL       sb_calc_b6ac
                assert!(self.call_hook(0xb6ac));

                //         ram:b2fc 7b              LD         A,E
                self.instr_hk__LD_A_E();
                //         ram:b2fd 32 89 c3        LD         (BYTE_ram_c389),A
                self.instr_hk__LD_iNNNN_A(0xc389);
                //         ram:b300 3a 89 c3        LD         A,(BYTE_ram_c389)
                self.instr_hk__LD_A_iNNNN(0xc389);
                //         ram:b303 87              ADD        A,A
                self.instr_hk__ADD_A_A();
                //         ram:b304 87              ADD        A,A
                self.instr_hk__ADD_A_A();
                //         ram:b305 6f              LD         L,A
                self.instr_hk__LD_L_A();
                //         ram:b306 26 00           LD         H,0x0
                self.instr_hk__LD_H_NN(0x0);
                //         ram:b308 11 e3 b2        LD         DE,WORD_ram_b2e3                                 = FFE0h
                self.instr_hk__LD_DE_NNNN(0xb2e3);
                //         ram:b30b 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:b30c 5e              LD         E,(HL=>LAB_ram_b2f3)
                self.instr_hk__LD_E_iHL();
                //         ram:b30d 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b30e 56              LD         D,(HL=>LAB_ram_b2f3+1)
                self.instr_hk__LD_D_iHL();
                //         ram:b30f 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b310 4e              LD         C,(HL=>LAB_ram_b2f3+2)
                self.instr_hk__LD_C_iHL();
                //         ram:b311 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b312 46              LD         B,(HL=>LAB_ram_b2f6)
                self.instr_hk__LD_B_iHL();
                //         ram:b313 2a ac c7        LD         HL,(DAT_ram_c7ac)
                self.instr_hk__LD_HL_iNNNN(0xc7ac);
                //         ram:b316 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:b317 19              ADD        HL,DE
                self.instr_hk__ADD_HL_DE();
                //         ram:b318 7e              LD         A,(HL)
                self.instr_hk__LD_A_iHL();
                //         ram:b319 3d              DEC        A
                self.instr_hk__DEC_A();
                //         ram:b31a c2 f3 b2        JP         NZ,LAB_ram_b2f3
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) == 0 {
                    // JP LAB_ram_b2f3;
                } else {
                    break;
                }
            }

            self.SetPC(0xb31d);
            //         ram:b31d 2a ac c7        LD         HL,(DAT_ram_c7ac)
            self.instr_hk__LD_HL_iNNNN(0xc7ac);
            //         ram:b320 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:b321 36 00           LD         (HL),0x0
            self.instr_hk__LD_iHL_NN(0x0);
            //         ram:b323 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:b324 36 00           LD         (HL),0x0
            self.instr_hk__LD_iHL_NN(0x0);
            //         ram:b326 22 ac c7        LD         (DAT_ram_c7ac),HL
            self.instr_hk__LD_iNNNN_HL(0xc7ac);
            //         ram:b329 3a 88 c3        LD         A,(BYTE_ram_c388)
            self.instr_hk__LD_A_iNNNN(0xc388);
            //         ram:b32c e6 0f           AND        0xf
            self.instr_hk__AND_NN(0xf);
            //         ram:b32e 81              ADD        A,C
            self.instr_hk__ADD_A_C();
            //         ram:b32f 4f              LD         C,A
            self.instr_hk__LD_C_A();
            //         ram:b330 3a 88 c3        LD         A,(BYTE_ram_c388)
            self.instr_hk__LD_A_iNNNN(0xc388);
            //         ram:b333 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:b335 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:b337 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:b339 cb 3f           SRL        A
            self.instr_hk__SRL_A();
            //         ram:b33b 80              ADD        A,B
            self.instr_hk__ADD_A_B();
            //         ram:b33c 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:b33d 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:b33e 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:b33f 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:b340 b1              OR         C
            self.instr_hk__OR_A_C();
            //         ram:b341 32 88 c3        LD         (BYTE_ram_c388),A
            self.instr_hk__LD_iNNNN_A(0xc388);
            //         ram:b344 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:b345 0b              DEC        BC
            self.instr_hk__DEC_BC();
            //         ram:b346 78              LD         A,B
            self.instr_hk__LD_A_B();
            //         ram:b347 b1              OR         C
            self.instr_hk__OR_A_C();
            //         ram:b348 c2 63 b2        JP         NZ,loop_1
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop_1;
            } else {
                break;
            }
        }

        self.assert_pc(0xb34b);
        //         ram:b34b c9              RET

        true
    }
    pub(crate) fn hook_b34c(&mut self) -> bool {
        //         ram:b34c af              XOR        A                                                IN hl: addr
        self.instr_hk__XOR_A_A();
        //                                                                                                 bc: cnt/cnt
        //                                                                                              OUT hl
        //         ram:b34d 11 20 00        LD         DE,0x20
        self.instr_hk__LD_DE_NNNN(0x20);
        //                              loop_1                                          XREF[1]:     ram:b35a(j)
        loop {
            self.SetPC(0xb350);
            //         ram:b350 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:b351 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //                              loop_1_a                                        XREF[1]:     ram:b354(j)
            loop {
                self.SetPC(0xb352);
                //         ram:b352 b6              OR         (HL)
                self.instr_hk__OR_A_iHL();
                //         ram:b353 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b354 10 fc           DJNZ       loop_1_a
                self.IncPC(2);
                self.decB();
                if self.data.B == 0 {
                    self.increase_cycles(8);
                    break;
                }
                self.increase_cycles(13);
                //JP loop_1_a;
            }

            //         ram:b356 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:b357 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:b358 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:b359 0d              DEC        C
            self.instr_hk__DEC_C();
            //         ram:b35a 20 f4           JR         NZ,loop_1
            self.IncPC(2);
            if (self.data.F & FLAG_Z) != 0 {
                self.increase_cycles(7);
                break;
            }
            self.increase_cycles(12);
            // JR(loop_1);
        }

        //         ram:b35c c9              RET
        // self.assert_pc(0xb35c);
        // return true;
        //
        true
    }
    pub(crate) fn hook_b35d(&mut self) -> bool {
        println!("hook_b35d");
        //         ram:b35d ed 5b bc c8     LD         DE,(BYTE_ram_c8bc)
        self.instr_hk__LD_DE_iNNNN(0xc8bc);
        //         ram:b361 2a b8 c8        LD         HL,(BYTE_ram_c8b8)
        self.instr_hk__LD_HL_iNNNN(0xc8b8);
        //         ram:b364 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b365 73              LD         (HL),E
        self.instr_hk__LD_iHL_E();
        //         ram:b366 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b367 72              LD         (HL),D
        self.instr_hk__LD_iHL_D();
        //         ram:b368 2a ba c8        LD         HL,(wd_addr_c8ba)
        self.instr_hk__LD_HL_iNNNN(0xc8ba);
        //         ram:b36b 11 05 00        LD         DE,0x5_c8ba
        self.instr_hk__LD_DE_NNNN(0x5);
        //         ram:b36e 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:b36f 46              LD         B,(HL)
        self.instr_hk__LD_B_iHL();
        //         ram:b370 3e 04           LD         A,0x4
        self.instr_hk__LD_A_NN(0x4);
        //         ram:b372 cd 34 b6        CALL       sb_rand_calc_B634                                IN a:
        assert!(self.call_hook(0xb634));
        //                                                                                                 b: cnt
        //                                                                                              OUT hl
        //         ram:b375 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:b376 2a b8 c8        LD         HL,(BYTE_ram_c8b8)
        self.instr_hk__LD_HL_iNNNN(0xc8b8);
        //         ram:b379 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b37a 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b37b 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b37c 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:b37d cd 92 8a        CALL       sb_set_vmem_guess_8A92                           undefined sb_set_vmem_guess_8A92()
        assert!(self.call_hook(0x8a92));
        //         ram:b380 cd 6f 8a        CALL       sb_fill_vram_guess_8A6F                          undefined sb_fill_vram_guess_8A6
        assert!(self.call_hook(0x8a6f));
        //         ram:b383 cd f5 89        CALL       sb_check_mem_fill_vram_89F5                      undefined sb_check_mem_fill_vram
        assert!(self.call_hook(0x89f5));
        //         ram:b386 c9              RET
        self.assert_pc(0xb386);
        true
    }
    pub(crate) fn hook_b387(&mut self) -> bool {
        //         ram:b387 3a b7 c8        LD         A,(BYTE_ram_c8b7)                                hl <- (*c8b7) >> 1
        self.instr_hk__LD_A_iNNNN(0xc8b7);
        //         ram:b38a 67              LD         H,A
        self.instr_hk__LD_H_A();
        //         ram:b38b 2e 00           LD         L,0x0
        self.instr_hk__LD_L_NN(0x0);
        //         ram:b38d cb 3c           SRL        H
        self.instr_hk__SRL_H();
        //         ram:b38f cb 1d           RR         L
        self.instr_hk__RR_L();
        //         ram:b391 c9              RET
        self.assert_pc(0xb391);
        //
        true
    }
    pub(crate) fn hook_b392(&mut self) -> bool {
        println!("hook_b392");
        //
        //                              **************************************************************
        //                              *                          FUNCTION                          *
        //                              **************************************************************
        //                              undefined FUN_ram_b392()
        //              undefined         A:1            <RETURN>
        //                              FUN_ram_b392                                    XREF[1]:     sb_shop_guess_b3b2:b4ea(c)
        //         ram:b392 3a 85 c3        LD         A,(BYTE_ram_c385)
        self.instr_hk__LD_A_iNNNN(0xc385);
        //         ram:b395 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:b396 cb 3f           SRL        A
        self.instr_hk__SRL_A();
        //         ram:b398 47              LD         B,A
        self.instr_hk__LD_B_A();
        //         ram:b399 3e 07           LD         A,7
        self.instr_hk__LD_A_NN(7);
        //         ram:b39b 90              SUB        B
        self.instr_hk__SUB_A_B();
        //         ram:b39c 30 01           JR         NC,LAB_ram_b39f
        self.IncPC(2);
        if (self.data.F & FLAG_C) == 0 {
            self.increase_cycles(12);
            // JR(LAB_ram_b39f);
        } else {
            self.increase_cycles(7);
            //         ram:b39e af              XOR        A
            self.instr_hk__XOR_A_A();
        }

        //                              LAB_ram_b39f                                    XREF[1]:     ram:b39c(j)
        //         ram:b39f 3c              INC        A
        self.instr_hk__INC_A();
        //         ram:b3a0 06 01           LD         B,0x1
        self.instr_hk__LD_B_NN(0x1);
        //         ram:b3a2 cd 34 b6        CALL       sb_rand_calc_B634                                IN a:
        assert!(self.call_hook(0xb634));
        //                                                                                                 b: cnt
        //                                                                                              OUT hl
        //         ram:b3a5 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:b3a6 c9              RET
        true
        //                              s_ITEM_ram_b3a7                                 XREF[1]:     sb_shop_guess_b3b2:b43e(*)
        //         ram:b3a7 49 54 45        ds         "ITEM"
        //                  4d 00
        //                              s_PRICE_ram_b3ac                                XREF[1]:     sb_shop_guess_b3b2:b447(*)
        //         ram:b3ac 50 52 49        ds         "PRICE"
        //                  43 45 00
        //
    }
    pub(crate) fn hook_b60e(&mut self) -> bool {
        //         ram:b60e ed  5b  44  c2    LD         DE,(wd_rand_seed_c244 )
        self.instr_hk__LD_DE_iNNNN(0xc244);
        //         ram:b612 01  cd  43       LD         BC,0x43cd
        self.instr_hk__LD_BC_NNNN(0x43cd);
        //         ram:b615 3e  10           LD         A,0x10
        self.instr_hk__LD_A_NN(0x10);
        //         ram:b617 26  00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:b619 69              LD         L,C
        self.instr_hk__LD_L_C();
        //                              LAB_ram_b61a
        loop {
            self.SetPC(0xb61a);
            //         ram:b61a 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:b61b eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:b61c 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:b61d eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:b61e d2  22  b6       JP         NC,LAB_ram_b622
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP LAB_ram_b622;
            } else {
                //         ram:b621 09              ADD        HL,BC
                self.instr_hk__ADD_HL_BC();
            }
            //                              LAB_ram_b622
            //         ram:b622 3d              DEC        A
            self.instr_hk__DEC_A();
            //         ram:b623 c2  1a  b6       JP         NZ,LAB_ram_b61a
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP LAB_ram_b61a;
            } else {
                break;
            }
        }
        self.SetPC(0xb626);
        //         ram:b626 11  45  13       LD         DE,0x1345
        self.instr_hk__LD_DE_NNNN(0x1345);
        //         ram:b629 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:b62a 22  44  c2       LD         (wd_rand_seed_c244 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc244);
        //         ram:b62d ed  5f           LD         A,R
        self.instr_hk__LD_A_R();
        //         ram:b62f ad              XOR        L
        self.instr_hk__XOR_A_L();
        //         ram:b630 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:b631 ac              XOR        H
        self.instr_hk__XOR_A_H();
        //         ram:b632 67              LD         H,A
        self.instr_hk__LD_H_A();
        //         ram:b633 c9              RET

        true
    }
    pub(crate) fn hook_b634(&mut self) -> bool {
        println!("hook_b634");
        //         ram:b634 21  00  00       LD         HL,0x0                                           :
        self.instr_hk__LD_HL_NNNN(0x0);

        //         ram:b637 54              LD         D,H
        self.instr_hk__LD_D_H();
        //         ram:b638 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //                              LAB_ram_b639
        loop {
            self.SetPC(0xb639);
            //         ram:b639 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:b63a d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:b63b e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b63c d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:b63d cd  0e  b6       CALL       sb_rand_guess_B60E
            assert!(self.call_hook(0xb60e));
            //         ram:b640 d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:b641 cd  ac  b6       CALL       sb_calc_b6ac
            assert!(self.call_hook(0xb6ac));

            //         ram:b644 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:b645 19              ADD        HL,DE
            self.instr_hk__ADD_HL_DE();
            //         ram:b646 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b647 d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:b648 c1              POP        BC
            self.instr_hk__POP_BC();
            //         ram:b649 10  ee           DJNZ       LAB_ram_b639
            self.IncPC(2);
            self.decB();
            if self.data.B != 0 {
                self.increase_cycles(13);
                //JP LAB_ram_b639;
            } else {
                self.increase_cycles(8);
                break;
            }
        }
        //         ram:b64b c9              RET
        self.assert_pc(0xb64b);

        true
    }
    pub(crate) fn hook_b64c(&mut self) -> bool {
        //         ram:b64c 2a 58 c2        LD         HL,(tmp_var_c258)
        self.instr_hk__LD_HL_iNNNN(0xc258);
        //         ram:b64f e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b650 2a 56 c2        LD         HL,(BYTE_ram_c256)
        self.instr_hk__LD_HL_iNNNN(0xc256);
        //         ram:b653 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b654 21 c6 41        LD         HL,LAB_ram_41c6
        self.instr_hk__LD_HL_NNNN(0x41c6);
        //         ram:b657 e5              PUSH       HL=>LAB_ram_41c6
        self.instr_hk__PUSH_HL();
        //         ram:b658 21 6d 4e        LD         HL,LAB_ram_4e6d
        self.instr_hk__LD_HL_NNNN(0x4e6d);
        //         ram:b65b e5              PUSH       HL=>LAB_ram_4e6d
        self.instr_hk__PUSH_HL();
        //         ram:b65c cd f1 b6        CALL       FUN_ram_b6f1                                     (undefined
        assert!(self.call_hook(0xb6f1));
        //         ram:b65f 21 00 00        LD         HL,0x0
        self.instr_hk__LD_HL_NNNN(0x0);
        //         ram:b662 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b663 21 39 30        LD         HL,12345
        self.instr_hk__LD_HL_NNNN(12345);
        //         ram:b666 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b667 cd cd b6        CALL       FUN_ram_b6cd                                     (undefined
        assert!(self.call_hook(0xb6cd));
        //         ram:b66a e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:b66b 22 56 c2        LD         (BYTE_ram_c256),HL
        self.instr_hk__LD_iNNNN_HL(0xc256);
        //         ram:b66e e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:b66f 22 58 c2        LD         (tmp_var_c258),HL
        self.instr_hk__LD_iNNNN_HL(0xc258);
        //         ram:b672 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b673 2a 56 c2        LD         HL,(BYTE_ram_c256)
        self.instr_hk__LD_HL_iNNNN(0xc256);
        //         ram:b676 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b677 21 01 00        LD         HL,0x1
        self.instr_hk__LD_HL_NNNN(0x1);
        //         ram:b67a e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b67b 21 00 00        LD         HL,0x0
        self.instr_hk__LD_HL_NNNN(0x0);
        //         ram:b67e e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b67f cd 9b b7        CALL       sb_change_mem_b79b                               ()
        assert!(self.call_hook(0xb79b));
        //         ram:b682 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:b683 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b684 7c              LD         A,H
        self.instr_hk__LD_A_H();
        //         ram:b685 e6 7f           AND        0x7f
        self.instr_hk__AND_NN(0x7f);
        //         ram:b687 67              LD         H,A
        self.instr_hk__LD_H_A();
        //         ram:b688 c9              RET
        self.assert_pc(0xb688);

        true
    }
    pub(crate) fn hook_b695(&mut self) -> bool {
        //         ram:b695 e5              PUSH       HL
        self.instr_hk__PUSH_HL();

        //         ram:b696 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:b697 26 00           LD         H,0x0
        self.instr_hk__LD_H_NN(0x0);
        //         ram:b699 0e 08           LD         C,0x8
        self.instr_hk__LD_C_NN(0x8);
        //                              loop
        loop {
            self.SetPC(0xb69b);
            //         ram:b69b 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:b69c 7c              LD         A,H
            self.instr_hk__LD_A_H();
            //         ram:b69d b8              CP         B
            self.instr_hk__CP_B();
            //         ram:b69e da a4 b6        JP         C,loop_chk_cond
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) != 0 {
                // JP loop_chk_cond;
            } else {
                //         ram:b6a1 90              SUB        B
                self.instr_hk__SUB_A_B();
                //         ram:b6a2 2c              INC        L
                self.instr_hk__INC_L();
                //         ram:b6a3 67              LD         H,A
                self.instr_hk__LD_H_A();
            }
            self.SetPC(0xb6a4);

            //                              loop_chk_cond
            //         ram:b6a4 0d              DEC        C
            self.instr_hk__DEC_C();
            //         ram:b6a5 c2 9b b6        JP         NZ,loop
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop;
            } else {
                break;
            }
        }

        //         ram:b6a8 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:b6a9 44              LD         B,H
        self.instr_hk__LD_B_H();
        //         ram:b6aa e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:b6ab c9              RET

        self.assert_pc(0xb6ab);

        true
    }
    pub(crate) fn hook_b6ac(&mut self) -> bool {
        //         ram:b6ac 42              LD         B,D
        self.instr_hk__LD_B_D();

        //         ram:b6ad 4b              LD         C,E
        self.instr_hk__LD_C_E();
        //         ram:b6ae eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:b6af 21  00  00       LD         HL,0x0
        self.instr_hk__LD_HL_NNNN(0x0);
        //         ram:b6b2 3e  10           LD         A,0x10
        self.instr_hk__LD_A_NN(0x10);
        //                              loop_b6b4
        loop {
            self.SetPC(0xb6b4);
            //         ram:b6b4 f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:b6b5 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:b6b6 af              XOR        A
            self.instr_hk__XOR_A_A();
            //         ram:b6b7 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:b6b8 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            //         ram:b6b9 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:b6ba 8d              ADC        A,L
            self.instr_hk__ADC_A_L();
            //         ram:b6bb 91              SUB        C
            self.instr_hk__SUB_A_C();
            //         ram:b6bc 6f              LD         L,A
            self.instr_hk__LD_L_A();
            //         ram:b6bd 7c              LD         A,H
            self.instr_hk__LD_A_H();
            //         ram:b6be 98              SBC        A,B
            self.instr_hk__SBC_A_B();
            //         ram:b6bf 67              LD         H,A
            self.instr_hk__LD_H_A();
            //         ram:b6c0 1c              INC        E
            self.instr_hk__INC_E();
            //         ram:b6c1 d2  c6  b6       JP         NC,chk_b6c6
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP chk_b6c6;
            } else {
                //         ram:b6c4 09              ADD        HL,BC
                self.instr_hk__ADD_HL_BC();
                //         ram:b6c5 1d              DEC        E
                self.instr_hk__DEC_E();
            }
            self.SetPC(0xb6c6);
            //                              chk_b6c6
            //         ram:b6c6 f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:b6c7 3d              DEC        A
            self.instr_hk__DEC_A();
            //         ram:b6c8 c2  b4  b6       JP         NZ,loop_b6b4
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop_b6b4;
            } else {
                break;
            }
        }
        //         ram:b6cb eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:b6cc c9              RET
        self.assert_pc(0xb6cc);

        true
    }
    pub(crate) fn hook_b6cd(&mut self) -> bool {
        //         ram:b6cd 21 06 00        LD         HL,0x6
        self.instr_hk__LD_HL_NNNN(0x6);
        //         ram:b6d0 39              ADD        HL,SP
        self.instr_hk__ADD_HL_SP();
        //         ram:b6d1 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:b6d2 21 02 00        LD         HL,0x2
        self.instr_hk__LD_HL_NNNN(0x2);
        //         ram:b6d5 39              ADD        HL,SP
        self.instr_hk__ADD_HL_SP();
        //         ram:b6d6 1a              LD         A,(DE=>param_3)
        self.instr_hk__LD_A_iDE();
        //         ram:b6d7 86              ADD        A,(HL=>param_1)
        self.instr_hk__ADD_A_iHL();
        //         ram:b6d8 12              LD         (DE=>param_3),A
        self.instr_hk__LD_iDE_A();
        //         ram:b6d9 13              INC        DE
        self.instr_hk__INC_DE();
        //         ram:b6da 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b6db 1a              LD         A,(DE=>Stack[0x7])
        self.instr_hk__LD_A_iDE();
        //         ram:b6dc 8e              ADC        A,(HL=>Stack[0x3])
        self.instr_hk__ADC_A_iHL();
        //         ram:b6dd 12              LD         (DE=>Stack[0x7]),A
        self.instr_hk__LD_iDE_A();
        //         ram:b6de 13              INC        DE
        self.instr_hk__INC_DE();
        //         ram:b6df 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b6e0 1a              LD         A,(DE=>param_4)
        self.instr_hk__LD_A_iDE();
        //         ram:b6e1 8e              ADC        A,(HL=>param_2)
        self.instr_hk__ADC_A_iHL();
        //         ram:b6e2 12              LD         (DE=>param_4),A
        self.instr_hk__LD_iDE_A();
        //         ram:b6e3 13              INC        DE
        self.instr_hk__INC_DE();
        //         ram:b6e4 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b6e5 1a              LD         A,(DE=>Stack[0x9])
        self.instr_hk__LD_A_iDE();
        //         ram:b6e6 8e              ADC        A,(HL=>Stack[0x5])
        self.instr_hk__ADC_A_iHL();
        //         ram:b6e7 12              LD         (DE=>Stack[0x9]),A
        self.instr_hk__LD_iDE_A();
        //         ram:b6e8 1b              DEC        DE
        self.instr_hk__DEC_DE();
        //         ram:b6e9 1b              DEC        DE
        self.instr_hk__DEC_DE();
        //         ram:b6ea 1b              DEC        DE
        self.instr_hk__DEC_DE();
        //         ram:b6eb eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:b6ec c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:b6ed f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b6ee f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b6ef c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:b6f0 c9              RET
        self.assert_pc(0xb6f0);

        true
    }
    pub(crate) fn hook_b6f1(&mut self) -> bool {
        //         ram:b6f1 21 06 00        LD         HL,0x6
        self.instr_hk__LD_HL_NNNN(0x6);
        //         ram:b6f4 39              ADD        HL,SP
        self.instr_hk__ADD_HL_SP();
        //         ram:b6f5 01 00 00        LD         BC,0x0
        self.instr_hk__LD_BC_NNNN(0x0);
        //         ram:b6f8 50              LD         D,B
        self.instr_hk__LD_D_B();
        //         ram:b6f9 58              LD         E,B
        self.instr_hk__LD_E_B();
        //         ram:b6fa e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b6fb c3 32 b7        JP         loop_chk_cond
        self.IncPC(3);
        self.increase_cycles(10);
        // JP loop_chk_cond;

        //                              loop
        loop {
            self.SetPC(0xb732);
            //                              loop_chk_cond
            //         ram:b732 21 04 00        LD         HL,0x4
            self.instr_hk__LD_HL_NNNN(0x4);
            //         ram:b735 39              ADD        HL,SP
            self.instr_hk__ADD_HL_SP();
            //         ram:b736 7e              LD         A,(HL=>param_1)
            self.instr_hk__LD_A_iHL();
            //         ram:b737 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b738 b6              OR         (HL=>Stack[0x3])
            self.instr_hk__OR_A_iHL();
            //         ram:b739 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b73a b6              OR         (HL=>param_2)
            self.instr_hk__OR_A_iHL();
            //         ram:b73b 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b73c b6              OR         (HL=>Stack[0x5])
            self.instr_hk__OR_A_iHL();
            //         ram:b73d c2 fe b6        JP         NZ,loop
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop;
            } else {
                break;
            }
            //         ram:b6fe 7e              LD         A,(HL=>Stack[0x5])
            self.instr_hk__LD_A_iHL();
            //         ram:b6ff 1f              RRA
            self.instr_hk__RRA();
            //         ram:b700 77              LD         (HL=>Stack[0x5]),A
            self.instr_hk__LD_iHL_A();
            //         ram:b701 2b              DEC        HL
            self.instr_hk__DEC_HL();
            //         ram:b702 7e              LD         A,(HL=>param_2)
            self.instr_hk__LD_A_iHL();
            //         ram:b703 1f              RRA
            self.instr_hk__RRA();
            //         ram:b704 77              LD         (HL=>param_2),A
            self.instr_hk__LD_iHL_A();
            //         ram:b705 2b              DEC        HL
            self.instr_hk__DEC_HL();
            //         ram:b706 7e              LD         A,(HL=>Stack[0x3])
            self.instr_hk__LD_A_iHL();
            //         ram:b707 1f              RRA
            self.instr_hk__RRA();
            //         ram:b708 77              LD         (HL=>Stack[0x3]),A
            self.instr_hk__LD_iHL_A();
            //         ram:b709 2b              DEC        HL
            self.instr_hk__DEC_HL();
            //         ram:b70a 7e              LD         A,(HL=>param_1)
            self.instr_hk__LD_A_iHL();
            //         ram:b70b 1f              RRA
            self.instr_hk__RRA();
            //         ram:b70c 77              LD         (HL=>param_1),A
            self.instr_hk__LD_iHL_A();
            //         ram:b70d d2 21 b7        JP         NC,LAB_ram_b721
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP LAB_ram_b721;
            } else {
                //         ram:b710 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:b711 e5              PUSH       HL
                self.instr_hk__PUSH_HL();
                //         ram:b712 7b              LD         A,E
                self.instr_hk__LD_A_E();
                //         ram:b713 86              ADD        A,(HL=>param_3)
                self.instr_hk__ADD_A_iHL();
                //         ram:b714 5f              LD         E,A
                self.instr_hk__LD_E_A();
                //         ram:b715 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b716 7a              LD         A,D
                self.instr_hk__LD_A_D();
                //         ram:b717 8e              ADC        A,(HL=>Stack[0x7])
                self.instr_hk__ADC_A_iHL();
                //         ram:b718 57              LD         D,A
                self.instr_hk__LD_D_A();
                //         ram:b719 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b71a 79              LD         A,C
                self.instr_hk__LD_A_C();
                //         ram:b71b 8e              ADC        A,(HL=>param_4)
                self.instr_hk__ADC_A_iHL();
                //         ram:b71c 4f              LD         C,A
                self.instr_hk__LD_C_A();
                //         ram:b71d 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b71e 78              LD         A,B
                self.instr_hk__LD_A_B();
                //         ram:b71f 8e              ADC        A,(HL=>Stack[0x9])
                self.instr_hk__ADC_A_iHL();
                //         ram:b720 47              LD         B,A
                self.instr_hk__LD_B_A();
            }
            self.SetPC(0xb721);

            //                              LAB_ram_b721
            //         ram:b721 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:b722 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b723 7e              LD         A,(HL=>param_3)
            self.instr_hk__LD_A_iHL();
            //         ram:b724 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:b725 77              LD         (HL=>param_3),A
            self.instr_hk__LD_iHL_A();
            //         ram:b726 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b727 7e              LD         A,(HL=>Stack[0x7])
            self.instr_hk__LD_A_iHL();
            //         ram:b728 17              RLA
            self.instr_hk__RLA();
            //         ram:b729 77              LD         (HL=>Stack[0x7]),A
            self.instr_hk__LD_iHL_A();
            //         ram:b72a 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b72b 7e              LD         A,(HL=>param_4)
            self.instr_hk__LD_A_iHL();
            //         ram:b72c 17              RLA
            self.instr_hk__RLA();
            //         ram:b72d 77              LD         (HL=>param_4),A
            self.instr_hk__LD_iHL_A();
            //         ram:b72e 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b72f 7e              LD         A,(HL=>Stack[0x9])
            self.instr_hk__LD_A_iHL();
            //         ram:b730 17              RLA
            self.instr_hk__RLA();
            //         ram:b731 77              LD         (HL=>Stack[0x9]),A
            self.instr_hk__LD_iHL_A();
        }

        //         ram:b740 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:b741 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b742 73              LD         (HL=>param_3),E
        self.instr_hk__LD_iHL_E();
        //         ram:b743 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b744 72              LD         (HL=>Stack[0x7]),D
        self.instr_hk__LD_iHL_D();
        //         ram:b745 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b746 71              LD         (HL=>param_4),C
        self.instr_hk__LD_iHL_C();
        //         ram:b747 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b748 70              LD         (HL=>Stack[0x9]),B
        self.instr_hk__LD_iHL_B();
        //         ram:b749 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:b74a c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:b74b f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b74c f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b74d c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:b74e c9              RET
        self.assert_pc(0xb74e);

        true
    }
    pub(crate) fn hook_b74f(&mut self) -> bool {
        //         ram:b74f 01 00 00        LD         BC,0x0                                           IN
        self.instr_hk__LD_BC_NNNN(0x0);

        //         ram:b752 50              LD         D,B
        self.instr_hk__LD_D_B();
        //         ram:b753 58              LD         E,B
        self.instr_hk__LD_E_B();
        //         ram:b754 3e 20           LD         A,0x20
        self.instr_hk__LD_A_NN(0x20);
        //                              loop
        loop {
            self.SetPC(0xb756);
            //         ram:b756 f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:b757 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b758 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:b759 87              ADD        A,A
            self.instr_hk__ADD_A_A();
            //         ram:b75a 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:b75b 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b75c 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:b75d 17              RLA
            self.instr_hk__RLA();
            //         ram:b75e 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:b75f 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b760 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:b761 17              RLA
            self.instr_hk__RLA();
            //         ram:b762 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:b763 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b764 7e              LD         A,(HL)
            self.instr_hk__LD_A_iHL();
            //         ram:b765 17              RLA
            self.instr_hk__RLA();
            //         ram:b766 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:b767 7b              LD         A,E
            self.instr_hk__LD_A_E();
            //         ram:b768 17              RLA
            self.instr_hk__RLA();
            //         ram:b769 5f              LD         E,A
            self.instr_hk__LD_E_A();
            //         ram:b76a 7a              LD         A,D
            self.instr_hk__LD_A_D();
            //         ram:b76b 17              RLA
            self.instr_hk__RLA();
            //         ram:b76c 57              LD         D,A
            self.instr_hk__LD_D_A();
            //         ram:b76d 79              LD         A,C
            self.instr_hk__LD_A_C();
            //         ram:b76e 17              RLA
            self.instr_hk__RLA();
            //         ram:b76f 4f              LD         C,A
            self.instr_hk__LD_C_A();
            //         ram:b770 78              LD         A,B
            self.instr_hk__LD_A_B();
            //         ram:b771 17              RLA
            self.instr_hk__RLA();
            //         ram:b772 47              LD         B,A
            self.instr_hk__LD_B_A();
            //         ram:b773 d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:b774 c5              PUSH       BC
            self.instr_hk__PUSH_BC();
            //         ram:b775 21 0e 00        LD         HL,0xe
            self.instr_hk__LD_HL_NNNN(0xe);
            //         ram:b778 39              ADD        HL,SP
            self.instr_hk__ADD_HL_SP();
            //         ram:b779 7b              LD         A,E
            self.instr_hk__LD_A_E();
            //         ram:b77a 96              SUB        (HL=>param_3)
            self.instr_hk__SUB_A_iHL();
            //         ram:b77b 5f              LD         E,A
            self.instr_hk__LD_E_A();
            //         ram:b77c 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b77d 7a              LD         A,D
            self.instr_hk__LD_A_D();
            //         ram:b77e 9e              SBC        A,(HL=>Stack[0x7])
            self.instr_hk__SBC_A_iHL();
            //         ram:b77f 57              LD         D,A
            self.instr_hk__LD_D_A();
            //         ram:b780 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b781 79              LD         A,C
            self.instr_hk__LD_A_C();
            //         ram:b782 9e              SBC        A,(HL=>param_4)
            self.instr_hk__SBC_A_iHL();
            //         ram:b783 4f              LD         C,A
            self.instr_hk__LD_C_A();
            //         ram:b784 23              INC        HL
            self.instr_hk__INC_HL();
            //         ram:b785 78              LD         A,B
            self.instr_hk__LD_A_B();
            //         ram:b786 9e              SBC        A,(HL=>Stack[0x9])
            self.instr_hk__SBC_A_iHL();
            //         ram:b787 47              LD         B,A
            self.instr_hk__LD_B_A();
            //         ram:b788 d2 91 b7        JP         NC,LAB_ram_b791
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP LAB_ram_b791;
                self.SetPC(0xb791);

                //                              LAB_ram_b791
                //         ram:b791 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:b792 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:b793 e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:b794 34              INC        (HL)
                self.instr_hk__INC_iHL();
            //                              loop_chk_cond
            } else {
                //         ram:b78b c1              POP        BC
                self.instr_hk__POP_BC();
                //         ram:b78c d1              POP        DE
                self.instr_hk__POP_DE();
                //         ram:b78d e1              POP        HL
                self.instr_hk__POP_HL();
                //         ram:b78e c3 95 b7        JP         loop_chk_cond
                self.IncPC(3);
                self.increase_cycles(10); //JP loop_chk_cond;
            }
            self.SetPC(0xb795);

            //                              loop_chk_cond
            //         ram:b795 f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:b796 3d              DEC        A
            self.instr_hk__DEC_A();
            //         ram:b797 c2 56 b7        JP         NZ,loop
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP loop;
            } else {
                break;
            }
        }

        //         ram:b79a c9              RET
        self.assert_pc(0xb79a);

        true
    }
    pub(crate) fn hook_b79b(&mut self) -> bool {
        //         ram:b79b 21 06 00        LD         HL,0x6
        self.instr_hk__LD_HL_NNNN(0x6);
        //         ram:b79e 39              ADD        HL,SP
        self.instr_hk__ADD_HL_SP();
        self.internal_b79f();

        true
    }
    fn internal_b79f(&mut self) {
        //         ram:b79f f5              PUSH       AF                                               IN
        self.instr_hk__PUSH_AF();
        //         ram:b7a0 cd 4f b7        CALL       sb_change_mem_B74F                               IN
        assert!(self.call_hook(0xb74f));
        //         ram:b7a3 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b7a4 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:b7a5 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b7a6 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b7a7 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:b7a8 c9              RET
    }
    pub(crate) fn hook_b79f(&mut self) -> bool {
        self.internal_b79f();
        true
    }
    pub(crate) fn hook_b7a9(&mut self) -> bool {
        // ram:b7a9 44              LD         B,H
        self.instr_hk__LD_B_H();
        // ram:b7aa 4d              LD         C,L
        self.instr_hk__LD_C_L();
        // ram:b7ab 21  00  00       LD         HL,0x0
        self.instr_hk__LD_HL_NNNN(0x0);
        // ram:b7ae 3e  10           LD         A,0x10
        self.instr_hk__LD_A_NN(0x10);
        //         LAB_ram_b7b0
        loop {
            self.SetPC(0xb7b0);
            // ram:b7b0 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            // ram:b7b1 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            // ram:b7b2 29              ADD        HL,HL
            self.instr_hk__ADD_HL_HL();
            // ram:b7b3 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            // ram:b7b4 d2  b8  b7       JP         NC,LAB_ram_b7b8
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                // JP LAB_ram_b7b8;
            } else {
                // ram:b7b7
                // ram:b7b7 09              ADD        HL,BC
                self.instr_hk__ADD_HL_BC();
            }
            //         LAB_ram_b7b8
            // ram:b7b8 3d              DEC        A
            self.instr_hk__DEC_A();
            // ram:b7b9 c2  b0  b7       JP         NZ,LAB_ram_b7b0
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP LAB_ram_b7b0;
            } else {
                break;
            }
        }
        // ram:b7bc c9              RET

        true
    }
    pub(crate) fn hook_b7bd(&mut self) -> bool {
        //         ram:b7bd e5              PUSH       HL
        self.instr_hk__PUSH_HL();

        //         ram:b7be eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:b7bf 22  68  c2       LD         (wd_p_buffer_c268 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc268);
        //         ram:b7c2 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:b7c3 af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:b7c4 32  67  c2       LD         (BYTE_ram_c267 ),A
        self.instr_hk__LD_iNNNN_A(0xc267);
        //         ram:b7c7 11  10  27       LD         DE,0x2710
        self.instr_hk__LD_DE_NNNN(0x2710);
        //         ram:b7ca 3e  01           LD         A,0x1
        self.instr_hk__LD_A_NN(0x1);
        //                              LAB_ram_b7cc
        loop {
            self.SetPC(0xb7cc);
            //         ram:b7cc fe  05           CP         0x5
            self.instr_hk__CP_NN(0x5);
            //         ram:b7ce d2  17  b8       JP         NC,LAB_ram_b817
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                break; //JP LAB_ram_b817;
            }
            //         ram:b7d1 f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:b7d2 d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:b7d3 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b7d4 cd  ac  b6       CALL       sb_calc_b6ac
            assert!(self.call_hook(0xb6ac));

            //         ram:b7d7 7d              LD         A,L
            self.instr_hk__LD_A_L();
            //         ram:b7d8 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:b7d9 ca  ee  b7       JP         Z,LAB_ram_b7ee
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                self.SetPC(0xb7ee);
                // JP LAB_ram_b7ee;
                //                              LAB_ram_b7ee
                //         ram:b7ee 3a  67  c2       LD         A,(BYTE_ram_c267 )
                self.instr_hk__LD_A_iNNNN(0xc267);
                //         ram:b7f1 b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:b7f2 ca  fe  b7       JP         Z,LAB_ram_b7fe
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) != 0 {
                    // JP LAB_ram_b7fe;
                } else {
                    self.SetPC(0xb7f5);
                    //         ram:b7f5
                    //         ram:b7f5 2a  68  c2       LD         HL,(wd_p_buffer_c268 )
                    self.instr_hk__LD_HL_iNNNN(0xc268);
                    //         ram:b7f8 36  30           LD         (HL),0x30
                    self.instr_hk__LD_iHL_NN(0x30);
                    //         ram:b7fa 23              INC        HL
                    self.instr_hk__INC_HL();
                    //         ram:b7fb 22  68  c2       LD         (wd_p_buffer_c268 ),HL
                    self.instr_hk__LD_iNNNN_HL(0xc268);
                    // JP LAB_ram_b7fe;
                }
            } else {
                self.SetPC(0xb7dc);
                //         ram:b7dc c6  30           ADD        A,0x30
                self.instr_hk__ADD_A_NN(0x30);
                //         ram:b7de 2a  68  c2       LD         HL,(wd_p_buffer_c268 )
                self.instr_hk__LD_HL_iNNNN(0xc268);
                //         ram:b7e1 77              LD         (HL),A
                self.instr_hk__LD_iHL_A();
                //         ram:b7e2 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b7e3 22  68  c2       LD         (wd_p_buffer_c268 ),HL
                self.instr_hk__LD_iNNNN_HL(0xc268);
                //         ram:b7e6 3e  01           LD         A,0x1
                self.instr_hk__LD_A_NN(0x1);
                //         ram:b7e8 32  67  c2       LD         (BYTE_ram_c267 ),A
                self.instr_hk__LD_iNNNN_A(0xc267);
                //         ram:b7eb c3  fe  b7       JP         LAB_ram_b7fe
                self.IncPC(3);
                self.increase_cycles(10); //JP LAB_ram_b7fe;
            }
            self.SetPC(0xb7fe);

            //                              LAB_ram_b7fe                                          ,
            //         ram:b7fe e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:b7ff d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:b800 d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:b801 cd  ac  b6       CALL       sb_calc_b6ac
            assert!(self.call_hook(0xb6ac));

            //         ram:b804 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:b805 d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:b806 f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:b807 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b808 f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:b809 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:b80a 11  0a  00       LD         DE,0xa
            self.instr_hk__LD_DE_NNNN(0xa);
            //         ram:b80d cd  ac  b6       CALL       sb_calc_b6ac
            assert!(self.call_hook(0xb6ac));

            //         ram:b810 eb              EX         DE,HL
            self.instr_hk__EX_DE_HL();
            //         ram:b811 f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:b812 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:b813 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:b814 c3  cc  b7       JP         LAB_ram_b7cc
            self.IncPC(3);
            self.increase_cycles(10); //JP LAB_ram_b7cc;
        }
        self.SetPC(0xb817);

        //                              LAB_ram_b817
        //         ram:b817 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:b818 c6  30           ADD        A,0x30
        self.instr_hk__ADD_A_NN(0x30);
        //         ram:b81a 2a  68  c2       LD         HL,(wd_p_buffer_c268 )
        self.instr_hk__LD_HL_iNNNN(0xc268);
        //         ram:b81d 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:b81e 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b81f 22  68  c2       LD         (wd_p_buffer_c268 ),HL
        self.instr_hk__LD_iNNNN_HL(0xc268);
        //         ram:b822 36  00           LD         (HL),0x0
        self.instr_hk__LD_iHL_NN(0x0);
        //         ram:b824 c9              RET

        true
    }
    pub(crate) fn hook_b825(&mut self) -> bool {
        //         ram:b825 22 65 c2        LD         (wd_l_paddr_c265),HL
        self.instr_hk__LD_iNNNN_HL(0xc265);
        //         ram:b828 af              XOR        A
        self.instr_hk__XOR_A_A();
        //         ram:b829 32 64 c2        LD         (bt_l_c264),A
        self.instr_hk__LD_iNNNN_A(0xc264);
        //         ram:b82c 21 40 42        LD         HL,0x4240                                        0xf4240 = 1000000 (1M)
        self.instr_hk__LD_HL_NNNN(0x4240);
        //         ram:b82f 22 60 c2        LD         (wd_l_c260),HL
        self.instr_hk__LD_iNNNN_HL(0xc260);
        //         ram:b832 21 0f 00        LD         HL,0xf
        self.instr_hk__LD_HL_NNNN(0xf);
        //         ram:b835 22 62 c2        LD         (wd_l_c262),HL
        self.instr_hk__LD_iNNNN_HL(0xc262);
        //         ram:b838 3e 01           LD         A,0x1
        self.instr_hk__LD_A_NN(0x1);
        //                              loop_1                                               FUN_ram_b87a:b89a(j)
        loop {
            //         ram:b83a fe 07           CP         0x7
            self.instr_hk__CP_NN(0x7);
            //         ram:b83c d2 9d b8        JP         NC,l_exit_x
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_C) == 0 {
                break; //JP l_exit_x;
            }

            //         ram:b83f f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:b840 21 04 00        LD         HL,0x4
            self.instr_hk__LD_HL_NNNN(0x4);
            //         ram:b843 39              ADD        HL,SP
            self.instr_hk__ADD_HL_SP();
            //         ram:b844 cd b4 b8        CALL       sb_push_2_addrs_B8B4                             IN
            assert!(self.call_hook(0xb8b4));

            //         ram:b847 2a 62 c2        LD         HL,(wd_l_c262)
            self.instr_hk__LD_HL_iNNNN(0xc262);
            //         ram:b84a e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b84b 2a 60 c2        LD         HL,(wd_l_c260)
            self.instr_hk__LD_HL_iNNNN(0xc260);
            //         ram:b84e e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b84f cd 9b b7        CALL       sb_change_mem_b79b                               ()
            assert!(self.call_hook(0xb79b));
            //         ram:b852 e1              POP        HL
            self.instr_hk__POP_HL();
            //         ram:b853 f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:b854 7d              LD         A,L
            self.instr_hk__LD_A_L();
            //         ram:b855 b7              OR         A
            self.instr_hk__OR_A_A();
            //         ram:b856 ca 6b b8        JP         Z,LAB_ram_b86b
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP LAB_ram_b86b;
                //                              LAB_ram_b86b
                //         ram:b86b 3a 64 c2        LD         A,(bt_l_c264)
                self.instr_hk__LD_A_iNNNN(0xc264);
                //         ram:b86e b7              OR         A
                self.instr_hk__OR_A_A();
                //         ram:b86f ca 7b b8        JP         Z,LAB_ram_b87b
                self.IncPC(3);
                self.increase_cycles(10);
                if (self.data.F & FLAG_Z) != 0 {
                    // JP LAB_ram_b87b;
                } else {
                    //         ram:b872 2a 65 c2        LD         HL,(wd_l_paddr_c265)
                    self.instr_hk__LD_HL_iNNNN(0xc265);
                    //         ram:b875 36 30           LD         (HL),0x30
                    self.instr_hk__LD_iHL_NN(0x30);
                    //         ram:b877 23              INC        HL
                    self.instr_hk__INC_HL();
                    //         ram:b878 22 65 c2        LD         (wd_l_paddr_c265),HL
                    self.instr_hk__LD_iNNNN_HL(0xc265);
                }
            } else {
                //         ram:b859 c6 30           ADD        A,0x30
                self.instr_hk__ADD_A_NN(0x30);
                //         ram:b85b 2a 65 c2        LD         HL,(wd_l_paddr_c265)
                self.instr_hk__LD_HL_iNNNN(0xc265);
                //         ram:b85e 77              LD         (HL),A
                self.instr_hk__LD_iHL_A();
                //         ram:b85f 23              INC        HL
                self.instr_hk__INC_HL();
                //         ram:b860 22 65 c2        LD         (wd_l_paddr_c265),HL
                self.instr_hk__LD_iNNNN_HL(0xc265);
                //         ram:b863 3e 01           LD         A,0x1
                self.instr_hk__LD_A_NN(0x1);
                //         ram:b865 32 64 c2        LD         (bt_l_c264),A
                self.instr_hk__LD_iNNNN_A(0xc264);
                //         ram:b868 c3 7b b8        JP         LAB_ram_b87b
                self.IncPC(3);
                self.increase_cycles(10); //JP LAB_ram_b87b;
            }

            //                              LAB_ram_b87b                                         sb_itoa_3bytes_B825:b868(j),
            //                                                                                           sb_itoa_3bytes_B825:b86f(j)
            //         ram:b87b 2a 62 c2        LD         HL,(wd_l_c262)
            self.instr_hk__LD_HL_iNNNN(0xc262);
            //         ram:b87e e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b87f 2a 60 c2        LD         HL,(wd_l_c260)
            self.instr_hk__LD_HL_iNNNN(0xc260);
            //         ram:b882 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b883 21 08 00        LD         HL,0x8
            self.instr_hk__LD_HL_NNNN(0x8);
            //         ram:b886 39              ADD        HL,SP
            self.instr_hk__ADD_HL_SP();
            //         ram:b887 cd c3 b8        CALL       sb_change_mem_B8C3                               IN
            assert!(self.call_hook(0xb8c3));
            //         ram:b88a 21 00 00        LD         HL,0x0
            self.instr_hk__LD_HL_NNNN(0x0);
            //         ram:b88d e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b88e 21 0a 00        LD         HL,0xa
            self.instr_hk__LD_HL_NNNN(0xa);
            //         ram:b891 e5              PUSH       HL
            self.instr_hk__PUSH_HL();
            //         ram:b892 21 60 c2        LD         HL,0xc260
            self.instr_hk__LD_HL_NNNN(0xc260);
            //         ram:b895 cd 9f b7        CALL       sb_change_mem_b79f                               IN
            assert!(self.call_hook(0xb79f));
            //         ram:b898 f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:b899 3c              INC        A
            self.instr_hk__INC_A();
            //         ram:b89a c3 3a b8        JP         sb_itoa_3bytes_B825::loop_1
            self.IncPC(3);
            self.increase_cycles(10); //JP sb_itoa_3bytes_B825::loop_1;
        }

        //                              l_exit_x
        //         ram:b89d 21 02 00        LD         HL,0x2
        self.instr_hk__LD_HL_NNNN(0x2);
        //         ram:b8a0 39              ADD        HL,SP
        self.instr_hk__ADD_HL_SP();
        //         ram:b8a1 cd b4 b8        CALL       sb_push_2_addrs_B8B4                             IN
        assert!(self.call_hook(0xb8b4));

        //         ram:b8a4 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:b8a5 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b8a6 79              LD         A,C
        self.instr_hk__LD_A_C();
        //         ram:b8a7 c6 30           ADD        A,0x30
        self.instr_hk__ADD_A_NN(0x30);
        //         ram:b8a9 2a 65 c2        LD         HL,(wd_l_paddr_c265)
        self.instr_hk__LD_HL_iNNNN(0xc265);
        //         ram:b8ac 77              LD         (HL),A
        self.instr_hk__LD_iHL_A();
        //         ram:b8ad 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b8ae 22 65 c2        LD         (wd_l_paddr_c265),HL
        self.instr_hk__LD_iNNNN_HL(0xc265);
        //         ram:b8b1 36 00           LD         (HL),0x0
        self.instr_hk__LD_iHL_NN(0x0);
        //         ram:b8b3 c9              RET
        true
    }
    pub(crate) fn hook_b8b4(&mut self) -> bool {
        //         ram:b8b4 c1              POP        BC                                               IN
        self.instr_hk__POP_BC();

        //         ram:b8b5 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b8b6 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b8b7 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b8b8 56              LD         D,(HL)
        self.instr_hk__LD_D_iHL();
        //         ram:b8b9 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:b8ba 5e              LD         E,(HL)
        self.instr_hk__LD_E_iHL();
        //         ram:b8bb d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:b8bc 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:b8bd 56              LD         D,(HL)
        self.instr_hk__LD_D_iHL();
        //         ram:b8be 2b              DEC        HL
        self.instr_hk__DEC_HL();
        //         ram:b8bf 5e              LD         E,(HL)
        self.instr_hk__LD_E_iHL();
        //         ram:b8c0 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:b8c1 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:b8c2 c9              RET

        self.assert_pc(0xb8c2);
        true
    }
    pub(crate) fn hook_b8c3(&mut self) -> bool {
        //         ram:b8c3 f5              PUSH       AF                                               IN
        self.instr_hk__PUSH_AF();
        //         ram:b8c4 cd 4f b7        CALL       sb_change_mem_B74F                               IN
        assert!(self.call_hook(0xb74f));

        //         ram:b8c7 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b8c8 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:b8c9 73              LD         (HL),E
        self.instr_hk__LD_iHL_E();
        //         ram:b8ca 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b8cb 72              LD         (HL),D
        self.instr_hk__LD_iHL_D();
        //         ram:b8cc 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b8cd 71              LD         (HL),C
        self.instr_hk__LD_iHL_C();
        //         ram:b8ce 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:b8cf 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:b8d0 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:b8d1 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:b8d2 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b8d3 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b8d4 c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:b8d5 c9              RET

        self.assert_pc(0xb8d5);
        true
    }
    pub(crate) fn hook_b8d6(&mut self) -> bool {
        //         ram:b8d6 69              LD         L,C
        self.instr_hk__LD_L_C();
        //         ram:b8d7 60              LD         H,B
        self.instr_hk__LD_H_B();
        //         ram:b8d8 22 6a c2        LD         (DAT_ram_c26a),HL
        self.instr_hk__LD_iNNNN_HL(0xc26a);
        //         ram:b8db f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:b8dc d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:b8dd eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:b8de 11 08 00        LD         DE,0x8
        self.instr_hk__LD_DE_NNNN(0x8);
        //         ram:b8e1 cd ac b6        CALL       sb_calc_b6ac                                     IN de, hl
        assert!(self.call_hook(0xb6ac));
        //                                                                                              OUT de, hl
        //WRONG? OUT de, hl
        //         ram:b8e4 ed 5b 6a c2     LD         DE,(DAT_ram_c26a)
        self.instr_hk__LD_DE_iNNNN(0xc26a);
        //         ram:b8e8 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:b8e9 22 6a c2        LD         (DAT_ram_c26a),HL
        self.instr_hk__LD_iNNNN_HL(0xc26a);
        //         ram:b8ec 0e 80           LD         C,0x80
        self.instr_hk__LD_C_NN(0x80);
        //         ram:b8ee d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:b8ef 7b              LD         A,E
        self.instr_hk__LD_A_E();
        //         ram:b8f0 e6 07           AND        0x7
        self.instr_hk__AND_NN(0x7);
        //         ram:b8f2 5f              LD         E,A
        self.instr_hk__LD_E_A();
        //                              loop_1                                          XREF[1]:     ram:b903(j)
        loop {
            self.SetPC(0xb8f3);
            //         ram:b8f3 1d              DEC        E
            self.instr_hk__DEC_E();
            //         ram:b8f4 7b              LD         A,E
            self.instr_hk__LD_A_E();
            //         ram:b8f5 fe ff           CP         0xff
            self.instr_hk__CP_NN(0xff);
            //         ram:b8f7 ca 06 b9        JP         Z,l_exit_x_A
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(l_exit_x_A);
                break;
            }

            //         ram:b8fa d5              PUSH       DE
            self.instr_hk__PUSH_DE();
            //         ram:b8fb 79              LD         A,C
            self.instr_hk__LD_A_C();
            //         ram:b8fc 06 02           LD         B,0x2
            self.instr_hk__LD_B_NN(0x2);
            //         ram:b8fe cd 95 b6        CALL       sb_calc_B695                                     IN a: val
            assert!(self.call_hook(0xb695));
            //                                                                                                 b: cmp
            //                                                                                              OUT a,b
            //WRONG? OUT a,b
            //         ram:b901 4f              LD         C,A
            self.instr_hk__LD_C_A();
            //         ram:b902 d1              POP        DE
            self.instr_hk__POP_DE();
            //         ram:b903 c3 f3 b8        JP         loop_1
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(loop_1);
        }

        //                              l_exit_x_A                                      XREF[1]:     ram:b8f7(j)
        //         ram:b906 f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:b907 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:b908 ca 12 b9        JP         Z,l_exit_x
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP(l_exit_x);
            //                              l_exit_x                                        XREF[1]:     ram:b908(j)
            self.SetPC(0xb912);
            //         ram:b912 2a 6a c2        LD         HL,(DAT_ram_c26a)
            self.instr_hk__LD_HL_iNNNN(0xc26a);
            //         ram:b915 79              LD         A,C
            self.instr_hk__LD_A_C();
            //         ram:b916 2f              CPL
            self.instr_hk__CPL();
            //         ram:b917 a6              AND        (HL)
            self.instr_hk__AND_A_iHL();
            //         ram:b918 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:b919 c9              RET
            self.assert_pc(0xb919);
            true
            //
            // true
        } else {
            self.SetPC(0xb90b);
            //         ram:b90b 2a 6a c2        LD         HL,(DAT_ram_c26a)
            self.instr_hk__LD_HL_iNNNN(0xc26a);
            //         ram:b90e 79              LD         A,C
            self.instr_hk__LD_A_C();
            //         ram:b90f b6              OR         (HL)
            self.instr_hk__OR_A_iHL();
            //         ram:b910 77              LD         (HL),A
            self.instr_hk__LD_iHL_A();
            //         ram:b911 c9              RET
            self.assert_pc(0xb911);
            true
        }
    }
    pub(crate) fn hook_bcc5(&mut self) -> bool {
        //         ram:bcc5 ed 53 7f c2     LD         (DAT_ram_c27f),DE                                OUT ax <- 1,0
        self.instr_hk__LD_iNNNN_DE(0xc27f);
        //         ram:bcc9 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:bcca 11 08 00        LD         DE,0x8
        self.instr_hk__LD_DE_NNNN(0x8);
        //         ram:bccd cd ac b6        CALL       sb_calc_b6ac                                     IN de, hl
        assert!(self.call_hook(0xb6ac));
        //                                                                                              OUT de, hl
        //WRONG? OUT de, hl
        //         ram:bcd0 ed 5b 7f c2     LD         DE,(DAT_ram_c27f)
        self.instr_hk__LD_DE_iNNNN(0xc27f);
        //         ram:bcd4 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:bcd5 22 7f c2        LD         (DAT_ram_c27f),HL
        self.instr_hk__LD_iNNNN_HL(0xc27f);
        //         ram:bcd8 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:bcd9 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:bcda e6 07           AND        0x7
        self.instr_hk__AND_NN(0x7);
        //         ram:bcdc f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:bcdd 2a 7f c2        LD         HL,(DAT_ram_c27f)
        self.instr_hk__LD_HL_iNNNN(0xc27f);
        //         ram:bce0 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:bce1 6f              LD         L,A
        self.instr_hk__LD_L_A();
        //         ram:bce2 f1              POP        AF
        self.instr_hk__POP_AF();
        //                              loop_1                                          XREF[1]:     ram:bcee(j)
        loop {
            self.SetPC(0xbce3);
            //         ram:bce3 3d              DEC        A
            self.instr_hk__DEC_A();
            //         ram:bce4 fe ff           CP         0xff
            self.instr_hk__CP_NN(0xff);
            //         ram:bce6 ca f1 bc        JP         Z,l_exit_check
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) != 0 {
                // JP(l_exit_check);
                break;
            }

            //         ram:bce9 f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:bcea 7d              LD         A,L
            self.instr_hk__LD_A_L();
            //         ram:bceb 85              ADD        A,L
            self.instr_hk__ADD_A_L();
            //         ram:bcec 6f              LD         L,A
            self.instr_hk__LD_L_A();
            //         ram:bced f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:bcee c3 e3 bc        JP         loop_1
            self.IncPC(3);
            self.increase_cycles(10);
            // JP(loop_1);
        }

        self.SetPC(0xbcf1);
        //                              l_exit_check                                    XREF[1]:     ram:bce6(j)
        //         ram:bcf1 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:bcf2 e6 80           AND        0x80
        self.instr_hk__AND_NN(0x80);
        //         ram:bcf4 ca fa bc        JP         Z,l_exit_a_0
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_Z) != 0 {
            // JP(l_exit_a_0);
            self.SetPC(0xbcfa);
            //                              l_exit_a_0                                      XREF[1]:     ram:bcf4(j)
            //         ram:bcfa af              XOR        A
            self.instr_hk__XOR_A_A();
            //         ram:bcfb c9              RET
            self.assert_pc(0xbcfb);
            //
            true
        } else {
            //         ram:bcf7 3e 01           LD         A,0x1
            self.instr_hk__LD_A_NN(0x1);
            //         ram:bcf9 c9              RET
            self.assert_pc(0xbcf9);
            true
        }
    }
    pub(crate) fn hook_c000(&mut self) -> bool {
        loop {
            self.SetPC(0xc000);
            //         ram:c000 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();

            //                              LAB_ram_c001
            //         ram:c001 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c003 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c004 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //                              LAB_ram_c005+1
            //         ram:c005 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c007 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c008 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c009 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c00b 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c00c 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c00d d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c00f 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c010 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c011 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c013 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c014 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c015 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c017 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c018 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c019 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c01b 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c01c 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c01d d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c01f 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c020 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c021 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c023 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c024 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c025 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c027 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c028 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c029 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c02b 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c02c 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c02d d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c02f 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c030 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c031 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c033 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c034 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c035 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c037 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c038 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c039 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c03b 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c03c 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c03d d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c03f 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c040 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c041 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c043 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c044 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c045 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c047 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c048 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c049 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c04b 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c04c 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c04d d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c04f 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c050 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c051 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c053 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c054 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c055 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c057 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c058 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c059 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c05b 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c05c 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c05d d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c05f 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c060 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c061 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c063 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c064 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c065 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c067 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c068 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c069 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c06b 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c06c 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c06d d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c06f 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c070 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c071 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c073 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c074 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c075 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c077 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c078 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c079 d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c07b 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c07c 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c07d d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c07f 13              INC        DE
            self.instr_hk__INC_DE();
            //                              LAB_ram_c080                                         ram:4083(*),
            //         ram:c080 05              DEC        B
            self.instr_hk__DEC_B();
            //         ram:c081 c2 00 c0        JP         NZ,fn_vdp_write_c000
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP fn_vdp_write_c000;
            } else {
                break;
            }
        }
        //         ram:c084 c9              RET
        self.assert_pc(0xc084);

        true
    }
    pub(crate) fn hook_c085(&mut self) -> bool {
        //         ram:c085 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:c086 cd 9e c0        CALL       fn_vdp_set_vaddr_to_write_c09e
        assert!(self.call_hook(0xc09e));
        //                              LAB_ram_c089
        loop {
            self.SetPC(0xc089);
            //         ram:c089 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c08a d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c08c 13              INC        DE
            self.instr_hk__INC_DE();
            //         ram:c08d 0b              DEC        BC
            self.instr_hk__DEC_BC();
            //         ram:c08e 79              LD         A,C
            self.instr_hk__LD_A_C();
            //         ram:c08f b0              OR         B
            self.instr_hk__OR_A_B();
            //         ram:c090 c2 89 c0        JP         NZ,LAB_ram_c089
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP LAB_ram_c089;
            } else {
                break;
            }
        }
        //         ram:c093 c9              RET
        self.assert_pc(0xc093);
        true
    }
    pub(crate) fn hook_c094(&mut self) -> bool {
        //         ram:c094 f5              PUSH       AF
        self.instr_hk__PUSH_AF();
        //         ram:c095 cd 9e c0        CALL       fn_vdp_set_vaddr_to_write_c09e
        assert!(self.call_hook(0xc09e));
        //         ram:c098 e3              EX         (SP),HL
        self.instr_hk__EX_iSP_HL();
        //         ram:c099 e3              EX         (SP),HL
        self.instr_hk__EX_iSP_HL();
        //         ram:c09a f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:c09b d3 98           OUT        (DAT_io_0098),A
        self.instr_hk__OUT_iNN_A(DAT_IO_0098);
        //         ram:c09d c9              RET
        self.assert_pc(0xc09d);
        true
    }
    pub(crate) fn hook_c09e(&mut self) -> bool {
        //         ram:c09e 7d              LD         A,L
        self.instr_hk__LD_A_L();
        //         ram:c09f d3 99           OUT        (DAT_io_0099),A
        self.instr_hk__OUT_iNN_A(DAT_IO_0099);
        //         ram:c0a1 7c              LD         A,H
        self.instr_hk__LD_A_H();
        //         ram:c0a2 e6 3f           AND        0x3f
        self.instr_hk__AND_NN(0x3f);
        //         ram:c0a4 f6 40           OR         0x40
        self.instr_hk__OR_NN(0x40);
        //         ram:c0a6 d3 99           OUT        (DAT_io_0099),A
        self.instr_hk__OUT_iNN_A(DAT_IO_0099);
        //         ram:c0a8 c9              RET

        self.assert_pc(0xc0a8);
        true
    }
    pub(crate) fn hook_c0ba(&mut self) -> bool {
        // println!("hook_c0ba BC?{} pc:{:04x}", self.BC(), self.PC());
        // assert!(false);
        //         ram:c0ba f5              PUSH       AF                                               IN
        self.instr_hk__PUSH_AF();
        //         ram:c0bb cd 9e c0        CALL       fn_vdp_set_vaddr_to_write_c09e
        assert!(self.call_hook(0xc09e));
        //                              LAB_ram_c0be
        loop {
            self.SetPC(0xc0be);
            //         ram:c0be f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:c0bf d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_IO_0098);
            //         ram:c0c1 f5              PUSH       AF
            self.instr_hk__PUSH_AF();
            //         ram:c0c2 0b              DEC        BC
            self.instr_hk__DEC_BC();
            //         ram:c0c3 79              LD         A,C
            self.instr_hk__LD_A_C();
            //         ram:c0c4 b0              OR         B
            self.instr_hk__OR_A_B();
            //         ram:c0c5 c2 be c0        JP         NZ,LAB_ram_c0be
            // println!("  hook_c0ba BC?{} F?0x:{:04x}", self.BC(), self.data.F);
            // if self.BC() == 0 {
            //     assert!(false);
            // }
            self.IncPC(3);
            self.increase_cycles(10);
            if (self.data.F & FLAG_Z) == 0 {
                // JP LAB_ram_c0be;
            } else {
                //         ram:c0c8 f1              POP        AF
                self.instr_hk__POP_AF();
                //         ram:c0c9 c9              RET
                // println!("hook_c0ba leaving BC?{} pc:{:04x}", self.BC(), self.PC());
                // assert!(false);
                self.assert_pc(0xc0c9);
                return true;
            }
        }
    }
}
