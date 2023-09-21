mod bios;
mod bus;
mod dma;
mod gpu;
mod instructions;
mod interrupts;
mod joypad;
mod registers;
mod timer;

use self::bus::{new_address_bus, Bus};
use crate::emu::{controller::Controller, display::Display, rom::Rom};
use instructions::*;
use registers::*;

type GameboyCycle = Box<dyn FnOnce(&mut Gameboy)>;
type GameboyCycles = Box<dyn ExactSizeIterator<Item = GameboyCycle>>;

const MAX_MCYCLES_PER_FRAME: u32 = 1050000 / 60;

pub struct Gameboy {
    registers: Registers,
    bus: Box<dyn Bus>,
    pc: u16,
    halted: bool,
    cycles_used: u32,
    trace: bool,
}

impl Gameboy {
    pub fn new() -> Self {
        let registers = Registers::new();
        let bus = new_address_bus();

        Self {
            registers,
            bus,
            pc: 0,
            cycles_used: 0,
            halted: false,
            trace: false,
        }
    }

    pub fn is_new_frame(&self) -> bool {
        self.cycles_used == 0
    }

    pub fn update_joypad(&mut self, controller: &Box<dyn Controller>) {
        self.bus.update_joypad(controller);
    }

    pub fn render_display(&mut self, display: &mut Box<dyn Display>) {
        self.bus.render_display(display);
    }

    pub fn cycles(&mut self) -> GameboyCycles {
        let cycles = self.bus.next_interrupt_cycles();
        let has_interrupts = cycles.len() != 0;

        if has_interrupts {
            cycles
        } else {
            self.fetch()
        }
    }

    pub fn execute(&mut self, step: GameboyCycle) {
        step(self);
        self.bus.update_dma();
        self.bus.update_ime();
        self.bus.update_gpu();
        self.bus.update_timer();
        self.update_cycles_used();
    }

    fn update_cycles_used(&mut self) {
        self.cycles_used += 1;

        if self.cycles_used == MAX_MCYCLES_PER_FRAME {
            self.cycles_used = 0;
        }
    }

    fn fetch_instruction_byte(&mut self) -> (u8, bool) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == BYTE_PREFIX;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }
        (instruction_byte, prefixed)
    }

    fn fetch_instruction(&mut self, byte: u8, prefixed: bool) -> Instruction {
        if self.halted {
            if self.bus.has_interrupt_pending() {
                self.halted = false;
                Instruction::NOP
            } else {
                Instruction::HALT
            }
        } else {
            Instruction::from_byte(byte, prefixed)
        }
    }

    fn fetch(&mut self) -> GameboyCycles {
        let (instruction_byte, prefixed) = self.fetch_instruction_byte();
        let instruction = self.fetch_instruction(instruction_byte, prefixed);
        self.trace(&instruction);
        instruction.fetch()
    }

    fn trace(&mut self, instruction: &Instruction) {
        if self.pc == 0x0100 {
            self.trace = false;
        }
        if self.trace {
            let pc = self.pc;
            let byte0 = self.read_byte(pc);
            let byte1 = self.read_byte(pc + 1);
            let byte2 = self.read_byte(pc + 2);
            let byte3 = self.read_byte(pc + 3);
            let f = self.registers.get_f();
            println!(
            "{:?} A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            instruction,
            self.registers.a,
            f,
            self.registers.b,
            self.registers.c,
            self.registers.d,
            self.registers.e,
            self.registers.h,
            self.registers.l,
            self.registers.get_sp(),
            self.pc,
            byte0,
            byte1,
            byte2,
            byte3,
        )
        }
    }

    fn disable_ime(&mut self) {
        self.bus.disable_ime();
    }

    fn schedule_ime(&mut self) {
        self.bus.schedule_ime();
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        self.bus.write_byte(address, byte);
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        self.bus.read_byte(address)
    }

    fn read_byte_pc_lower(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(1))
    }

    fn read_byte_pc_upper(&mut self) -> u8 {
        self.read_byte(self.pc.wrapping_add(2))
    }

    fn carry_flag(&mut self) -> bool {
        self.registers.flags.carry
    }

    fn write_carry_flag(&mut self, carry: bool) {
        self.registers.flags.carry = carry;
    }

    fn set_carry_flag(&mut self) {
        self.write_carry_flag(true);
    }

    fn reset_carry_flag(&mut self) {
        self.write_carry_flag(false);
    }

    fn half_carry_flag(&mut self) -> bool {
        self.registers.flags.half_carry
    }

    fn write_half_carry_flag(&mut self, half_carry: bool) {
        self.registers.flags.half_carry = half_carry;
    }

    fn set_half_carry_flag(&mut self) {
        self.write_half_carry_flag(true);
    }

    fn reset_half_carry_flag(&mut self) {
        self.write_half_carry_flag(false);
    }

    fn zero_flag(&mut self) -> bool {
        self.registers.flags.zero
    }

    fn write_zero_flag(&mut self, zero: bool) {
        self.registers.flags.zero = zero;
    }

    fn set_zero_flag(&mut self) {
        self.write_zero_flag(true);
    }

    fn reset_zero_flag(&mut self) {
        self.write_zero_flag(false);
    }

    fn negative_flag(&mut self) -> bool {
        self.registers.flags.negative
    }

    fn write_negative_flag(&mut self, negative: bool) {
        self.registers.flags.negative = negative;
    }

    fn set_negative_flag(&mut self) {
        self.write_negative_flag(true);
    }

    fn reset_negative_flag(&mut self) {
        self.write_negative_flag(false);
    }

    fn push(&mut self, byte: u8) {
        self.registers
            .set_sp(self.registers.get_sp().wrapping_sub(1));
        self.write_byte(self.registers.get_sp(), byte);
    }

    fn pop(&mut self) -> u8 {
        let byte = self.read_byte(self.registers.get_sp());
        self.registers
            .set_sp(self.registers.get_sp().wrapping_add(1));
        byte
    }

    pub fn load_rom(&mut self, rom: &Rom) {
        self.bus.load_rom(rom);
    }
}
