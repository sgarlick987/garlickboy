use super::execute::Executed;
use super::Comparison;
use crate::cpu::CPU;
use crate::utils::*;

pub trait Jump {
    fn jp(&mut self) -> Executed;
    fn jr(&mut self) -> Executed;
    fn jrf(&mut self, comparison: &Comparison) -> Executed;
    fn call(&mut self) -> Executed;
    fn ret(&mut self) -> Executed;
}

impl Jump for CPU {
    fn jp(&mut self) -> Executed {
        let address = merge_bytes(self.read_byte(self.pc + 2), self.read_byte(self.pc + 1));

        Executed {
            cycles_used: 16,
            next_pc: address,
        }
    }

    fn jr(&mut self) -> Executed {
        let offset = self.read_byte(self.pc + 1) as i8;

        Executed {
            cycles_used: 12,
            next_pc: self.pc.wrapping_add(2).wrapping_add(offset as u16),
        }
    }

    fn jrf(&mut self, comparison: &Comparison) -> Executed {
        // init assuming no branch
        let mut pc = self.pc.wrapping_add(2);
        let mut cycles = 8;

        match comparison {
            Comparison::NONZERO => {
                if !self.registers.flags.zero {
                    let offset = self.read_byte(self.pc + 1) as i8;

                    pc = self.pc.wrapping_add(2).wrapping_add(offset as u16);
                    cycles = 12;
                }
            }
            Comparison::ZERO => {
                if self.registers.flags.zero {
                    let offset = self.read_byte(self.pc + 1) as i8;

                    pc = self.pc.wrapping_add(2).wrapping_add(offset as u16);
                    cycles = 12;
                }
            }
            _ => {
                panic!("{:?} unimplemented JRF Instruction", comparison);
            }
        }

        Executed {
            cycles_used: cycles,
            next_pc: pc,
        }
    }

    fn call(&mut self) -> Executed {
        //fetch
        let return_address = self.pc.wrapping_add(3);
        let bytes = split_bytes(return_address);
        let mut cycles = self.sync();

        //read lower
        let lower = self.read_byte(self.pc + 1);
        cycles += self.sync();

        //read upper
        let upper = self.read_byte(self.pc + 2);
        cycles += self.sync();

        //branch
        let pc = merge_bytes(upper, lower);
        cycles += self.sync();

        //write upper
        self.registers.sp -= 1;
        self.write_byte(self.registers.sp, bytes[0]);
        cycles += self.sync();

        //write lower
        self.registers.sp -= 1;
        self.write_byte(self.registers.sp, bytes[1]);
        cycles += self.sync();

        Executed {
            cycles_used: cycles,
            next_pc: pc,
        }
    }

    fn ret(&mut self) -> Executed {
        //fetch
        let mut cycles = self.sync();

        //read lower
        let lower = self.read_byte(self.registers.sp);
        self.registers.sp += 1;
        cycles += self.sync();

        //read upper
        let upper = self.read_byte(self.registers.sp);
        self.registers.sp += 1;
        cycles += self.sync();

        //set pc
        let pc = merge_bytes(upper, lower);
        cycles += self.sync();

        Executed {
            cycles_used: cycles,
            next_pc: pc,
        }
    }
}
