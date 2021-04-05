use crate::chip8::audio::Speaker;
use crate::chip8::display::Screen;
use crate::chip8::keyboard::Keyboard;
use crate::chip8::memory::Memory;
use crate::chip8::registers::{Register, Registers};
use crate::chip8::stack::Stack;

use sdl2::AudioSubsystem;

use std::{thread, time::Duration};

pub struct Chip8 {
    pub memory: Memory,
    pub registers: Registers,
    pub stack: Stack,
    pub keyboard: Keyboard,
    pub screen: Screen,
    pub speaker: Speaker,
}

impl Chip8 {
    pub fn new(audio_subsystem: &AudioSubsystem) -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            registers: Registers::new(),
            stack: Stack::new(),
            keyboard: Keyboard::new(),
            screen: Screen::new(),
            speaker: Speaker::new(audio_subsystem),
        }
    }

    pub fn handle_timers(&mut self) -> () {
        self.handle_delay_timer();
        self.handle_sound_timer();
    }

    pub fn handle_delay_timer(&mut self) -> () {
        let delay_timer = self.registers.get(Register::DT);
        if delay_timer > 0 {
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            self.registers.set(Register::DT, delay_timer - 1);
        }
    }

    pub fn handle_sound_timer(&mut self) {
        let sound_timer = self.registers.get(Register::ST);
        let status = sound_timer > 0;
        self.speaker.beep(status);
        if status {
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            self.registers.set(Register::ST, sound_timer - 1);
        }
    }
}
