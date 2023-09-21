use crate::{
    gameboy::{instructions::RstVector, Gameboy, GameboyCycle, GameboyCycles},
    utils::split_bytes,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// RST 28h - 0xEF
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (16t)
// fetch
// internal
// write	PC:upper->(--SP)
// write	PC:lower->(--SP)
struct Context {
    target: RstVector,
    upper: u8,
    lower: u8,
}

pub fn new(target: &RstVector) -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context {
        target: target.clone(),
        upper: 0,
        lower: 0,
    }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        let return_address = gameboy.pc.wrapping_add(1);
        (context.upper, context.lower) = split_bytes(return_address);
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.push(context.upper);
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let context = context_ref.borrow();
        gameboy.push(context.lower);
        gameboy.pc = u16::from(context.target);
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use coverage_helper::test;

    const CYCLES: usize = 4;
    const PC: u16 = 0xFF00;
    const STACK_ADDRESS: u16 = 0xFFFE;

    const RST_VECTORS: [RstVector; 8] = [
        RstVector::H00,
        RstVector::H08,
        RstVector::H10,
        RstVector::H18,
        RstVector::H20,
        RstVector::H28,
        RstVector::H30,
        RstVector::H38,
    ];

    #[test]
    fn test_rst() {
        for vector in RST_VECTORS {
            let mut gameboy = &mut Gameboy::new();
            gameboy.registers.set_sp(STACK_ADDRESS);
            gameboy.pc = PC;
            let cycles = new(&vector);
            assert_eq!(cycles.len(), CYCLES);

            for cycle in cycles {
                gameboy.execute(cycle);
            }

            assert_eq!(
                gameboy.pc,
                match vector {
                    RstVector::H00 => 0x0000,
                    RstVector::H08 => 0x0008,
                    RstVector::H10 => 0x0010,
                    RstVector::H18 => 0x0018,
                    RstVector::H20 => 0x0020,
                    RstVector::H28 => 0x0028,
                    RstVector::H30 => 0x0030,
                    RstVector::H38 => 0x0038,
                }
            );
            let (upper, lower) = split_bytes(PC.wrapping_add(1));
            assert_eq!(gameboy.pop(), lower);
            assert_eq!(gameboy.pop(), upper);
        }
    }
}
