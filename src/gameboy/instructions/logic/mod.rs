use crate::{
    gameboy::Gameboy,
    utils::{add_bytes_half_carry, sub_bytes_half_carry},
};

pub mod adc_hl;
pub mod adc_r8;
pub mod adc_u8;
pub mod add_hl;
pub mod add_r16;
pub mod add_r8;
pub mod add_sp;
pub mod add_u8;
pub mod and_hl;
pub mod and_r8;
pub mod and_u8;
pub mod cp_hl;
pub mod cp_r8;
pub mod cp_u8;
pub mod cpl;
pub mod daa;
pub mod dec;
pub mod inc;
pub mod or_hl;
pub mod or_r8;
pub mod or_u8;
pub mod sbc_hl;
pub mod sbc_r8;
pub mod sbc_u8;
pub mod sub_hl;
pub mod sub_r8;
pub mod sub_u8;
pub mod xor_hl;
pub mod xor_r8;
pub mod xor_u8;

fn add(gameboy: &mut Gameboy, value: u8, carry: bool) -> u8 {
    let carry = carry as u8;
    let register = gameboy.registers.a;
    let (added, overflowed) = register.overflowing_add(value);
    let half_carried = add_bytes_half_carry(register, value);
    let (carry_added, carry_overflowed) = added.overflowing_add(carry);
    let carry_half_carried = add_bytes_half_carry(added, carry);

    gameboy.write_zero_flag(carry_added == 0);
    gameboy.write_carry_flag(overflowed || carry_overflowed);
    gameboy.write_half_carry_flag(half_carried || carry_half_carried);
    gameboy.reset_negative_flag();

    carry_added
}

fn sub(gameboy: &mut Gameboy, value: u8, carry: bool) -> u8 {
    let register = gameboy.registers.a;
    let (subbed, overflowed) = register.overflowing_sub(value);
    let half_carried = sub_bytes_half_carry(register, value);
    let (carry_subbed, carry_overflowed) = subbed.overflowing_sub(carry as u8);
    let carry_half_carried = sub_bytes_half_carry(subbed, carry as u8);

    gameboy.write_zero_flag(carry_subbed == 0);
    gameboy.write_carry_flag(overflowed || carry_overflowed);
    gameboy.write_half_carry_flag(half_carried || carry_half_carried);
    gameboy.set_negative_flag();

    carry_subbed
}
