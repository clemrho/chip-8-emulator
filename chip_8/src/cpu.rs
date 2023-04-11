const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Clone)] 
#[derive(Copy)]
pub enum Dstatus{
    on,
    off,
}
pub struct CPU{
    ram : [u8;4096], 
    vreg : [u8;16],  //V register
    stack : [u16;16],
    sound_tmr : i8, // soundtimer
    delay_tmr : i8, // delaytimer
    I : u16, // index register
    SP : u16, //stack pointer
    PC : u16, // program counter
    screen : [Dstatus; 64 * 32],
}

impl CPU {
    pub fn new() -> Self {
        let mut init_cpu = Self {
            ram : [0;4096],
            vreg : [0;16],
            stack : [0;16],
            sound_tmr : 0,
            delay_tmr : 0,
            I : 0,
            SP : 0,
            PC : 0x200, // from 0x200 to 0x1FF
            screen : [Dstatus::off;64*32]
        };
        //copy the font into the first 80 bits of memory.
        init_cpu.ram[0..80].copy_from_slice(&FONTSET);
        return init_cpu;
    }

    pub fn push(&mut self, input: u16) {
        let SP_r = self.SP as usize;
        self.stack[SP_r] = input;
        self.SP+=1;
    }

    pub fn pop(&mut self) -> u16 {
        self.SP-=1;
        let SP_r = self.SP as usize;
        return self.stack[SP_r];
    }

    pub fn main_loop(&mut self) {
        // Fetch
        //let opcode = self.fetch();
        // Decode
        // Execute
    }

    fn fetch(&mut self) -> u16 {
        let first_byte = self.ram[self.PC as usize] as u16;
        let second_byte = self.ram[(self.PC + 1) as usize] as u16;
        let opcode = (first_byte << 8) | second_byte;
        self.PC += 2;
        return opcode;
    }




}
