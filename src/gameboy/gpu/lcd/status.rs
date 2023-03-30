pub(crate) struct Status {
    pub(crate) ly: u8,
    pub(crate) lyc: u8,
    pub(crate) stat: Stat,
}

#[derive(Eq, Hash, PartialEq)]
pub(crate) enum Mode {
    Hblank,
    Vblank,
    Oam,
    Transfer,
}

pub(crate) struct Stat {
    pub(crate) mode: Mode,
}

impl Stat {
    fn new() -> Self {
        Self { mode: Mode::Oam }
    }
}

impl Status {
    pub fn is_mode_vblank(&self) -> bool {
        self.stat.mode == Mode::Vblank
    }

    pub fn inc_ly(&mut self) {
        self.ly += 1;
        if self.ly == 155 {
            self.ly = 0;
        }

        if self.ly == 0 {
            self.stat.mode = Mode::Oam;
        }

        if self.ly == 21 {
            self.stat.mode = Mode::Transfer;
        }

        if self.ly == 121 {
            self.stat.mode = Mode::Hblank;
        }

        if self.ly == 145 {
            self.stat.mode = Mode::Vblank;
        }
    }

    pub fn reset_ly(&mut self) {
        self.ly = 0;
    }

    pub fn write_lyc(&mut self, lyc: u8) {
        self.lyc = lyc;
    }

    pub fn read_stat(&self) -> u8 {
        match self.stat.mode {
            Mode::Hblank => 0,
            Mode::Vblank => 1,
            Mode::Oam => 2,
            Mode::Transfer => 3,
        }
    }

    pub fn write_stat(&mut self, _: u8) {}

    pub fn new() -> Self {
        Self {
            ly: 0,
            lyc: 0,
            stat: Stat::new(),
        }
    }
}
