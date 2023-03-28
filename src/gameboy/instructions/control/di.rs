use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// DI - 0xF3
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/misc
// Timing
// without branch (4t)
// fetch
struct Inst {
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn ExactSizeIterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions.push_back(Box::new(|gameboy: &mut Gameboy| {
        gameboy.interrupt_handler.disable_ime();
        gameboy.pc = gameboy.pc.wrapping_add(1);
    }));

    Box::new(inst)
}

impl Iterator for Inst {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executions.is_empty() {
            return None;
        }

        self.executions.pop_front()
    }
}

impl ExactSizeIterator for Inst {
    fn len(&self) -> usize {
        self.executions.len()
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[test]
    fn test_di() {
        const PC: u16 = 1;
        const CYCLES: usize = 1;

        let mut gameboy = Gameboy::new();

        let cycles = new();
        assert_eq!(cycles.len(), CYCLES);
        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, PC);
    }
}
