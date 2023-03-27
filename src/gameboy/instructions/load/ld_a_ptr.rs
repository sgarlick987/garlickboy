use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetPointer, Gameboy, GameboyCycle};

// LD A,(BC) - 0x0A
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (8t)
// fetch
// read	(BC)->A
struct Inst {
    target: TargetPointer,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(target: &TargetPointer) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            gameboy.registers.a = match inst.target {
                TargetPointer::BC => gameboy.read_byte(gameboy.registers.get_bc()),
                TargetPointer::DE => gameboy.read_byte(gameboy.registers.get_de()),
                TargetPointer::HL => gameboy.read_byte(gameboy.registers.get_hl()),
            };
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
