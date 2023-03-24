//     pub fn reti(&mut self) -> u8 {
//         let mut cycles = self.ret();
//         self.interrupt_handler.schedule_ime();
//         cycles += self.sync();
//         cycles
//     }
