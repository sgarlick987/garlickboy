use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{chip::GameboyChip, utils::merge_bytes};

// RET - 0xC9
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timingwith branch (16t)
// fetch
// read	(SP++)->lower
// read	(SP++)->upper
// internal	set PC?
pub struct Inst {
    upper: u8,
    lower: u8,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let inst = Rc::new(RefCell::new(Inst { upper: 0, lower: 0 }));

    let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> = VecDeque::with_capacity(4);
    executions.push_back(Box::new(move |_: &mut GameboyChip| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        inst.lower = chip.pop();
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        inst.upper = chip.pop();
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let inst = inst_ref.borrow_mut();
        chip.pc = merge_bytes(inst.upper, inst.lower);
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
