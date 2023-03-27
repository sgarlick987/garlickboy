use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::gameboy::{instructions::Comparison, Gameboy, GameboyCycle};

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
    executions: VecDeque<GameboyCycle>,
}

pub fn new(comparison: &Comparison) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst {
        offset: 0,
        branch: false,
        comparison: comparison.clone(),
    }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        //need to determine this upfront since we have
        //variable cycles based on if we branch or not
        inst.branch = match inst.comparison {
            Comparison::NONZERO => !gameboy.zero_flag(),
            Comparison::ZERO => gameboy.zero_flag(),
            Comparison::CARRY => gameboy.carry_flag(),
            Comparison::NOCARRY => !gameboy.carry_flag(),
        };
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.offset = gameboy.read_byte_pc_lower() as i8;

        if !inst.branch {
            gameboy.pc = gameboy.pc.wrapping_add(2);
        }
    }));

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            let mut inst = self.inst.borrow_mut();
            if inst.branch {
                let offset = inst.offset;
                inst.branch = false;
                return Some(Box::new(move |gameboy: &mut Gameboy| {
                    gameboy.pc = gameboy.pc.wrapping_add(2).wrapping_add(offset as u16);
                }));
            }
            return None;
        }

        self.executions.pop_front()
    }
}
