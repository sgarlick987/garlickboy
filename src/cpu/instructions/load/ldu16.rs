use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{cpu::instructions::TargetRegister16, cpu::GameboyChip};

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
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(
    target: &TargetRegister16,
) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let inst = Rc::new(RefCell::new(Inst {
        upper: 0,
        lower: 0,
        target: target.clone(),
    }));

    let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> = VecDeque::with_capacity(3);

    executions.push_back(Box::new(|_: &mut GameboyChip| {
        //fetch
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        inst.lower = chip.read_byte_pc_lower();
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        inst.upper = chip.read_byte_pc_upper();

        match inst.target {
            TargetRegister16::SP => {
                chip.registers.set_sp(inst.upper, inst.lower);
            }
            TargetRegister16::HL => {
                chip.registers.l = inst.lower;
                chip.registers.h = inst.upper;
            }
            TargetRegister16::DE => {
                chip.registers.e = inst.lower;
                chip.registers.d = inst.upper;
            }
            TargetRegister16::BC => {
                chip.registers.c = inst.lower;
                chip.registers.b = inst.upper;
            }
            _ => {
                panic!("{:?} unimplemented LDU16", inst.target);
            }
        }

        chip.pc = chip.pc.wrapping_add(3);
    }));

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}
