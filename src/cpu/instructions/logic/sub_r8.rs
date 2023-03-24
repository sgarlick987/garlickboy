use std::collections::VecDeque;

use crate::cpu::{instructions::TargetRegister8, GameboyChip};

// SUB A,B - 0x90
// Length: 1 byte
// FlagsZero	dependent
// Negative	set
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timingwithout branch (4t)
// fetch
struct Inst {
    target: TargetRegister8,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(
    target: &TargetRegister8,
) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(1),
    };

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            chip.registers.a = match inst.target {
                TargetRegister8::A => sub(chip, chip.registers.a, false),
                TargetRegister8::B => sub(chip, chip.registers.b, false),
                TargetRegister8::C => sub(chip, chip.registers.c, false),
                TargetRegister8::D => sub(chip, chip.registers.d, false),
                TargetRegister8::E => sub(chip, chip.registers.e, false),
                TargetRegister8::H => sub(chip, chip.registers.h, false),
                TargetRegister8::L => sub(chip, chip.registers.l, false),
            };

            chip.pc = chip.pc.wrapping_add(1);
        }));

    Box::new(inst)
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

fn sub(chip: &mut GameboyChip, value: u8, carry: bool) -> u8 {
    let (subbed, overflowed) = chip.registers.a.borrowing_sub(value, carry);
    chip.registers.flags.zero = subbed == 0;
    chip.registers.flags.negative = true;
    chip.registers.flags.carry = overflowed;
    (_, chip.registers.flags.half_carry) = (chip.registers.a & 0x0F).overflowing_sub(value & 0x0F);

    subbed
}
