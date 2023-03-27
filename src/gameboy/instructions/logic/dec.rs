use std::collections::VecDeque;

use crate::{
    gameboy::{
        instructions::{TargetIncDec, TargetRegister8},
        Gameboy, GameboyCycle,
    },
    utils::sub_bytes_half_carry,
};

struct Inst {
    target: TargetIncDec,
    executions: VecDeque<GameboyCycle>,
}

// DEC B - 0x05
// Length: 1 byte
// FlagsZero	dependent
// Negative	set
// Half Carry	dependent
// Carry	unmodified
// Group: x8/alu
// Timingwithout branch (4t)
// fetch
fn new_r8(target: &TargetIncDec) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let register = match inst.target {
                TargetIncDec::A => gameboy.registers.get_from_enum(&TargetRegister8::A),
                TargetIncDec::B => gameboy.registers.get_from_enum(&TargetRegister8::B),
                TargetIncDec::C => gameboy.registers.get_from_enum(&TargetRegister8::C),
                TargetIncDec::D => gameboy.registers.get_from_enum(&TargetRegister8::D),
                TargetIncDec::E => gameboy.registers.get_from_enum(&TargetRegister8::E),
                TargetIncDec::H => gameboy.registers.get_from_enum(&TargetRegister8::H),
                TargetIncDec::L => gameboy.registers.get_from_enum(&TargetRegister8::L),
                _ => panic!("invalid register for inc new_r8"),
            };
            let value = register.wrapping_sub(1);

            match inst.target {
                TargetIncDec::A => gameboy.registers.set_from_enum(&TargetRegister8::A, value),
                TargetIncDec::B => gameboy.registers.set_from_enum(&TargetRegister8::B, value),
                TargetIncDec::C => gameboy.registers.set_from_enum(&TargetRegister8::C, value),
                TargetIncDec::D => gameboy.registers.set_from_enum(&TargetRegister8::D, value),
                TargetIncDec::E => gameboy.registers.set_from_enum(&TargetRegister8::E, value),
                TargetIncDec::H => gameboy.registers.set_from_enum(&TargetRegister8::H, value),
                TargetIncDec::L => gameboy.registers.set_from_enum(&TargetRegister8::L, value),
                _ => panic!("invalid register for inc new_r8"),
            }

            gameboy.update_zero_flag(value == 0);
            gameboy.set_negative_flag();
            gameboy.update_half_carry_flag(sub_bytes_half_carry(register, 1));
            gameboy.pc = gameboy.pc.wrapping_add(1);
        }));

    Box::new(inst)
}

// DEC BC - 0x0B
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/alu
// Timingwithout branch (8t)
// fetch	Probably writes to C here
// internal	Probably writes to B here
fn new_r16(target: &TargetIncDec) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    //TODO: does it matter if we actually set c and b in separate cycles?
    inst.executions.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            match inst.target {
                TargetIncDec::BC => {
                    gameboy
                        .registers
                        .set_bc(gameboy.registers.get_bc().wrapping_sub(1));
                }
                TargetIncDec::DE => {
                    gameboy
                        .registers
                        .set_de(gameboy.registers.get_de().wrapping_sub(1));
                }
                TargetIncDec::HL => {
                    gameboy
                        .registers
                        .set_hl(gameboy.registers.get_hl().wrapping_sub(1));
                }
                TargetIncDec::SP => {
                    gameboy.registers.sp = gameboy.registers.sp.wrapping_sub(1);
                }
                _ => panic!("invalid register for inc new_r16"),
            }
            gameboy.pc = gameboy.pc.wrapping_add(1);
        }));

    Box::new(inst)
}

// DEC (HL) - 0x35
// Length: 1 byte
// Flags
// Zero	dependent
// Negative	set
// Half Carry	dependent
// Carry	unmodified
// Group: x8/alu
// Timing
// without branch (12t)
// fetch
// read	(HL)
// write	(HL)
fn new_ptr() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        target: TargetIncDec::HLPOINTER,
        executions: VecDeque::with_capacity(3),
    };

    //TODO: actually do cycles
    inst.executions
        .push_back(Box::new(move |_: &mut Gameboy| {}));
    inst.executions
        .push_back(Box::new(move |_: &mut Gameboy| {}));
    inst.executions
        .push_back(Box::new(move |gameboy: &mut Gameboy| {
            let address = gameboy.registers.get_hl();
            let byte = gameboy.read_byte(address);
            let value = byte.wrapping_sub(1);

            gameboy.write_byte(address, value);
            gameboy.set_negative_flag();
            gameboy.update_half_carry_flag(sub_bytes_half_carry(byte, 1));
            gameboy.update_zero_flag(value == 0);
            //carry unmodified

            gameboy.pc = gameboy.pc.wrapping_add(1);
        }));

    Box::new(inst)
}

pub fn new(target: &TargetIncDec) -> Box<dyn Iterator<Item = GameboyCycle>> {
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

impl Iterator for Inst {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
