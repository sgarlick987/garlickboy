use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    gameboy::{instructions::Comparison, Gameboy, GameboyCycle},
    utils::{merge_bytes, split_bytes},
};

// CALL NZ,u16 - 0xC4
// Length: 3 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (12t)	with branch (24t)
// fetch	fetch
// read	read
// u16:lower	u16:lower
// read	read
// u16:upper	u16:upper
// internal
// branch decision?
// write
// PC:upper->(--SP)
// write
// PC:lower->(--SP)
pub struct Inst {
    upper: u8,
    lower: u8,
    return_upper: u8,
    return_lower: u8,
    branch: bool,
    comparison: Comparison,
}

struct InstWrapper {
    inst: Rc<RefCell<Inst>>,
    executions: VecDeque<GameboyCycle>,
}

pub fn new(comparison: &Comparison) -> Box<dyn Iterator<Item = GameboyCycle>> {
    let inst = Rc::new(RefCell::new(Inst {
        upper: 0,
        lower: 0,
        return_upper: 0,
        return_lower: 0,
        branch: false,
        comparison: comparison.clone(),
    }));

    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

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

        inst.branch = match inst.comparison {
            Comparison::NONZERO => !gameboy.zero_flag(),
            Comparison::ZERO => gameboy.zero_flag(),
            Comparison::CARRY => gameboy.carry_flag(),
            Comparison::NOCARRY => !gameboy.carry_flag(),
        };

        if !inst.branch {
            gameboy.pc = gameboy.pc.wrapping_add(3);
        }
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
            let mut inst = self.inst.borrow_mut();
            if inst.branch {
                inst.branch = false;
                let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let mut inst = inst_ref.borrow_mut();
                    let (upper, lower) = split_bytes(gameboy.pc);
                    inst.return_upper = upper;
                    inst.return_lower = lower;
                }));

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let inst = inst_ref.borrow();
                    gameboy.push(inst.return_upper);
                }));

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let inst = inst_ref.borrow();
                    gameboy.push(inst.return_upper);
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
