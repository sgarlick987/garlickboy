use super::TargetRegister8;
use crate::cpu::GameboyChip;

// impl GameboyChip {
//     // SET 7,(HL) - 0xFE
//     // Length: 2 bytes
//     // Flags
//     // Zero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: x8/rsb
//     // Timing
//     // without branch (16t)
//     // fetch	(0xCB)
//     // fetch
//     // read	(HL)
//     // write	(HL)
//     pub fn set_hl(&mut self, bit: &u8) {
//         let hl = self.registers.get_hl();
//         //fetch

//         //fetch

//         let bit = 1 << bit;
//         //read
//         let mut value = self.read_byte(hl);
//         value |= bit;

//         //write
//         self.write_byte(hl, value);

//         self.pc = self.pc.wrapping_add(2);
//     }

//     // RLCA - 0x07
//     // Length: 1 byte
//     // Flags
//     // Zero	unset
//     // Negative	unset
//     // Half Carry	unset
//     // Carry	dependent
//     // Group: x8/rsb
//     // Timing
//     // without branch (4t)
//     // fetch
//     pub fn rlca() -> Box<dyn FnMut(&mut GameboyChip)> {
//         let inst = Box::new(|cpu: &mut GameboyChip| {
//             let new_a = cpu.registers.a << 1;

//             cpu.registers.flags.carry = cpu.registers.a >> 7 == 1;
//             cpu.registers.flags.half_carry = false;
//             cpu.registers.flags.negative = false;
//             cpu.registers.flags.zero = false;
//             cpu.registers.a = new_a;
//         });

//         inst
//     }

// #[cfg(test)]
// mod tests {
//     use crate::address::*;
//     use coverage_helper::test;

//     use super::*;

//     fn setup_cpu(cycles: u8) -> GameboyChip {
//         let syncs = cycles / 4;
//         let mut bus = Box::new(MockBus::new());
//         bus.expect_sync().times(syncs as usize).return_const(0);

//         GameboyChip::new(bus)
//     }

//     #[test]
//     fn test_bit_not_set() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let targets = [
//             TargetRegister8::A,
//             TargetRegister8::B,
//             TargetRegister8::C,
//             TargetRegister8::D,
//             TargetRegister8::E,
//             TargetRegister8::H,
//             TargetRegister8::L,
//         ];

//         for target in targets {
//             for bit in 0..8 {
//                 let mut cpu = setup_cpu(CYCLES);
//                 let check = 0xFF ^ (1 << bit);

//                 cpu.set_register_from_enum(&target, check);

//                 cpu.bit(&bit, &target);

//                 assert_eq!(cpu.pc, LENGTH);
//                 assert!(cpu.registers.flags.zero, "zero flag should be set");
//                 assert!(
//                     cpu.registers.flags.half_carry,
//                     "half carry flag should be set"
//                 );
//                 assert!(
//                     !cpu.registers.flags.negative,
//                     "negative flag should not be set"
//                 );
//             }
//         }
//     }

//     #[test]
//     fn test_bit_set() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let targets = [
//             TargetRegister8::A,
//             TargetRegister8::B,
//             TargetRegister8::C,
//             TargetRegister8::D,
//             TargetRegister8::E,
//             TargetRegister8::H,
//             TargetRegister8::L,
//         ];

//         for target in targets {
//             for bit in 0..8 {
//                 let mut cpu = setup_cpu(CYCLES);
//                 let check = 1 << bit;

//                 cpu.set_register_from_enum(&target, check);
//                 cpu.bit(&bit, &target);

//                 assert_eq!(cpu.pc, LENGTH);
//                 assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//                 assert!(
//                     cpu.registers.flags.half_carry,
//                     "half carry flag should be set"
//                 );
//                 assert!(
//                     !cpu.registers.flags.negative,
//                     "negative flag should not be set"
//                 );
//             }
//         }
//     }

//     #[test]
//     fn test_rla_carryin_carryout() {
//         const LENGTH: u16 = 1;
//         const CYCLES: u8 = 4;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.a = 0b10000011; //carryout
//         cpu.registers.flags.carry = true; //carryin

//         cpu.rla();

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.a, 0b00000111);
//         assert!(cpu.registers.flags.carry, "carry flag should be set");
//         assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rla_no_carryin_carryout() {
//         const LENGTH: u16 = 1;
//         const CYCLES: u8 = 4;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.a = 0b10000011; //carryout
//         cpu.registers.flags.carry = false; //no carryin

//         cpu.rla();

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.a, 0b00000110);
//         assert!(cpu.registers.flags.carry, "carry flag should be set");
//         assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }
//     #[test]
//     fn test_rla_carryin_no_carryout() {
//         const LENGTH: u16 = 1;
//         const CYCLES: u8 = 4;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.a = 0b00000011; //no carryout
//         cpu.registers.flags.carry = true; //carryin

//         cpu.rla();

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.a, 0b00000111);
//         assert!(!cpu.registers.flags.carry, "carry flag should not be set");
//         assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rla_no_carryin_no_carryout() {
//         const LENGTH: u16 = 1;
//         const CYCLES: u8 = 4;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.a = 0b00000011; //no carryout
//         cpu.registers.flags.carry = false; //no carryin

//         cpu.rla();

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.a, 0b00000110);
//         assert!(!cpu.registers.flags.carry, "carry flag should not be set");
//         assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_carryin_carryout() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b10000011; //carryout
//         cpu.registers.flags.carry = true; //carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000111);
//         assert!(cpu.registers.flags.carry, "carry flag should be set");
//         assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_carryin_no_carryout() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b00000011; // no carryout
//         cpu.registers.flags.carry = true; //carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000111);
//         assert!(!cpu.registers.flags.carry, "carry flag should not be set");
//         assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_no_carryin_carryout() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b10000011; //carryout
//         cpu.registers.flags.carry = false; //no carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000110);
//         assert!(cpu.registers.flags.carry, "carry flag should be set");
//         assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_no_carryin_no_carryout() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b00000011; //no carryout
//         cpu.registers.flags.carry = false; //no carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000110);
//         assert!(!cpu.registers.flags.carry, "carry flag should not be set");
//         assert!(!cpu.registers.flags.zero, "zero flag should not be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_zero() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b00000000; //no carryout
//         cpu.registers.flags.carry = false; //no carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000000);
//         assert!(!cpu.registers.flags.carry, "carry flag should not be set");
//         assert!(cpu.registers.flags.zero, "zero flag should be set");
//         assert!(
//             !cpu.registers.flags.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.registers.flags.negative,
//             "negative flag should not be set"
//         );
//     }
// }
