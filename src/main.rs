mod i4004emu;
mod fileio;

fn main() {
    let mut cpu: i4004emu::CPU = i4004emu::CPU{
        ixr: [0; 16], 
        rom: [0; 4096], 
        rom_io: [0; 16],
        rom_page: 0,
        ram_d: [0; 1024], 
        ram_s: [0; 256],
        ram_o: [0; 8],
        ram_bank: 0,
        ram_addr: 0,
        pc: 0, 
        stack: [0; 3], 
        stack_ptr: 0, 
        acc: 0, 
        carry: 0, 
        test: 0,
        cycle: 0

    };

    let conf = fileio::load_config().expect("Couldn't load Config settings.toml");

    println!("Intel 4004 Emulator");

    cpu.rom = fileio::load_bin(&conf.config.filepath);

    cpu.execute(conf.config.maxcycles, &conf.config.outputpath);

    println!("Done.")
    // println!("{:?}",cpu.ram_d);

}
