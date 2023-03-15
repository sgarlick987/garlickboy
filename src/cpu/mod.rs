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
    address_bus: AddressBus,
    pc: u16,
}

impl CPU {
    pub fn new(address_bus: AddressBus) -> CPU {
        let registers = registers::new_registers();
        CPU {
            registers,
            address_bus,
            pc: 0,
        }
    }

    pub fn render(&mut self) {
        self.address_bus.gpu.render();
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.address_bus.write_byte(address, byte);
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.address_bus.read_byte(address)
    }

    pub fn read_byte_lower(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(1))
    }

    pub fn read_byte_upper(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(2))
    }

    pub fn write_bytes(&mut self, address: u16, bytes: Vec<u8>) {
        self.address_bus.write_bytes(address, bytes);
    }

    fn sync(&mut self) -> u8 {
        4
    }

    pub fn step(&mut self) -> u8 {
        let mut instruction_byte = self.address_bus.read_byte(self.pc);

        let prefixed = instruction_byte == BYTE_PREFIX;
        if prefixed {
            instruction_byte = self.address_bus.read_byte(self.pc.wrapping_add(1));
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }
        if instruction == Instruction::NOP {
            return 255;
        }
        instruction.execute(self)
    }
}
