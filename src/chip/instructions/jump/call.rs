use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    chip::GameboyChip,
    utils::{merge_bytes, split_bytes},
};

// CALL u16 - 0xCD
// Length: 3 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timingwith branch (24t)
// fetch
// read	u16:lower
// read	u16:upper
// internal	branch decision?
// write	PC:upper->(--SP)
// write	PC:lower->(--SP)
pub struct Inst {
    upper: u8,
    lower: u8,
    return_upper: u8,
    return_lower: u8,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let inst = Rc::new(RefCell::new(Inst {
        upper: 0,
        lower: 0,
        return_upper: 0,
        return_lower: 0,
    }));

    let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> = VecDeque::with_capacity(6);

    executions.push_back(Box::new(move |_: &mut GameboyChip| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        inst.lower = chip.read_byte_pc_lower();
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        inst.upper = chip.read_byte_pc_upper();
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        (inst.return_upper, inst.return_lower) = split_bytes(chip.pc.wrapping_add(3));
        let address = merge_bytes(inst.upper, inst.lower);
        chip.pc = address;
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let inst = inst_ref.borrow_mut();
        chip.push(inst.return_upper);
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let inst = inst_ref.borrow_mut();
        chip.push(inst.return_lower);
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
