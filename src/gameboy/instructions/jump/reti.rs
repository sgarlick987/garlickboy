use crate::{
    gameboy::{Gameboy, GameboyCycle, GameboyCycles},
    utils::merge_bytes,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// RETI - 0xD9
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// with branch (16t)
// fetch
// read	(SP++)->lower
// read	(SP++)->upper
// internal	set PC?
struct Context {
    upper: u8,
    lower: u8,
}

pub fn new() -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context { upper: 0, lower: 0 }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        gameboy.schedule_ime();
        //fetch
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.lower = gameboy.pop();
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.upper = gameboy.pop();
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.pc = merge_bytes(context.upper, context.lower);
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::split_bytes;
    use coverage_helper::test;

    const CYCLES: usize = 4;
    const RETURN_ADDRESS: u16 = 0xFF00;
    const STACK_ADDRESS: u16 = 0xFFFE;

    #[test]
    fn test_ret() {
        let gameboy = &mut Gameboy::new();
        let cycles = new();
        let (upper, lower) = split_bytes(RETURN_ADDRESS);
        gameboy.registers.set_sp(STACK_ADDRESS);
        gameboy.push(upper);
        gameboy.push(lower);
        assert_eq!(cycles.len(), CYCLES);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, RETURN_ADDRESS);
    }
}
