use sdl2::pixels::Color;

pub const DEFAULT_PALETTE: Palette = Palette {
    zero: Color::WHITE,
    one: Color::RGB(169, 169, 169),
    two: Color::RGB(84, 84, 84),
    three: Color::BLACK,
};

pub struct Palette {
    pub zero: Color,
    pub one: Color,
    pub two: Color,
    pub three: Color,
}

impl std::convert::From<u8> for Palette {
    fn from(byte: u8) -> Self {
        Palette {
            zero: Palette::map_color(byte >> 1 & 1, byte & 1),
            one: Palette::map_color(byte >> 3 & 1, byte >> 2 & 1),
            two: Palette::map_color(byte >> 5 & 1, byte >> 4 & 1),
            three: Palette::map_color(byte >> 7 & 1, byte >> 6 & 1),
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

    pub fn decode_color(&self, upper: u8, lower: u8) -> Color {
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

    pub fn map_color(upper: u8, lower: u8) -> Color {
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
