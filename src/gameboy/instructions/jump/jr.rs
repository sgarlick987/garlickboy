use crate::gameboy::{Gameboy, GameboyCycle, GameboyCycles};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// JR i8 - 0x18
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timingwith branch (12t)
// fetch
// read	i8
// internal	modify PC
pub fn new() -> GameboyCycles {
    let offset = Rc::new(RefCell::new(0i8));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    let offset_ref = offset.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        offset_ref.replace(gameboy.read_byte_pc_lower() as i8);
    }));

    let offset_ref = offset.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        gameboy.pc = gameboy
            .pc
            .wrapping_add(2)
            .wrapping_add(offset_ref.take() as u16);
    }));

    Box::new(cycles.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::{bios::BIOS_MAPPED_ADDRESS, bus::HRAM_ADDRESS_START};
    use coverage_helper::test;

    const CYCLES: usize = 3;
    const PC: u16 = HRAM_ADDRESS_START;

    #[test]
    fn test_jr_negative() {
        const JR_OFFSET: i8 = -5;
        let gameboy = &mut Gameboy::new();
        let cycles = new();
        gameboy.pc = PC;
        gameboy.write_byte(BIOS_MAPPED_ADDRESS, 1);
        gameboy.write_byte(gameboy.pc + 1, JR_OFFSET as u8);
        assert_eq!(cycles.len(), CYCLES);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(
            gameboy.pc,
            PC.wrapping_add(2).wrapping_add(JR_OFFSET as u16)
        );
    }

    #[test]
    fn test_jr_positive() {
        const JR_OFFSET: i8 = 5;
        let gameboy = &mut Gameboy::new();
        let cycles = new();
        gameboy.pc = PC;
        gameboy.write_byte(BIOS_MAPPED_ADDRESS, 1);
        gameboy.write_byte(gameboy.pc + 1, JR_OFFSET as u8);
        assert_eq!(cycles.len(), CYCLES);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(
            gameboy.pc,
            PC.wrapping_add(2).wrapping_add(JR_OFFSET as u16)
        );
    }

    #[test]
    fn test_jr_loop() {
        const JR_OFFSET: i8 = -2;
        let gameboy = &mut Gameboy::new();
        let cycles = new();
        gameboy.pc = PC;
        gameboy.write_byte(BIOS_MAPPED_ADDRESS, 1);
        gameboy.write_byte(gameboy.pc + 1, JR_OFFSET as u8);
        assert_eq!(cycles.len(), CYCLES);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, PC);
    }
}
