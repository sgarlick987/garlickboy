use crate::{display::Display, gpu::*};

#[cfg_attr(test, mockall::automock)]
pub trait Bus {
    fn read_byte(&mut self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
    fn write_bytes(&mut self, address: usize, bytes: Vec<u8>);
    // fn render(&mut self);
    fn update_display(&mut self, display: &mut Display);
    // fn sync(&mut self) -> u8;
    fn write_boot(&mut self, bytes: [u8; 0x100]);
}

pub struct AddressBus {
    boot_rom_mapped: bool,
    boot_rom: [u8; 0x100],
    memory: [u8; 0x10000],
    pub gpu: Box<dyn GPU>,
}

impl AddressBus {
    pub fn new(gpu: Box<dyn GPU>) -> AddressBus {
        AddressBus {
            boot_rom_mapped: true,
            boot_rom: [0; 0x100],
            memory: [0; 0x10000],
            gpu,
        }
    }
}

impl Bus for AddressBus {
    fn update_display(&mut self, display: &mut Display) {
        self.gpu.update_display(display);
    }

    // fn sync(&mut self) -> u8 {
    //     self.gpu.sync()
    // }

    // fn render(&mut self) {
    //     self.gpu.render();
    // }

    fn read_byte(&mut self, address: u16) -> u8 {
        let address = address as usize;
        if PPU_REGISTERS.contains(&address) {
            return self.gpu.read_registers(address);
        }
        match address {
            0x0..=0xFF => {
                if self.boot_rom_mapped {
                    self.boot_rom[address]
                } else {
                    self.memory[address]
                }
            }
            0xFF00 => 0xFF,
            0xFF80 => 0,
            0xFF81 => 0,
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            _ => self.memory[address],
        }
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;
        if PPU_REGISTERS.contains(&address) {
            self.gpu.write_registers(address, byte);
            return;
        }
        match address {
            0xFF50 => self.boot_rom_mapped = false,
            0x0000..=0x7FFF => (), // ignore writes to rom
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, byte);
            }
            _ => {
                self.memory[address] = byte;
            }
        }
    }

    fn write_bytes(&mut self, address: usize, bytes: Vec<u8>) {
        let end = address + bytes.len();
        self.memory[address..end].copy_from_slice(bytes.as_slice());
    }

    fn write_boot(&mut self, bytes: [u8; 0x100]) {
        self.boot_rom = bytes;
    }
}
