use std::collections::VecDeque;

use crate::gameboy::{
    instructions::TargetPushPop, registers::FlagsRegister, Gameboy, GameboyCycle,
};

// POP BC - 0xC1
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/lsm
// Timingwithout branch (12t)
// fetch
// read	(SP++)->C
// read	(SP++)->B
struct Inst {
    target: TargetPushPop,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(target: &TargetPushPop) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(3),
    };

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| match inst.target {
            TargetPushPop::AF => {
                gameboy.registers.flags = FlagsRegister::from(gameboy.pop());
            }
            TargetPushPop::HL => {
                gameboy.registers.l = gameboy.pop();
            }
            TargetPushPop::BC => {
                gameboy.registers.c = gameboy.pop();
            }
            TargetPushPop::DE => {
                gameboy.registers.e = gameboy.pop();
            }
        }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            match inst.target {
                TargetPushPop::AF => {
                    gameboy.registers.a = gameboy.pop();
                }
                TargetPushPop::HL => {
                    gameboy.registers.h = gameboy.pop();
                }
                TargetPushPop::BC => {
                    gameboy.registers.b = gameboy.pop();
                }
                TargetPushPop::DE => {
                    gameboy.registers.d = gameboy.pop();
                }
            }
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
