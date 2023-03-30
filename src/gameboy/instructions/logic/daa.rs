// DAA - 0x27
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	unmodified
// Half Carry	unset
// Carry	dependent
// Group: x8/alu
// Timing
// without branch (4t)
// fetch
use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

struct Inst {
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let mut register = gameboy.registers.a;
            if gameboy.negative_flag() {
                if gameboy.carry_flag() {
                    register = register.wrapping_sub(0x60);
                }
                if gameboy.half_carry_flag() {
                    register = register.wrapping_sub(0x6);
                }
            } else {
                if gameboy.carry_flag() || register > 0x99 {
                    register = register.wrapping_add(0x60);
                    gameboy.set_carry_flag();
                }
                if gameboy.half_carry_flag() || (register & 0xF) > 0x9 {
                    register = register.wrapping_add(0x6);
                }
            }

            gameboy.registers.a = register;
            gameboy.write_zero_flag(register == 0);
            gameboy.reset_half_carry_flag();
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
