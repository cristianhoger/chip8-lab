mod chip8;

use chip8::Chip8;


fn main() {
    let mut chip8= Chip8::new();

    let program = [
        0x60, 0x0A, // V0 = 0x0A
        0x70, 0x05, // V0 += 0x05
        0xA3,0x50
    ];

    chip8.load_program(&program);

    chip8.cycle();
    chip8.cycle();
    chip8.cycle();
    
    chip8.print_state();



}
