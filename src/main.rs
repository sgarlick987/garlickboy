#![feature(bigint_helper_methods)]
mod cpu;
mod roms;

static GB_ROM: &str = "./data/Tetris.gb";
fn main() {
    let _rom = roms::load_rom(GB_ROM);
}
