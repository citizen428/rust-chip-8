pub mod speaker;

use crate::chip8::{self, DISPLAY_HEIGHT, DISPLAY_WIDTH, NUM_KEYS, TICKS_PER_FRAME};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8Emulator {
    chip8: chip8::Chip8<'static>,
}

#[wasm_bindgen]
impl Chip8Emulator {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let speaker = Box::new(speaker::WebSpeaker::new());
        Chip8Emulator {
            chip8: chip8::Chip8::new(speaker),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<usize, JsValue> {
        self.chip8
            .load_rom(rom.to_vec())
            .map_err(|e| JsValue::from_str(&e))
    }

    pub fn tick(&mut self) {
        for _ in 0..TICKS_PER_FRAME {
            self.chip8.exec();
        }
        self.chip8.update_timers();
    }

    pub fn key_down(&mut self, key: usize) {
        if key < NUM_KEYS {
            self.chip8.key_down(key);
        }
    }

    pub fn key_up(&mut self, key: usize) {
        if key < NUM_KEYS {
            self.chip8.key_up(key);
        }
    }

    pub fn get_display_buffer(&self) -> Vec<u8> {
        let mut buffer = vec![0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let idx = y * DISPLAY_WIDTH + x;
                buffer[idx] = if self.chip8.is_pixel_set(x, y) { 1 } else { 0 };
            }
        }
        buffer
    }

    pub fn reset(&mut self) {
        let speaker = Box::new(speaker::WebSpeaker::new());
        self.chip8 = chip8::Chip8::new(speaker);
    }
}
