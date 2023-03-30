use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle};

// AND A,B - 0xA0
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	set
// Carry	unset
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
            gameboy.registers.a &= byte;
            gameboy.reset_negative_flag();
            gameboy.reset_carry_flag();
            gameboy.write_zero_flag(gameboy.registers.a == 0);
            gameboy.set_half_carry_flag();
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
