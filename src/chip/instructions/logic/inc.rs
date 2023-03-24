use std::collections::VecDeque;

use crate::chip::{
    instructions::{TargetIncDec, TargetRegister8},
    GameboyChip,
};

struct Inst {
    target: TargetIncDec,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

// INC B - 0x04
// Length: 1 byte
// FlagsZero	dependent
// Negative	unset
// Half Carry	dependent
// Carry	unmodified
// Group: x8/alu
// Timingwithout branch (4t)
// fetch
fn new_r8(target: &TargetIncDec) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let value = match inst.target {
                TargetIncDec::A => chip.registers.get_from_enum(&TargetRegister8::A),
                TargetIncDec::B => chip.registers.get_from_enum(&TargetRegister8::B),
                TargetIncDec::C => chip.registers.get_from_enum(&TargetRegister8::C),
                TargetIncDec::D => chip.registers.get_from_enum(&TargetRegister8::D),
                TargetIncDec::E => chip.registers.get_from_enum(&TargetRegister8::E),
                TargetIncDec::H => chip.registers.get_from_enum(&TargetRegister8::H),
                TargetIncDec::L => chip.registers.get_from_enum(&TargetRegister8::L),
                _ => panic!("invalid register for inc new_r8"),
            }
            .wrapping_add(1);

            match inst.target {
                TargetIncDec::A => chip.registers.set_from_enum(&TargetRegister8::A, value),
                TargetIncDec::B => chip.registers.set_from_enum(&TargetRegister8::B, value),
                TargetIncDec::C => chip.registers.set_from_enum(&TargetRegister8::C, value),
                TargetIncDec::D => chip.registers.set_from_enum(&TargetRegister8::D, value),
                TargetIncDec::E => chip.registers.set_from_enum(&TargetRegister8::E, value),
                TargetIncDec::H => chip.registers.set_from_enum(&TargetRegister8::H, value),
                TargetIncDec::L => chip.registers.set_from_enum(&TargetRegister8::L, value),
                _ => panic!("invalid register for inc new_r8"),
            }

            chip.registers.flags.zero = value == 0;
            chip.registers.flags.negative = false;
            //TODO: half carry
            chip.pc = chip.pc.wrapping_add(1);
        }));

    Box::new(inst)
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
fn new_r16(target: &TargetIncDec) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(2),
    };

    //TODO: does it matter if we actually set c and b in separate cycles?
    inst.executions
        .push_back(Box::new(move |_: &mut GameboyChip| {
            //fetch
        }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            match inst.target {
                TargetIncDec::BC => {
                    chip.registers
                        .set_bc(chip.registers.get_bc().wrapping_add(1));
                }
                TargetIncDec::DE => {
                    chip.registers
                        .set_de(chip.registers.get_de().wrapping_add(1));
                }
                TargetIncDec::HL => {
                    chip.registers
                        .set_hl(chip.registers.get_hl().wrapping_add(1));
                }
                TargetIncDec::SP => {
                    chip.registers.sp = chip.registers.sp.wrapping_add(1);
                }
                _ => panic!("invalid register for inc new_r16"),
            }
            chip.pc = chip.pc.wrapping_add(1);
        }));

    Box::new(inst)
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
fn new_ptr() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: TargetIncDec::HLPOINTER,
        executions: VecDeque::with_capacity(3),
    };

    //TODO: actually do cycles
    inst.executions
        .push_back(Box::new(move |_: &mut GameboyChip| {}));
    inst.executions
        .push_back(Box::new(move |_: &mut GameboyChip| {}));
    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let address = chip.registers.get_hl();
            let byte = chip.read_byte(address).wrapping_add(1);
            chip.write_byte(address, byte);
            chip.pc = chip.pc.wrapping_add(1);
        }));

    Box::new(inst)
}

pub fn new(target: &TargetIncDec) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
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
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
