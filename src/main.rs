mod chip8;

use chip8::Chip8;


fn main() {
    let mut chip8= Chip8::new();

    let program = [
        0x60, 0xFF,
        0x70, 0x02
    ];

    chip8.load_program(&program);

    chip8.cycle();
    chip8.cycle();

    chip8.print_state();



}
