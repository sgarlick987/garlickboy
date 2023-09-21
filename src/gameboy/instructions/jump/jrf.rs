use super::comparison_branch;
use crate::gameboy::{instructions::Comparison, Gameboy, GameboyCycle, GameboyCycles};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// JR Z,i8 - 0x28
// Length: 2 bytes
// FlagsZero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (8t)
// fetch	fetch
// with branch (12t)
// read	read
// i8	i8
//     internal
//     modify PC
struct Context {
    offset: i8,
    branch: bool,
    comparison: Comparison,
}

struct ContextWrapper {
    context: Rc<RefCell<Context>>,
    cycles: VecDeque<GameboyCycle>,
}

pub fn new(comparison: &Comparison) -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context {
        offset: 0,
        branch: false,
        comparison: comparison.clone(),
    }));

    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.branch = comparison_branch(gameboy, &context.comparison);
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.offset = gameboy.read_byte_pc_lower() as i8;

        if !context.branch {
            gameboy.pc = gameboy.pc.wrapping_add(2);
        }
    }));

    Box::new(ContextWrapper { context, cycles })
}

impl ExactSizeIterator for ContextWrapper {
    fn len(&self) -> usize {
        if self.context.borrow().branch {
            self.cycles.len() + 1
        } else {
            self.cycles.len()
        }
    }
}

impl Iterator for ContextWrapper {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles.is_empty() {
            let mut context = self.context.borrow_mut();
            if context.branch {
                let offset = context.offset;
                context.branch = false;
                return Some(Box::new(move |gameboy: &mut Gameboy| {
                    gameboy.pc = gameboy.pc.wrapping_add(2).wrapping_add(offset as u16);
                }));
            }
            return None;
        }

        self.cycles.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use crate::gameboy::bus::HRAM_ADDRESS_START;

    use super::*;
    use coverage_helper::test;

    const PC: u16 = HRAM_ADDRESS_START;
    const COMPARISONS: [Comparison; 4] = [
        Comparison::NONZERO,
        Comparison::ZERO,
        Comparison::CARRY,
        Comparison::NOCARRY,
    ];

    #[test]
    fn test_jrf_without_branch() {
        const CYCLES: usize = 2;
        const LENGTH: u16 = 2;

        for comparison in COMPARISONS {
            let mut gameboy = Gameboy::new();
            gameboy.pc = PC;
            match comparison {
                Comparison::CARRY => gameboy.reset_carry_flag(),
                Comparison::NOCARRY => gameboy.set_carry_flag(),
                Comparison::NONZERO => gameboy.set_zero_flag(),
                Comparison::ZERO => gameboy.reset_zero_flag(),
            }
            let cycles = new(&comparison);
            assert_eq!(cycles.len(), CYCLES);

            for (i, cycle) in cycles.enumerate() {
                if i > CYCLES - 1 {
                    assert!(false);
                }
                gameboy.execute(cycle);
            }

            assert_eq!(gameboy.pc, PC + LENGTH);
        }
    }

    #[test]
    fn test_jrf_with_branch_positive() {
        const CYCLES: usize = 2;
        const OFFSET: i8 = 5;

        for comparison in COMPARISONS {
            let mut gameboy = Gameboy::new();
            gameboy.pc = PC;
            gameboy.write_byte(PC + 1, OFFSET as u8);
            match comparison {
                Comparison::NOCARRY => gameboy.reset_carry_flag(),
                Comparison::CARRY => gameboy.set_carry_flag(),
                Comparison::ZERO => gameboy.set_zero_flag(),
                Comparison::NONZERO => gameboy.reset_zero_flag(),
            }
            let cycles = new(&comparison);
            assert_eq!(cycles.len(), CYCLES);

            let mut extra_cycle = false;
            for (i, cycle) in cycles.enumerate() {
                if i == CYCLES {
                    extra_cycle = true;
                }
                if i > CYCLES {
                    assert!(false);
                }
                gameboy.execute(cycle);
            }
            assert!(extra_cycle);
            assert_eq!(gameboy.pc, PC.wrapping_add(2).wrapping_add(OFFSET as u16));
        }
    }

    #[test]
    fn test_jrf_with_branch_negative() {
        const CYCLES: usize = 2;
        const OFFSET: i8 = -5;

        for comparison in COMPARISONS {
            let mut gameboy = Gameboy::new();
            gameboy.pc = PC;
            gameboy.write_byte(PC + 1, OFFSET as u8);
            match comparison {
                Comparison::NOCARRY => gameboy.reset_carry_flag(),
                Comparison::CARRY => gameboy.set_carry_flag(),
                Comparison::ZERO => gameboy.set_zero_flag(),
                Comparison::NONZERO => gameboy.reset_zero_flag(),
            }
            let cycles = new(&comparison);
            assert_eq!(cycles.len(), CYCLES);

            let mut extra_cycle = false;
            for (i, cycle) in cycles.enumerate() {
                if i == CYCLES {
                    extra_cycle = true;
                }
                if i > CYCLES {
                    assert!(false);
                }
                gameboy.execute(cycle);
            }
            assert!(extra_cycle);
            assert_eq!(gameboy.pc, PC.wrapping_add(2).wrapping_add(OFFSET as u16));
        }
    }
}
