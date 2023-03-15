use super::*;
use crate::cpu::CPU;
use bitwise::Bitwise;
use control::Control;
use jump::Jump;
use load::Load;
use logic::Logic;

pub trait Execution {
    fn execute(&self, cpu: &mut CPU) -> u8;
}

impl Execution for Instruction {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        match self {
            Instruction::NOP => cpu.nop(),
            Instruction::ADCR8(target) => cpu.adc_r8(target),
            Instruction::ADDHL => cpu.add_hl(),
            Instruction::ADDR8(target) => cpu.add_r8(target),
            Instruction::SUBR8(target) => cpu.sub_r8(target),
            Instruction::INC(target) => cpu.inc(target),
            Instruction::DEC(target) => cpu.dec(target),
            Instruction::LDU16(target) => cpu.ld_u16(target),
            Instruction::LDAPTR(target) => cpu.ld_a_ptr(target),
            Instruction::XORR8(target) => cpu.xor_r8(target),
            Instruction::LDU8(target) => cpu.ld_u8(target),
            Instruction::LDFF00CA => cpu.ld_ff00c_a(),
            Instruction::LDAFF00U8 => cpu.ld_a_ff00u8(),
            Instruction::LDDHLA => cpu.ldi_a_hl(),
            Instruction::LDIHLA => cpu.ldi_hl_a(),
            Instruction::LDHLR8(target) => cpu.ld_hl_r8(target),
            Instruction::LDFF00U8A => cpu.ld_ff00u8_a(),
            Instruction::PUSH(target) => cpu.push(target),
            Instruction::POP(target) => cpu.pop(target),
            Instruction::BIT(bit, target) => cpu.bit(bit, target),
            Instruction::JP => cpu.jp(),
            Instruction::JR => cpu.jr(),
            Instruction::JRF(comparison) => cpu.jrf(comparison),
            Instruction::LDR8U8(target) => cpu.ld_r8_u8(target),
            Instruction::LDR8R8(target, source) => cpu.ld_r8_r8(target, source),
            Instruction::LDU16A => cpu.ld_u16_a(),
            Instruction::RL(target) => cpu.rl(target),
            Instruction::RLA => cpu.rla(),
            Instruction::CPU8 => cpu.cp_u8(),
            Instruction::CPHL => cpu.cp_hl(),
            Instruction::CALL => cpu.call(),
            Instruction::RET => cpu.ret(),
            _ => {
                panic!("{:?} unimplemented Instruction pc: {:x}", self, cpu.pc);
            }
        }
    }
}
