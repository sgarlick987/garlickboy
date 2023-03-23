//     // RET Z - 0xC8
//     // Length: 1 byte
//     // Flags
//     // Zero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: control/br
//     // Timing
//     // without branch (8t)	with branch (20t)
//     // fetch	fetch
//     // internal	internal
//     // branch decision?	branch decision?
//     // read
//     // (SP++)->lower
//     // read
//     // (SP++)->upper
//     // internal
//     // set PC?
//     pub fn retf(&mut self, comparison: &Comparison) -> u8 {
//         //fetch
//         let mut cycles_used = self.sync();

//         if match comparison {
//             Comparison::NONZERO => !self.registers.flags.zero,
//             Comparison::ZERO => self.registers.flags.zero,
//             Comparison::CARRY => self.registers.flags.carry,
//             Comparison::NOCARRY => !self.registers.flags.carry,
//         } {
//             //read lower
//             let lower = self._pop();
//             cycles_used += self.sync();

//             //read upper
//             let upper = self._pop();
//             cycles_used += self.sync();

//             //set pc
//             self.pc = merge_bytes(upper, lower);
//         } else {
//             self.pc = self.pc.wrapping_add(1);
//         }

//         cycles_used += self.sync();
//         cycles_used
//     }
