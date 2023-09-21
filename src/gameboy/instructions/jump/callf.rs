use super::comparison_branch;
use crate::{
    gameboy::{instructions::Comparison, Gameboy, GameboyCycle, GameboyCycles},
    utils::{merge_bytes, split_bytes},
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
struct Context {
    upper: u8,
    lower: u8,
    return_upper: u8,
    return_lower: u8,
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
        return_upper: 0,
        return_lower: 0,
        branch: false,
        comparison: comparison.clone(),
    }));
    let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

    cycles.push_back(Box::new(move |_: &mut Gameboy| {
        //fetch
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
        context.branch = comparison_branch(gameboy, &context.comparison);

        if !context.branch {
            gameboy.pc = gameboy.pc.wrapping_add(3);
        }
    }));

    Box::new(ContextWrapper {
        context: context,
        cycles,
    })
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
            let mut context = self.context.borrow_mut();
            if context.branch {
                context.branch = false;
                let mut cycles: VecDeque<GameboyCycle> = VecDeque::with_capacity(3);

                let context_ref = self.context.clone();
                cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let mut context = context_ref.borrow_mut();
                    let (upper, lower) = split_bytes(gameboy.pc.wrapping_add(3));
                    context.return_upper = upper;
                    context.return_lower = lower;
                }));

                let context_ref = self.context.clone();
                cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let context = context_ref.borrow();
                    gameboy.push(context.return_upper);
                }));

                let context_ref = self.context.clone();
                cycles.push_back(Box::new(move |gameboy: &mut Gameboy| {
                    let context = context_ref.borrow();
                    gameboy.push(context.return_lower);
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
    use crate::gameboy::bus::HRAM_ADDRESS_START;

    use super::*;
    use coverage_helper::test;

    const CYCLES: usize = 3;
    const CALL_ADDRESS: u16 = 0xFF00;
    const STACK_ADDRESS: u16 = 0xFFFE;
    const PC: u16 = HRAM_ADDRESS_START;
    const LENGTH: u16 = 3;

    const COMPARISONS: [Comparison; 4] = [
        Comparison::NONZERO,
        Comparison::ZERO,
        Comparison::CARRY,
        Comparison::NOCARRY,
    ];

    #[test]
    fn test_call_with_branch() {
        for comparison in COMPARISONS {
            let gameboy = &mut Gameboy::new();
            let cycles = new(&comparison);
            let (upper, lower) = split_bytes(CALL_ADDRESS);
            gameboy.pc = PC;
            match comparison {
                Comparison::NOCARRY => gameboy.reset_carry_flag(),
                Comparison::CARRY => gameboy.set_carry_flag(),
                Comparison::ZERO => gameboy.set_zero_flag(),
                Comparison::NONZERO => gameboy.reset_zero_flag(),
            }
            gameboy.registers.set_sp(STACK_ADDRESS);
            gameboy.write_byte(gameboy.pc + 1, lower);
            gameboy.write_byte(gameboy.pc + 2, upper);
            assert_eq!(cycles.len(), CYCLES);

            for cycle in cycles {
                gameboy.execute(cycle);
            }

            assert_eq!(gameboy.pc, CALL_ADDRESS);
            let (upper, lower) = split_bytes(PC + 3);
            assert_eq!(gameboy.registers.get_sp(), STACK_ADDRESS.wrapping_sub(2));
            assert_eq!(gameboy.pop(), lower);
            assert_eq!(gameboy.pop(), upper);
        }
    }

    #[test]
    fn test_call_without_branch() {
        for comparison in COMPARISONS {
            let gameboy = &mut Gameboy::new();
            let cycles = new(&comparison);
            let (upper, lower) = split_bytes(CALL_ADDRESS);
            gameboy.pc = PC;
            match comparison {
                Comparison::CARRY => gameboy.reset_carry_flag(),
                Comparison::NOCARRY => gameboy.set_carry_flag(),
                Comparison::NONZERO => gameboy.set_zero_flag(),
                Comparison::ZERO => gameboy.reset_zero_flag(),
            }
            gameboy.registers.set_sp(STACK_ADDRESS);
            gameboy.write_byte(gameboy.pc + 1, lower);
            gameboy.write_byte(gameboy.pc + 2, upper);
            assert_eq!(cycles.len(), CYCLES);

            for cycle in cycles {
                gameboy.execute(cycle);
            }

            assert_eq!(gameboy.pc, PC.wrapping_add(LENGTH));
            assert_eq!(gameboy.registers.get_sp(), STACK_ADDRESS);
        }
    }
}
