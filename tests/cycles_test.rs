use std::{cell::RefCell, rc::Rc};

use rsmsx::libs::{
    graphics::GraphicsType, memory::Memory, ports::Ports, ppi::PPI, psg::PSG, sound::SoundType,
    vdp::Vdp, z80::z80_base::Z80,
};

#[test]
fn test1() {
    // LD A, 0 (8 cycles)
    // HALT    (5 cycles)
    let ar = [0x3e, 0x00, 0x76];
    let nc = check_cycles(&ar);
    assert_eq!(nc, 13);
}

#[test]
fn test2() {
    // LD A, 0  (8 cycles)
    // JP Z, 1  (11 cycles)
    // HALT     (5 cycles)
    let ar = [0x3e, 0x00, 0xca, 0x00, 0x00, 0x76];
    let nc = check_cycles(&ar);
    assert_eq!(nc, 24);
}

#[test]
fn test3() {
    //     LD B, 5  (8 cycles)
    // xx: INC A    (5 cycles)
    //     DJNZ xx  (14/9 cycles)
    //     HALT     (5 cycles)
    let ar = [0x06, 0x05, 0x3c, 0x10, 0xfd, 0x76];
    let nc = check_cycles(&ar);
    assert_eq!(nc, 103);
}

fn check_cycles(ar: &[u8]) -> isize {
    let ppi = Rc::new(RefCell::new(PPI::new()));
    let mut memory = Memory::new(ppi.clone());
    for (i, item) in ar.iter().enumerate() {
        memory.write_byte(i as u16, *item);
    }
    let psg = PSG::new(SoundType::None);
    let vdp = Rc::new(RefCell::new(Vdp::new(GraphicsType::None, false)));
    let ports = Ports::new(vdp.clone(), ppi.clone(), psg);
    let mut cpu_z80 = Z80::new(memory, ports);
    cpu_z80.reset();
    cpu_z80.SetPC(0);
    cpu_z80.reset_cycles();

    while !cpu_z80.is_halted() {
        cpu_z80.do_opcode()
    }

    cpu_z80.get_cycles() as isize
}
