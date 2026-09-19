mod chip8;

use chip8::Chip8;


fn main() {
    let mut chip8= Chip8::new();

    let program = [
        0x70, 0x0A,
        0x61, 0x05
    ];

    chip8.load_program(&program);

    let opcode=chip8.fetch_opcode();

     println!("Opcode leido: {:#06X}",opcode);

    chip8.execute_opcode(opcode);

   println!("V0= {:#04X}", chip8.get_register(0));

}
