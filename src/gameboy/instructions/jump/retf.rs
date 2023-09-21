use super::comparison_branch;
use crate::{
    gameboy::{instructions::Comparison, Gameboy, GameboyCycle, GameboyCycles},
    utils::merge_bytes,
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// RET Z - 0xC8
// Length: 1 byte
// Flags
// Zero	unmodified
// Negative	unmodified
// Half Carry	unmodified
// Carry	unmodified
// Group: control/br
// Timing
// without branch (8t)	with branch (20t)
// fetch	fetch
// internal	internal
// branch decision?	branch decision?
// read
// (SP++)->lower
// read
// (SP++)->upper
// internal
// set PC?
struct Context {
    branch: bool,
    comparison: Comparison,
    upper: u8,
    lower: u8,
}

struct ContextWrapper {
    context: Rc<RefCell<Context>>,
    cycles: VecDeque<GameboyCycle>,
}

pub fn new(comparison: &Comparison) -> GameboyCycles {
    let context = Rc::new(RefCell::new(Context {
        branch: false,
        comparison: comparison.clone(),
        upper: 0,
        lower: 0,
    }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(2);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {}));

    let context_ref = context.clone();
    cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
        let mut context = context_ref.borrow_mut();
        context.branch = comparison_branch(gameboy, &context.comparison);

        if !context.branch {
            gameboy.pc = gameboy.pc.wrapping_add(1);
        }
    }));

    Box::new(ContextWrapper { context, cycles })
}

impl ExactSizeIterator for ContextWrapper {
    fn len(&self) -> usize {
        if self.context.borrow().branch {
            self.cycles.len() + 3
        } else {
            self.cycles.len()
        }
    }
}

impl Iterator for ContextWrapper {
    type Item = GameboyCycle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles.is_empty() {
            let context = self.context.clone();
            let mut context = context.borrow_mut();
            if context.branch {
                let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);
                context.branch = false;

                let context_ref = self.context.clone();
                cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let mut context = context_ref.borrow_mut();
                    context.lower = gameboy.pop();
                }));

                let context_ref = self.context.clone();
                cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let mut context = context_ref.borrow_mut();
                    context.upper = gameboy.pop();
                }));

                let context_ref = self.context.clone();
                cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let context = context_ref.borrow();
                    gameboy.pc = merge_bytes(context.upper, context.lower);
                }));
                self.cycles = cycles;
                return self.cycles.pop_front();
            }
            return None;
        }

        self.cycles.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::split_bytes;

    use super::*;
    use coverage_helper::test;

    const CYCLES: usize = 2;
    const RETURN_ADDRESS: u16 = 0xFF00;
    const STACK_ADDRESS: u16 = 0xFFFE;
    const PC: u16 = 0x1234;
    const LENGTH: u16 = 1;

    const COMPARISONS: [Comparison; 4] = [
        Comparison::NONZERO,
        Comparison::ZERO,
        Comparison::CARRY,
        Comparison::NOCARRY,
    ];

    #[test]
    fn test_ret_with_branch() {
        for comparison in COMPARISONS {
            let gameboy = &mut Gameboy::new();
            let cycles = new(&comparison);
            let (upper, lower) = split_bytes(RETURN_ADDRESS);
            gameboy.pc = PC;
            match comparison {
                Comparison::NOCARRY => gameboy.reset_carry_flag(),
                Comparison::CARRY => gameboy.set_carry_flag(),
                Comparison::ZERO => gameboy.set_zero_flag(),
                Comparison::NONZERO => gameboy.reset_zero_flag(),
            }
            gameboy.registers.set_sp(STACK_ADDRESS);
            gameboy.push(upper);
            gameboy.push(lower);
            assert_eq!(cycles.len(), CYCLES);

            for cycle in cycles {
                gameboy.execute(cycle);
            }

            assert_eq!(gameboy.pc, RETURN_ADDRESS);
            assert_eq!(gameboy.registers.get_sp(), STACK_ADDRESS);
        }
    }

    #[test]
    fn test_call_without_branch() {
        for comparison in COMPARISONS {
            let gameboy = &mut Gameboy::new();
            let cycles = new(&comparison);
            let (upper, lower) = split_bytes(RETURN_ADDRESS);
            gameboy.pc = PC;
            match comparison {
                Comparison::CARRY => gameboy.reset_carry_flag(),
                Comparison::NOCARRY => gameboy.set_carry_flag(),
                Comparison::NONZERO => gameboy.set_zero_flag(),
                Comparison::ZERO => gameboy.reset_zero_flag(),
            }
            gameboy.registers.set_sp(STACK_ADDRESS);
            gameboy.push(upper);
            gameboy.push(lower);
            assert_eq!(cycles.len(), CYCLES);

            for cycle in cycles {
                gameboy.execute(cycle);
            }

            assert_eq!(gameboy.pc, PC.wrapping_add(LENGTH));
            assert_eq!(gameboy.pop(), lower);
            assert_eq!(gameboy.pop(), upper);
            assert_eq!(gameboy.registers.get_sp(), STACK_ADDRESS);
        }
    }
}
