use super::{execute::Executed};
use crate::cpu::CPU;

pub trait Control {
    fn nop(&mut self) -> Executed;
}

impl Control for CPU {
    fn nop(&mut self) -> Executed {
        //fetch
        let cycles_used = self.sync();
        let next_pc = self.pc + 1;

        Executed {
            cycles_used,
            next_pc,
        }
    }
}
