use crate::gameboy::Gameboy;

pub mod bit;
pub mod bit_hl;
pub mod res;
pub mod res_hl;
pub mod set;
pub mod set_hl;

fn bit(gameboy: &mut Gameboy, byte: u8, bit: u8) {
    let check = 1 << bit;

    gameboy.write_zero_flag(byte & check == 0);
    gameboy.reset_negative_flag();
    gameboy.set_half_carry_flag();

    gameboy.pc = gameboy.pc.wrapping_add(2);
}

#[cfg(test)]
mod tests {
    use crate::gameboy::{Gameboy, GameboyCycles};

    #[no_coverage]
    pub fn test_bit_not_set(
        gameboy: &mut Gameboy,
        cycles: GameboyCycles,
        cycle_count: usize,
        length: u16,
    ) {
        assert_eq!(cycles.len(), cycle_count);

        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, length);
        assert!(gameboy.zero_flag(), "zero flag should be set");
        assert!(gameboy.half_carry_flag(), "half carry flag should be set");
        assert!(!gameboy.negative_flag(), "negative flag should not be set");
    }

    #[no_coverage]
    pub fn test_bit_set(
        gameboy: &mut Gameboy,
        cycles: GameboyCycles,
        cycle_count: usize,
        length: u16,
    ) {
        assert_eq!(cycles.len(), cycle_count);
        for cycle in cycles {
            gameboy.execute(cycle);
        }

        assert_eq!(gameboy.pc, length);
        assert!(!gameboy.zero_flag(), "zero flag should not be set");
        assert!(gameboy.half_carry_flag(), "half carry flag should be set");
        assert!(!gameboy.negative_flag(), "negative flag should not be set");
    }
}
