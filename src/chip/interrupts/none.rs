use std::collections::VecDeque;

use crate::chip::GameboyChip;

struct Interrupt {
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

impl Iterator for Interrupt {
    type Item = Box<dyn FnOnce(&mut GameboyChip)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}

impl ExactSizeIterator for Interrupt {
    fn len(&self) -> usize {
        self.executions.len()
    }
}

pub fn new() -> Box<dyn ExactSizeIterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    Box::new(Interrupt {
        executions: VecDeque::with_capacity(0),
    })
}
