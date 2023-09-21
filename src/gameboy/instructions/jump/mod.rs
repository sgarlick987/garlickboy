use super::Comparison;
use crate::gameboy::Gameboy;

pub mod call;
pub mod callf;
pub mod jp;
pub mod jpf;
pub mod jphl;
pub mod jr;
pub mod jrf;
pub mod ret;
pub mod retf;
pub mod reti;
pub mod rst;

fn comparison_branch(gameboy: &mut Gameboy, comparison: &Comparison) -> bool {
    match comparison {
        Comparison::NONZERO => !gameboy.zero_flag(),
        Comparison::ZERO => gameboy.zero_flag(),
        Comparison::CARRY => gameboy.carry_flag(),
        Comparison::NOCARRY => !gameboy.carry_flag(),
    }
}
