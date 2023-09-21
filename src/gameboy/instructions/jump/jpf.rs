use super::comparison_branch;
use crate::{
    gameboy::{instructions::Comparison, Gameboy, GameboyCycle, GameboyCycles},
    utils::merge_bytes,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// JP Z,u16 - 0xCA
// Length: 3 bytes
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (12t)	with branch (16t)
// fetch	fetch
// read	read
// u16:lower	u16:lower
// read	read
// u16:upper	u16:upper
// internal
// branch decision?
struct Context {
    upper: u8,
    lower: u8,
    branch: bool,
    comparison: Comparison,
}
struct ContextWrapper {
    context: Rc<RefCell<Context>>,
    cycles: VecDeque<GameboyCycle>,
}

pub fn new(comparison: &Comparison) -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context {
        upper: 0,
        lower: 0,
        branch: false,
        comparison: comparison.clone(),
    }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.branch = comparison_branch(gameboy, &context.comparison);

        if !context.branch {
            gameboy.pc = gameboy.pc.wrapping_add(3);
        }
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.lower = gameboy.read_byte_pc_lower();
    }));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.upper = gameboy.read_byte_pc_upper();
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
                let address = merge_bytes(context.upper, context.lower);
                context.branch = false;
                return Some(Box::new(move |gameboy: &mut Gameboy| {
                    gameboy.pc = address;
                }));
            }
            return None;
        }

        self.cycles.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gameboy::bus::HRAM_ADDRESS_START, utils::split_bytes};
    use coverage_helper::test;

    const CYCLES: usize = 3;
    const JP_ADDRESS: u16 = 0xFF00;
    const PC: u16 = HRAM_ADDRESS_START;
    const LENGTH: u16 = 3;
    const COMPARISONS: [Comparison; 4] = [
        Comparison::NONZERO,
        Comparison::ZERO,
        Comparison::CARRY,
        Comparison::NOCARRY,
    ];

    #[test]
    fn test_jp_with_branch() {
        for comparison in COMPARISONS {
            let gameboy = &mut Gameboy::new();
            let cycles = new(&comparison);
            let (upper, lower) = split_bytes(JP_ADDRESS);
            gameboy.pc = PC;
            match comparison {
                Comparison::NOCARRY => gameboy.reset_carry_flag(),
                Comparison::CARRY => gameboy.set_carry_flag(),
                Comparison::ZERO => gameboy.set_zero_flag(),
                Comparison::NONZERO => gameboy.reset_zero_flag(),
            }
            gameboy.write_byte(gameboy.pc + 1, lower);
            gameboy.write_byte(gameboy.pc + 2, upper);
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
            assert_eq!(gameboy.pc, JP_ADDRESS);
        }
    }

    #[test]
    fn test_jp_without_branch() {
        for comparison in COMPARISONS {
            let gameboy = &mut Gameboy::new();
            let cycles = new(&comparison);
            let (upper, lower) = split_bytes(JP_ADDRESS);
            gameboy.pc = PC;
            match comparison {
                Comparison::CARRY => gameboy.reset_carry_flag(),
                Comparison::NOCARRY => gameboy.set_carry_flag(),
                Comparison::NONZERO => gameboy.set_zero_flag(),
                Comparison::ZERO => gameboy.reset_zero_flag(),
            }
            gameboy.write_byte(gameboy.pc + 1, lower);
            gameboy.write_byte(gameboy.pc + 2, upper);
            assert_eq!(cycles.len(), CYCLES);

            for cycle in cycles {
                gameboy.execute(cycle);
            }

            assert_eq!(gameboy.pc, PC.wrapping_add(LENGTH));
        }
    }
}
