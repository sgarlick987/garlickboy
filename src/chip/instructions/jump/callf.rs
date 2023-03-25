use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    chip::{instructions::Comparison, GameboyChip},
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
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new(comparison: &Comparison) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let inst = Rc::new(RefCell::new(Inst {
        upper: 0,
        lower: 0,
        return_upper: 0,
        return_lower: 0,
        branch: false,
        comparison: comparison.clone(),
    }));

    let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> = VecDeque::with_capacity(3);

    executions.push_back(Box::new(move |_: &mut GameboyChip| {
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

        inst.branch = match inst.comparison {
            Comparison::NONZERO => !chip.zero_flag(),
            Comparison::ZERO => chip.zero_flag(),
            Comparison::CARRY => chip.carry_flag(),
            Comparison::NOCARRY => !chip.carry_flag(),
        };

        if !inst.branch {
            chip.pc = chip.pc.wrapping_add(3);
        }
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut inst = inst_ref.borrow_mut();
        (inst.return_upper, inst.return_lower) = split_bytes(chip.pc.wrapping_add(3));
        let address = merge_bytes(inst.upper, inst.lower);
        chip.pc = address;
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let inst = inst_ref.borrow_mut();
        chip.push(inst.return_upper);
    }));

    let inst_ref = inst.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let inst = inst_ref.borrow_mut();
        chip.push(inst.return_lower);
    }));

    Box::new(InstWrapper { inst, executions })
}

impl Iterator for InstWrapper {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            let mut inst = self.inst.borrow_mut();
            if inst.branch {
                inst.branch = false;
                let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> =
                    VecDeque::with_capacity(3);

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |chip: &mut GameboyChip| {
                    let mut inst = inst_ref.borrow_mut();
                    let (upper, lower) = split_bytes(chip.pc);
                    inst.return_upper = upper;
                    inst.return_lower = lower;
                }));

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |chip: &mut GameboyChip| {
                    let inst = inst_ref.borrow();
                    chip.push(inst.return_upper);
                }));

                let inst_ref = self.inst.clone();
                executions.push_back(Box::new(move |chip: &mut GameboyChip| {
                    let inst = inst_ref.borrow();
                    chip.push(inst.return_upper);
                    chip.pc = merge_bytes(inst.upper, inst.lower);
                }));

                self.executions = executions;

                return self.executions.pop_front();
            }
            return None;
        }
        self.executions.pop_front()
    }
}
