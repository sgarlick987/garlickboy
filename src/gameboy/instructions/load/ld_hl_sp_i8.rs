use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::gameboy::{Gameboy, GameboyCycle};

// LD HL,SP+i8 - 0xF8
// Length: 2 bytes
// Flags
// Zero	unset
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x16/alu
// Timing
// without branch (12t)
// fetch
// read	i8
// internal
struct Inst {
    byte: i8,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst { byte: 0 }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        inst.byte = gameboy.read_byte_pc_lower() as i8;
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow();
        let sp = gameboy.registers.sp.wrapping_add(inst.byte as u16);
        gameboy.registers.set_hl(sp);
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
