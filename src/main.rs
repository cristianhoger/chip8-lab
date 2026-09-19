mod chip8;

use chip8::Chip8;


fn main() {
    let mut chip8= Chip8::new();

    let program = [
        0x60, 0xFF,
        0x70, 0x02
    ];

    chip8.load_program(&program);

    let opcode=chip8.fetch_opcode();
    chip8.execute_opcode(opcode);
    chip8.next_instruction();

    let opcode=chip8.fetch_opcode();
    chip8.execute_opcode(opcode);
    chip8.next_instruction();

    chip8.print_state();



}
