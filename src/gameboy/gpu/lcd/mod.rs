use self::{control::Control, status::Status};

mod control;
mod status;

pub(crate) struct Lcd {
    status: Status,
    control: Control,
}

impl Lcd {
    pub fn is_lcd_enabled(&self) -> bool {
        self.control.is_lcd_enabled()
    }

    pub fn is_mode_vblank(&self) -> bool {
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

    pub fn inc_ly(&mut self) {
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

    pub fn new() -> Self {
        let status = Status::new();
        let control = Control::new();
        Self { status, control }
    }
}
