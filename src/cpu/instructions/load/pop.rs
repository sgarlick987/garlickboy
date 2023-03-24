use std::collections::VecDeque;

use crate::cpu::{instructions::TargetPushPop, registers::FlagsRegister, GameboyChip};

// POP BC - 0xC1
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: x16/lsm
// Timingwithout branch (12t)
// fetch
// read	(SP++)->C
// read	(SP++)->B
struct Inst {
    target: TargetPushPop,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(target: &TargetPushPop) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        target: target.clone(),
        executions: VecDeque::with_capacity(3),
    };

    inst.executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| match inst.target {
            TargetPushPop::AF => {
                chip.registers.flags = FlagsRegister::from(chip.pop());
            }
            TargetPushPop::HL => {
                chip.registers.l = chip.pop();
            }
            TargetPushPop::BC => {
                chip.registers.c = chip.pop();
            }
            TargetPushPop::DE => {
                chip.registers.e = chip.pop();
            }
        }));

    inst.executions
        .push_back(Box::new(move |chip: &mut GameboyChip| {
            match inst.target {
                TargetPushPop::AF => {
                    chip.registers.a = chip.pop();
                }
                TargetPushPop::HL => {
                    chip.registers.h = chip.pop();
                }
                TargetPushPop::BC => {
                    chip.registers.b = chip.pop();
                }
                TargetPushPop::DE => {
                    chip.registers.d = chip.pop();
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
