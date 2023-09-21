use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::merge_bytes,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// JP u16 - 0xC3
// Length: 3 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timingwith branch (16t)
// fetch
// read	u16:lower
// read	u16:upper
// internal	branch decision?
struct Context {
    upper: u8,
    lower: u8,
}

pub fn new() -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context { upper: 0, lower: 0 }));

    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);
    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.lower = gameboy.read_byte_pc_lower();
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.upper = gameboy.read_byte_pc_upper();
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        let address = merge_bytes(context.upper, context.lower);
        gameboy.pc = address;
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        gameboy::{bios::BIOS_MAPPED_ADDRESS, bus::HRAM_ADDRESS_START},
        utils::split_bytes,
    };
    use coverage_helper::test;

    const CYCLES: usize = 4;
    const JP_ADDRESS: u16 = 0xFF00;
    const PC: u16 = HRAM_ADDRESS_START;

    #[test]
    fn test_jp() {
        let gameboy = &mut Gameboy::new();
        let cycles = new();
        let (upper, lower) = split_bytes(JP_ADDRESS);
        gameboy.pc = PC;
        gameboy.write_byte(BIOS_MAPPED_ADDRESS, 1);
        gameboy.write_byte(gameboy.pc + 1, lower);
        gameboy.write_byte(gameboy.pc + 2, upper);
        assert_eq!(cycles.len(), CYCLES);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, JP_ADDRESS);
    }
}
