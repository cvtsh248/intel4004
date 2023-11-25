use crate::fileio::load_bin;

mod i4004emu;
mod fileio;

fn main() {
    let mut cpu: i4004emu::cpu = i4004emu::cpu{
        ixr: [0; 16], 
        rom: [0; 4096], 
        ram_d: [0; 1024], 
        ram_s: [0; 256],
        ram_bank: 0,
        ram_addr: 0,
        pc: 0, 
        stack: [0; 3], 
        stack_ptr: 0, 
        acc: 0, 
        carry: 0, 
        test: 0

    };

    cpu.rom = load_bin("programs/bin/test.bin");

    cpu.execute(5);
    println!("{:?}",cpu.ixr);

}
