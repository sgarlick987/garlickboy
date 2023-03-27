use std::collections::VecDeque;

use crate::{
    gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle},
    utils::sub_bytes_half_carry,
};

// CP A,B - 0xB8
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	set
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timing
// without branch (4t)
// fetch
struct Inst {
    target: TargetRegister8,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(target: &TargetRegister8) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let byte = gameboy.registers.get_from_enum(&inst.target);
            let a = gameboy.registers.a;
            gameboy.set_negative_flag();
            gameboy.update_zero_flag(a == byte);
            gameboy.update_carry_flag(a < byte);
            gameboy.update_half_carry_flag(sub_bytes_half_carry(a, byte));
            gameboy.pc = gameboy.pc.wrapping_add(1);
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

//     pub fn cp_r8(&mut self, target: &TargetRegister8) -> u8 {
//         //fetch

//         self.pc = self.pc.wrapping_add(1);
//         self.sync()
//     }
