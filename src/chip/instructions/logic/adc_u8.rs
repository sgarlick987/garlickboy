use std::collections::VecDeque;

use crate::chip::GameboyChip;

// ADC A,u8 - 0xCE
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x8/alu
// Timing
// without branch (8t)
// fetch
// read	u8
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
            let byte = chip.read_byte_pc_lower();
            let carry = chip.carry_flag();
            chip.registers.a = chip.add(byte, carry);
            chip.pc = chip.pc.wrapping_add(2);
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