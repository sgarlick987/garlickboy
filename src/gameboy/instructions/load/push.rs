use std::collections::VecDeque;

use crate::gameboy::{instructions::TargetPushPop, Gameboy, GameboyCycle, GameboyCycles};

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
pub fn new(target: &TargetPushPop) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);
    let target = target.clone();

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //internal
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| match target {
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

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        match target {
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

    Box::new(cycles.into_iter())
}
