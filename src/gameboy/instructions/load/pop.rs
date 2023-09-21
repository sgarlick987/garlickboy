use crate::gameboy::{
    instructions::TargetPushPop, registers::FlagsRegister, Gameboy, GameboyCycle, GameboyCycles,
};
use std::collections::VecDeque;

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
pub fn new(target: &TargetPushPop) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);
    let target = target.clone();

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| match target {
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

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        match target {
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

    Box::new(cycles.into_iter())
}
