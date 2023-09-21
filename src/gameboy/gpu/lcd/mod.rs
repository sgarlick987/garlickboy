use crate::gameboy::interrupts::InterruptHandler;

use self::{control::Control, status::Status};

mod control;
mod status;

pub(crate) struct Lcd {
    ly_cycle_counter: u8,
    status: Status,
    control: Control,
}

impl Lcd {
    pub fn tile_map_area_address(&self) -> u16 {
        self.control.tile_map_area_address()
    }

    pub fn is_lcd_enabled(&self) -> bool {
        self.control.is_lcd_enabled()
    }

    fn is_mode_vblank(&self) -> bool {
        self.status.is_mode_vblank()
    }

    pub fn read_control(&self) -> u8 {
        self.control.read()
    }

    pub fn read_stat(&self) -> u8 {
        self.status.read_stat()
    }

    pub fn write_control(&mut self, control: u8) {
        self.control.write(control);
        if !self.is_lcd_enabled() {
            self.status.reset_ly();
        }
    }

    pub fn write_stat(&mut self, stat: u8) {
        self.status.write_stat(stat);
    }

    pub fn write_lyc(&mut self, lyc: u8) {
        self.status.write_lyc(lyc);
    }

    fn inc_ly(&mut self) {
        if !self.is_lcd_enabled() {
            panic!("gpu inc_ly should not be called while lcd is not enable");
        }
        self.status.inc_ly();
    }

    pub fn read_lyc(&self) -> u8 {
        self.status.lyc
    }

    pub fn read_ly(&self) -> u8 {
        self.status.ly
    }

    pub fn update(&mut self, interrupt_handler: &mut InterruptHandler) {
        if self.is_lcd_enabled() {
            self.ly_cycle_counter += 1;
            if self.ly_cycle_counter == 114 {
                self.ly_cycle_counter = 0;
                self.inc_ly();
                if self.is_mode_vblank() {
                    interrupt_handler.set_vblank_flag();
                }
            }
        } else {
            self.ly_cycle_counter = 0;
        }
    }

    pub fn new() -> Self {
        let ly_cycle_counter = 0;
        let status = Status::new();
        let control = Control::new();
        Self {
            ly_cycle_counter,
            status,
            control,
        }
    }
}
