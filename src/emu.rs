use crate::{address::*, bios::*, display::Display, gpu::PPU, joypad::Joypad, rom::*};

use sdl2::{gfx::framerate::FPSManager, Sdl};

use crate::cpu::GameboyChip;

const MAX_MCYCLES_PER_FRAME: u32 = 1050000 / 60;
const GB_ROM: &str = "./data/Tetris.gb";

pub struct Emu {
    cycles: u32,
    chip: GameboyChip,
    fps_manager: FPSManager,
    display: Display,
    // joypad: Joypad,
    rom: Rom,
    bios: Bios,
}

impl Emu {
    pub fn new() -> Emu {
        // let event_pump = sdl.event_pump().expect("failed to get event_pump");
        let display = Display::new();
        let mut fps_manager = FPSManager::new();
        fps_manager
            .set_framerate(60)
            .expect("failed to set fps_manager framerate to 60");

        let rom = load_rom(GB_ROM);
        let bios = load_bios("data/dmg_boot.bin");
        // let joypad = Joypad::new(event_pump);
        let gpu = Box::new(PPU::new());
        let bus = Box::new(AddressBus::new(gpu));
        let chip = GameboyChip::new(bus);

        let mut emu = Emu {
            fps_manager,
            display,
            chip,
            rom,
            bios,
            // joypad,
            cycles: 0,
        };
        emu.write_bios();
        emu.write_rom();

        emu
    }

    pub fn run(&mut self) -> bool {
        // loop {
        //     self.joypad.read();
        //     // self.chip.update_joypad(self.joypad);

        //     let mut cycles_used = 0;
        //     let insts = self.chip.fetch();
        //     for inst in insts {
        //         self.chip.execute(inst);
        //     }
        //     cycles_used += 1;

        //     if cycles_used == MAX_MCYCLES_PER_FRAME {
        //         break;
        //     }
        // }

        // self.chip.update_display(&mut self.display);
        self.display.off();
        self.display.present();
        // self.fps_manager.delay();
        true
    }

    fn write_bios(&mut self) {
        self.chip.write_bios(self.bios.data);
    }

    fn write_rom(&mut self) {
        self.chip.write_bytes(0, self.rom.data.to_vec());
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    #[test]
    fn test_schedule() {}
}

// use crate::{
//     address::*,
//     bios::*,
//     chip::*,
//     gpu::{display::Display, *},
//     rom::*,
// };
// use sdl2::{event::Event, gfx::framerate::FPSManager, keyboard::Keycode, EventPump};

// const MAX_CYCLES: u32 = 69905;
// const GB_ROM: &str = "./data/Tetris.gb";

// pub struct Emu {
//     fps_manager: FPSManager,
//     chip: GameboyChip,
//     rom: Rom,
//     bios: Bios,
//     event_pump: EventPump,
// }

// impl Emu {
//     pub fn new() -> Emu {
//         let rom = load_rom(GB_ROM);
//         let bios = load_bios("data/dmg_boot.bin");
//         let display = Display::new();
//         let event_pump = display.event_pump();

//         let mut fps_manager = FPSManager::new();
//         fps_manager
//             .set_framerate(60)
//             .expect("failed to set fps_manager framerate to 60");
//         let gpu = Box::new(PPU::new());
//         let bus = Box::new(AddressBus::new(gpu));
//         let chip = GameboyChip::new(bus);

//         Emu {
//             fps_manager,
//             chip,
//             rom,
//             bios,
//             event_pump,
//         }
//     }

//     pub fn init(&mut self) {
//         self.write_bios();
//         self.write_rom();
//     }

//     fn write_bios(&mut self) {
//         self.chip.write_bios(self.bios.data);
//     }

//     fn write_rom(&mut self) {
//         self.chip.write_bytes(0, self.rom.data.to_vec());
//     }

//     // pub fn update(&mut self) -> bool {
//     //     for event in self.event_pump.poll_iter() {
//     //         match event {
//     //             Event::Quit { .. }
//     //             | Event::KeyDown {
//     //                 keycode: Some(Keycode::Escape),
//     //                 ..
//     //             } => return false,
//     //             _ => {}
//     //         }
//     //     }
//     //     let mut cycles_used = 0;
//     //     while cycles_used < MAX_CYCLES {
//     //         let cycles = self.chip.step_old();
//     //         cycles_used += cycles as u32;
//     //     }
//     //     self.chip.render();
//     //     self.fps_manager.delay();
//     //     true
//     // }
// }
