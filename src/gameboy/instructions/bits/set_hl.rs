use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::gameboy::{Gameboy, GameboyCycle};

// SET 7,(HL) - 0xFE
// Length: 2 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x8/rsb
// Timing
// without branch (16t)
// fetch	(0xCB)
// fetch
// read	(HL)
// write	(HL)
struct Inst {
    byte: u8,
    bit: u8,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(bit: &u8) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst {
        byte: 0,
        bit: bit.clone(),
    }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(4);

    executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    executions.push_back(Box::new(|_: &mut Gameboy| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        let hl = gameboy.registers.get_hl();
        inst.byte = gameboy.read_byte(hl);
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow();
        let hl = gameboy.registers.get_hl();
        let bit = 1 << inst.bit;
        let value = inst.byte | bit;
        gameboy.write_byte(hl, value);
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
