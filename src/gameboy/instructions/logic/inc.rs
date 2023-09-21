use crate::{
    gameboy::{
        instructions::{TargetIncDec, TargetRegister8},
        Gameboy, GameboyCycle, GameboyCycles,
    },
    utils::add_bytes_half_carry,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// INC B - 0x04
// Length: 1 byte
// FlagsZero	dependent
// Negative	unset
// Half Carry	dependent
// Carry	unmodified
// Group: x8/alu
// Timingwithout branch (4t)
// fetch
fn new_r8(target: &TargetIncDec) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(1);
    let target = target.clone();

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let register = match target {
            TargetIncDec::A => gameboy.registers.get_from_enum(&TargetRegister8::A),
            TargetIncDec::B => gameboy.registers.get_from_enum(&TargetRegister8::B),
            TargetIncDec::C => gameboy.registers.get_from_enum(&TargetRegister8::C),
            TargetIncDec::D => gameboy.registers.get_from_enum(&TargetRegister8::D),
            TargetIncDec::E => gameboy.registers.get_from_enum(&TargetRegister8::E),
            TargetIncDec::H => gameboy.registers.get_from_enum(&TargetRegister8::H),
            TargetIncDec::L => gameboy.registers.get_from_enum(&TargetRegister8::L),
            _ => panic!("invalid register for inc new_r8"),
        };
        let value = register.wrapping_add(1);

        match target {
            TargetIncDec::A => gameboy.registers.set_from_enum(&TargetRegister8::A, value),
            TargetIncDec::B => gameboy.registers.set_from_enum(&TargetRegister8::B, value),
            TargetIncDec::C => gameboy.registers.set_from_enum(&TargetRegister8::C, value),
            TargetIncDec::D => gameboy.registers.set_from_enum(&TargetRegister8::D, value),
            TargetIncDec::E => gameboy.registers.set_from_enum(&TargetRegister8::E, value),
            TargetIncDec::H => gameboy.registers.set_from_enum(&TargetRegister8::H, value),
            TargetIncDec::L => gameboy.registers.set_from_enum(&TargetRegister8::L, value),
            _ => panic!("invalid register for inc new_r8"),
        }

        gameboy.write_zero_flag(value == 0);
        gameboy.reset_negative_flag();
        gameboy.write_half_carry_flag(add_bytes_half_carry(register, 1));
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}

// INC BC - 0x03
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/alu
// Timingwithout branch (8t)
// fetch	Probably writes to C here
// internal	Probably writes to B here
fn new_r16(target: &TargetIncDec) -> GameboyCycles {
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);
    let target = target.clone();

    //TODO: does it matter if we actually set c and b in separate cycles?
    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        match target {
            TargetIncDec::BC => {
                gameboy
                    .registers
                    .set_bc(gameboy.registers.get_bc().wrapping_add(1));
            }
            TargetIncDec::DE => {
                gameboy
                    .registers
                    .set_de(gameboy.registers.get_de().wrapping_add(1));
            }
            TargetIncDec::HL => {
                gameboy
                    .registers
                    .set_hl(gameboy.registers.get_hl().wrapping_add(1));
            }
            TargetIncDec::SP => {
                gameboy
                    .registers
                    .set_sp(gameboy.registers.get_sp().wrapping_add(1));
            }
            _ => panic!("invalid register for inc new_r16"),
        }
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(cycles.into_iter())
}

// INC (HL) - 0x34
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	dependent
// Carry	unmodified
// Group: x8/alu
// Timing
// without branch (12t)
// fetch
// read	(HL)
// write	(HL)
fn new_ptr() -> GameboyCycles {
    let byte = Rc::new(RefCell::new(0u8));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    //TODO: actually do cycles
    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    let byte_ref = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let address = gameboy.registers.get_hl();
        byte_ref.replace(gameboy.read_byte(address));
    }));

    let byte_ref = byte.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let byte = byte_ref.take();
        let value = byte.wrapping_add(1);
        let address = gameboy.registers.get_hl();
        gameboy.write_byte(address, value);
        gameboy.reset_negative_flag();
        gameboy.write_half_carry_flag(add_bytes_half_carry(byte, 1));
        gameboy.write_zero_flag(value == 0);
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));
    //carry unmodified
    Box::new(cycles.into_iter())
}

pub fn new(target: &TargetIncDec) -> GameboyCycles {
    match target {
        TargetIncDec::A
        | TargetIncDec::B
        | TargetIncDec::C
        | TargetIncDec::D
        | TargetIncDec::E
        | TargetIncDec::H
        | TargetIncDec::L => new_r8(target),
        TargetIncDec::BC | TargetIncDec::DE | TargetIncDec::HL | TargetIncDec::SP => {
            new_r16(target)
        }
        TargetIncDec::HLPOINTER => new_ptr(),
    }
}
