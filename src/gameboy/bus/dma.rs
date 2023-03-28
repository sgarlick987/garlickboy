use crate::utils::merge_bytes;

use super::OAM_ADDRESS_START;

pub(crate) struct Dma {
    pub(crate) cycle: u16,
    pub(crate) address: u16,
    pub(crate) in_progress: bool,
}

impl Dma {
    pub(crate) fn new() -> Self {
        Self {
            cycle: 0,
            address: 0,
            in_progress: false,
        }
    }

    pub(crate) fn start(&mut self, address: u8) {
        if address != 0 {
            self.address = merge_bytes(address, 0x00);
            self.cycle = 0;
            self.in_progress = true;
        }
    }

    pub(crate) fn progress(&mut self) {
        self.cycle += 1;
        if self.cycle == 160 {
            self.cycle = 0;
            self.in_progress = false;
        }
    }

    pub(crate) fn source(&self) -> u16 {
        self.address + self.cycle
    }
    pub(crate) fn destination(&self) -> u16 {
        OAM_ADDRESS_START + self.cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dma_start() {
        let mut dma = Dma::new();
        dma.start(0xC0);

        assert_eq!(dma.address, 0xC000);
        assert_eq!(dma.cycle, 0);
        assert!(dma.in_progress);
    }

    #[test]
    fn test_dma_progress() {
        let mut dma = Dma::new();
        dma.start(0xC0);
        dma.progress();

        assert_eq!(dma.cycle, 1);
        assert!(dma.in_progress);
    }
    #[test]
    fn test_dma_progress_finish() {
        let mut dma = Dma::new();
        dma.start(0xC0);
        dma.cycle = 159;
        dma.progress();

        assert_eq!(dma.cycle, 0);
        assert!(!dma.in_progress);
    }
}
