use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetPushPop, Gameboy, GameboyCycle};

// PUSH BC - 0xC5
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/lsm
// Timingwithout branch (16t)
// fetch
// internal
// write	B->(--SP)
// write	C->(--SP)
struct Inst {
    target: TargetPushPop,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(target: &TargetPushPop) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(4),
    };

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    inst.executions.push_back(Box::new(|_: &mut Gameboy| {
        //internal
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| match inst.target {
            TargetPushPop::AF => {
                gameboy.push(gameboy.registers.a);
            }
            TargetPushPop::HL => {
                gameboy.push(gameboy.registers.h);
            }
            TargetPushPop::BC => {
                gameboy.push(gameboy.registers.b);
            }
            TargetPushPop::DE => {
                gameboy.push(gameboy.registers.d);
            }
        }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            match inst.target {
                TargetPushPop::AF => {
                    gameboy.push(gameboy.registers.get_f());
                }
                TargetPushPop::HL => {
                    gameboy.push(gameboy.registers.l);
                }
                TargetPushPop::BC => {
                    gameboy.push(gameboy.registers.c);
                }
                TargetPushPop::DE => {
                    gameboy.push(gameboy.registers.e);
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
