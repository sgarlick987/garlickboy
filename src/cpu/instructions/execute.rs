use super::*;
use crate::cpu::GameboyChip;

// pub trait Execution {
//     fn execute(&self, cpu: &mut GameboyChip) -> u8;
// }

impl Instruction {
    pub fn fetch(&self) -> Vec<Box<dyn FnMut(&mut GameboyChip)>> {
        match self {
            Instruction::RLCA => bitwise::rcla(),
            _ => {
                panic!("{:?} unimplemented Instruction", self);
            }
        }
    }
}

// impl Execution for Instruction {
//     fn execute(&self, cpu: &mut GameboyChip) -> u8 {
//         match self {
//             Instruction::NOP => cpu.nop(),
//             Instruction::ADCR8(target) => cpu.adc_r8(target),
//             Instruction::ADDHL => cpu.add_hl(),
//             Instruction::ADDR8(target) => cpu.add_r8(target),
//             Instruction::SUBR8(target) => cpu.sub_r8(target),
//             Instruction::DI => cpu.di(),
//             Instruction::EI => cpu.ei(),
//             Instruction::INC(target) => cpu.inc(target),
//             Instruction::DEC(target) => cpu.dec(target),
//             Instruction::LDU16(target) => cpu.ld_u16(target),
//             Instruction::LDHLU8 => cpu.ld_hl_u8(),
//             Instruction::LDAPTR(target) => cpu.ld_a_ptr(target),
//             Instruction::XORR8(target) => cpu.xor_r8(target),
//             Instruction::LDU8(target) => cpu.ld_u8(target),
//             Instruction::LDFF00CA => cpu.ld_ff00c_a(),
//             Instruction::LDAFF00U8 => cpu.ld_a_ff00u8(),
//             Instruction::LDDHLA => cpu.ldd_hl_a(),
//             Instruction::LDIHLA => cpu.ldi_hl_a(),
//             Instruction::LDDAHL => cpu.ldd_a_hl(),
//             Instruction::LDIAHL => cpu.ldi_a_hl(),
//             Instruction::LDHLR8(target) => cpu.ld_hl_r8(target),
//             Instruction::LDFF00U8A => cpu.ld_ff00u8_a(),
//             Instruction::PUSH(target) => cpu.push(target),
//             Instruction::POP(target) => cpu.pop(target),
//             Instruction::BIT(bit, target) => cpu.bit(bit, target),
//             Instruction::JP => cpu.jp(),
//             Instruction::JR => cpu.jr(),
//             Instruction::JRF(comparison) => cpu.jrf(comparison),
//             Instruction::LDR8U8(target) => cpu.ld_r8_u8(target),
//             Instruction::LDR8R8(target, source) => cpu.ld_r8_r8(target, source),
//             Instruction::LDU16A => cpu.ld_u16_a(),
//             Instruction::RL(target) => cpu.rl(target),
//             Instruction::RLA => cpu.rla(),
//             Instruction::CPU8 => cpu.cp_u8(),
//             Instruction::CPHL => cpu.cp_hl(),
//             Instruction::CPL => cpu.cpl(),
//             Instruction::CALL => cpu.call(),
//             Instruction::RET => cpu.ret(),
//             Instruction::ORR8(target) => cpu.or_r8(target),
//             Instruction::ANDU8 => cpu.and_u8(),
//             Instruction::ANDR8(target) => cpu.and_r8(target),
//             Instruction::SWAP(target) => cpu.swap(target),
//             Instruction::RST(target) => cpu.rst(target),
//             Instruction::ADDR16(target) => cpu.add_r16(target),
//             Instruction::LDR8HL(target) => cpu.ld_r8_hl(target),
//             Instruction::JPHL => cpu.jp_hl(),
//             Instruction::RES(bit, target) => cpu.res(bit, target),
//             Instruction::LDDEA => cpu.ld_de_a(),
//             Instruction::LDAU16 => cpu.ld_a_u16(),
//             Instruction::SETHL(bit) => cpu.set_hl(bit),
//             Instruction::SLA(target) => cpu.sla(target),
//             Instruction::JPF(target) => cpu.jpf(target),
//             Instruction::RETF(target) => cpu.retf(target),
//             Instruction::ADDU8 => cpu.add_u8(),
//             Instruction::BITHL(bit) => cpu.bit_hl(bit),
//             Instruction::RETI => cpu.reti(),
//             Instruction::XORU8 => cpu.xor_u8(),
//             // Instruction::RLCA => cpu.rlca(),
//             Instruction::CPR8(target) => cpu.cp_r8(target),
//             _ => {
//                 panic!("{:?} unimplemented Instruction pc: {:x}", self, cpu.pc);
//             }
//         }
//     }
// }
