use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    gameboy::{Gameboy, GameboyCycle},
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
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst {
        upper: 0,
        lower: 0,
        return_upper: 0,
        return_lower: 0,
    }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(6);

    executions.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.lower = gameboy.read_byte_pc_lower();
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.upper = gameboy.read_byte_pc_upper();
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        (inst.return_upper, inst.return_lower) = split_bytes(gameboy.pc.wrapping_add(3));
        let address = merge_bytes(inst.upper, inst.lower);
        gameboy.pc = address;
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow_mut();
        gameboy.push(inst.return_upper);
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow_mut();
        gameboy.push(inst.return_lower);
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
