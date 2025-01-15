use super::z80_base::Z80;

#[allow(non_snake_case, dead_code)]
impl Z80 {
    // 0x8a21 |
    pub(crate) fn has_hook(&self, addr: u16) -> bool {
        match addr {
            0x42ba | 0x4453 | 0x44b6 | 0x46ac | 0x46ea | 0x46fa | 0x4700 | 0x4705 | 0x471c
            | 0x473d | 0x4747 | 0x4751 | 0x4760 | 0x4763 | 0x476b | 0x4778 | 0x4787 | 0x4797
            | 0x47ab | 0x47b0 | 0x47c6 | 0x47da | 0x47f1 | 0x47f5 | 0x47f9 | 0x4801 | 0x4809
            | 0x480d | 0x4811 | 0x4815 | 0x49a1 | 0x4b61 | 0x4c17 | 0x4fce | 0x5194 | 0x53fb
            | 0x5445 | 0x547c | 0x5491 | 0x54a9 | 0x562d | 0x5647 | 0x566f | 0x5687 | 0x600a
            | 0x606f | 0x6079 | 0x60db | 0x60e5 | 0x67f7 | 0x69ac | 0x69e3 | 0x6a81 | 0x6b4a
            | 0x6bba | 0x6ed6 | 0x7335 | 0x75d2 | 0x7f80 | 0x8018 | 0x8097 | 0x80c0 | 0x80ea
            | 0x8115 | 0x8140 | 0x81ec | 0x823d | 0x825d | 0x82d7 | 0x8559 | 0x859e | 0x85c8
            | 0x8824 | 0x882f | 0x8840 | 0x8860 | 0x88ba | 0x894c | 0x8959 | 0x8964 | 0x8984
            | 0x899a | 0x89bc | 0x89c7 | 0x89d6 | 0x89f5 | 0x8a60 | 0x8a6f | 0x8a86 | 0x8a92
            | 0x8a9e | 0x8ac9 | 0x8b1b | 0x8b21 | 0x8b6c | 0x8b72 | 0x8baf | 0x8bc4 | 0x8bca
            | 0x8bd1 | 0x8be4 | 0x8bea | 0x8bf1 | 0xae82 | 0xaec4 | 0xaef5 | 0xb181 | 0xb191
            | 0xb260 | 0xb34c | 0xb35d | 0xb387 | 0xb392 | 0xb60e | 0xb634 | 0xb64c | 0xb695
            | 0xb6ac | 0xb6cd | 0xb6f1 | 0xb74f | 0xb7a9 | 0xb79b | 0xb79f | 0xb7bd | 0xb825
            | 0xb8b4 | 0xb8c3 | 0xb8d6 | 0xbcc5 | 0xc000 | 0xc085 | 0xc094 | 0xc09e | 0xc0ba => {
                true
            }
            _ => false,
        }
    }
    fn call_hook_internal(&mut self, addr: u16) -> bool {
        match addr {
            0x42ba => self.hook_42ba(),
            // 0x4308 => self.hook_4308(),
            0x4453 => self.hook_4453(),
            0x44b6 => self.hook_44b6(),
            0x46ac => self.hook_46ac(),
            0x46ea => self.hook_46ea(),
            0x46fa => self.hook_46fa(),
            0x4700 => self.hook_4700(),
            0x4705 => self.hook_4705(),
            0x471c => self.hook_471c(),
            0x473d => self.hook_473d(),
            0x4747 => self.hook_4747(),
            0x4751 => self.hook_4751(),
            0x4760 => self.hook_4760(),
            0x4763 => self.hook_4763(),
            0x476b => self.hook_476b(),
            0x4778 => self.hook_4778(),
            0x4787 => self.hook_4787(),
            0x4797 => self.hook_4797(),
            0x47ab => self.hook_47ab(),
            0x47b0 => self.hook_47b0(),
            0x47c6 => self.hook_47c6(),
            0x47da => self.hook_47da(),
            0x47f1 => self.hook_47f1(),
            0x47f5 => self.hook_47f5(),
            0x47f9 => self.hook_47f9(),
            0x4801 => self.hook_4801(),
            0x4809 => self.hook_4809(),
            0x480d => self.hook_480d(),
            0x4811 => self.hook_4811(),
            0x4815 => self.hook_4815(),
            0x49a1 => self.hook_49a1(),
            0x4b61 => self.hook_4b61(),
            0x4c17 => self.hook_4c17(),
            0x4fce => self.hook_4fce(),
            0x5194 => self.hook_5194(),
            0x53fb => self.hook_53fb(),
            0x5445 => self.hook_5445(),
            0x547c => self.hook_547c(),
            0x5491 => self.hook_5491(),
            0x54a9 => self.hook_54a9(),
            0x5647 => self.hook_5647(),
            0x562d => self.hook_562d(),
            0x566f => self.hook_566f(),
            0x5687 => self.hook_5687(),
            0x600a => self.hook_600a(),
            0x606f => self.hook_606f(),
            0x6079 => self.hook_6079(),
            0x60db => self.hook_60db(),
            0x60e5 => self.hook_60e5(),
            0x67f7 => self.hook_67f7(),
            0x69ac => self.hook_69ac(),
            0x69e3 => self.hook_69e3(),
            0x6a81 => self.hook_6a81(),
            // 0x6acb => self.hook_6acb(),
            // 0x6b23 => self.hook_6b23(),
            0x6b4a => self.hook_6b4a(),
            0x6bba => self.hook_6bba(),
            0x6ed6 => self.hook_6ed6(),
            0x7335 => self.hook_7335(),
            0x75d2 => self.hook_75d2(),
            0x7f80 => self.hook_7f80(),
            0x8018 => self.hook_8018(),
            0x8097 => self.hook_8097(),
            0x80c0 => self.hook_80c0(),
            0x80ea => self.hook_80ea(),
            0x8115 => self.hook_8115(),
            0x8140 => self.hook_8140(),
            0x81ec => self.hook_81ec(),
            0x823d => self.hook_823d(),
            0x825d => self.hook_825d(),
            0x82d7 => self.hook_82d7(),
            0x8559 => self.hook_8559(),
            0x859e => self.hook_859e(),
            0x85c8 => self.hook_85c8(),
            0x8824 => self.hook_8824(),
            0x882f => self.hook_882f(),
            0x8840 => self.hook_8840(),
            0x8860 => self.hook_8860(),
            0x88ba => self.hook_88ba(),
            0x894c => self.hook_894c(),
            0x8959 => self.hook_8959(),
            0x8964 => self.hook_8964(),
            0x8984 => self.hook_8984(),
            0x899a => self.hook_899a(),
            0x89bc => self.hook_89bc(),
            0x89c7 => self.hook_89c7(),
            0x89d6 => self.hook_89d6(),
            0x89f5 => self.hook_89f5(),
            // 0x8a21 => self.hook_8a21(),
            0x8a60 => self.hook_8a60(),
            0x8a6f => self.hook_8a6f(),
            0x8a86 => self.hook_8a86(),
            0x8a92 => self.hook_8a92(),
            0x8a9e => self.hook_8a9e(),
            0x8ac9 => self.hook_8ac9(),
            0x8b1b => self.hook_8b1b(),
            0x8b21 => self.hook_8b21(),
            0x8b6c => self.hook_8b6c(),
            0x8b72 => self.hook_8b72(),
            0x8baf => self.hook_8baf(),
            0x8bc4 => self.hook_8bc4(),
            0x8bca => self.hook_8bca(),
            0x8bd1 => self.hook_8bd1(),
            0x8be4 => self.hook_8be4(),
            0x8bea => self.hook_8bea(),
            0x8bf1 => self.hook_8bf1(),
            0xae82 => self.hook_ae82(),
            0xaec4 => self.hook_aec4(),
            0xaef5 => self.hook_aef5(),
            0xb181 => self.hook_b181(),
            0xb191 => self.hook_b191(),
            0xb260 => self.hook_b260(),
            0xb34c => self.hook_b34c(),
            0xb35d => self.hook_b35d(),
            0xb387 => self.hook_b387(),
            0xb392 => self.hook_b392(),
            0xb60e => self.hook_b60e(),
            0xb634 => self.hook_b634(),
            0xb64c => self.hook_b64c(),
            0xb695 => self.hook_b695(),
            0xb6ac => self.hook_b6ac(),
            0xb6cd => self.hook_b6cd(),
            0xb6f1 => self.hook_b6f1(),
            0xb74f => self.hook_b74f(),
            0xb79b => self.hook_b79b(),
            0xb79f => self.hook_b79f(),
            0xb7a9 => self.hook_b7a9(),
            0xb7bd => self.hook_b7bd(),
            0xb825 => self.hook_b825(),
            0xb8b4 => self.hook_b8b4(),
            0xb8c3 => self.hook_b8c3(),
            0xb8d6 => self.hook_b8d6(),
            0xbcc5 => self.hook_bcc5(),
            0xc000 => self.hook_c000(),
            0xc085 => self.hook_c085(),
            0xc094 => self.hook_c094(),
            0xc09e => self.hook_c09e(),
            0xc0ba => self.hook_c0ba(),
            _ => false,
        }
    }
    pub(crate) fn call_hook(&mut self, addr: u16) -> bool {
        let need_guard = matches!(
            addr,
            0xb6cd | 0xb6f1 | 0xb74f | 0xb79b | 0xb79f | 0xb825 | 0xb8b4 | 0xb8c3
        );
        let old_pc = self.PC() + 3; // cd xx xx
        self.SetPC(addr);
        if need_guard {
            let l = 0xde;
            let h = 0xad;
            self.push16(l, h);
        }
        let r = self.call_hook_internal(addr);
        if need_guard {
            let (l, h) = self.pop16();
            assert!(l == 0xde);
            assert!(h == 0xad);
        }
        self.SetPC(old_pc);
        r
    }
    pub(crate) fn is_known_caller(&self, addr: u16) -> bool {
        match addr {
            0x4010..=0x422b => true, // in looped func
            0x422e..=0x42b7 => true, // in looped func
            0x42e2..=0x4307 => true, // in spin lock? func
            0x4308..=0x4452 => true, // in looped func
            0x44b6..=0x44e9 => true, // in propagated call
            0x44ed..0x46ab => true,  // in looped func
            0x481e..=0x49a0 => true, // in looped func
            0x4a21..=0x4b60 => true, // in looped func
            0x4c5b..0x4e51 => true,  // in bios call func
            0x4e54..0x4e61 => true,  // in looped func
            0x4fe0..0x5138 => true,  // in looped func
            0x51c2..0x53fa => true,  // in looped func
            0x54e3..0x55ef => true,  // in looped func
            0x55f2..0x5629 => true,  // in looped func
            0x587b..0x6009 => true,  // in looped func
            0x61b5..0x6265 => true,  // in looped func
            0x6448..0x6650 => true,  // in looped func
            0x6699..0x67de => true,  // in looped func
            0x6acb..0x6b49 => true,  // in looped func
            0x6bd1..0x6bdf => true,  // called from known caller
            0x6c41..=0x6e81 => true, // in looped func
            0x6f2d..0x7037 => true,  // in looped func
            0x70a5..0x7209 => true,  // in bios call func
            0x720c..0x7332 => true,  // in bios call func
            0x7537..0x7586 => true,  // in bios call func
            0x79dc..0x7b34 => true,  // in looped func
            0xb028..0xb17a => true,  // long loop func
            0xb3b2..0xb5dd => true,  // long loop func
            0x8c02..=0x8c57 => true, // in bios call func
            0x8c58..=0x8cec => true, // in bios call func
            0xad6b..=0xae81 => true, // in looped func
            _ => false,
        }
    }
}
