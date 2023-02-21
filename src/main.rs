mod roms;

static GB_ROM: &str = "Legend of Zelda, The - Link's Awakening (G) [!].gb";
static GBC_ONLY_ROM: &str = "Bust-A-Move Millennium (USA, Europe).gbc";
static GBC_ROM: &str = "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (SGB Enhanced).gbc";

fn main() {
    let rom_path = format!("./data/{}", GB_ROM);
    let rom = roms::rom::load_rom(rom_path);
    println!("{}", rom.data.len());
    println!("{:X?}", &rom.data[0..50])
}
