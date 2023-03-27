use std::collections::VecDeque;

use crate::gameboy::{Gameboy, GameboyCycle};

// HALT - 0x76
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/misc
// Timing
// without branch (4t)
// fetch	This can actually last forever
struct Inst {
    executions: VecDeque<GameboyCycle>,
}

pub fn new() -> Box<dyn Iterator<Item = GameboyCycle>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions.push_back(Box::new(|gameboy: &mut Gameboy| {
        if !gameboy.halted {
            gameboy.halted = true;
        }
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

#[cfg(test)]
mod tests {
    use crate::gameboy::bus::*;
    use coverage_helper::test;

    use super::*;

    #[test]
    fn test_di() {
        const PC: u16 = 1;

        let bus = Box::new(MockBus::new());
        let mut gameboy = Gameboy::new(bus);

        let mut cycles = 0;
        for inst in new() {
            inst(&mut gameboy);
            cycles += 1;
        }

        assert_eq!(cycles, 1);
        assert_eq!(gameboy.pc, PC);
    }
}
