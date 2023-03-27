use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    gameboy::Gameboy,
    gameboy::{instructions::TargetRegister16, GameboyCycle},
};

// LD BC,u16 - 0x01
// Length: 3 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/lsm
// Timingwithout branch (12t)
// fetch
// read	u16:lower->C
// read	u16:upper->B
struct Inst {
    lower: u8,
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
        lower: 0,
        target: target.clone(),
    }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    executions.push_back(Box::new(|_: &mut Gameboy| {
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

        match inst.target {
            TargetRegister16::SP => {
                gameboy.registers.set_sp(inst.upper, inst.lower);
            }
            TargetRegister16::HL => {
                gameboy.registers.l = inst.lower;
                gameboy.registers.h = inst.upper;
            }
            TargetRegister16::DE => {
                gameboy.registers.e = inst.lower;
                gameboy.registers.d = inst.upper;
            }
            TargetRegister16::BC => {
                gameboy.registers.c = inst.lower;
                gameboy.registers.b = inst.upper;
            }
            _ => {
                panic!("{:?} unimplemented LDU16", inst.target);
            }
        }

        gameboy.pc = gameboy.pc.wrapping_add(3);
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
