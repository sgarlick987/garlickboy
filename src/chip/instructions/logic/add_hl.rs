use std::collections::VecDeque;

use crate::chip::GameboyChip;

// ADD A,(HL) - 0x86
// Length: 1 byte
// FlagsZero	dependent
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timingwithout branch (8t)
// fetch
// read	(HL)
struct Inst {
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(2),
    };

    inst.executions
        .push_back(Box::new(move |_: &mut GameboyChip| {}));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let hl = chip.registers.get_hl();
            let byte = chip.read_byte(hl);
            chip.registers.a = add(chip, byte, false);

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

pub fn add(chip: &mut GameboyChip, value: u8, carry: bool) -> u8 {
    let (added, overflowed) = chip.registers.a.carrying_add(value, carry);
    chip.registers.flags.zero = added == 0;
    chip.registers.flags.negative = false;
    chip.registers.flags.carry = overflowed;
    chip.registers.flags.half_carry = (chip.registers.a & 0x0F) + (value & 0x0F) > 0x0F;

    added
}
