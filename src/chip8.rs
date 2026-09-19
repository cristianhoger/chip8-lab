pub struct Chip8 {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,

}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let start = 0x200;
        self.memory[start..start + program.len()].copy_from_slice(program);
    }

    pub fn fetch_opcode(&self) -> u16 {
        let pc = self.pc as usize;
        let high_byte = self.memory[pc]  as u16;
        let low_byte = self.memory[pc+ 1]  as u16;
        (high_byte << 8) | low_byte
    }

    pub fn read_opcode(&self,opcode: u16)->[u16;3]{
        
        let family= (opcode & 0xF000) >> 12;
        let x= (opcode & 0x0F00) >> 8;
        let value= opcode & 0x00FF;

        [family,x,value]
       
    }

    pub fn execute_opcode(&mut self,opcode:u16){
        let family = (opcode & 0xF000) >> 12;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let value = (opcode & 0x00FF) as u8;

        match family {
            0x06 => {
                self.v[x] = value;
            }

            0x07 => {

                self.v[x] = self.v[x].wrapping_add(value);
            }

            _ => {
                println!("Opcode no implementado: {:#06X}",opcode);
            }
        }

    }

    pub fn next_instruction(&mut self) {
    self.pc += 2;
    }

    pub fn get_register(&self, index: usize) -> u8 {
        self.v[index]
    }

    pub fn print_state(&self) {
    println!("=== CHIP-8 STATE ===");
    println!("PC: {:#06X}", self.pc);
    println!("I:  {:#06X}", self.i);

    for i in 0..16 {
        println!("V{:X}: {:#04X}", i, self.v[i]);
    }

    println!("====================");
    }

}