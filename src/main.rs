use garlickboy::rom;

static GB_ROM: &str = "./data/Tetris.gb";
fn main() {
    let _rom = rom::load_rom(GB_ROM);
}
