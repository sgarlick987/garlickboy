use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::chip::{instructions::Comparison, GameboyChip};

// JR Z,i8 - 0x28
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (8t)
// fetch	fetch
// with branch (12t)
// read	read
// i8	i8
//     internal
//     modify PC
pub struct Inst {
    offset: i8,
    branch: bool,
    comparison: Comparison,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(comparison: &Comparison) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let inst = Rc::new(RefCell::new(Inst {
        offset: 0,
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
            Comparison::NONZERO => !chip.registers.flags.zero,
            Comparison::ZERO => chip.registers.flags.zero,
            Comparison::CARRY => chip.registers.flags.carry,
            Comparison::NOCARRY => !chip.registers.flags.carry,
        };
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        inst.offset = chip.read_byte_pc_lower() as i8;

        if !inst.branch {
            chip.pc = chip.pc.wrapping_add(2);
        }
    }));

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            let mut inst = self.inst.borrow_mut();
            if inst.branch {
                let offset = inst.offset;
                inst.branch = false;
                return Some(Box::new(move |chip: &mut GameboyChip| {
                    chip.pc = chip.pc.wrapping_add(2).wrapping_add(offset as u16);
                }));
            }
            return None;
        }

        self.executions.pop_front()
    }
}
