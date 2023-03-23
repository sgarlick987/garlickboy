use crate::cpu::GameboyChip;

pub mod call;
pub mod jp;
pub mod jpf;
pub mod jphl;
pub mod jr;
pub mod jrf;
pub mod ret;
pub mod retf;
pub mod reti;
pub mod rst;

fn push(chip: &mut GameboyChip, byte: u8) {
    chip.registers.sp = chip.registers.sp.wrapping_sub(1);
    chip.write_byte(chip.registers.sp, byte);
}

fn pop(chip: &mut GameboyChip) -> u8 {
    let byte = chip.read_byte(chip.registers.sp);
    chip.registers.sp = chip.registers.sp.wrapping_add(1);
    byte
}
