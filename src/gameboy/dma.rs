use super::gpu::OAM_BEGIN;
use crate::{gameboy::bus::Bus, utils::merge_bytes};

pub const DMA_ADDRESS: u16 = 0xFF46;

#[derive(Eq, Hash, PartialEq, Debug)]
enum Status {
    Off,
    ScheduledInitial,
    ScheduledNext,
    InProgress,
}

pub struct Dma {
    pub cycle: u16,
    pub address: u16,
    status: Status,
    pub upper: u8,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            address: 0,
            status: Status::Off,
            upper: 0,
        }
    }

    pub fn update(&mut self, bus: &mut dyn Bus) {
        match self.status {
            Status::ScheduledInitial => self.status = Status::ScheduledNext,
            Status::ScheduledNext => self.status = Status::InProgress,
            Status::InProgress => {
                let source = self.source();
                let destination = self.destination();
                let value = bus.read_byte(source);
                bus.write_byte(destination, value);
                self.progress();
            }
            Status::Off => (),
        }
    }

    pub fn start(&mut self, address: u8) {
        if address != 0 {
            self.upper = address;
            self.address = merge_bytes(self.upper, 0x00);
            self.cycle = 0;
            self.status = Status::ScheduledInitial
        }
    }

    fn progress(&mut self) {
        self.cycle += 1;
        if self.cycle == 160 {
            self.cycle = 0;
            self.status = Status::Off;
        }
    }

    fn source(&self) -> u16 {
        self.address + self.cycle
    }

    fn destination(&self) -> u16 {
        OAM_BEGIN + self.cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::bus::MockBus;
    use mockall::predicate::eq;

    #[test]
    fn test_update_not_in_progress() {
        let mut dma = Dma::new();
        let mut bus = MockBus::new();
        bus.expect_read_byte().never();
        bus.expect_write_byte().never();

        dma.update(&mut bus);

        assert_eq!(dma.cycle, 0);
    }

    #[test]
    fn test_update_in_progress_finish() {
        const ADDRESS_UPPER: u8 = 0xC0;
        const ADDRESS: u16 = 0xC000;
        const CYCLE: u16 = 159;
        const BYTE: u8 = 0xFF;

        let mut dma = Dma::new();
        let mut bus = MockBus::new();
        bus.expect_read_byte()
            .once()
            .with(eq(ADDRESS + CYCLE))
            .return_const(BYTE);
        bus.expect_write_byte()
            .once()
            .with(eq(OAM_BEGIN + CYCLE), eq(BYTE))
            .return_const(());
        dma.start(ADDRESS_UPPER);
        dma.cycle = 159;

        dma.update(&mut bus);

        assert_eq!(dma.cycle, 0);
        assert_eq!(dma.status, Status::Off);
    }

    #[test]
    fn test_update_in_progress() {
        const ADDRESS_UPPER: u8 = 0xC0;
        const ADDRESS: u16 = 0xC000;
        const CYCLE: u16 = 0;
        const BYTE: u8 = 0xFF;

        let mut dma = Dma::new();
        let mut bus = MockBus::new();
        bus.expect_read_byte()
            .once()
            .with(eq(ADDRESS + CYCLE))
            .return_const(BYTE);
        bus.expect_write_byte()
            .once()
            .with(eq(OAM_BEGIN + CYCLE), eq(BYTE))
            .return_const(());
        dma.start(ADDRESS_UPPER);

        dma.update(&mut bus);

        assert_eq!(dma.cycle, CYCLE + 1);
    }

    #[test]
    fn test_source() {
        let mut dma = Dma::new();
        dma.cycle = 5;
        dma.address = 0xC000;

        let source = dma.source();

        assert_eq!(source, 0xC005)
    }

    #[test]
    fn test_desination() {
        const CYCLE: u16 = 5;
        let mut dma = Dma::new();
        dma.cycle = CYCLE;

        let destination = dma.destination();

        assert_eq!(destination, OAM_BEGIN + CYCLE)
    }

    #[test]
    fn test_start() {
        let mut dma = Dma::new();
        dma.start(0xC0);

        assert_eq!(dma.address, 0xC000);
        assert_eq!(dma.cycle, 0);
        assert_eq!(dma.status, Status::ScheduledInitial);
    }

    #[test]
    fn test_not_start() {
        let mut dma = Dma::new();
        dma.start(0);

        assert_eq!(dma.address, 0);
        assert_eq!(dma.cycle, 0);
        assert_eq!(dma.status, Status::Off);
    }

    #[test]
    fn test_progress() {
        let mut dma = Dma::new();
        let bus = &mut MockBus::new();
        dma.start(0xC0);
        dma.update(bus);
        dma.update(bus);
        dma.update(bus);

        assert_eq!(dma.cycle, 1);
        assert_eq!(dma.status, Status::InProgress);
    }

    #[test]
    fn test_progress_finish() {
        let mut dma = Dma::new();
        dma.start(0xC0);
        dma.cycle = 159;
        dma.progress();

        assert_eq!(dma.cycle, 0);
        assert_eq!(dma.status, Status::Off);
    }
}
