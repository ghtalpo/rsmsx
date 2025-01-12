use crate::libs::z80::z80_base::FLAG_Z;

use super::z80_base::{join_bytes, Z80};

#[allow(non_snake_case, dead_code)]
impl Z80 {
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

    pub(crate) fn has_hook(&self, addr: u16) -> bool {
        match addr {
            0x46ac | 0x46ea | 0x4705 | 0x471c | 0x4763 | 0x8bc4 | 0x8bca | 0x8bd1 | 0x8be4
            | 0x8bea | 0x8bf1 => true,
            _ => false,
        }
    }
    pub(crate) fn call_hook(&mut self, addr: u16) -> bool {
        match addr {
            0x46ac => self.hook_46ac(),
            0x46ea => self.hook_46ea(),
            0x4705 => self.hook_4705(),
            0x471c => self.hook_471c(),
            0x4763 => self.hook_4763(),
            0x8bc4 => self.hook_8bc4(),
            0x8bca => self.hook_8bca(),
            0x8bd1 => self.hook_8bd1(),
            0x8be4 => self.hook_8be4(),
            0x8bea => self.hook_8bea(),
            0x8bf1 => self.hook_8bf1(),
            _ => false,
        }
    }
    pub(crate) fn is_known_caller(&self, addr: u16) -> bool {
        match addr {
            0x431c..0x4403 => true,  // in looped func
            0x4e54..0x4e61 => true,  // in looped func
            0x587b..0x6009 => true,  // in looped func
            0x61b5..0x6265 => true,  // in looped func
            0x6448..0x6650 => true,  // in looped func
            0x6c41..0x6e81 => true,  // in looped func
            0x6f48..0x7037 => true,  // in looped func
            0x8c02..=0x8c57 => true, // in bios call func
            0x8c72 => true,          // on tick
            _ => false,
        }
    }
    fn hook_46ac(&mut self) -> bool {
        //
        //                              *************************************************************
        //                              *                           FUNCTION
        //                              *************************************************************
        //                              undefined  FUN_ram_46ac ()
        //              undefined         A:1            <RETURN>
        // self.instr_hk__RET();
        //                              FUN_ram_46ac                                    XREF[4]:     ram:45f8 (c) , ram:4608 (c) ,
        //                                                                                           ram:4618 (c) , ram:4628 (c)
        //         ram:46ac c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:46ad 22  be  c8       LD         (BYTE_ram_c8be ),HL
        self.instr_hk__LD_iNNNN_HL(0xc8be);
        //         ram:46b0 af              XOR        A
        self.instr_hk__XOR_A_A();
        //                              LAB_ram_46b1                                    XREF[1]:     ram:46cc (j)
        loop {
            //         ram:46b1 32  1b  c2       LD         (bt_player_idx_c21b ),A
            self.instr_hk__LD_iNNNN_A(0xc21b);
            //         ram:46b4 cd  05  47       CALL       sb_read_mem_for_player_4705                      undefined sb_read_mem_for_player
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
                    //                              LAB_ram_46d4                                    XREF[1]:     ram:46c4 (j)
                    //         ram:46d4 2a  be  c8       LD         HL,(BYTE_ram_c8be )
                    self.instr_hk__LD_HL_iNNNN(0xc8be);
                    //         ram:46d7 c1              POP        BC
                    self.instr_hk__POP_BC();
                    //         ram:46d8 37              SCF
                    self.instr_hk__SCF();
                    //         ram:46d9 c9              RET
                    // self.instr_hk__RET();
                    //
                    return true;
                } else {
                    // JR 46c6
                }
            }
            //                              LAB_ram_46c6                                    XREF[1]:     ram:46b9 (j)
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
        //         ram:46ce 2a  be  c8       LD         HL,(BYTE_ram_c8be )
        self.instr_hk__LD_HL_iNNNN(0xc8be);
        //         ram:46d1 c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:46d2 b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:46d3 c9              RET
        // self.instr_hk__RET();
        true
    }

    fn hook_46ea(&mut self) -> bool {
        //         ram:46ea 21  ee  c1       LD         HL,BYTE_ram_c1ee                                 OUT d,e
        self.instr_hk__LD_HL_NNNN(0xc1ee);
        //                              LAB_ram_46ed                                    XREF[1]:     sb_read_mem_for_player_46fa:46fd
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
        self.instr_hk__LD_D_iNNNN(0xc1ee);
        //         ram:46f7 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:46f8 5e              LD         E,(HL=>BYTE_ram_c1ef )
        self.instr_hk__LD_E_iNNNN(0xc1ef);
        //         ram:46f9 c9              RET
        //
        true
    }

    fn hook_4705(&mut self) -> bool {
        //
        //         *************************************************************
        //     *                           FUNCTION
        //     *************************************************************
        //     undefined  sb_read_mem_for_player_4705 ()
        // undefined         A:1            <RETURN>
        // self.instr_hk__RET();
        //     sb_read_mem_for_player_4705                     XREF[9]:     ram:4324 (c) ,
        //                                                                  FUN_ram_46ac:46b4 (c) ,
        //                                                                  FUN_ram_481e:489f (c) ,
        //                                                                  FUN_ram_49a1:49e2 (c) ,
        //                                                                  FUN_ram_61b5:646a (c) ,
        //                                                                  FUN_ram_61b5:64bb (c) ,
        //                                                                  ram:66f4 (c) , ram:6739 (c) ,
        //                                                                  FUN_ram_6ed6:6eda (c)
        // ram:4705 3a  1b  c2       LD         A,(bt_player_idx_c21b )                          HL <- c349 + player_idx + 3
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
        //
        true
    }

    fn hook_471c(&mut self) -> bool {
        // log::info!("hook_471c");
        //         ram:471c 21  bd  c1       LD         HL,by_player_controller_c1bd
        self.instr_hk__LD_HL_NNNN(0xc1bd);
        //         ram:471f cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //                                                                                              bc <- player_idx
        //         ram:4722 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:4723 c9              RET
        true
    }
    fn hook_4763(&mut self) -> bool {
        // log::info!("hook_4763");
        //         ram:4763 3a  1b  c2       LD         A,(bt_player_idx_c21b )                          hl <- hl + player_idx
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //                                                                                              bc <- player_idx
        //         ram:4766 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:4767 06  00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:4769 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:476a c9              RET
        //
        true
    }
    fn hook_4801(&mut self) -> bool {
        // log::info!("hook_4801");
        self.instr_hk__LD_C_NN(0xa);
        // ram:4803 18  12           JR         sb_get_char_internal_4817                        undefined sb_get_char_internal_4
        //         ram:4817 06  00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:4819 2a  54  c2       LD         HL,(pt_char_c254 )
        self.instr_hk__LD_HL_iNNNN(0xc254);
        //         ram:481c 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:481d c9              RET
        true
    }
    fn hook_8bc4(&mut self) -> bool {
        //         ram:8bc4 cd  ca  8b       CALL       sb_read_mem_for_player_8BCA                      hl <- *c290 + *c21b
        assert!(self.call_hook(0x8BCA));
        //                                                                                              b <- 0
        //                                                                                              c <- *21b
        //         ram:8bc7 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8bc8 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:8bc9 c9              RET
        //
        true
    }
    fn hook_8bca(&mut self) -> bool {
        //         ram:8bca 21  90  c2       LD         HL,DAT_ram_c290                                  hl <- *c290 + *c21b
        self.instr_hk__LD_HL_NNNN(0xc290);
        //                                                                                              c <- *21b
        //         ram:8bcd cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //                                                                                              bc <- player_idx
        //         ram:8bd0 c9              RET
        true
    }
    fn hook_8bd1(&mut self) -> bool {
        // log::info!("hook_8bd1");
        //         ram:8bd1 21  a2  c2       LD         HL,0xc2a2
        self.instr_hk__LD_HL_NNNN(0xc2a2);
        //         ram:8bd4 cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //                                                                                              bc <- player_idx
        //         ram:8bd7 c9              RET
        //
        true
    }
    fn hook_8be4(&mut self) -> bool {
        // log::info!("hook_8be4");
        //         ram:8be4 cd  ea  8b       CALL       sb_read_mem_for_player_8bea                      undefined sb_read_mem_for_player
        assert!(self.call_hook(0x8bea));
        //         ram:8be7 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8be8 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:8be9 c9              RET
        //
        true
    }
    fn hook_8bea(&mut self) -> bool {
        // log::info!("hook_8bea");
        //         ram:8bea 21  93  c2       LD         HL,DAT_ram_c293
        self.instr_hk__LD_HL_NNNN(0xc293);
        //         ram:8bed cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //                                                                                              bc <- player_idx
        //         ram:8bf0 c9              RET
        true
    }
    fn hook_8bf1(&mut self) -> bool {
        // log::info!("hook_8bf1");
        //         ram:8bf1 21  a5  c2       LD         HL,DAT_ram_c2a5
        self.instr_hk__LD_HL_NNNN(0xc2a5);
        //         ram:8bf4 cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //                                                                                              bc <- player_idx
        //         ram:8bf7 c9              RET
        true
    }
}
