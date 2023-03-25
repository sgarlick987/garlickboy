use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    chip::{instructions::Comparison, GameboyChip},
    utils::merge_bytes,
};

// JP Z,u16 - 0xCA
// Length: 3 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (12t)	with branch (16t)
// fetch	fetch
// read	read
// u16:lower	u16:lower
// read	read
// u16:upper	u16:upper
// internal
// branch decision?
pub struct Inst {
    upper: u8,
    lower: u8,
    branch: bool,
    comparison: Comparison,
}
struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(comparison: &Comparison) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let inst = Rc::new(RefCell::new(Inst {
        upper: 0,
        lower: 0,
        branch: false,
        comparison: comparison.clone(),
    }));

    let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> = VecDeque::with_capacity(3);

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        //need to determine this upfront since we have
        //variable cycles based on if we branch or not
        inst.branch = match inst.comparison {
            Comparison::NONZERO => !chip.zero_flag(),
            Comparison::ZERO => chip.zero_flag(),
            Comparison::CARRY => chip.carry_flag(),
            Comparison::NOCARRY => !chip.carry_flag(),
        };

        if !inst.branch {
            chip.pc = chip.pc.wrapping_add(3);
        }
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

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            let mut inst = self.inst.borrow_mut();
            if inst.branch {
                let address = merge_bytes(inst.upper, inst.lower);
                inst.branch = false;
                return Some(Box::new(move |chip: &mut GameboyChip| {
                    chip.pc = address;
                }));
            }
            return None;
        }

        self.executions.pop_front()
    }
}
