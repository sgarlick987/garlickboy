use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::gameboy::{Gameboy, GameboyCycle};

// JR i8 - 0x18
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timingwith branch (12t)
// fetch
// read	i8
// internal	modify PC
pub struct Inst {
    offset: i8,
}
struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst { offset: 0 }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    executions.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.offset = gameboy.read_byte_pc_lower() as i8;
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow_mut();
        gameboy.pc = gameboy.pc.wrapping_add(2).wrapping_add(inst.offset as u16);
    }));

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
