use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// SET 7,(HL) - 0xFE
// Length: 2 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/rsb
// Timing
// without branch (16t)
// fetch	(0xCB)
// fetch
// read	(HL)
// write	(HL)
pub fn new(bit: &u8) -> GameboyCycles {
    let bit = bit.clone();
    let byte = Rc::new(RefCell::new(0u8));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let byte_cell = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let hl = gameboy.registers.get_hl();
        byte_cell.replace(gameboy.read_byte(hl));
    }));

    let byte_cell = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let hl = gameboy.registers.get_hl();
        let bit = 1u8 << bit;
        let value = byte_cell.take() | bit;
        gameboy.write_byte(hl, value);
        gameboy.pc = gameboy.pc.wrapping_add(2);
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::bus::HRAM_ADDRESS_START;
    use coverage_helper::test;

    const LENGTH: u16 = 2;
    const CYCLES: usize = 4;

    #[test]
    fn test_already_set() {
        for bit in 0..8 {
            let gameboy = &mut Gameboy::new();
            let byte = 0xFF;
            gameboy.registers.set_hl(HRAM_ADDRESS_START);
            gameboy.write_byte(gameboy.registers.get_hl(), byte);
            let steps = new(&bit);
            assert_eq!(steps.len(), CYCLES);

            for step in steps {
                gameboy.execute(step);
            }

            assert_eq!(gameboy.pc, LENGTH);
            assert_eq!(
                gameboy.read_byte(HRAM_ADDRESS_START),
                byte,
                "bit should be 1 if already set"
            );
        }
    }

    #[test]
    fn test_set() {
        for bit in 0..8 {
            let gameboy = &mut Gameboy::new();
            let byte = 0;
            let check = 1u8 << bit;
            gameboy.registers.set_hl(HRAM_ADDRESS_START);
            gameboy.write_byte(gameboy.registers.get_hl(), byte);
            let steps = new(&bit);
            assert_eq!(steps.len(), CYCLES);

            for step in steps {
                gameboy.execute(step);
            }

            assert_eq!(gameboy.pc, LENGTH);
            assert_eq!(
                gameboy.read_byte(HRAM_ADDRESS_START),
                check,
                "bit should be 1 if set"
            );
        }
    }
}
