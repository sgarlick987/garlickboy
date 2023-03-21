pub const IF_ADDRESS: u16 = 0xFF0F;
pub const IE_ADDRESS: u16 = 0xFFFF;

enum MasterEnable {
    Disabled,
    ScheduledInitial,
    ScheduledNext,
    Enabled,
}

impl MasterEnable {
    fn is_enabled(&self) -> bool {
        matches!(self, MasterEnable::Enabled)
    }

    fn next(&self) -> MasterEnable {
        match self {
            MasterEnable::Disabled => MasterEnable::Disabled,
            MasterEnable::ScheduledInitial => MasterEnable::ScheduledNext,
            MasterEnable::ScheduledNext => MasterEnable::Enabled,
            MasterEnable::Enabled => MasterEnable::Enabled,
        }
    }
}

pub struct InterruptHandler {
    ime: MasterEnable,
    flags: Interrupts,
    enable: Interrupts,
}

impl InterruptHandler {
    pub fn new() -> InterruptHandler {
        let ime = MasterEnable::Disabled;
        let flags = Interrupts::new();
        let enable = Interrupts::new();

        InterruptHandler { ime, flags, enable }
    }

    pub fn flag_vblank(&mut self) {
        self.flags.vblank = true;
    }

    pub fn set_flags(&mut self, byte: u8) {
        self.flags = Interrupts::from(byte);
    }

    pub fn get_flags(&self) -> u8 {
        u8::from(self.flags)
    }

    pub fn set_enable(&mut self, byte: u8) {
        self.enable = Interrupts::from(byte);
    }

    pub fn get_enable(&self) -> u8 {
        u8::from(self.enable)
    }

    pub fn disable_ime(&mut self) {
        self.ime = MasterEnable::Disabled;
    }

    pub fn schedule_ime(&mut self) {
        self.ime = MasterEnable::ScheduledInitial;
    }

    fn update_ime(&mut self) {
        self.ime = self.ime.next();
    }

    pub fn handle(&mut self) -> u16 {
        self.update_ime();
        if self.ime.is_enabled() {
            if self.flags.vblank {
                self.flags.vblank = false;
                self.disable_ime();
                return 0x0040;
            }
        }
        0x0000
    }
}

#[derive(Clone, Copy)]
struct Interrupts {
    vblank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl Interrupts {
    fn new() -> Interrupts {
        Interrupts::from(0)
    }
}

const INTERRUPTS_VBLANK_BIT: u8 = 1;
const INTERRUPTS_LCD_STAT_BIT: u8 = 1 << 1;
const INTERRUPTS_TIMER_BIT: u8 = 1 << 2;
const INTERRUPTS_SERIAL_BIT: u8 = 1 << 3;
const INTERRUPTS_JOYPAD_BIT: u8 = 1 << 4;

impl std::convert::From<Interrupts> for u8 {
    fn from(interrupts: Interrupts) -> u8 {
        (if interrupts.vblank {
            INTERRUPTS_VBLANK_BIT
        } else {
            0
        }) | (if interrupts.lcd_stat {
            INTERRUPTS_LCD_STAT_BIT
        } else {
            0
        }) | (if interrupts.timer {
            INTERRUPTS_TIMER_BIT
        } else {
            0
        }) | (if interrupts.serial {
            INTERRUPTS_SERIAL_BIT
        } else {
            0
        } | (if interrupts.joypad {
            INTERRUPTS_JOYPAD_BIT
        } else {
            0
        }))
    }
}

impl std::convert::From<u8> for Interrupts {
    fn from(byte: u8) -> Self {
        let vblank = (byte & INTERRUPTS_VBLANK_BIT) != 0;
        let lcd_stat = (byte & INTERRUPTS_LCD_STAT_BIT) != 0;
        let timer = (byte & INTERRUPTS_TIMER_BIT) != 0;
        let serial = (byte & INTERRUPTS_SERIAL_BIT) != 0;
        let joypad = (byte & INTERRUPTS_JOYPAD_BIT) != 0;

        Interrupts {
            vblank,
            lcd_stat,
            timer,
            serial,
            joypad,
        }
    }
}
