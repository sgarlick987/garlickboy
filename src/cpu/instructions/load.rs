use super::{execute::Executed, *};
use crate::cpu::{FlagsRegister, CPU};
use crate::utils::*;

pub trait Load {
    fn ld_hl_r8(&mut self, target: &TargetRegister8) -> Executed;
    fn ldi_a_hl(&mut self) -> Executed;
    fn ldi_hl_a(&mut self) -> Executed;
    fn ld_ff00u8_a(&mut self) -> Executed;
    fn ld_a_ff00u8(&mut self) -> Executed;
    fn ld_ff00c_a(&mut self) -> Executed;
    fn ld_u8(&mut self, target: &TargetRegister8) -> Executed;
    fn ld_a_ptr(&mut self, target: &TargetPointer) -> Executed;
    fn ld_u16(&mut self, target: &TargetRegister16) -> Executed;
    fn pop(&mut self, target: &TargetPushPop) -> Executed;
    fn push(&mut self, target: &TargetPushPop) -> Executed;
    fn ld_r8_u8(&mut self, target: &TargetRegister8) -> Executed;
    fn ld_r8_r8(&mut self, target: &TargetRegister8, source: &TargetRegister8) -> Executed;
    fn ld_u16_a(&mut self) -> Executed;
}

impl Load for CPU {
    fn ld_hl_r8(&mut self, target: &TargetRegister8) -> Executed {
        match target {
            TargetRegister8::A => {
                self.write_bytes(self.registers.get_hl(), [self.registers.a].to_vec());

                Executed {
                    cycles_used: 8,
                    next_pc: self.pc.wrapping_add(1),
                }
            }
            _ => {
                panic!("{:?} unimplemented LDHLR8 Instruction", target);
            }
        }
    }

    fn ldi_a_hl(&mut self) -> Executed {
        let hl = self.registers.get_hl();
        self.write_bytes(hl, [self.registers.a].to_vec());
        self.registers.set_hl(hl - 1);

        Executed {
            cycles_used: 8,
            next_pc: self.pc.wrapping_add(1),
        }
    }

    fn ldi_hl_a(&mut self) -> Executed {
        let hl = self.registers.get_hl();
        self.write_bytes(hl, [self.registers.a].to_vec());
        self.registers.set_hl(hl + 1);

        Executed {
            cycles_used: 8,
            next_pc: self.pc.wrapping_add(1),
        }
    }

    fn ld_ff00u8_a(&mut self) -> Executed {
        let address = 0xFF00 + self.read_byte(self.pc + 1) as u16;
        self.write_bytes(address, [self.registers.a].to_vec());

        Executed {
            cycles_used: 12,
            next_pc: self.pc.wrapping_add(2),
        }
    }

    fn ld_a_ff00u8(&mut self) -> Executed {
        let address = 0xFF00 + self.read_byte(self.pc + 1) as u16;
        self.registers.a = self.read_byte(address);

        Executed {
            cycles_used: 12,
            next_pc: self.pc.wrapping_add(2),
        }
    }

    fn ld_ff00c_a(&mut self) -> Executed {
        let address = 0xFF00 + self.registers.c as u16;
        self.write_bytes(address, [self.registers.a].to_vec());

        Executed {
            cycles_used: 8,
            next_pc: self.pc.wrapping_add(1),
        }
    }

    fn ld_u8(&mut self, target: &TargetRegister8) -> Executed {
        let value = self.read_byte(self.pc + 1);

        match target {
            TargetRegister8::A => {
                self.registers.a = value;
            }
            TargetRegister8::B => {
                self.registers.b = value;
            }
            TargetRegister8::C => {
                self.registers.c = value;
            }
            TargetRegister8::D => {
                self.registers.d = value;
            }
            TargetRegister8::E => {
                self.registers.e = value;
            }
            TargetRegister8::H => {
                self.registers.h = value;
            }
            TargetRegister8::L => {
                self.registers.l = value;
            }
        }

        Executed {
            cycles_used: 8,
            next_pc: self.pc.wrapping_add(2),
        }
    }

    fn ld_a_ptr(&mut self, target: &TargetPointer) -> Executed {
        self.registers.a = match target {
            TargetPointer::BC => self.read_byte(self.registers.get_bc()),
            TargetPointer::DE => self.read_byte(self.registers.get_de()),
            TargetPointer::HL => self.read_byte(self.registers.get_hl()),
        };

        Executed {
            cycles_used: 8,
            next_pc: self.pc.wrapping_add(1),
        }
    }

