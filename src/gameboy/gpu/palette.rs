use sdl2::pixels::Color;

pub struct Palettes {
    pub background: Palette,
    pub object0: Palette,
    pub object1: Palette,
}

impl Palettes {
    pub fn new() -> Self {
        let background = Palette::from(0);
        let object0 = Palette::from(0);
        let object1 = Palette::from(0);

        Self {
            background,
            object0,
            object1,
        }
    }
}

pub struct Palette {
    pub zero: Color,
    one: Color,
    two: Color,
    three: Color,
    pub byte: u8,
}

impl std::convert::From<u8> for Palette {
    fn from(byte: u8) -> Self {
        Palette {
            zero: Palette::map_color(byte >> 1 & 1, byte & 1),
            one: Palette::map_color(byte >> 3 & 1, byte >> 2 & 1),
            two: Palette::map_color(byte >> 5 & 1, byte >> 4 & 1),
            three: Palette::map_color(byte >> 7 & 1, byte >> 6 & 1),
            byte,
        }
    }
}

impl Palette {
    pub fn bytes_to_color(&self, upper: u8, lower: u8) -> Vec<Color> {
        vec![
            self.decode_color((upper >> 7) & 1, (lower >> 7) & 1),
            self.decode_color((upper >> 6) & 1, (lower >> 6) & 1),
            self.decode_color((upper >> 5) & 1, (lower >> 5) & 1),
            self.decode_color((upper >> 4) & 1, (lower >> 4) & 1),
            self.decode_color((upper >> 3) & 1, (lower >> 3) & 1),
            self.decode_color((upper >> 2) & 1, (lower >> 2) & 1),
            self.decode_color((upper >> 1) & 1, (lower >> 1) & 1),
            self.decode_color(upper & 1, lower & 1),
        ]
    }

    fn decode_color(&self, upper: u8, lower: u8) -> Color {
        match upper {
            1 => match lower {
                1 => self.three,
                0 => self.two,
                _ => panic!("invalid color"),
            },
            0 => match lower {
                0 => self.zero,
                1 => self.one,
                _ => panic!("invalid color"),
            },
            _ => panic!("invalid color"),
        }
    }

    fn map_color(upper: u8, lower: u8) -> Color {
        match upper {
            1 => match lower {
                1 => Color::BLACK,
                0 => Color::RGB(84, 84, 84),
                _ => panic!("invalid color"),
            },
            0 => match lower {
                0 => Color::WHITE,
                1 => Color::RGB(169, 169, 169),
                _ => panic!("invalid color"),
            },
            _ => panic!("invalid color"),
        }
    }
}
