#![allow(dead_code)]
pub mod instructions;
pub mod registers;
use registers::Registers;
use sdl2::render::Canvas;

use crate::gpu::*;
use crate::{bios::Bios, rom::Rom};

use self::{
    instructions::*,
    registers::{bytes_half_carry, merge_bytes, split_bytes, FlagsRegister},
};

#[derive(Debug)]
pub struct CPU {
    registers: Registers,
    bus: AddressBus,
    pc: u16,
}

pub fn new() -> CPU {
    let registers = registers::new_registers();
    let mut bus = AddressBus {
        memory: [0; 0xFFFF],
        gpu: GPU::new(),
    };
    bus.write_bytes(0xFF44, [0x90].to_vec());

    CPU {
        registers,
        bus,
        pc: 0,
    }
}

#[derive(Debug)]
pub struct AddressBus {
    pub memory: [u8; 0xFFFF],
    gpu: GPU,
}

impl AddressBus {
    fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            _ => self.memory[address],
        }
    }

    fn write_bytes(&mut self, address: u16, bytes: Vec<u8>) {
        let address = address as usize;
        match address {
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, bytes);
            }
            0xFF47 => {
                if bytes.len() != 1 {
                    panic!(
                        "only one byte should be supplied when writing to palette address 0xFF47"
                    );
                }
                self.gpu.set_palette(bytes[0]);
            }
            _ => {
                let end = address + bytes.len();
                self.memory[address..end].copy_from_slice(bytes.as_slice());
            }
        }
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        self.write_bytes(address, [byte].to_vec());
    }
}

