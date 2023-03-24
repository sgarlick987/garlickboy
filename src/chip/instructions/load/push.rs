use std::collections::VecDeque;

use crate::chip::{instructions::TargetPushPop, GameboyChip};

// PUSH BC - 0xC5
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/lsm
// Timingwithout branch (16t)
// fetch
// internal
// write	B->(--SP)
// write	C->(--SP)
struct Inst {
    target: TargetPushPop,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(target: &TargetPushPop) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(4),
    };

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //internal
    }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| match inst.target {
            TargetPushPop::AF => {
                chip.push(chip.registers.a);
            }
            TargetPushPop::HL => {
                chip.push(chip.registers.h);
            }
            TargetPushPop::BC => {
                chip.push(chip.registers.b);
            }
            TargetPushPop::DE => {
                chip.push(chip.registers.d);
            }
        }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            match inst.target {
                TargetPushPop::AF => {
                    chip.push(chip.registers.get_f());
                }
                TargetPushPop::HL => {
                    chip.push(chip.registers.l);
                }
                TargetPushPop::BC => {
                    chip.push(chip.registers.c);
                }
                TargetPushPop::DE => {
                    chip.push(chip.registers.e);
                }
            }
            chip.pc = chip.pc.wrapping_add(1);
        }));

    Box::new(inst)
}

impl Iterator for Inst {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
