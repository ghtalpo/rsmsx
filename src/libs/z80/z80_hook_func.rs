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
            0x471c | 0x4763 | 0x8bd1 | 0x8be4 | 0x8bea | 0x8bf1 => true,
            _ => false,
        }
    }
    pub(crate) fn call_hook(&mut self, addr: u16) -> bool {
        match addr {
            0x471c => self.hook_471c(),
            0x4763 => self.hook_4763(),
            0x8bd1 => self.hook_8bd1(),
            0x8be4 => self.hook_8be4(),
            0x8bea => self.hook_8bea(),
            0x8bf1 => self.hook_8bf1(),
            _ => false,
        }
    }
    pub(crate) fn is_known_caller(&self, addr: u16) -> bool {
        match addr {
            0x5a4d => true, // loop
            0x8c72 => true, // on tick
            _ => false,
        }
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
        log::info!("hook_4801");
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
    fn hook_8bd1(&mut self) -> bool {
        log::info!("hook_8bd1");
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
        log::info!("hook_8be4");
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
        log::info!("hook_8bea");
        //         ram:8bea 21  93  c2       LD         HL,DAT_ram_c293
        self.instr_hk__LD_HL_NNNN(0xc293);
        //         ram:8bed cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //                                                                                              bc <- player_idx
        //         ram:8bf0 c9              RET
        true
    }
    fn hook_8bf1(&mut self) -> bool {
        log::info!("hook_8bf1");
        //         ram:8bf1 21  a5  c2       LD         HL,DAT_ram_c2a5
        self.instr_hk__LD_HL_NNNN(0xc2a5);
        //         ram:8bf4 cd  63  47       CALL       fn_add_player_idx_to_addr_4763                   hl <- hl + player_idx
        assert!(self.call_hook(0x4763));
        //                                                                                              bc <- player_idx
        //         ram:8bf7 c9              RET
        true
    }
}
