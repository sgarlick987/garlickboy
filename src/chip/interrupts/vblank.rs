use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{chip::GameboyChip, utils::split_bytes};

struct Interrupt {
    upper: u8,
    lower: u8,
}

struct InterruptWrapper {
    interrupt: Rc<RefCell<Interrupt>>,
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

impl Iterator for InterruptWrapper {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}

impl ExactSizeIterator for InterruptWrapper {
    fn len(&self) -> usize {
        self.executions.len()
    }
}
pub fn new() -> Box<dyn ExactSizeIterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>> = VecDeque::with_capacity(5);
    let interrupt = Rc::new(RefCell::new(Interrupt { upper: 0, lower: 0 }));

    executions.push_back(Box::new(move |_: &mut GameboyChip| {
        //nop
    }));

    let interrupt_ref = interrupt.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let mut interrupt = interrupt_ref.borrow_mut();
        (interrupt.upper, interrupt.lower) = split_bytes(chip.pc);
    }));

    let interrupt_ref = interrupt.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let interrupt = interrupt_ref.borrow();
        chip.push(interrupt.upper);
    }));

    let interrupt_ref = interrupt.clone();
    executions.push_back(Box::new(move |chip: &mut GameboyChip| {
        let interrupt = interrupt_ref.borrow();
        chip.push(interrupt.lower);
    }));

    executions.push_back(Box::new(move |chip: &mut GameboyChip| chip.pc = 0x0040));

    Box::new(InterruptWrapper {
        interrupt,
        executions,
    })
}
