use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    gameboy::{instructions::RstVector, Gameboy, GameboyCycle},
    utils::split_bytes,
};

// RST 28h - 0xEF
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (16t)
// fetch
// internal
// write	PC:upper->(--SP)
// write	PC:lower->(--SP)
pub struct Inst {
    target: RstVector,
    upper: u8,
    lower: u8,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(target: &RstVector) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst {
        target: target.clone(),
        upper: 0,
        lower: 0,
    }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);

    executions.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        let return_address = gameboy.pc.wrapping_add(1);
        (inst.upper, inst.lower) = split_bytes(return_address);
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow();
        gameboy.push(inst.upper);
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow();
        gameboy.push(inst.lower);
        gameboy.pc = u16::from(inst.target);
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
