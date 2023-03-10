
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
