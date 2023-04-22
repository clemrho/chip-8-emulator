use chip8_core::{*, cpu::CPU, utils::Kstatus};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

#[wasm_bindgen]
pub struct WebCPU{
    chip8 : CPU,
}


#[wasm_bindgen]
impl WebCPU {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebCPU {
        WebCPU {
            chip8:CPU::new(),
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.chip8.main_loop();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.chip8.loop_timers();
    }

    // #[wasm_bindgen]
    // pub fn reset(&mut self) {
    //     self.chip8.reset();
    // }

    #[wasm_bindgen]
    pub fn keypress(&mut self, evt: KeyboardEvent, b_pressed: bool) {
        let key = evt.key();
        let key_status: Kstatus = match b_pressed {
            true => Kstatus::Pressed,
            false => Kstatus::Default,
        };
        if let Some(k) = self.key_map(&key) {
            self.chip8.keypress(k, key_status);
        }
        //if self.key_map(&key) == None {key_status = Kstatus::Abnormal}
    }

    fn key_map(&mut self, key: &str) -> Option<usize> {
        match key {
            "1" => Some(0x1),
            "2" => Some(0x2),
            "3" => Some(0x3),
            "4" => Some(0xC),
            "q" => Some(0x4),
            "w" => Some(0x5),
            "e" => Some(0x6),
            "r" => Some(0xD),
            "a" => Some(0x7),
            "s" => Some(0x8),
            "d" => Some(0x9),
            "f" => Some(0xE),
            "z" => Some(0xA),
            "x" => Some(0x0),
            "c" => Some(0xB),
            "v" => Some(0xF),
            _ =>   None,
        }
    }

    #[wasm_bindgen]
    pub fn load_game(&mut self, data: Uint8Array) {
        self.chip8.load(&data.to_vec());
    }



    #[wasm_bindgen]
    pub fn draw_screen(&mut self, scale: usize) {
        // TODO
    }
    
}