use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    chip::{instructions::TargetRegister16, GameboyChip},
    utils::split_bytes,
};

// ADD HL,DE - 0x19
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x16/alu
// Timing
// without branch (8t)
// fetch	Probably writes to L here
// internal	Probably writes to H here
struct Inst {
    upper: u8,
    target: TargetRegister16,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(
    target: &TargetRegister16,
) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let inst = Rc::new(RefCell::new(Inst {
        upper: 0,
        target: target.clone(),
    }));
    let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> = VecDeque::with_capacity(2);

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        let hl = chip.registers.get_hl();
        let value = match inst.target {
            TargetRegister16::DE => chip.registers.get_de(),
            TargetRegister16::HL => chip.registers.get_hl(),
            TargetRegister16::BC => chip.registers.get_bc(),
            TargetRegister16::SP => chip.registers.sp,
            _ => panic!("{:?} not implemented for add r16", inst.target),
        };

        let (added, overflowed) = hl.carrying_add(value, chip.carry_flag());
        let (upper, lower) = split_bytes(added);
        chip.registers.l = lower;
        chip.update_carry_flag(overflowed);

        inst.upper = upper;
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let inst = inst_ref.borrow();
        chip.registers.h = inst.upper;
        chip.reset_negative_flag();
        chip.reset_half_carry_flag();
        chip.pc = chip.pc.wrapping_add(1);
    }));

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
