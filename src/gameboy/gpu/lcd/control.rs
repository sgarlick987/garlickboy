pub(crate) struct Control {
    obj_size_16: bool,
    lcd_enabled: bool,
    window_enabled: bool,
    obj_enabled: bool,
    bg_window_priority: bool,
    bg_tile_map_area: bool,
}

impl Control {
    pub fn tile_map_area_address(&self) -> u16 {
        if self.bg_tile_map_area {
            0x1C00
        } else {
            0x1800
        }
    }

    pub fn is_lcd_enabled(&self) -> bool {
        self.lcd_enabled
    }

    pub fn read(&self) -> u8 {
        (if self.lcd_enabled { 1 << 7 } else { 0 })
            | (if self.window_enabled { 1 << 5 } else { 0 })
            | (if self.bg_tile_map_area { 1 << 3 } else { 0 })
            | (if self.obj_size_16 { 1 << 2 } else { 0 })
            | (if self.obj_enabled { 1 << 1 } else { 0 })
            | (if self.bg_window_priority { 1 } else { 0 })
    }

    pub fn write(&mut self, control: u8) {
        self.lcd_enabled = control >> 7 & 1 == 1;
        self.window_enabled = control >> 5 & 1 == 1;
        self.bg_tile_map_area = control >> 3 & 1 == 1;
        self.obj_size_16 = control >> 2 & 1 == 1;
        self.obj_enabled = control >> 1 & 1 == 1;
        self.bg_window_priority = control & 1 == 1;
    }

    pub fn new() -> Self {
        Self {
            obj_size_16: false,
            lcd_enabled: false,
            window_enabled: false,
            obj_enabled: false,
            bg_window_priority: false,
            bg_tile_map_area: false,
        }
    }
}
