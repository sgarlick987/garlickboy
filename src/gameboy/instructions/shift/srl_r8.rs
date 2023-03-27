use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetRegister8, Gameboy, GameboyCycle};

// SRL B - 0x38
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	unset
// Carry	dependent
// Group: x8/rsb
// Timing
// without branch (8t)
// fetch	(0xCB)
// fetch
struct Inst {
    target: TargetRegister8,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(target: &TargetRegister8) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    inst.executions
        .push_back(Box::new(move |_: &mut Gameboy| {}));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let register = gameboy.registers.get_from_enum(&inst.target);
            let carry_out = register & 1 == 1;
            let byte = register >> 1;
            gameboy.registers.set_from_enum(&inst.target, byte);

            gameboy.update_carry_flag(carry_out);
            gameboy.update_zero_flag(byte == 0);
            gameboy.reset_half_carry_flag();
            gameboy.reset_negative_flag();

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
