// #[cfg(test)]
// mod tests {
//     use coverage_helper::test;
//     use mockall::{predicate, Sequence};

//     use crate::address::*;

//     use super::*;

//     const COMPARISONS: [Comparison; 4] = [
//         Comparison::NONZERO,
//         Comparison::ZERO,
//         Comparison::CARRY,
//         Comparison::NOCARRY,
//     ];

//     #[test]
//     fn test_jp() {
//         const CYCLES: u8 = 16;

//         const ADDRESS: u16 = 0x4000;
//         const LOWER: u8 = 0x00;
//         const UPPER: u8 = 0x40;

//         let syncs = CYCLES / 4;
//         let mut bus = Box::new(MockBus::new());
//         bus.expect_sync().times(syncs as usize).return_const(0);

//         let mut seq = Sequence::new();
//         bus.expect_read_byte()
//             .with(predicate::eq(1))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(LOWER);
//         bus.expect_read_byte()
//             .with(predicate::eq(2))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(UPPER);

//         let mut cpu = Gameboygameboy::new(bus);

//         cpu.jp();

//         assert_eq!(cpu.pc, ADDRESS);
//     }

//     #[test]
//     fn test_jr() {
//         const CYCLES: u8 = 12;
//         const LENGTH: u16 = 2;
//         const JUMP_OFFSETS: [i8; 2] = [-2, 2];
//         const PC: u16 = 2;

//         let syncs = CYCLES / 4;

//         for jump_offset in JUMP_OFFSETS {
//             let mut bus = Box::new(MockBus::new());
//             bus.expect_sync().times(syncs as usize).return_const(0);

//             bus.expect_read_byte()
//                 .with(predicate::eq(PC + 1))
//                 .once()
//                 .return_const(jump_offset as u8);

//             let mut cpu = Gameboygameboy::new(bus);
//             cpu.pc = PC;

//             cpu.jr();

//             assert_eq!(
//                 cpu.pc,
//                 PC.wrapping_add(LENGTH).wrapping_add(jump_offset as u16)
//             );
//         }
//     }

//     #[test]
//     fn test_jrf_without_branch() {
//         const CYCLES: u8 = 8;
//         const LENGTH: u16 = 2;

//         let syncs = CYCLES / 4;

//         for comparison in COMPARISONS {
//             let mut bus = Box::new(MockBus::new());
//             bus.expect_sync().times(syncs as usize).return_const(0);
//             bus.expect_read_byte()
//                 .with(predicate::eq(1))
//                 .once()
//                 .return_const(0); //return value doesnt matter since we aren't jumping this test

//             let mut cpu = Gameboygameboy::new(bus);

//             match comparison {
//                 Comparison::ZERO => cpu.reset_zero(),
//                 Comparison::NONZERO => cpu.set_zero(),
//                 Comparison::CARRY => cpu.reset_carry(),
//                 Comparison::NOCARRY => cpu.set_carry(),
//             }
//             cpu.jrf(&comparison);
//             assert_eq!(cpu.pc, LENGTH);
//         }
//     }

//     #[test]
//     fn test_jrf_with_branch() {
//         const CYCLES: u8 = 12;
//         const LENGTH: u16 = 2;

//         const JUMP_OFFSETS: [i8; 2] = [-2, 2];
//         const PC: u16 = 2;

//         let syncs = CYCLES / 4;

//         for jump_offset in JUMP_OFFSETS {
//             for comparison in COMPARISONS {
//                 let mut bus = Box::new(MockBus::new());
//                 bus.expect_sync().times(syncs as usize).return_const(0);
//                 bus.expect_read_byte()
//                     .with(predicate::eq(PC.wrapping_add(1)))
//                     .once()
//                     .return_const(jump_offset as u8); //return value doesnt matter since we aren't jumping this test

//                 let mut cpu = Gameboygameboy::new(bus);
//                 cpu.pc = PC;

//                 match comparison {
//                     Comparison::ZERO => cpu.set_zero(),
//                     Comparison::NONZERO => cpu.reset_zero(),
//                     Comparison::CARRY => cpu.set_carry(),
//                     Comparison::NOCARRY => cpu.reset_carry(),
//                 }
//                 cpu.jrf(&comparison);

//                 assert_eq!(
//                     cpu.pc,
//                     PC.wrapping_add(LENGTH).wrapping_add(jump_offset as u16)
//                 );
//             }
//         }
//     }

//     #[test]
//     fn test_call() {
//         const CYCLES: u8 = 24;

//         const RETURN_ADDRESS_LOWER: u8 = 0x03;
//         const RETURN_ADDRESS_UPPER: u8 = 0x00;

//         const CALL_ADDRESS: u16 = 0x4000;
//         const CALL_ADDRESS_LOWER: u8 = 0x00;
//         const CALL_ADDRESS_UPPER: u8 = 0x40;

//         let syncs = CYCLES / 4;
//         let mut bus = Box::new(MockBus::new());
//         bus.expect_sync().times(syncs as usize).return_const(0);

//         let mut seq = Sequence::new();
//         bus.expect_read_byte()
//             .with(predicate::eq(1))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(CALL_ADDRESS_LOWER);
//         bus.expect_read_byte()
//             .with(predicate::eq(2))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(CALL_ADDRESS_UPPER);
//         bus.expect_write_byte()
//             .with(predicate::eq(1), predicate::eq(RETURN_ADDRESS_UPPER))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(());
//         bus.expect_write_byte()
//             .with(predicate::eq(0), predicate::eq(RETURN_ADDRESS_LOWER))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(());

//         let mut cpu = Gameboygameboy::new(bus);
//         cpu.registers.sp = 2;

//         cpu.call();

//         assert_eq!(cpu.pc, CALL_ADDRESS);
//         assert_eq!(cpu.registers.sp, 0);
//     }

//     #[test]
//     fn test_ret() {
//         const CYCLES: u8 = 16;
//         const LOWER: u8 = 0x00;
//         const UPPER: u8 = 0x10;
//         const ADDRESS: u16 = 0x1000;

//         let syncs = CYCLES / 4;
//         let mut bus = Box::new(MockBus::new());
//         bus.expect_sync().times(syncs as usize).return_const(0);

//         let mut seq = Sequence::new();
//         bus.expect_read_byte()
//             .with(predicate::eq(0))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(LOWER);
//         bus.expect_read_byte()
//             .with(predicate::eq(1))
//             .once()
//             .in_sequence(&mut seq)
//             .return_const(UPPER);

//         let mut cpu = Gameboygameboy::new(bus);

//         cpu.ret();

//         assert_eq!(cpu.pc, ADDRESS);
//         assert_eq!(cpu.registers.sp, 2);
//     }
// }
