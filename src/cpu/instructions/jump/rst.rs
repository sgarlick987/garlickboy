//     // RST 28h - 0xEF
//     // Length: 1 byte
//     // Flags
//     // Zero	unmodified
//     // Negative	unmodified
//     // Half Carry	unmodified
//     // Carry	unmodified
//     // Group: control/br
//     // Timing
//     // without branch (16t)
//     // fetch
//     // internal
//     // write	PC:upper->(--SP)
//     // write	PC:lower->(--SP)
//     pub fn rst(&mut self, target: &RstVector) -> u8 {
//         let return_address = self.pc.wrapping_add(1);
//         let (return_address_upper, return_address_lower) = split_bytes(return_address);
//         //fetch
//         let mut cycles_used = self.sync();

//         //internal
//         cycles_used += self.sync();

//         //write
//         self._push(return_address_upper);
//         cycles_used += self.sync();

//         //write
//         self._push(return_address_lower);

//         //set pc
//         self.pc = match target {
//             RstVector::H28 => 0x28,
//             _ => panic!("unimplemented rst {:?}", target),
//         };

//         cycles_used += self.sync();
//         cycles_used
//     }
// }
