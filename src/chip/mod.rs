#![allow(dead_code)]
pub mod bios;
pub mod bus;
pub mod gpu;
mod instructions;
mod interrupts;
pub mod joypad;
mod registers;

use crate::{
    controller::Controller,
    display::Display,
    utils::{add_bytes_half_carry, sub_bytes_half_carry},
};

use self::{
    bus::Bus,
    instructions::*,
    interrupts::{InterruptHandler, IE_ADDRESS, IF_ADDRESS},
    registers::*,
};

pub struct GameboyChip {
    pub registers: Registers,
    interrupt_handler: InterruptHandler,
    bus: Box<dyn Bus>,
    pc: u16,
    pub halted: bool,
    print: bool,
}

impl GameboyChip {
    pub fn dma(&mut self) {
        self.bus.handle_dma();
    }

    pub fn fetch(&mut self) -> Box<dyn Iterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == BYTE_PREFIX;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }
        let instruction = if self.halted {
            if self.interrupt_handler.is_pending() {
                Instruction::NOP
            } else {
                Instruction::HALT
            }
        } else {
            Instruction::from_byte(instruction_byte, prefixed)
        };
        if self.pc == 0x0100 {
            self.print = false;
        }
        if instruction == Instruction::UNIMPLEMENTED {
            panic!("Unkown Instruction found for: 0x{:x}", instruction_byte);
        }
        if self.print {
            println!("{:?} pc {:x}", instruction, self.pc);
            println!("registers before {:X?}", self.registers)
        }
        instruction.fetch()
    }

    pub fn execute(&mut self, step: Box<dyn FnOnce(&mut GameboyChip)>) {
        step(self);
    }

    pub fn update_joypad(&mut self, controller: &Controller) {
        self.bus.update_joypad(controller);
    }

    pub fn update_display(&mut self, display: &mut Display) {
        self.bus.update_display(display);
    }

    pub fn inc_ly(&mut self) {
        self.bus.inc_ly();
    }

    pub fn lcd_is_enabled(&mut self) -> bool {
        self.bus.lcd_is_enabled()
    }
}

impl GameboyChip {
    pub fn new(bus: Box<dyn Bus>) -> GameboyChip {
        let registers = Registers::new();
        let interrupt_handler = InterruptHandler::new();
        GameboyChip {
            registers,
            interrupt_handler,
            bus,
            pc: 0,
            halted: false,
            print: false,
        }
    }

    pub fn interrupts(
        &mut self,
    ) -> Box<dyn ExactSizeIterator<Item = Box<dyn FnOnce(&mut GameboyChip)>>> {
        self.interrupt_handler.step()
    }

    pub fn inc_div(&mut self) {
        self.bus.inc_div();
    }

    pub fn flag_vblank(&mut self) {
        self.interrupt_handler.set_vblank_flag();
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            IF_ADDRESS => self.interrupt_handler.set_flags(byte),
            IE_ADDRESS => self.interrupt_handler.set_enable(byte),
            _ => self.bus.write_byte(address, byte),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            IF_ADDRESS => self.interrupt_handler.get_flags(),
            IE_ADDRESS => self.interrupt_handler.get_enable(),
            _ => self.bus.read_byte(address),
        }
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

    fn carry_flag(&mut self) -> bool {
        self.registers.flags.carry
    }

    fn update_carry_flag(&mut self, carry: bool) {
        self.registers.flags.carry = carry;
    }

    fn set_carry_flag(&mut self) {
        self.update_carry_flag(true);
    }

    fn reset_carry_flag(&mut self) {
        self.update_carry_flag(false);
    }

    fn half_carry_flag(&mut self) -> bool {
        self.registers.flags.half_carry
    }

    fn update_half_carry_flag(&mut self, half_carry: bool) {
        self.registers.flags.half_carry = half_carry;
    }

    fn set_half_carry_flag(&mut self) {
        self.update_half_carry_flag(true);
    }

    fn reset_half_carry_flag(&mut self) {
        self.update_half_carry_flag(false);
    }

    fn zero_flag(&mut self) -> bool {
        self.registers.flags.zero
    }

    fn update_zero_flag(&mut self, zero: bool) {
        self.registers.flags.zero = zero;
    }

    fn set_zero_flag(&mut self) {
        self.update_zero_flag(true);
    }

    fn reset_zero_flag(&mut self) {
        self.update_zero_flag(false);
    }

    fn update_negative_flag(&mut self, negative: bool) {
        self.registers.flags.negative = negative;
    }

    fn negative_flag(&mut self) -> bool {
        self.registers.flags.negative
    }

    fn set_negative_flag(&mut self) {
        self.update_negative_flag(true);
    }

    fn reset_negative_flag(&mut self) {
        self.update_negative_flag(false);
    }

    fn push(&mut self, byte: u8) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, byte);
    }

    fn pop(&mut self) -> u8 {
        let byte = self.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        byte
    }

    fn add(&mut self, value: u8, carry: bool) -> u8 {
        let (added, overflowed) = self.registers.a.carrying_add(value, carry);
        self.update_zero_flag(added == 0);
        self.reset_negative_flag();
        self.update_carry_flag(overflowed);
        self.update_half_carry_flag(add_bytes_half_carry(self.registers.a, value));

        added
    }

    fn sub(&mut self, value: u8, carry: bool) -> u8 {
        let (subbed, overflowed) = self.registers.a.borrowing_sub(value, carry);
        self.update_zero_flag(subbed == 0);
        self.set_negative_flag();
        self.update_carry_flag(overflowed);
        self.update_half_carry_flag(sub_bytes_half_carry(self.registers.a, value));

        subbed
    }
}
