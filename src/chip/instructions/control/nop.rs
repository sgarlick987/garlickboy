use std::collections::VecDeque;

use crate::chip::GameboyChip;

// NOP - 0x00
// Length: 1 byte
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/misc
// Timingwithout branch (4t)
// fetch
struct Inst {
    executions: VecDeque<Box<dyn FnOnce(&mut GameboyChip)>>,
}

pub fn new() -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
    let mut inst = Inst {
        executions: VecDeque::with_capacity(1),
    };

    inst.executions.push_back(Box::new(|cpu: &mut GameboyChip| {
        cpu.pc = cpu.pc.wrapping_add(1);
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

#[cfg(test)]
mod tests {
    use crate::chip::address::*;
    use coverage_helper::test;

    use super::*;

    #[test]
    fn test_nop() {
        const PC: u16 = 1;

        let bus = Box::new(MockBus::new());
        let mut chip = GameboyChip::new(bus);

        let mut cycles = 0;
        for inst in new() {
            inst(&mut chip);
            cycles += 1;
        }

        assert_eq!(cycles, 1);
        assert_eq!(chip.pc, PC);
    }
}
