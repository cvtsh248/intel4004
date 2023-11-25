mod i4004emu;
use i4004emu::intel4004;
fn main() {
    let mut cpu: intel4004::cpu = intel4004::cpu{
        ixr: [0; 16], 
        rom: [0; 4096], 
        ram_d: [0; 1024], 
        ram_s: [0; 256],
        pc: 0, 
        stack: [0; 3], 
        stack_ptr: 0, 
        acc: 0, 
        carry: 0, 
        test: 0

    };

    // cpu.rom[0] = 0b00110010;
    // cpu.rom[1] = 0b01100010;
    // cpu.ixr[0] = 0b00000101;
    // cpu.rom[5] = 0b11111111;
    // cpu.rom[0] = 0b00100010;
    // cpu.rom[1] = 0b00000101;
    // cpu.rom[5] = 0b11111111;
    cpu.rom[0] = 0x61;
    cpu.rom[1] = 0xD5;
    cpu.rom[2] = 0x81;
    cpu.rom[3] = 0xB1;
    cpu.rom[4] = 0x01;

    cpu.execute(5);
    println!("{:?}",cpu.ixr);

}
