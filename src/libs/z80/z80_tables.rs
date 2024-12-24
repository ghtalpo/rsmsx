use super::z80_base::{tern_op_b, FLAG_H, FLAG_V};

/* Whether a half carry occurred or not can be determined by looking at
the 3rd bit of the two arguments and the result; these are hashed
into this table in the form r12, where r is the 3rd bit of the
result, 1 is the 3rd bit of the 1st argument and 2 is the
third bit of the 2nd argument; the tables differ for add and subtract
operations */
pub const HALF_CARRY_ADD_TABLE: [u8; 8] = [0, FLAG_H, FLAG_H, FLAG_H, 0, 0, 0, FLAG_H];
pub const HALF_CARRY_SUB_TABLE: [u8; 8] = [0, 0, FLAG_H, 0, FLAG_H, 0, FLAG_H, FLAG_H];

/* Similarly, overflow can be determined by looking at the 7th bits; again
the hash into this table is r12 */
pub const OVERFLOW_ADD_TABLE: [u8; 8] = [0, 0, 0, FLAG_V, FLAG_V, 0, 0, 0];
pub const OVERFLOW_SUB_TABLE: [u8; 8] = [0, FLAG_V, 0, 0, 0, 0, FLAG_V, 0];

pub struct Z80Tables {
    pub sz53_table: [u8; 0x100],
    pub sz53p_table: [u8; 0x100],
    pub parity_table: [u8; 0x100],
}

impl Default for Z80Tables {
    fn default() -> Self {
        Self::new()
    }
}

impl Z80Tables {
    pub fn new() -> Self {
        let mut sz53_table = [0; 0x100];
        let mut sz53p_table = [0; 0x100];
        let mut parity_table = [0; 0x100];
        // init
        for i in 0..0x100 {
            sz53_table[i] = (i as u8) & (0x08 | 0x20 | 0x80);
            let mut j = i as u8;
            let mut parity = 0;
            for _ in 0..8 {
                parity ^= j & 1;
                j >>= 1;
            }
            parity_table[i] = tern_op_b(parity != 0, 0, 0x04);
            sz53p_table[i] = sz53_table[i] | parity_table[i];
        }

        sz53_table[0] |= 0x40;
        sz53p_table[0] |= 0x40;

        Self {
            sz53_table,
            sz53p_table,
            parity_table,
        }
    }
}
