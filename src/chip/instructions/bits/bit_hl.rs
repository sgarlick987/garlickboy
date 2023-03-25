use std::collections::VecDeque;

use crate::chip::GameboyChip;

// BIT 0,(HL) - 0x46
// Length: 2 bytes
// Flags
// Zero	dependent
// Negative	unset
// Half Carry	set
// Carry	unmodified
// Group: x8/rsb
// Timing
// without branch (12t)
// fetch	(0xCB)
// fetch
// read	(HL)
struct Inst {
    bit: u8,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(bit: &u8) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        bit: *bit,
        executions: VecDeque::with_capacity(3),
    };

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            let check = 1 << inst.bit;
            let hl = chip.registers.get_hl();
            let byte = chip.read_byte(hl);

            chip.update_zero_flag(byte & check == 0);
            chip.reset_negative_flag();
            chip.set_half_carry_flag();

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
