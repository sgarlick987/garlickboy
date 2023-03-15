use super::{execute::Executed, *};
use crate::cpu::CPU;
use crate::utils::*;

pub trait Logic {
    fn xor_r8(&mut self, target: &TargetRegister8) -> Executed;
    fn adc_r8(&mut self, target: &TargetRegister8) -> Executed;
    fn add_r8(&mut self, target: &TargetRegister8) -> Executed;
    fn add_hl(&mut self) -> Executed;
    fn sub_r8(&mut self, target: &TargetRegister8) -> Executed;
    fn inc(&mut self, target: &TargetIncDec) -> Executed;
    fn dec(&mut self, target: &TargetIncDec) -> Executed;
    fn cp_hl(&mut self) -> Executed;
    fn cp_u8(&mut self) -> Executed;
}

impl Logic for CPU {
    fn xor_r8(&mut self, target: &TargetRegister8) -> Executed {
        match target {
            TargetRegister8::A => {
                self.registers.a ^= self.registers.a;
            }
            _ => {
                panic!("{:?} unimplemented XORR8", target);
            }
        }
        Executed {
            cycles_used: 4,
            next_pc: self.pc.wrapping_add(1),
        }
    }

    fn cp_u8(&mut self) -> Executed {
        //fetch
        let mut cycles_used = self.sync();
        let next_pc = self.pc.wrapping_add(2);

        //ready
        let byte = self.read_byte(self.pc + 1);
        let a = self.registers.a;
        cycles_used += self.sync();

        self.registers.flags.negative = true;
        self.registers.flags.zero = a == byte;
        self.registers.flags.carry = a < byte;
        self.registers.flags.half_carry = bytes_half_carry(a, byte);

        Executed {
            cycles_used,
            next_pc,
        }
    }

    fn cp_hl(&mut self) -> Executed {
        //fetch
        let mut cycles_used = self.sync();
        let next_pc = self.pc.wrapping_add(1);

        //read
        let hl = self.registers.get_hl();
        let byte = self.read_byte(hl);
        let a = self.registers.a;
        cycles_used += self.sync();

        self.registers.flags.negative = true;
        self.registers.flags.zero = a == byte;
        self.registers.flags.carry = a < byte;
        self.registers.flags.half_carry = bytes_half_carry(a, byte);

        Executed {
            cycles_used,
            next_pc,
        }
    }

    fn inc(&mut self, target: &TargetIncDec) -> Executed {
        //fetch
        let mut cycles_used = self.sync();
        let next_pc = self.pc.wrapping_add(1);

        match target {
            TargetIncDec::A => {
                self.registers.a = self.registers.a.wrapping_add(1);
                self.registers.flags.zero = self.registers.a == 0;
                self.registers.flags.negative = false;
            }
            TargetIncDec::B => {
                self.registers.b = self.registers.b.wrapping_add(1);
                self.registers.flags.zero = self.registers.b == 0;
                self.registers.flags.negative = false;
            }
            TargetIncDec::C => {
                self.registers.c = self.registers.c.wrapping_add(1);
                self.registers.flags.zero = self.registers.c == 0;
                self.registers.flags.negative = false;
            }
            TargetIncDec::D => {
                self.registers.d = self.registers.d.wrapping_add(1);
                self.registers.flags.zero = self.registers.d == 0;
                self.registers.flags.negative = false;
            }
            TargetIncDec::E => {
                self.registers.e = self.registers.e.wrapping_add(1);
                self.registers.flags.zero = self.registers.e == 0;
                self.registers.flags.negative = false;
            }
            TargetIncDec::H => {
                self.registers.h = self.registers.h.wrapping_add(1);
                self.registers.flags.zero = self.registers.h == 0;
                self.registers.flags.negative = false;
            }
            TargetIncDec::L => {
                self.registers.l = self.registers.l.wrapping_add(1);
                self.registers.flags.zero = self.registers.l == 0;
                self.registers.flags.negative = false;
            }
            TargetIncDec::BC => {
                self.registers
                    .set_bc(self.registers.get_bc().wrapping_add(1));
                cycles_used = 8;
            }
            TargetIncDec::DE => {
                self.registers
                    .set_de(self.registers.get_de().wrapping_add(1));
                cycles_used = 8;
            }
            TargetIncDec::HL => {
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_add(1));
                cycles_used = 8;
            }
            TargetIncDec::SP => {
                self.registers.sp = self.registers.sp.wrapping_add(1);
                cycles_used = 8;
            }
            TargetIncDec::HLPOINTER => {
                let address = self.registers.get_hl();
                let byte = self.read_byte(address) - 1;
                self.write_byte(address, byte);
                cycles_used = 12;
            }
        }

