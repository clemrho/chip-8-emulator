use std::usize;
use crate::utils::FONTSET;
use crate::utils::Dstatus;

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
            screen : [Dstatus::Off;64*32]
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
        let opcode = self.fetch();
        // Decode
        // Execute
        self.execute(opcode);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_tmr > 0 {
        self.delay_tmr -= 1;
        }
        if self.sound_tmr > 0 {
        if self.sound_tmr == 1 {
        // BEEP
        }
        self.sound_tmr -= 1;
        }
        }

    fn fetch(&mut self) -> u16 {
        let first_byte = self.ram[self.PC as usize] as u16;
        let second_byte = self.ram[(self.PC + 1) as usize] as u16;
        let opcode = (first_byte << 8) | second_byte;
        self.PC += 2;
        return opcode;
    }


    pub fn execute(&mut self, op: u16) {
        // get every digit of opcode
        let d1 = (op & 0xF000) >> 12;
        let d2 = (op & 0x0F00) >> 8;
        let d3 = (op & 0x00F0) >> 4;
        let d4 = op & 0x000F;
        
        let NNN = (op & 0x0fff) as u16;
        let NN = (op & 0x00ff) as u8;
        let X = d2 as usize;
        let Y = d3 as usize;
        let N = d4 as usize;

        match (d1,d2,d3,d4) {
            (0,0,0,0) => return,
            (0,0,0xE,0) => self.f_00e0(),
            (0,0,0xE,0xE) => self.f_00ee(),
            (1,_,_,_) => self.f_1nnn(NNN),
            (2,_,_,_) => self.f_2nnn(NNN),
            (3,_,_,_) => self.f_3xnn(X,NN),
            (4,_,_,_) => self.f_4xnn(X,NN),
            (5,_,_,0) => self.f_5xy0(X,Y),
            (6,_,_,_) => self.f_6xnn(X,NN),
            (7, _, _, _) => self.f_7xnn(X, NN),
            (8,_,_, _ ) => match (d2,d3,d4) {
                ( _, _, 0) => self.f_8xy0(X, Y),
                ( _, _, 1) => self.f_8xy1(X, Y),
                ( _, _, 2) => self.f_8xy2(X, Y),
                ( _, _, 3) => self.f_8xy3(X, Y),
                ( _, _, 4) => self.f_8xy4(X, Y),
                ( _, _, 5) => self.f_8xy5(X, Y),
                ( _, _, 6) => self.f_8x06(X),
                ( _, _, 7) => self.f_8xy7(X, Y),
                ( _, _, 0xE) => self.f_8x0e(X),
            },
            (9, _, _, 0) => self.f_9xy0(X, Y),
            (0xA, _, _, _) => self.f_annn(NNN),
            (0xB, _, _, _) => self.f_bnnn(NNN),
            (0xC, _, _, _) => self.f_cxnn(X, NN),
            (0xD, _, _, _) => self.f_dxyn(X, Y, N),
            (0xE, _, 9, 0xE) => self.f_ex9e(X),
            (0xE, _, 0xA, 1) => self.f_exa1(X),
            (0xF,_,_,_) => match (d2,d3,d4) {
                ( _, 0, 7) => self.f_fx07(X),
                ( _, 0, 0xA) => self.f_fx0a(X),
                ( _, 1, 5) => self.f_fx15(X),
                ( _, 1, 8) => self.f_fx18(X),
                ( _, 1, 0xE) => self.f_fx1e(X),
                ( _, 2, 9) => self.f_fx29(X),
                ( _, 3, 3) => self.f_fx33(X),
                ( _, 5, 5) => self.f_fx55(X),
                ( _, 6, 5) => self.f_fx65(X),
            },
            (_,_,_,_) => unimplemented!("X"),
        }

    }

    //It should clear the display, turning all pixels off to 0.
    fn f_00e0(&mut self){
        self.screen = [Dstatus::Off; 64*32];
    }

    //return from subroutine
        fn f_00ee(&mut self){
        let addr = self.pop();
        self.PC = addr;
    }

    //jump
    fn f_1nnn(&mut self, NNN:u16){
        self.PC = NNN;
    }

    //call subroutine
    fn f_2nnn(&mut self, NNN: u16){
        self.push(self.PC);
        self.PC = NNN;
    }

    //skip vx =nn
    fn f_3xnn(&mut self,X:usize, NN: u8){
        if self.vreg[X] == NN{
            self.PC += 2;
        }
    }

    //skip if vx!=nn
    fn f_4xnn(&mut self, X:usize, NN: u8){
        if self.vreg[X] != NN{
            self.PC +=2;
        }
    }

    //skip next if vx==vy
    fn f_5xy0(&mut self, X:usize, Y : usize){
        if self.vreg[X] == self.vreg[Y] {
            self.PC +=2;
        }
    }

    //vx <- nn
    fn f_6xnn(&mut self, X:usize, NN: u8) {
        self.vreg[X] = NN;
    }

    fn f_7xnn(&mut self, X:usize, NN: u8) {
        //self.vreg[X] += NN;
        self.vreg[X] = self.vreg[X].wrapping_add(NN);
    }

    fn f_8xy0(&mut self,X:usize,Y : usize) {
        self.vreg[X] = self.vreg[Y];
    }

    fn f_8xy1(&mut self,X:usize,Y : usize) {
        self.vreg[X] |= self.vreg[Y];
    }

    fn f_8xy2(&mut self, X:usize, Y :usize) {
        self.vreg[X] &= self.vreg[Y];
    }


}
