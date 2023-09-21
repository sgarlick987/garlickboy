use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// LD (HL),u8 - 0x36
// Length: 2 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timing
// without branch (12t)
// fetch
// read	u8
// write	(HL)
pub fn new() -> GameboyCycles {
    let byte = Rc::new(RefCell::new(0u8));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let byte_ref = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        byte_ref.replace(gameboy.read_byte_pc_lower());
    }));

    let byte_ref = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = byte_ref.take();
        gameboy.write_byte(gameboy.registers.get_hl(), byte);
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}