        Executed {
            cycles_used,
            next_pc,
        }
    }

    fn dec(&mut self, target: &TargetIncDec) -> Executed {
        //fetch
        let mut cycles_used = self.sync();
        let next_pc = self.pc.wrapping_add(1);

        match target {
            TargetIncDec::A => {
                self.registers.a = self.registers.a.wrapping_sub(1);
                self.registers.flags.zero = self.registers.a == 0;
                self.registers.flags.negative = true;
            }
            TargetIncDec::B => {
                self.registers.b = self.registers.b.wrapping_sub(1);
                self.registers.flags.zero = self.registers.b == 0;
                self.registers.flags.negative = true;
            }
            TargetIncDec::C => {
                self.registers.c = self.registers.c.wrapping_sub(1);
                self.registers.flags.zero = self.registers.c == 0;
                self.registers.flags.negative = true;
            }
            TargetIncDec::D => {
                self.registers.d = self.registers.d.wrapping_sub(1);
                self.registers.flags.zero = self.registers.d == 0;
                self.registers.flags.negative = true;
            }
            TargetIncDec::E => {
                self.registers.e = self.registers.e.wrapping_sub(1);
                self.registers.flags.zero = self.registers.e == 0;
                self.registers.flags.negative = true;
            }
            TargetIncDec::DE => {
                self.registers
                    .set_de(self.registers.get_de().wrapping_sub(1));
            }
            TargetIncDec::HL => {
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1));
                cycles_used = 12;
            }

            _ => {
                panic!("{:?} unimplemented DEC", target);
            }
        }

        Executed {
            cycles_used,
            next_pc,
        }
    }

    fn add_hl(&mut self) -> Executed {
        //fetch
        let cycles_used = self.sync();
        let next_pc = self.pc.wrapping_add(1);

        let hl = self.registers.get_hl();
        let stored = self.read_byte(hl);
        let added = self.add(stored, false);
        self.registers.a = added;

        Executed {
            cycles_used,
            next_pc,
        }
    }

    fn sub_r8(&mut self, target: &TargetRegister8) -> Executed {
        //fetch
        let cycles_used = self.sync();
        let next_pc = self.pc.wrapping_add(1);

        match target {
            TargetRegister8::A => {
                let stored = self.registers.a;
                let subbed = self.sub(stored, false);
                self.registers.a = subbed;
            }
            TargetRegister8::B => {
                let stored = self.registers.b;
                let subbed = self.sub(stored, false);
                self.registers.a = subbed;
            }
            TargetRegister8::C => {
                let stored = self.registers.c;
                let subbed = self.sub(stored, false);
                self.registers.a = subbed;
            }
            TargetRegister8::D => {
                let stored = self.registers.d;
                let subbed = self.sub(stored, false);
                self.registers.a = subbed;
            }
            TargetRegister8::E => {
                let stored = self.registers.e;
                let subbed = self.sub(stored, false);
                self.registers.a = subbed;
            }
            TargetRegister8::H => {
                let stored = self.registers.h;
                let subbed = self.sub(stored, false);
                self.registers.a = subbed;
            }
            TargetRegister8::L => {
                let stored = self.registers.l;
                let subbed = self.sub(stored, false);
                self.registers.a = subbed;
            }
        }

        Executed {
            cycles_used,
            next_pc,
        }
    }

    fn add_r8(&mut self, target: &TargetRegister8) -> Executed {
        //fetch
        let cycles_used = self.sync();
        let next_pc = self.pc.wrapping_add(1);

        match target {
            TargetRegister8::A => {
                let stored = self.registers.a;
                let added = self.add(stored, false);
                self.registers.a = added;
            }
            TargetRegister8::B => {
                let stored = self.registers.b;
                let added = self.add(stored, false);
                self.registers.a = added;
            }
            TargetRegister8::C => {
                let stored = self.registers.c;
                let added = self.add(stored, false);
                self.registers.a = added;
            }
            TargetRegister8::D => {
                let stored = self.registers.d;
                let added = self.add(stored, false);
                self.registers.a = added;
            }
            TargetRegister8::E => {
                let stored = self.registers.e;
                let added = self.add(stored, false);
                self.registers.a = added;
            }
            TargetRegister8::H => {
                let stored = self.registers.h;
                let added = self.add(stored, false);
                self.registers.a = added;
            }
            TargetRegister8::L => {
                let stored = self.registers.l;
                let added = self.add(stored, false);
                self.registers.a = added;
            }
        }

        Executed {
            cycles_used,
            next_pc,
        }
    }

    fn adc_r8(&mut self, target: &TargetRegister8) -> Executed {
        //fetch
        let cycles_used = self.sync();
        let next_pc = self.pc.wrapping_add(1);

        match target {
            TargetRegister8::A => {
                let stored = self.registers.a;
                let added = self.add(stored, self.registers.flags.carry);
                self.registers.a = added;
            }
            TargetRegister8::B => {
                let stored = self.registers.b;
                let added = self.add(stored, self.registers.flags.carry);
                self.registers.a = added;
            }
            TargetRegister8::C => {
                let stored = self.registers.c;
                let added = self.add(stored, self.registers.flags.carry);
                self.registers.a = added;
            }
            TargetRegister8::D => {
                let stored = self.registers.d;
                let added = self.add(stored, self.registers.flags.carry);
                self.registers.a = added;
            }
            TargetRegister8::E => {
                let stored = self.registers.e;
                let added = self.add(stored, self.registers.flags.carry);
                self.registers.a = added;
            }
            TargetRegister8::H => {
                let stored = self.registers.h;
                let added = self.add(stored, self.registers.flags.carry);
                self.registers.a = added;
            }
            TargetRegister8::L => {
                let stored = self.registers.l;
                let added = self.add(stored, self.registers.flags.carry);
                self.registers.a = added;
            }
        }

        Executed {
            cycles_used,
            next_pc,
        }
    }
}

impl CPU {
    fn add(&mut self, value: u8, carry: bool) -> u8 {
        let (added, overflowed) = self.registers.a.carrying_add(value, carry);
        self.registers.flags.zero = added == 0;
        self.registers.flags.negative = false;
        self.registers.flags.carry = overflowed;
        self.registers.flags.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;

        added
    }

    fn sub(&mut self, value: u8, carry: bool) -> u8 {
        let (subbed, overflowed) = self.registers.a.borrowing_sub(value, carry);
        self.registers.flags.zero = subbed == 0;
        self.registers.flags.negative = true;
        self.registers.flags.carry = overflowed;
        (_, self.registers.flags.half_carry) =
            (self.registers.a & 0x0F).overflowing_sub(value & 0x0F);

        subbed
    }
}