    fn ld_u16(&mut self, target: &TargetRegister16) -> Executed {
        match target {
            TargetRegister16::SP => {
                let lower = self.read_byte(self.pc + 1);
                let upper = self.read_byte(self.pc + 2);

                self.registers.set_sp(upper, lower);
            }
            TargetRegister16::HL => {
                self.registers.l = self.read_byte(self.pc + 1);
                self.registers.h = self.read_byte(self.pc + 2);
            }
            TargetRegister16::DE => {
                self.registers.e = self.read_byte(self.pc + 1);
                self.registers.d = self.read_byte(self.pc + 2);
            }
            _ => {
                panic!("{:?} unimplemented LDU16", target);
            }
        }

        Executed {
            cycles_used: 12,
            next_pc: self.pc.wrapping_add(3),
        }
    }

    fn push(&mut self, target: &TargetPushPop) -> Executed {
        self.registers.sp -= 2;

        match target {
            TargetPushPop::AF => {
                self.write_bytes(
                    self.registers.sp,
                    [self.registers.get_f(), self.registers.a].to_vec(),
                );
            }
            TargetPushPop::HL => {
                self.write_bytes(
                    self.registers.sp,
                    [self.registers.l, self.registers.h].to_vec(),
                );
            }
            TargetPushPop::BC => {
                self.write_bytes(
                    self.registers.sp,
                    [self.registers.c, self.registers.b].to_vec(),
                );
            }
            TargetPushPop::DE => {
                self.write_bytes(
                    self.registers.sp,
                    [self.registers.e, self.registers.d].to_vec(),
                );
            }
        }

        Executed {
            cycles_used: 16,
            next_pc: self.pc.wrapping_add(1),
        }
    }

    fn pop(&mut self, target: &TargetPushPop) -> Executed {
        match target {
            TargetPushPop::AF => {
                self.registers.a = self.read_byte(self.registers.sp + 1);
                self.registers.flags = FlagsRegister::from(self.read_byte(self.registers.sp));
            }
            TargetPushPop::HL => {
                self.registers.h = self.read_byte(self.registers.sp + 1);
                self.registers.l = self.read_byte(self.registers.sp);
            }
            TargetPushPop::BC => {
                self.registers.b = self.read_byte(self.registers.sp + 1);
                self.registers.c = self.read_byte(self.registers.sp);
            }
            TargetPushPop::DE => {
                self.registers.d = self.read_byte(self.registers.sp + 1);
                self.registers.e = self.read_byte(self.registers.sp);
            }
        }
        self.registers.sp += 2;

        Executed {
            cycles_used: 16,
            next_pc: self.pc.wrapping_add(1),
        }
    }

    fn ld_r8_u8(&mut self, target: &TargetRegister8) -> Executed {
        let byte = self.read_byte(self.pc + 1);
        match target {
            TargetRegister8::A => {
                self.registers.a = byte;
            }
            TargetRegister8::B => {
                self.registers.b = byte;
            }
            TargetRegister8::C => {
                self.registers.c = byte;
            }
            TargetRegister8::D => {
                self.registers.d = byte;
            }
            TargetRegister8::E => {
                self.registers.e = byte;
            }
            TargetRegister8::H => {
                self.registers.h = byte;
            }
            TargetRegister8::L => {
                self.registers.l = byte;
            }
        }

        Executed {
            cycles_used: 8,
            next_pc: self.pc.wrapping_add(2),
        }
    }

    fn ld_r8_r8(&mut self, target: &TargetRegister8, source: &TargetRegister8) -> Executed {
        match target {
            TargetRegister8::A => {
                self.registers.a = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::B => {
                self.registers.b = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::C => {
                self.registers.c = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::D => {
                self.registers.d = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::E => {
                self.registers.e = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::H => {
                self.registers.h = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
            TargetRegister8::L => {
                self.registers.l = match source {
                    TargetRegister8::A => self.registers.a,
                    TargetRegister8::B => self.registers.b,
                    TargetRegister8::C => self.registers.c,
                    TargetRegister8::D => self.registers.d,
                    TargetRegister8::E => self.registers.e,
                    TargetRegister8::H => self.registers.h,
                    TargetRegister8::L => self.registers.l,
                }
            }
        }

        Executed {
            cycles_used: 4,
            next_pc: self.pc.wrapping_add(1),
        }
    }
    fn ld_u16_a(&mut self) -> Executed {
        let address = merge_bytes(self.read_byte(self.pc + 2), self.read_byte(self.pc + 1));
        self.write_bytes(address, [self.registers.a].to_vec());

        Executed {
            cycles_used: 16,
            next_pc: self.pc.wrapping_add(3),
        }
    }
}
