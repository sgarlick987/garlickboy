use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    gameboy::{Gameboy, GameboyCycle},
    utils::split_bytes,
};

struct Interrupt {
    upper: u8,
    lower: u8,
}

struct InterruptWrapper {
    interrupt: Rc<RefCell<Interrupt>>,
    executions: VecDeque<GameboyCycle>,
}

impl Iterator for InterruptWrapper {
    type Item = GameboyCycle;

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

pub fn new() -> Box<dyn ExactSizeIterator<Item = GameboyCycle>> {
    let mut executions: VecDeque<GameboyCycle> = VecDeque::with_capacity(5);
    let interrupt = Rc::new(RefCell::new(Interrupt { upper: 0, lower: 0 }));

    executions.push_back(Box::new(move |_: &mut Gameboy| {
        //nop
    }));

    executions.push_back(Box::new(move |_: &mut Gameboy| {
        //nop
    }));

    let interrupt_ref = interrupt.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut interrupt = interrupt_ref.borrow_mut();
        (interrupt.upper, interrupt.lower) = split_bytes(gameboy.pc);
        gameboy.push(interrupt.upper);
    }));

    let interrupt_ref = interrupt.clone();
    executions.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let interrupt = interrupt_ref.borrow();
        gameboy.push(interrupt.lower);
    }));

    executions.push_back(Box::new(move |gameboy: &mut Gameboy| gameboy.pc = 0x0040));

    Box::new(InterruptWrapper {
        interrupt,
        executions,
    })
}
