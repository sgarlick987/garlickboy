use crate::gameboy::GameboyCycle;

use super::*;
use bits::*;
use control::*;
use jump::*;
use load::*;
use logic::*;
use shift::*;

impl Instruction {
    pub fn fetch(&self) -> Box<dyn Iterator<Item = GameboyCycle>> {
        match self {
            //bits
            Instruction::BITHL(bit) => bit_hl::new(bit),
            Instruction::BIT(bit, target) => bit::new(bit, target),
            Instruction::RESHL(bit) => res_hl::new(bit),
            Instruction::RES(bit, target) => res::new(bit, target),
            Instruction::SETHL(bit) => set_hl::new(bit),

            //control
            Instruction::DI => di::new(),
            Instruction::EI => ei::new(),
            Instruction::HALT => halt::new(),
            Instruction::NOP => nop::new(),

            //jump
            Instruction::CALL => call::new(),
            Instruction::CALLF(comparison) => callf::new(comparison),
            Instruction::RET => ret::new(),
            Instruction::JP => jp::new(),
            Instruction::JPF(comparison) => jpf::new(comparison),
            Instruction::JPHL => jphl::new(),
            Instruction::JR => jr::new(),
            Instruction::JRF(comparison) => jrf::new(comparison),
            Instruction::RETF(comparison) => retf::new(comparison),
            Instruction::RETI => reti::new(),
            Instruction::RST(target) => rst::new(target),

            //load
            Instruction::LDAPTR(target) => ld_a_ptr::new(target),
            Instruction::LDAU16 => ld_a_u16::new(),
            Instruction::LDDEA => ld_de_a::new(),
            Instruction::LDHLR8(target) => ld_hl_r8::new(target),
            Instruction::LDHLSPI8 => ld_hl_sp_i8::new(),
            Instruction::LDHLU8 => ld_hl_u8::new(),
            Instruction::LDR8HL(target) => ld_r8_hl::new(target),
            Instruction::LDR8R8(target, source) => ld_r8_r8::new(target, source),
            Instruction::LDR8U8(target) => ld_r8_u8::new(target),
            Instruction::LDU8(target) => ld_u8::new(target),
            Instruction::LDU16A => ld_u16_a::new(),
            Instruction::LDU16(target) => ld_u16::new(target),
            Instruction::LDDAHL => ldd_a_hl::new(),
            Instruction::LDDHLA => ldd_hl_a::new(),
            Instruction::LDHAU8 => ldh_a_u8::new(),
            Instruction::LDHCA => ldh_c_a::new(),
            Instruction::LDHU8A => ldh_u8_a::new(),
            Instruction::LDIAHL => ldi_a_hl::new(),
            Instruction::LDIHLA => ldi_hl_a::new(),
            Instruction::POP(target) => pop::new(target),
            Instruction::PUSH(target) => push::new(target),

            //logic
            Instruction::ADCHL => adc_hl::new(),
            Instruction::ADCR8(target) => adc_r8::new(target),
            Instruction::ADCU8 => adc_u8::new(),
            Instruction::ADDHL => add_hl::new(),
            Instruction::ADDR8(target) => add_r8::new(target),
            Instruction::ADDR16(target) => add_r16::new(target),
            Instruction::ADDU8 => add_u8::new(),
            Instruction::ANDR8(target) => and_r8::new(target),
            Instruction::ANDU8 => and_u8::new(),
            Instruction::CPHL => cp_hl::new(),
            Instruction::CPR8(target) => cp_r8::new(target),
            Instruction::CPU8 => cp_u8::new(),
            Instruction::CPL => cpl::new(),
            Instruction::DAA => daa::new(),
            Instruction::DEC(target) => dec::new(target),
            Instruction::INC(target) => inc::new(target),
            Instruction::ORHL => or_hl::new(),
            Instruction::ORR8(target) => or_r8::new(target),
            Instruction::ORU8 => or_u8::new(),
            Instruction::SUBHL => sub_hl::new(),
            Instruction::SUBR8(target) => sub_r8::new(target),
            Instruction::SUBU8 => sub_u8::new(),
            Instruction::XORHL => xor_hl::new(),
            Instruction::XORR8(target) => xor_r8::new(target),
            Instruction::XORU8 => xor_u8::new(),

            //shift
            Instruction::RL(target) => rl_r8::new(target),
            Instruction::RLA => rla::new(),
            Instruction::RLCA => rlca::new(),
            Instruction::RR(target) => rr_r8::new(target),
            Instruction::RRA => rra::new(),
            Instruction::SLA(target) => sla_r8::new(target),
            Instruction::SRL(target) => srl_r8::new(target),
            Instruction::SWAP(target) => swap_r8::new(target),
            _ => {
                panic!("{:?} unimplemented Instruction", self);
            }
        }
    }
}
