//     pub fn retf(&mut self, comparison: &Comparison) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         if match comparison {
//             Comparison::NONZERO => !self.registers.flags.zero,
//             Comparison::ZERO => self.registers.flags.zero,
//             Comparison::CARRY => self.registers.flags.carry,
//             Comparison::NOCARRY => !self.registers.flags.carry,
//         } {
//             //read lower
//             let lower = self._pop();
//             cycles_used += self.sync();

//             //read upper
//             let upper = self._pop();
//             cycles_used += self.sync();

//             //set pc
//             self.pc = merge_bytes(upper, lower);
//         } else {
//             self.pc = self.pc.wrapping_add(1);
//         }

//         cycles_used += self.sync();
//         cycles_used
//     }
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    chip::{instructions::Comparison, GameboyChip},
    utils::merge_bytes,
};

// RET Z - 0xC8
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (8t)	with branch (20t)
// fetch	fetch
// internal	internal
// branch decision?	branch decision?
// read
// (SP++)->lower
// read
// (SP++)->upper
// internal
// set PC?
pub struct Inst {
    branch: bool,
    comparison: Comparison,
    upper: u8,
    lower: u8,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(comparison: &Comparison) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let inst = Rc::new(RefCell::new(Inst {
        branch: false,
        comparison: comparison.clone(),
        upper: 0,
        lower: 0,
    }));

    let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> = VecDeque::with_capacity(2);

    executions.push_back(Box::new(move |_: &mut GameboyChip| {}));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        inst.branch = match inst.comparison {
            Comparison::NONZERO => !chip.registers.flags.zero,
            Comparison::ZERO => chip.registers.flags.zero,
            Comparison::CARRY => chip.registers.flags.carry,
            Comparison::NOCARRY => !chip.registers.flags.carry,
        };

        if !inst.branch {
            chip.pc = chip.pc.wrapping_add(1);
        }
    }));

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            let inst = self.inst.clone();
            let mut inst = inst.borrow_mut();
            if inst.branch {
                let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> =
                    VecDeque::with_capacity(3);
                inst.branch = false;

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |chip: &mut GameboyChip| {
                    let mut inst = inst_ref.borrow_mut();
                    inst.lower = chip.pop();
                }));

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |chip: &mut GameboyChip| {
                    let mut inst = inst_ref.borrow_mut();
                    inst.upper = chip.pop();
                }));

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |chip: &mut GameboyChip| {
                    let inst = inst_ref.borrow();
                    chip.pc = merge_bytes(inst.upper, inst.lower);
                }));
                self.executions = executions;
                return self.executions.pop_front();
            }
            return None;
        }

        self.executions.pop_front()
    }
}
