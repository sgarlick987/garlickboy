use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    gameboy::{instructions::TargetRegister16, Gameboy, GameboyCycle},
    utils::split_bytes,
};

// ADD HL,DE - 0x19
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unset
// Half Carry	dependent
// Carry	dependent
// Group: x16/alu
// Timing
// without branch (8t)
// fetch	Probably writes to L here
// internal	Probably writes to H here
struct Inst {
    upper: u8,
    target: TargetRegister16,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(target: &TargetRegister16) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst {
        upper: 0,
        target: target.clone(),
    }));
    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut inst = inst_ref.borrow_mut();
        let hl = gameboy.registers.get_hl();
        let value = match inst.target {
            TargetRegister16::DE => gameboy.registers.get_de(),
            TargetRegister16::HL => gameboy.registers.get_hl(),
            TargetRegister16::BC => gameboy.registers.get_bc(),
            TargetRegister16::SP => gameboy.registers.sp,
            _ => panic!("{:?} not implemented for add r16", inst.target),
        };

        let (added, overflowed) = hl.overflowing_add(value);
        let (upper, lower) = split_bytes(added);
        gameboy.registers.l = lower;
        gameboy.write_carry_flag(overflowed);
        gameboy.write_half_carry_flag((hl & 0xFF) + (value & 0xFF) > 0xFF);

        inst.upper = upper;
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let inst = inst_ref.borrow();
        gameboy.registers.h = inst.upper;
        gameboy.reset_negative_flag();

        gameboy.pc = gameboy.pc.wrapping_add(1);
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
