mod chip8;

use chip8::Chip8;


fn main() {
    let mut chip8= Chip8::new();

    let program = [
        0x60, 0x05, // 0x200: V0 = 5
        0x12, 0x08, // 0x202: JUMP 0x208
        0x60, 0xFF, // 0x204: V0 = 255 (se salta)
        0x60, 0xAA, // 0x206: V0 = 170 (se salta)
        0x70, 0x03, // 0x208: V0 += 3
    ];

    chip8.load_program(&program);

    chip8.cycle();
    chip8.cycle();
    chip8.cycle();
    
    chip8.print_state();



}
