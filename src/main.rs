use garlickboy::bios;
use garlickboy::cpu;
use garlickboy::rom;

static GB_ROM: &str = "./data/Tetris.gb";
fn main() {
    let rom = rom::load_rom(GB_ROM);
    let bios = bios::load_bios("data/DMG_ROM.bin");
    let mut cpu = cpu::new_cpu();
    cpu.write_bios(bios);
    cpu.write_rom(rom);
    loop {
        cpu.step();
    }
}
