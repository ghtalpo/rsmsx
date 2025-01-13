use crate::libs::z80::z80_base::{FLAG_C, FLAG_Z};

use super::z80_base::{join_bytes, Z80};

const SCREEN2_PATTERN_GENERATOR_TABLE_SIZE: u16 = 0x1800;
const SCREEN_2_VRAM_SPRITE_TABLE_BEGIN: u16 = 0x1b00;
const DAT_io_0098: u8 = 0x98;
const DAT_io_0099: u8 = 0x99;

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
        matches!(
            addr,
            0x46ac
                | 0x46ea
                | 0x4705
                | 0x471c
                | 0x4763
                | 0x4b61
                | 0x4c17
                | 0x6ed6
                | 0x8840
                | 0x8964
                | 0x8984
                | 0x899a
                | 0x89bc
                | 0x89c7
                | 0x89d6
                | 0x8b1b
                | 0x8b21
                | 0x8bc4
                | 0x8bca
                | 0x8bd1
                | 0x8be4
                | 0x8bea
                | 0x8bf1
                | 0xc085
                | 0xc094
                | 0xc09e
                | 0xc0ba
        )
    }
    fn call_hook_internal(&mut self, addr: u16) -> bool {
        match addr {
            0x46ac => self.hook_46ac(),
            0x46ea => self.hook_46ea(),
            0x4705 => self.hook_4705(),
            0x471c => self.hook_471c(),
            0x4763 => self.hook_4763(),
            0x4b61 => self.hook_4b61(),
            0x4c17 => self.hook_4c17(),
            0x6ed6 => self.hook_6ed6(),
            0x8840 => self.hook_8840(),
            0x8964 => self.hook_8964(),
            0x8984 => self.hook_8984(),
            0x899a => self.hook_899a(),
            0x89bc => self.hook_89bc(),
            0x89c7 => self.hook_89c7(),
            0x89d6 => self.hook_89d6(),
            0x8b1b => self.hook_8b1b(),
            0x8bc4 => self.hook_8bc4(),
            0x8b21 => self.hook_8b21(),
            0x8bca => self.hook_8bca(),
            0x8bd1 => self.hook_8bd1(),
            0x8be4 => self.hook_8be4(),
            0x8bea => self.hook_8bea(),
            0x8bf1 => self.hook_8bf1(),
            0xc085 => self.hook_c085(),
            0xc094 => self.hook_c094(),
            0xc09e => self.hook_c09e(),
            0xc0ba => self.hook_c0ba(),
            _ => false,
        }
    }
    pub(crate) fn call_hook(&mut self, addr: u16) -> bool {
        let old_pc = self.PC() + 3; // cd xx xx
        self.SetPC(addr);
        let r = self.call_hook_internal(addr);
        self.SetPC(old_pc);
        r
    }
    pub(crate) fn is_known_caller(&self, addr: u16) -> bool {
        match addr {
            0x4010..=0x422b => true, // in looped func
            0x431c..0x4403 => true,  // in looped func
            0x4a21..=0x4b60 => true, // in looped func
            0x4c5b..0x4c6d => true,  // in bios call func
            0x4c8c..0x4e51 => true,  // in bios call func
            0x4e54..0x4e61 => true,  // in looped func
            0x587b..0x6009 => true,  // in looped func
            0x61b5..0x6265 => true,  // in looped func
            0x6448..0x6650 => true,  // in looped func
            0x6c41..0x6e81 => true,  // in looped func
            0x6f48..0x7037 => true,  // in looped func
            0x8c02..=0x8c57 => true, // in bios call func
            0x8c58..=0x8cec => true, // in bios call func
            _ => false,
        }
    }
    fn hook_46ac(&mut self) -> bool {
        //         ram:46ac c5              PUSH       BC
        self.instr_hk__PUSH_BC();
        //         ram:46ad 22  be  c8       LD         (BYTE_ram_c8be ),HL
        self.instr_hk__LD_iNNNN_HL(0xc8be);
        //         ram:46b0 af              XOR        A
        self.instr_hk__XOR_A_A();
        //                              LAB_ram_46b1                                    XREF[1]:     ram:46cc (j)
        loop {
            self.SetPC(0x46b1);
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
        assert!(
            self.PC() == 0x46d3,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x46d3
        );
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
        self.instr_hk__LD_D_iHL();
        //         ram:46f7 23              INC        HL
        self.instr_hk__INC_HL();
        //         ram:46f8 5e              LD         E,(HL=>BYTE_ram_c1ef )
        self.instr_hk__LD_E_iHL();
        //         ram:46f9 c9              RET
        assert!(
            self.PC() == 0x46f9,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x46f9
        );
        true
    }

    fn hook_4705(&mut self) -> bool {
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
        assert!(
            self.PC() == 0x4711,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x4711
        );
        true
    }

    fn hook_471c(&mut self) -> bool {
        // log::info!("hook_471c");
        //         ram:471c 21  bd  c1       LD         HL,by_player_controller_c1bd
        self.instr_hk__LD_HL_NNNN(0xc1bd);
        //         ram:471f cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //         ram:4722 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:4723 c9              RET
        assert!(
            self.PC() == 0x4723,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x4723
        );
        true
    }
    fn hook_4763(&mut self) -> bool {
        // log::info!("hook_4763");
        //         ram:4763 3a  1b  c2       LD         A,(bt_player_idx_c21b )                          hl <- hl + player_idx
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:4766 4f              LD         C,A
        self.instr_hk__LD_C_A();
        //         ram:4767 06  00           LD         B,0x0
        self.instr_hk__LD_B_NN(0x0);
        //         ram:4769 09              ADD        HL,BC
        self.instr_hk__ADD_HL_BC();
        //         ram:476a c9              RET
        assert!(
            self.PC() == 0x476a,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x476a
        );
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
        assert!(
            self.PC() == 0x481d,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x481d
        );
        true
    }
    fn hook_4c17(&mut self) -> bool {
        // println!("hook_4c17");
        //         ram:4c17 d5              PUSH       DE                                               IN bc:wh?
        self.instr_hk__PUSH_DE();
        //                                                                                                 de:origin?
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
        //         ram:4c1e cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
        assert!(self.call_hook(0x89d6));
        //                                                                                                 de: xy?
        //                                                                                              OUT d: d+1
        //                              LAB_ram_4c21                                    XREF[1]:     ram:4c26 (j)
        loop {
            self.SetPC(0x4c21);
            //         ram:4c21 3e  26           LD         A,'&'
            self.instr_hk__LD_A_NN(0x26);
            //         ram:4c23 cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
            assert!(self.call_hook(0x89d6));
            //                                                                                                 de: xy?
            //                                                                                              OUT d: d+1
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
        //         ram:4c2a cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
        assert!(self.call_hook(0x89d6));
        //                                                                                                 de: xy?
        //                                                                                              OUT d: d+1
        //         ram:4c2d c1              POP        BC
        self.instr_hk__POP_BC();
        //         ram:4c2e d1              POP        DE
        self.instr_hk__POP_DE();
        //                              LAB_ram_4c2f                                    XREF[1]:     ram:4c46 (j)
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
            //         ram:4c34 cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
            assert!(self.call_hook(0x89d6));
            //                                                                                                 de: xy?
            //                                                                                              OUT d: d+1
            //                              LAB_ram_4c37                                    XREF[1]:     ram:4c3c (j)
            loop {
                self.SetPC(0x4c37);
                //         ram:4c37 3e  20           LD         A,' '
                self.instr_hk__LD_A_NN(0x20);
                //         ram:4c39 cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
                assert!(self.call_hook(0x89d6));
                //                                                                                                 de: xy?
                //                                                                                              OUT d: d+1
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
            //         ram:4c40 cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
            assert!(self.call_hook(0x89d6));
            //                                                                                                 de: xy?
            //                                                                                              OUT d: d+1
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
        //         ram:4c4b cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
        assert!(self.call_hook(0x89d6));
        //                                                                                                 de: xy?
        //                                                                                              OUT d: d+1
        //                              LAB_ram_4c4e                                    XREF[1]:     ram:4c53 (j)
        assert!(
            self.PC() == 0x4c4e,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x4c4e
        );
        loop {
            self.SetPC(0x4c4e);
            //         ram:4c4e 3e  26           LD         A,'&'
            self.instr_hk__LD_A_NN(0x26);
            //         ram:4c50 cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
            assert!(self.call_hook(0x89d6));
            //                                                                                                 de: xy?
            //                                                                                              OUT d: d+1
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
        //         ram:4c57 cd  d6  89       CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
        assert!(self.call_hook(0x89d6));
        //                                                                                                 de: xy?
        //                                                                                              OUT d: d+1
        //         ram:4c5a c9              RET
        //
        assert!(
            self.PC() == 0x4c5a,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x4c5a
        );
        true
    }
    fn hook_4b61(&mut self) -> bool {
        //         ram:4b61 21 08 28        LD         HL,0x2808                                        IN a: val
        self.instr_hk__LD_HL_NNNN(0x2808);
        //         ram:4b64 01 30 02        LD         BC,0x230
        self.instr_hk__LD_BC_NNNN(0x230);
        //         ram:4b67 cd ba c0        CALL       sb_fill_vram_guess_c0ba                          IN
        assert!(self.call_hook(0xc0ba));
        //         ram:4b6a c9              RET
        assert!(
            self.PC() == 0x4b6a,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x4b6a
        );
        true
    }
    fn hook_6ed6(&mut self) -> bool {
        //         ram:6ed6 ed 53 3c c2     LD         (DAT_ram_c23c),DE
        self.instr_hk__LD_iNNNN_DE(0xc23c);
        //         ram:6eda cd 05 47        CALL       sb_read_mem_for_player_4705                      HL <- c349 + player_idx + 3
        assert!(self.call_hook(0x4705));
        //         ram:6edd 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:6ede b7              OR         A
        self.instr_hk__OR_A_A();
        //         ram:6edf c8              RET        Z
        self.IncPC(1);
        if (self.data.F & FLAG_Z) != 0 {
            self.increase_cycles(11);
            assert!(
                self.PC() == 0x6ee0,
                "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
                self.PC(),
                0x6ee0
            );
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
        //         ram:6efc cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21                   IN a, e
        assert!(self.call_hook(0x8b21));
        //         ram:6eff d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:6f00 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:6f03 c6 03           ADD        A,0x3
        self.instr_hk__ADD_A_NN(0x3);
        //         ram:6f05 0e 01           LD         C,0x1
        self.instr_hk__LD_C_NN(0x1);
        //         ram:6f07 cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21                   IN a, e
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
        //         ram:6f1d cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21                   IN a, e
        assert!(self.call_hook(0x8b21));
        //         ram:6f20 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:6f21 3a 1b c2        LD         A,(bt_player_idx_c21b)
        self.instr_hk__LD_A_iNNNN(0xc21b);
        //         ram:6f24 c6 03           ADD        A,0x3
        self.instr_hk__ADD_A_NN(0x3);
        //         ram:6f26 0e 00           LD         C,0x0
        self.instr_hk__LD_C_NN(0x0);
        //         ram:6f28 cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21                   IN a, e
        assert!(self.call_hook(0x8b21));
        //         ram:6f2b fb              EI
        self.instr_hk__EI();
        //         ram:6f2c c9              RET
        //
        assert!(
            self.PC() == 0x6f2c,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x6f2c
        );
        true
    }
    fn hook_8840(&mut self) -> bool {
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
        //
        assert!(
            self.PC() == 0x885f,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x885f
        );
        true
    }
    fn hook_8964(&mut self) -> bool {
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
        assert!(
            self.PC() == 0x8983,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8983
        );
        true
    }
    fn hook_8984(&mut self) -> bool {
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
        assert!(
            self.PC() == 0x8999,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8999
        );
        true
    }
    fn hook_899a(&mut self) -> bool {
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
        //         ram:89a2 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085                   IN bc: count
        assert!(self.call_hook(0xC085));
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
        //         ram:89b5 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085                   IN bc: count
        assert!(self.call_hook(0xC085));
        //         ram:89b8 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:89b9 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:89ba fb              EI
        self.instr_hk__EI();
        //         ram:89bb c9              RET
        assert!(
            self.PC() == 0x89bb,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x89bb
        );
        true
    }
    fn hook_89bc(&mut self) -> bool {
        //         ram:89bc 5c              LD         E,H                                              hl <- (hl & 0xff) * 20 + (hl >> 8)
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
        //
        assert!(
            self.PC() == 0x89c6,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x89c6
        );
        true
    }
    fn hook_89c7(&mut self) -> bool {
        //         ram:89c7 d5              PUSH       DE                                               IN de: pos
        self.instr_hk__PUSH_DE();
        //                              loop                                            XREF[1]:     ram:89d1(j)
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
            //         ram:89ce cd d6 89        CALL       fn_putchar_xy_89d6                               IN a: char(not ascii?)
            assert!(self.call_hook(0x89d6));
            //         ram:89d1 c3 c8 89        JP         loop
            self.IncPC(3);
            self.increase_cycles(10); //JP loop;
        }
        //                              l_exit                                          XREF[1]:     ram:89cb(j)
        self.SetPC(0x89d4);
        //         ram:89d4 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:89d5 c9              RET
        assert!(
            self.PC() == 0x89d5,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x89d5
        );
        true
    }
    fn hook_89d6(&mut self) -> bool {
        //         ram:89d6 fe a0           CP         0xa0                                             IN a: char(not ascii?)
        self.instr_hk__CP_NN(0xa0);
        //         ram:89d8 d2 e0 89        JP         NC,LAB_ram_89e0
        self.IncPC(3);
        self.increase_cycles(10);
        if (self.data.F & FLAG_C) == 0 {
            // JP LAB_ram_89e0;
            self.SetPC(0x89e0);
            //                              LAB_ram_89e0                                    XREF[1]:     ram:89d8(j)
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
        //                              LAB_ram_89e2                                    XREF[1]:     ram:89dd(j)
        //         ram:89e2 e5              PUSH       HL
        self.instr_hk__PUSH_HL();
        //         ram:89e3 d5              PUSH       DE
        self.instr_hk__PUSH_DE();
        //         ram:89e4 eb              EX         DE,HL
        self.instr_hk__EX_DE_HL();
        //         ram:89e5 cd bc 89        CALL       fn_calc_voffset_89BC                             hl <- (hl & 0xff) * 20 + (hl >> 8)
        assert!(self.call_hook(0x89BC));
        //         ram:89e8 11 00 18        LD         DE,SCREEN2_PATTERN_GENERATOR_TABLE_SIZE
        self.instr_hk__LD_DE_NNNN(SCREEN2_PATTERN_GENERATOR_TABLE_SIZE);
        //         ram:89eb 19              ADD        HL,DE
        self.instr_hk__ADD_HL_DE();
        //         ram:89ec f3              DI
        self.instr_hk__DI();
        //         ram:89ed cd 94 c0        CALL       sb_vram_write_1_byte_C094                        IN hl: vram addr?
        assert!(self.call_hook(0xC094));
        //         ram:89f0 d1              POP        DE
        self.instr_hk__POP_DE();
        //         ram:89f1 e1              POP        HL
        self.instr_hk__POP_HL();
        //         ram:89f2 14              INC        D
        self.instr_hk__INC_D();
        //         ram:89f3 fb              EI
        self.instr_hk__EI();
        //         ram:89f4 c9              RET
        assert!(
            self.PC() == 0x89f4,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x89f4
        );
        true
    }
    fn hook_8ac9(&mut self) -> bool {
        println!("hook_8ac9");
        //         ram:8ac9 21 1a cc        LD         HL,font_cc1a                                     IN a: char_class
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
        //                              LAB_ram_8ad5                                    XREF[1]:     ram:8ace(j)
        //         ram:8ad5 11 00 02        LD         DE,0x200
        self.instr_hk__LD_DE_NNNN(0x200);
        //         ram:8ad8 cb b8           RES        0x7,B
        self.instr_hk__RES_7_B();
        //         ram:8ada b7              OR         A
        self.instr_hk__OR_A_A();
        //                              loop_1                                          XREF[1]:     ram:8ae0(j)
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

        //                              LAB_ram_8ae3                                    XREF[1]:     ram:8adb(j)
        //         ram:8ae3 11 20 00        LD         DE,0x20
        self.instr_hk__LD_DE_NNNN(0x20);
        //         ram:8ae6 78              LD         A,B
        self.instr_hk__LD_A_B();
        //         ram:8ae7 b7              OR         A
        self.instr_hk__OR_A_A();
        //                              loop_2                                          XREF[1]:     ram:8aed(j)
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
        //                              LAB_ram_8af0                                    XREF[1]:     ram:8ae8(j)
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
        //         ram:8b05 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085                   IN bc: count
        assert!(self.call_hook(0xC085));
        //                                                                                                  de: targe vram addr
        //                                                                                                  hl: source addr
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
        //         ram:8b16 cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085                   IN bc: count
        assert!(self.call_hook(0xC085));
        //         ram:8b19 fb              EI
        self.instr_hk__EI();
        //         ram:8b1a c9              RET
        assert!(
            self.PC() == 0x8b1a,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8b1a
        );
        true
    }
    fn hook_8b1b(&mut self) -> bool {
        //         ram:8b1b f3              DI                                                          IN a,c,e
        self.instr_hk__DI();
        //         ram:8b1c cd 21 8b        CALL       sb_blit_ram_to_vram_guess_8b21                   IN a, e
        assert!(self.call_hook(0x8b21));
        //         ram:8b1f fb              EI
        self.instr_hk__EI();
        //         ram:8b20 c9              RET
        assert!(
            self.PC() == 0x8b20,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8b20
        );
        true
    }
    fn hook_8b21(&mut self) -> bool {
        //         ram:8b21 06 00           LD         B,0x0                                            IN a, e
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
        //                              loop                                            XREF[1]:     ram:8b3b(j)
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
        //                              l_exit_x                                        XREF[1]:     ram:8b36(j)
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
        //         ram:8b4d cd 85 c0        CALL       sb_blit_ram_to_vram_guess_C085                   IN bc: count
        assert!(self.call_hook(0xC085));
        //         ram:8b50 c9              RET
        assert!(
            self.PC() == 0x8b50,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8b50
        );
        true
    }
    fn hook_8bc4(&mut self) -> bool {
        //         ram:8bc4 cd  ca  8b       CALL       sb_read_mem_for_player_8BCA                      hl <- *c290 + *c21b
        assert!(self.call_hook(0x8BCA));
        //         ram:8bc7 7e              LD         A,(HL)
        self.instr_hk__LD_A_iHL();
        //         ram:8bc8 70              LD         (HL),B
        self.instr_hk__LD_iHL_B();
        //         ram:8bc9 c9              RET
        assert!(
            self.PC() == 0x8bc9,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8bc9
        );
        true
    }
    fn hook_8bca(&mut self) -> bool {
        //         ram:8bca 21  90  c2       LD         HL,DAT_ram_c290                                  hl <- *c290 + *c21b
        self.instr_hk__LD_HL_NNNN(0xc290);
        //         ram:8bcd cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //         ram:8bd0 c9              RET
        assert!(
            self.PC() == 0x8bd0,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8bd0
        );
        true
    }
    fn hook_8bd1(&mut self) -> bool {
        // log::info!("hook_8bd1");
        //         ram:8bd1 21  a2  c2       LD         HL,0xc2a2
        self.instr_hk__LD_HL_NNNN(0xc2a2);
        //         ram:8bd4 cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //         ram:8bd7 c9              RET
        assert!(
            self.PC() == 0x8bd7,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8bd7
        );
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
        assert!(
            self.PC() == 0x8be9,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8be9
        );
        true
    }
    fn hook_8bea(&mut self) -> bool {
        // log::info!("hook_8bea");
        //         ram:8bea 21  93  c2       LD         HL,DAT_ram_c293
        self.instr_hk__LD_HL_NNNN(0xc293);
        //         ram:8bed cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //         ram:8bf0 c9              RET
        assert!(
            self.PC() == 0x8bf0,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0x8bf0
        );
        true
    }
    fn hook_8bf1(&mut self) -> bool {
        // log::info!("hook_8bf1");
        //         ram:8bf1 21  a5  c2       LD         HL,DAT_ram_c2a5
        self.instr_hk__LD_HL_NNNN(0xc2a5);
        //         ram:8bf4 cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //         ram:8bf7 c9              RET
        true
    }
    fn hook_c085(&mut self) -> bool {
        //         ram:c085 eb              EX         DE,HL                                            IN bc: count
        self.instr_hk__EX_DE_HL();
        //         ram:c086 cd 9e c0        CALL       fn_vdp_set_vaddr_to_write_c09e                   IN hl <- vram addr?
        assert!(self.call_hook(0xc09e));
        //                              LAB_ram_c089                                    XREF[1]:     ram:c090(j)
        loop {
            self.SetPC(0xc089);
            //         ram:c089 1a              LD         A,(DE)
            self.instr_hk__LD_A_iDE();
            //         ram:c08a d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_io_0098);
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
        assert!(
            self.PC() == 0xc093,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0xc093
        );
        true
    }
    fn hook_c094(&mut self) -> bool {
        //         ram:c094 f5              PUSH       AF                                               IN hl: vram addr?
        self.instr_hk__PUSH_AF();
        //         ram:c095 cd 9e c0        CALL       fn_vdp_set_vaddr_to_write_c09e                   IN hl <- vram addr?
        assert!(self.call_hook(0xc09e));
        //         ram:c098 e3              EX         (SP),HL
        self.instr_hk__EX_iSP_HL();
        //         ram:c099 e3              EX         (SP),HL
        self.instr_hk__EX_iSP_HL();
        //         ram:c09a f1              POP        AF
        self.instr_hk__POP_AF();
        //         ram:c09b d3 98           OUT        (DAT_io_0098),A
        self.instr_hk__OUT_iNN_A(DAT_io_0098);
        //         ram:c09d c9              RET
        assert!(
            self.PC() == 0xc09d,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0xc09d
        );
        true
    }
    fn hook_c09e(&mut self) -> bool {
        //         ram:c09e 7d              LD         A,L                                              IN hl <- vram addr?
        self.instr_hk__LD_A_L();
        //         ram:c09f d3 99           OUT        (DAT_io_0099),A
        self.instr_hk__OUT_iNN_A(DAT_io_0099);
        //         ram:c0a1 7c              LD         A,H
        self.instr_hk__LD_A_H();
        //         ram:c0a2 e6 3f           AND        0x3f
        self.instr_hk__AND_NN(0x3f);
        //         ram:c0a4 f6 40           OR         0x40
        self.instr_hk__OR_NN(0x40);
        //         ram:c0a6 d3 99           OUT        (DAT_io_0099),A
        self.instr_hk__OUT_iNN_A(DAT_io_0099);
        //         ram:c0a8 c9              RET
        //
        assert!(
            self.PC() == 0xc0a8,
            "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
            self.PC(),
            0xc0a8
        );
        true
    }
    fn hook_c0ba(&mut self) -> bool {
        // println!("hook_c0ba BC?{} pc:{:04x}", self.BC(), self.PC());
        // assert!(false);
        //         ram:c0ba f5              PUSH       AF                                               IN
        self.instr_hk__PUSH_AF();
        //         ram:c0bb cd 9e c0        CALL       fn_vdp_set_vaddr_to_write_c09e                   IN hl <- vram addr?
        assert!(self.call_hook(0xc09e));
        //                              LAB_ram_c0be                                    XREF[1]:     ram:c0c5(j)
        loop {
            self.SetPC(0xc0be);
            //         ram:c0be f1              POP        AF
            self.instr_hk__POP_AF();
            //         ram:c0bf d3 98           OUT        (DAT_io_0098),A
            self.instr_hk__OUT_iNN_A(DAT_io_0098);
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
                assert!(
                    self.PC() == 0xc0c9,
                    "cur.pc:0x{:04x} ~= tgt.pc:0x{:04x}",
                    self.PC(),
                    0xc0c9
                );
                return true;
            }
        }
    }
}
