// fn decode_color(upper: u8, lower: u8) -> Color {
//     match upper {
//         1 => match lower {
//             1 => Color::BLACK.clone(),
//             0 => Color {
//                 r: 169,
//                 g: 169,
//                 b: 169,
//             },
//             _ => panic!("invalid color"),
//         },
//         0 => match lower {
//             0 => Color::WHITE.clone(),
//             1 => Color {
//                 r: 221,
//                 g: 221,
//                 b: 221,
//             },
//             _ => panic!("invalid color"),
//         },
//         _ => panic!("invalid color"),
//     }
// }

// fn bytes_to_color(upper: u8, lower: u8) -> Vec<Color> {
//     vec![
//         decode_color((upper >> 7) & 0b1, (lower >> 7) & 0b1),
//         decode_color((upper >> 6) & 0b1, (lower >> 6) & 0b1),
//         decode_color((upper >> 5) & 0b1, (lower >> 5) & 0b1),
//         decode_color((upper >> 4) & 0b1, (lower >> 4) & 0b1),
//         decode_color((upper >> 3) & 0b1, (lower >> 3) & 0b1),
//         decode_color((upper >> 2) & 0b1, (lower >> 2) & 0b1),
//         decode_color((upper >> 1) & 0b1, (lower >> 1) & 0b1),
//         decode_color(upper & 0b1, lower & 0b1),
//     ]
// }

// +------+------------+
// | 0b11 | white      |
// | 0b10 | dark-gray  |
// | 0b01 | light-gray |
// | 0b00 | black      |
// +------+------------+

// let canvas = Canvas::new(256, 256).title("garlickboy");
// canvas.render(move |_, image| {
//     let width = image.width() as usize;
//     for (y, row) in image.chunks_mut(width).enumerate() {
//         let lower = (y / 32 * 2) + 256;
//         let upper = lower + 1;
//         let colors = bytes_to_color(rom.tile_data[upper] as u8, rom.tile_data[lower] as u8);
//         for (x, pixels) in row.chunks_mut(32).enumerate() {
//             for pixel in pixels {
//                 *pixel = colors[x];
//             }
//         }
//     }
// });