impl CPU {
    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        self.bus.gpu.render(canvas);
    }

    pub fn write_bios(&mut self, bios: Bios) {
        self.bus.write_bytes(0x0000, bios.data.to_vec());
    }

    pub fn write_rom(&mut self, rom: Rom) {
        self.bus.write_bytes(0x0100, rom.data[0x0100..].to_vec());
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::ADCR8(target) => match target {
                TargetRegister8::A => {
                    let stored = self.registers.a;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::B => {
                    let stored = self.registers.b;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::C => {
                    let stored = self.registers.c;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::D => {
                    let stored = self.registers.d;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::E => {
                    let stored = self.registers.e;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::H => {
                    let stored = self.registers.h;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::L => {
                    let stored = self.registers.l;
                    let added = self.add(stored, self.registers.flags.carry);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::ADDHL => {
                let hl = self.registers.get_hl();
                let stored = self.bus.read_byte(hl);
                let added = self.add(stored, false);
                self.registers.a = added;
                self.pc.wrapping_add(1)
            }
            Instruction::ADDR8(target) => match target {
                TargetRegister8::A => {
                    let stored = self.registers.a;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::B => {
                    let stored = self.registers.b;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::C => {
                    let stored = self.registers.c;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::D => {
                    let stored = self.registers.d;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::E => {
                    let stored = self.registers.e;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::H => {
                    let stored = self.registers.h;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::L => {
                    let stored = self.registers.l;
                    let added = self.add(stored, false);
                    self.registers.a = added;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::SUBR8(target) => match target {
                TargetRegister8::A => {
                    let stored = self.registers.a;
                    let subbed = self.sub(stored, false);
                    self.registers.a = subbed;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::B => {
                    let stored = self.registers.b;
                    let subbed = self.sub(stored, false);
                    self.registers.a = subbed;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::C => {
                    let stored = self.registers.c;
                    let subbed = self.sub(stored, false);
                    self.registers.a = subbed;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::D => {
                    let stored = self.registers.d;
                    let subbed = self.sub(stored, false);
                    self.registers.a = subbed;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::E => {
                    let stored = self.registers.e;
                    let subbed = self.sub(stored, false);
                    self.registers.a = subbed;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::H => {
                    let stored = self.registers.h;
                    let subbed = self.sub(stored, false);
                    self.registers.a = subbed;
                    self.pc.wrapping_add(1)
                }
                TargetRegister8::L => {
                    let stored = self.registers.l;
                    let subbed = self.sub(stored, false);
                    self.registers.a = subbed;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::INC(target) => {
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
                    }
                    TargetIncDec::DE => {
                        self.registers
                            .set_de(self.registers.get_de().wrapping_add(1));
                    }
                    TargetIncDec::HL => {
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_add(1));
                    }
                    TargetIncDec::SP => {
                        self.registers.sp = self.registers.sp.wrapping_add(1);
                    }
                    TargetIncDec::HLPOINTER => {
                        let address = self.registers.get_hl();
                        let byte = self.bus.read_byte(address) - 1;
                        self.bus.write_byte(address, byte);
                    }
                }
                self.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
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
                    }

                    _ => {
                        panic!("{:?} unimplemented DEC", target);
                    }
                }

                self.pc.wrapping_add(1)
            }
            Instruction::LDU16(target) => {
                match target {
                    TargetRegister16::SP => {
                        let lower = self.bus.read_byte(self.pc + 1);
                        let upper = self.bus.read_byte(self.pc + 2);

                        self.registers.set_sp(upper, lower);
                    }
                    TargetRegister16::HL => {
                        self.registers.l = self.bus.read_byte(self.pc + 1);
                        self.registers.h = self.bus.read_byte(self.pc + 2);
                    }
                    TargetRegister16::DE => {
                        self.registers.e = self.bus.read_byte(self.pc + 1);
                        self.registers.d = self.bus.read_byte(self.pc + 2);
                    }
                    _ => {
                        panic!("{:?} unimplemented LDU16", target);
                    }
                }

                self.pc.wrapping_add(3)
            }
            Instruction::LDAPTR(target) => {
                self.registers.a = match target {
                    TargetPointer::BC => self.bus.read_byte(self.registers.get_bc()),
                    TargetPointer::DE => self.bus.read_byte(self.registers.get_de()),
                    TargetPointer::HL => self.bus.read_byte(self.registers.get_hl()),
                };

                self.pc.wrapping_add(1)
            }
            Instruction::XORR8(target) => match target {
                TargetRegister8::A => {
                    self.registers.a ^= self.registers.a;

                    self.pc.wrapping_add(1)
                }
                _ => {
                    panic!("{:?} unimplemented XORR8", target);
                }
            },
            Instruction::LDU8(target) => {
                let value = self.bus.read_byte(self.pc + 1);

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

                self.pc.wrapping_add(2)
            }
            Instruction::LDFF00CA => {
                let address = 0xFF00 + self.registers.c as u16;
                self.bus.write_bytes(address, [self.registers.a].to_vec());

                self.pc.wrapping_add(1)
            }
            Instruction::LDAFF00U8 => {
                let address = 0xFF00 + self.bus.read_byte(self.pc + 1) as u16;
                self.registers.a = self.bus.read_byte(address);

                self.pc.wrapping_add(2)
            }
            Instruction::LDDHLA => {
                let hl = self.registers.get_hl();
                self.bus.write_bytes(hl, [self.registers.a].to_vec());
                self.registers.set_hl(hl - 1);

                self.pc.wrapping_add(1)
            }
            Instruction::LDIHLA => {
                let hl = self.registers.get_hl();
                self.bus.write_bytes(hl, [self.registers.a].to_vec());
                self.registers.set_hl(hl + 1);

                self.pc.wrapping_add(1)
            }
            Instruction::LDHLR8(target) => match target {
                TargetRegister8::A => {
                    self.bus
                        .write_bytes(self.registers.get_hl(), [self.registers.a].to_vec());

                    self.pc.wrapping_add(1)
                }
                _ => {
                    panic!("{:?} unimplemented LDHLR8 Instruction", target);
                }
            },
            Instruction::LDFF00U8A => {
                let address = 0xFF00 + self.bus.read_byte(self.pc + 1) as u16;
                self.bus.write_bytes(address, [self.registers.a].to_vec());

                self.pc.wrapping_add(2)
            }
            Instruction::PUSH(target) => {
                self.registers.sp -= 2;

                match target {
                    TargetPushPop::AF => {
                        self.bus.write_bytes(
                            self.registers.sp,
                            [self.registers.get_f(), self.registers.a].to_vec(),
                        );
                    }
                    TargetPushPop::HL => {
                        self.bus.write_bytes(
                            self.registers.sp,
                            [self.registers.l, self.registers.h].to_vec(),
                        );
                    }
                    TargetPushPop::BC => {
                        self.bus.write_bytes(
                            self.registers.sp,
                            [self.registers.c, self.registers.b].to_vec(),
                        );
                    }
                    TargetPushPop::DE => {
                        self.bus.write_bytes(
                            self.registers.sp,
                            [self.registers.e, self.registers.d].to_vec(),
                        );
                    }
                }

                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                match target {
                    TargetPushPop::AF => {
                        self.registers.a = self.bus.read_byte(self.registers.sp + 1);
                        self.registers.flags =
                            FlagsRegister::from(self.bus.read_byte(self.registers.sp));
                    }
                    TargetPushPop::HL => {
                        self.registers.h = self.bus.read_byte(self.registers.sp + 1);
                        self.registers.l = self.bus.read_byte(self.registers.sp);
                    }
                    TargetPushPop::BC => {
                        self.registers.b = self.bus.read_byte(self.registers.sp + 1);
                        self.registers.c = self.bus.read_byte(self.registers.sp);
                    }
                    TargetPushPop::DE => {
                        self.registers.d = self.bus.read_byte(self.registers.sp + 1);
                        self.registers.e = self.bus.read_byte(self.registers.sp);
                    }
                }
                self.registers.sp += 2;

                self.pc.wrapping_add(1)
            }
            Instruction::BIT(bit, target) => match target {
                TargetRegister8::H => {
                    self.registers.flags.negative = false;
                    self.registers.flags.half_carry = true;

                    let check = 1 << bit;
                    self.registers.flags.zero = self.registers.h & check == 0;

                    self.pc.wrapping_add(2)
                }
                _ => {
                    panic!("{:?} unimplemented BIT Instruction", target);
                }
            },
            Instruction::JP => {
                let address = merge_bytes(
                    self.bus.read_byte(self.pc + 2),
                    self.bus.read_byte(self.pc + 1),
                );

                address
            }
            Instruction::JR => {
                let offset = self.bus.read_byte(self.pc + 1) as i8;

                self.pc.wrapping_add(2).wrapping_add(offset as u16)
            }
            Instruction::JRF(comparison) => match comparison {
                Comparison::NONZERO => {
                    if !self.registers.flags.zero {
                        let offset = self.bus.read_byte(self.pc + 1) as i8;

                        self.pc.wrapping_add(2).wrapping_add(offset as u16)
                    } else {
                        self.pc.wrapping_add(2)
                    }
                }
                Comparison::ZERO => {
                    if self.registers.flags.zero {
                        let offset = self.bus.read_byte(self.pc + 1) as i8;

                        self.pc.wrapping_add(2).wrapping_add(offset as u16)
                    } else {
                        self.pc.wrapping_add(2)
                    }
                }
                _ => {
                    panic!("{:?} unimplemented JRF Instruction", comparison);
                }
            },
            Instruction::LDR8U8(target) => {
                let byte = self.bus.read_byte(self.pc + 1);
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

                self.pc.wrapping_add(2)
            }
            Instruction::LDR8R8(target, source) => {
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

                self.pc.wrapping_add(1)
            }
            Instruction::LDU16A => {
                let address =
                    merge_bytes(self.bus.read_byte(self.pc + 1), self.bus.read_byte(self.pc));
                self.bus.write_bytes(address, [self.registers.a].to_vec());

                self.pc.wrapping_add(3)
            }
            Instruction::RL(target) => {
                self.registers.flags.half_carry = false;
                self.registers.flags.negative = false;
                match target {
                    TargetRegister8::C => {
                        let mut new_c = self.registers.c << 1;

                        if self.registers.flags.carry {
                            new_c |= 1
                        }

                        if self.registers.c >> 7 == 1 {
                            self.registers.flags.carry = true;
                        }

                        self.registers.flags.zero = new_c == 0;
                        self.registers.c = new_c;
                    }
                    _ => {
                        panic!("{:?} unimplemented RL Instruction", target);
                    }
                }

                self.pc.wrapping_add(2)
            }
            Instruction::RLA => {
                self.registers.flags.half_carry = false;
                self.registers.flags.negative = false;
                self.registers.flags.zero = false;

                let mut new_a = self.registers.a << 1;

                if self.registers.flags.carry {
                    new_a |= 1
                }

                if self.registers.a >> 7 == 1 {
                    self.registers.flags.carry = true;
                }
                self.registers.a = new_a;

                self.pc.wrapping_add(1)
            }
            Instruction::CPU8 => {
                let byte = self.bus.read_byte(self.pc + 1);
                let a = self.registers.a;

                self.registers.flags.negative = true;
                self.registers.flags.zero = a == byte;
                self.registers.flags.carry = a < byte;
                self.registers.flags.half_carry = bytes_half_carry(a, byte);

                self.pc.wrapping_add(2)
            }
            Instruction::CPHL => {
                let hl = self.registers.get_hl();
                let byte = self.bus.read_byte(hl);
                let a = self.registers.a;

                self.registers.flags.negative = true;
                self.registers.flags.zero = a == byte;
                self.registers.flags.carry = a < byte;
                self.registers.flags.half_carry = bytes_half_carry(a, byte);

                self.pc.wrapping_add(1)
            }
            Instruction::CALL => {
                self.registers.sp -= 2;
                let bytes = split_bytes(self.pc.wrapping_add(3));
                self.bus
                    .write_bytes(self.registers.sp, [bytes[1], bytes[0]].to_vec());
                let lower = self.bus.read_byte(self.pc + 1);
                let upper = self.bus.read_byte(self.pc + 2);
                let pc = merge_bytes(upper, lower);

                pc
            }
            Instruction::RET => {
                let pc = merge_bytes(
                    self.bus.read_byte(self.registers.sp + 1),
                    self.bus.read_byte(self.registers.sp),
                );
                self.registers.sp += 2;

                pc
            }
            _ => {
                panic!(
                    "{:?} unimplemented Instruction pc: {:x}",
                    instruction, self.pc
                );
            }
        }
    }

    pub fn step(&mut self) -> bool {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == INSTRUCTION_PREFIX_BYTE;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }
        if instruction == Instruction::NOP {
            return false;
        }
        self.pc = self.execute(instruction);
        return true;
    }

    //handles the add Instructions by adding a given value to our a register
    //and setting appropiate flags.
    fn add(&mut self, value: u8, carry: bool) -> u8 {
        let (added, overflowed) = self.registers.a.carrying_add(value, carry);
        self.registers.flags.zero = added == 0;
        self.registers.flags.negative = false;
        self.registers.flags.carry = overflowed;
        //this is documented as taking the lower 4 bits of the a register and value add
        //checking if they are greater than 0x0F
        self.registers.flags.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;

        added
    }

    //handles the sub Instructions by subtracing a given value to our a register
    //and setting appropiate flags.
    fn sub(&mut self, value: u8, carry: bool) -> u8 {
        let (subbed, overflowed) = self.registers.a.borrowing_sub(value, carry);
        self.registers.flags.zero = subbed == 0;
        self.registers.flags.negative = true;
        self.registers.flags.carry = overflowed;
        //this is documented as taking the lower 4 bits of the a register and value add
        //checking if they are greater than 0x0F
        (_, self.registers.flags.half_carry) =
            (self.registers.a & 0x0F).overflowing_sub(value & 0x0F);

        subbed
    }
}
