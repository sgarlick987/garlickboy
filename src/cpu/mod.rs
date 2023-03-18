#![allow(dead_code)]
pub mod instructions;
pub mod registers;

use crate::address::*;

use self::{
    instructions::{execute::Execution, *},
    registers::*,
};

pub struct CPU {
    registers: Registers,
    bus: Box<dyn Bus>,
    pc: u16,
    ime: bool,
    print: bool,
}

impl CPU {
    pub fn new(bus: Box<dyn Bus>) -> CPU {
        let registers = registers::new_registers();
        CPU {
            registers,
            bus,
            pc: 0,
            ime: false,
            print: false,
        }
    }

    pub fn get_register_from_enum(&mut self, target: &TargetRegister8) -> u8 {
        match target {
            TargetRegister8::A => self.registers.a,
            TargetRegister8::B => self.registers.b,
            TargetRegister8::C => self.registers.c,
            TargetRegister8::D => self.registers.d,
            TargetRegister8::E => self.registers.e,
            TargetRegister8::H => self.registers.h,
            TargetRegister8::L => self.registers.l,
        }
    }

    pub fn set_register_from_enum(&mut self, target: &TargetRegister8, value: u8) {
        match target {
            TargetRegister8::A => self.registers.a = value,
            TargetRegister8::B => self.registers.b = value,
            TargetRegister8::C => self.registers.c = value,
            TargetRegister8::D => self.registers.d = value,
            TargetRegister8::E => self.registers.e = value,
            TargetRegister8::H => self.registers.h = value,
            TargetRegister8::L => self.registers.l = value,
        }
    }

    pub fn render(&mut self) {
        self.bus.render();
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.bus.write_byte(address, byte);
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.bus.read_byte(address)
    }

    pub fn read_byte_pc_lower(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(1))
    }

    pub fn read_byte_pc_upper(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(2))
    }

    pub fn write_bytes(&mut self, address: u16, bytes: Vec<u8>) {
        self.bus.write_bytes(address as usize, bytes);
    }

    pub fn write_bios(&mut self, bytes: [u8; 0x100]) {
        self.bus.write_boot(bytes);
    }

    fn sync(&mut self) -> u8 {
        self.bus.sync();
        4
    }

    pub fn step(&mut self) -> u8 {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == BYTE_PREFIX;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }
        if self.pc == 0x64d3 {
            self.print = true;
        }
        if self.print {
            println!("pc: {:x}, inst: {:?}", self.pc, instruction);
        }
        instruction.execute(self)
    }
}
