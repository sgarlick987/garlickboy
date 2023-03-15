use super::{execute::Executed, TargetRegister8};
use crate::cpu::CPU;

pub trait Bitwise {
    fn bit(&mut self, bit: &u8, target: &TargetRegister8) -> Executed;
    fn rla(&mut self) -> Executed;
    fn rl(&mut self, target: &TargetRegister8) -> Executed;
}

impl Bitwise for CPU {
    fn bit(&mut self, bit: &u8, target: &TargetRegister8) -> Executed {
        match target {
            TargetRegister8::H => {
                self.registers.flags.negative = false;
                self.registers.flags.half_carry = true;

                let check = 1 << bit;
                self.registers.flags.zero = self.registers.h & check == 0;
            }
            _ => {
                panic!("{:?} unimplemented BIT Instruction", target);
            }
        }

        Executed {
            cycles_used: 12,
            next_pc: self.pc.wrapping_add(2),
        }
    }

    // RL C - 0x11
    // Length: 2 bytes
    // FlagsZero	dependent
    // Negative	unset
    // Half Carry	unset
    // Carry	dependent
    // Group: x8/rsb
    // Timingwithout branch (8t)
    // fetch	(0xCB)
    // fetch
    fn rl(&mut self, target: &TargetRegister8) -> Executed {
        let next_pc = self.pc.wrapping_add(2);

        //fetch
        let mut cycles_used = self.sync();

        //fetch
        cycles_used += self.sync();

        self.registers.flags.half_carry = false;
        self.registers.flags.negative = false;
        match target {
            TargetRegister8::C => {
                let mut new_c = self.registers.c << 1;

                if self.registers.flags.carry {
                    new_c |= 1;
                }

                self.registers.flags.carry = self.registers.c >> 7 == 1;

                self.registers.flags.zero = new_c == 0;
                self.registers.c = new_c;
            }
            _ => {
                panic!("{:?} unimplemented RL Instruction", target);
            }
        }

        Executed {
            cycles_used,
            next_pc,
        }
    }

    // RLA - 0x17
    // Length: 1 byte
    // FlagsZero	unset
    // Negative	unset
    // Half Carry	unset
    // Carry	dependent
    // Group: x8/rsb
    // Timingwithout branch (4t)
    // fetch
    fn rla(&mut self) -> Executed {
        let next_pc = self.pc.wrapping_add(1);

        //fetch
        let cycles_used = self.sync();

        self.registers.flags.half_carry = false;
        self.registers.flags.negative = false;
        self.registers.flags.zero = false;

        let mut new_a = self.registers.a << 1;

        if self.registers.flags.carry {
            new_a |= 1;
        }

        self.registers.flags.carry = self.registers.a >> 7 == 1;

        self.registers.a = new_a;

        Executed {
            cycles_used,
            next_pc,
        }
    }
}
