use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::gameboy::{Gameboy, GameboyCycle};

// LD (FF00+u8),A - 0xE0
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/lsm
// Timingwithout branch (12t)
// fetch
// read	u8
// write	A->(FF00+u8)
struct Inst {
    address: u16,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst { address: 0 }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.address = 0xFF00 + gameboy.read_byte_pc_lower() as u16;
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow_mut();
        gameboy.write_byte(inst.address, gameboy.registers.a);
        gameboy.pc = gameboy.pc.wrapping_add(2);
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
