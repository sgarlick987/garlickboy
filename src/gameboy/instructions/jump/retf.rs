//     pub fn retf(&mut self, comparison: &Comparison) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         if match comparison {
//             Comparison::NONZERO => !self.zero,
//             Comparison::ZERO => self.zero,
//             Comparison::CARRY => self.carry_flag(),
//             Comparison::NOCARRY => !self.carry_flag(),
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
    gameboy::{instructions::Comparison, Gameboy, GameboyCycle},
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
    executions: VecDeque<GameboyCycle>,
}

pub fn new(comparison: &Comparison) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst {
        branch: false,
        comparison: comparison.clone(),
        upper: 0,
        lower: 0,
    }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    executions.push_back(Box::new(move |_: &mut Gameboy| {}));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.branch = match inst.comparison {
            Comparison::NONZERO => !gameboy.zero_flag(),
            Comparison::ZERO => gameboy.zero_flag(),
            Comparison::CARRY => gameboy.carry_flag(),
            Comparison::NOCARRY => !gameboy.carry_flag(),
        };

        if !inst.branch {
            gameboy.pc = gameboy.pc.wrapping_add(1);
        }
    }));

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            let inst = self.inst.clone();
            let mut inst = inst.borrow_mut();
            if inst.branch {
                let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);
                inst.branch = false;

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let mut inst = inst_ref.borrow_mut();
                    inst.lower = gameboy.pop();
                }));

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let mut inst = inst_ref.borrow_mut();
                    inst.upper = gameboy.pop();
                }));

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let inst = inst_ref.borrow();
                    gameboy.pc = merge_bytes(inst.upper, inst.lower);
                }));
                self.executions = executions;
                return self.executions.pop_front();
            }
            return None;
        }

        self.executions.pop_front()
    }
}
