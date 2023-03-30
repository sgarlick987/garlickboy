use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// BIT 0,(HL) - 0x46
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	set
// Carry	unmodified
// Group: x8/rsb
// Timing
// without branch (12t)
// fetch	(0xCB)
// fetch
// read	(HL)
struct Inst {
    bit: u8,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(bit: &u8) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        bit: *bit,
        executions: VecDeque::with_capacity(3),
    };

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let check = 1 << inst.bit;
            let hl = gameboy.registers.get_hl();
            let byte = gameboy.read_byte(hl);

            gameboy.write_zero_flag(byte & check == 0);
            gameboy.reset_negative_flag();
            gameboy.set_half_carry_flag();

            gameboy.pc = gameboy.pc.wrapping_add(2);
        }));

    Box::new(inst)
}

impl Iterator for Inst {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
