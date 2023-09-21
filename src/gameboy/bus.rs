use super::{
    bios::{Bios, BIOS_ADDRESS_END, BIOS_ADDRESS_START, BIOS_MAPPED_ADDRESS},
    dma::{Dma, DMA_ADDRESS},
    joypad::{Joypad, JOYPAD_ADDRESS},
    GameboyCycles,
};
use super::{
    gpu::{new_ppu, Gpu, OAM_BEGIN, OAM_END, VRAM_BEGIN, VRAM_END},
    interrupts::{InterruptHandler, IE_ADDRESS, IF_ADDRESS},
    timer::Timer,
};
use crate::emu::{controller::Controller, display::Display, rom::Rom};

pub const HRAM_ADDRESS_START: u16 = 0xFF80;
// const HRAM_ADDRESS_END: u16 = 0xFFFE;
const UNUSED_ADDRESSES: [u16; 13] = [
    0xFF03, 0xFF08, 0xFF09, 0xFF0A, 0xFF0B, 0xFF0C, 0xFF0D, 0xFF0E, 0xFF15, 0xFF1F, 0xFF27, 0xFF28,
    0xFF29,
];

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
pub trait Bus {
    fn update_dma(&mut self);
    fn update_gpu(&mut self);
    fn update_timer(&mut self);
    fn update_ime(&mut self);
    fn update_joypad(&mut self, controller: &Box<dyn Controller>);
    fn render_display(&mut self, display: &mut Box<dyn Display>);
    fn schedule_ime(&mut self);
    fn disable_ime(&mut self);
    fn has_interrupt_pending(&self) -> bool;
    fn next_interrupt_cycles(&mut self) -> GameboyCycles;
    fn read_byte(&mut self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
    fn load_rom(&mut self, rom: &Rom);
}

pub fn new_address_bus() -> Box<dyn Bus> {
    AddressBus::new()
}

pub struct AddressBus {
    bios: Bios,
    timer: Timer,
    dma: Option<Dma>,
    interrupt_handler: InterruptHandler,
    memory: [u8; 0x10000],
    gpu: Box<dyn Gpu>,
    joypad: Joypad,
}

impl AddressBus {
    fn new() -> Box<dyn Bus> {
        let gpu = new_ppu();
        let bios = Bios::new();
        let dma = Some(Dma::new());
        let joypad = Joypad::new();
        let timer = Timer::new();
        let interrupt_handler = InterruptHandler::new();

        Box::new(Self {
            bios,
            timer,
            dma,
            interrupt_handler,
            memory: [0; 0x10000],
            gpu,
            joypad,
        })
    }
}

impl Bus for AddressBus {
    fn next_interrupt_cycles(&mut self) -> GameboyCycles {
        self.interrupt_handler.next_cycles()
    }

    fn has_interrupt_pending(&self) -> bool {
        self.interrupt_handler.is_pending()
    }

    fn update_dma(&mut self) {
        let mut dma = self.dma.take().unwrap();
        dma.update(self);
        self.dma = Some(dma);
    }

    fn update_ime(&mut self) {
        self.interrupt_handler.update_ime();
    }

    fn update_gpu(&mut self) {
        self.gpu.update(&mut self.interrupt_handler);
    }

    fn update_joypad(&mut self, controller: &Box<dyn Controller>) {
        self.joypad.update(controller);
    }

    fn render_display(&mut self, display: &mut Box<dyn Display>) {
        self.gpu.render_display(display);
    }

    fn update_timer(&mut self) {
        self.timer.update(&mut self.interrupt_handler);
    }

    fn schedule_ime(&mut self) {
        self.interrupt_handler.schedule_ime();
    }

    fn disable_ime(&mut self) {
        self.interrupt_handler.disable_ime();
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0xFF4C..=0xFF7F => 0xFF,
            address if UNUSED_ADDRESSES.contains(&address) => 0xFF,
            address if self.gpu.handles(address) => self.gpu.read_register(address),
            address if self.timer.handles(address) => self.timer.read_register(address),
            BIOS_ADDRESS_START..=BIOS_ADDRESS_END => {
                if self.bios.mapped {
                    self.bios.data[address as usize]
                } else {
                    self.memory[address as usize]
                }
            }
            IF_ADDRESS => self.interrupt_handler.read_flags(),
            IE_ADDRESS => self.interrupt_handler.read_enable(),
            0xFF02 => 0xFF,
            0xFF10 => self.memory[address as usize] | 0b1_0000000,
            0xFF1A => self.memory[address as usize] | 0b0_1111111,
            0xFF1C => self.memory[address as usize] | 0b1_00_11111,
            0xFF20 => self.memory[address as usize] | 0b11_000000,
            0xFF23 => self.memory[address as usize] | 0b00_111111,
            0xFF26 => self.memory[address as usize] | 0b0_111_0000,
            0xFEA0..=0xFEFF => 0,
            JOYPAD_ADDRESS => self.joypad.read(),
            DMA_ADDRESS => {
                let dma = self.dma.as_ref().unwrap();
                dma.upper
            }
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            OAM_BEGIN..=OAM_END => self.gpu.read_oam(address - OAM_BEGIN),
            _ => self.memory[address as usize],
        }
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            BIOS_MAPPED_ADDRESS => self.bios.mapped = false,
            0x0000..=0x7FFF => (), // ignore writes to rom
            0xFEA0..=0xFEFF => (), // prohibited
            0xFF4C..=0xFF7F => (),
            address if UNUSED_ADDRESSES.contains(&address) => (),
            address if self.gpu.handles(address) => self.gpu.write_register(address, byte),
            address if self.timer.handles(address) => self.timer.write_register(address, byte),
            IF_ADDRESS => self.interrupt_handler.write_flags(byte),
            IE_ADDRESS => self.interrupt_handler.write_enable(byte),
            0xFF02 => (),
            JOYPAD_ADDRESS => self.joypad.select(byte),
            DMA_ADDRESS => {
                let dma = self.dma.as_mut().unwrap();
                dma.start(byte);
            }
            VRAM_BEGIN..=VRAM_END => self.gpu.write_vram(address - VRAM_BEGIN, byte),
            OAM_BEGIN..=OAM_END => self.gpu.write_oam(address - OAM_BEGIN, byte),
            _ => {
                self.memory[address as usize] = byte;
            }
        }
    }

    fn load_rom(&mut self, rom: &Rom) {
        let bytes = rom.data.as_slice();
        let end = bytes.len();
        self.memory[0..end].copy_from_slice(bytes);
    }
}
