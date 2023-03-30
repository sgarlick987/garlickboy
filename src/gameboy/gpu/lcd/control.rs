pub(crate) struct Control {
    obj_size: ObjSize,
    lcd_enabled: bool,
    window_enabled: bool,
    obj_enabled: bool,
    bg_window_priority: bool,
}

enum ObjSize {
    Eight,
    Sixteen,
}

impl Control {
    pub fn is_lcd_enabled(&self) -> bool {
        self.lcd_enabled
    }

    pub fn read(&self) -> u8 {
        let mut lcd: u8 = 0;
        if self.lcd_enabled {
            lcd |= 1 << 7;
        }
        lcd
    }

    pub fn write(&mut self, control: u8) {
        self.lcd_enabled = control >> 7 & 1 == 1;
        self.window_enabled = control >> 5 & 1 == 1;
        self.obj_size = if control >> 2 & 1 == 1 {
            ObjSize::Sixteen
        } else {
            ObjSize::Eight
        };
        self.obj_enabled = control >> 1 & 1 == 1;
        self.bg_window_priority = control & 1 == 1;
    }

    pub fn new() -> Self {
        Self {
            obj_size: ObjSize::Eight,
            lcd_enabled: false,
            window_enabled: false,
            obj_enabled: false,
            bg_window_priority: false,
        }
    }
}
