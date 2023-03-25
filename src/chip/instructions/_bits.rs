use super::TargetRegister8;
use crate::cpu::GameboyChip;

// impl GameboyChip {
//     #[test]
//     fn test_rl_carryin_carryout() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b10000011; //carryout
//         cpu.set_carry(); //carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000111);
//         assert!(cpu.carry_flag(), "carry flag should be set");
//         assert!(!cpu.zero, "zero flag should not be set");
//         assert!(
//             !cpu.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_carryin_no_carryout() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b00000011; // no carryout
//         cpu.set_carry(); //carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000111);
//         assert!(!cpu.carry_flag(), "carry flag should not be set");
//         assert!(!cpu.zero, "zero flag should not be set");
//         assert!(
//             !cpu.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_no_carryin_carryout() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b10000011; //carryout
//         cpu.reset_carry(); //no carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000110);
//         assert!(cpu.carry_flag(), "carry flag should be set");
//         assert!(!cpu.zero, "zero flag should not be set");
//         assert!(
//             !cpu.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_no_carryin_no_carryout() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b00000011; //no carryout
//         cpu.reset_carry(); //no carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000110);
//         assert!(!cpu.carry_flag(), "carry flag should not be set");
//         assert!(!cpu.zero, "zero flag should not be set");
//         assert!(
//             !cpu.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.negative,
//             "negative flag should not be set"
//         );
//     }

//     #[test]
//     fn test_rl_zero() {
//         const LENGTH: u16 = 2;
//         const CYCLES: u8 = 8;

//         let mut cpu = setup_cpu(CYCLES);
//         cpu.registers.c = 0b00000000; //no carryout
//         cpu.reset_carry(); //no carryin

//         cpu.rl(&TargetRegister8::C);

//         assert_eq!(cpu.pc, LENGTH);
//         assert_eq!(cpu.registers.c, 0b00000000);
//         assert!(!cpu.carry_flag(), "carry flag should not be set");
//         assert!(cpu.zero, "zero flag should be set");
//         assert!(
//             !cpu.half_carry,
//             "half carry flag should not be set"
//         );
//         assert!(
//             !cpu.negative,
//             "negative flag should not be set"
//         );
//     }
// }
