mod tac;

use self::tac::TimerControl;
use super::interrupts::InterruptHandler;

const TIMER_REGISTER_ADDRESSES: [u16; 4] = [TIMA_ADDRESS, TMA_ADDRESS, TAC_ADDRESS, DIV_ADDRESS];
const TIMA_ADDRESS: u16 = 0xFF05;
const TMA_ADDRESS: u16 = 0xFF06;
const TAC_ADDRESS: u16 = 0xFF07;
const DIV_ADDRESS: u16 = 0xFF04;

pub struct Timer {
    reloaded: bool,
    overflowed: bool,
    div: u16,              // DIV
    counter: u8,           // TIMA
    modulo: u8,            // TMA
    control: TimerControl, // TAC
}

impl Timer {
    pub fn new() -> Self {
        let reloaded = false;
        let overflowed = false;
        let div = 0;
        let counter = 0;
        let modulo = 0;
        let control = TimerControl::new();

        Self {
            reloaded,
            overflowed,
            div,
            counter,
            modulo,
            control,
        }
    }

    pub fn handles(&self, address: u16) -> bool {
        TIMER_REGISTER_ADDRESSES.contains(&address)
    }

    pub fn read_register(&self, address: u16) -> u8 {
        match address {
            DIV_ADDRESS => (self.div >> 8) as u8,
            TIMA_ADDRESS => self.counter,
            TMA_ADDRESS => self.modulo,
            TAC_ADDRESS => u8::from(self.control),
            _ => panic!("invalid timer register address {}", address),
        }
    }

    pub fn write_register(&mut self, address: u16, byte: u8) {
        match address {
            DIV_ADDRESS => self.reset_div(),
            TIMA_ADDRESS => self.write_tima(byte),
            TMA_ADDRESS => self.write_tma(byte),
            TAC_ADDRESS => self.write_tac(byte),
            _ => panic!("invalid timer register address {}", address),
        }
    }

    pub fn update(&mut self, interrupt_handler: &mut InterruptHandler) {
        self.reloaded = false;
        if self.overflowed {
            self.div = self.div.wrapping_add(4);
            self.overflowed = false;
            self.reloaded = true;
            self.counter = self.modulo;
            interrupt_handler.set_timer_flag();
        } else if self.control.enabled && self.is_frequency_bit_set() {
            self.div = self.div.wrapping_add(4);
            let new = self.is_frequency_bit_set();
            if !new {
                self.inc();
            }
        } else {
            self.div = self.div.wrapping_add(4);
        }
    }

    fn write_tima(&mut self, byte: u8) {
        if self.reloaded {
            return;
        }
        if self.overflowed {
            self.overflowed = false;
        }
        self.counter = byte;
    }

    fn write_tma(&mut self, byte: u8) {
        if self.reloaded {
            self.counter = byte;
        }
        self.modulo = byte;
    }

    fn write_tac(&mut self, byte: u8) {
        let old = self.control.enabled && self.is_frequency_bit_set();
        self.control = TimerControl::from(byte);
        let new = self.control.enabled && self.is_frequency_bit_set();
        if old && !new {
            self.inc();
        }
    }

    fn reset_div(&mut self) {
        if self.control.is_frequency_bit_set(self.div) {
            self.inc();
        }
        self.div = 0;
    }

    fn inc(&mut self) {
        (self.counter, self.overflowed) = self.counter.overflowing_add(1);
    }

    fn is_frequency_bit_set(&self) -> bool {
        self.control.is_frequency_bit_set(self.div)
    }
}
