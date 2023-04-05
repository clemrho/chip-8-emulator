pub struct CPU{
    memory : [u8;4096],
    register : [u8;16],
    stack : [u16;16],
    sound_tmr : i8, // soundtimer
    delay_tmr : i8, // delaytimer
    I : i16, // index register
    stackptr : i8, 
    progctr : i16, // program counter
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory : [0;4096],
            register : [0;16],
            stack : [0;16],
            sound_tmr : 0,
            delay_tmr : 0,
            I : 0,
            stackptr : -1,
            progctr : 0x200, // from 0x200 to 0x1FF
        }
    }
}
