use crate::chip8::audio::Speaker;
use crate::chip8::display::Screen;
use crate::chip8::keyboard::Keyboard;
use crate::chip8::memory::{self, Memory};
use crate::chip8::registers::{Register::*, Registers};
use crate::chip8::stack::Stack;

use sdl2::AudioSubsystem;

use std::{fs, thread, time::Duration};

pub struct Chip8 {
    pub memory: Memory,
    pub registers: Registers,
    pub stack: Stack,
    pub keyboard: Keyboard,
    pub screen: Screen,
    pub speaker: Speaker,
}

impl Chip8 {
    pub fn new(audio_subsystem: &AudioSubsystem) -> Self {
        Chip8 {
            memory: Memory::new(),
            registers: Registers::new(),
            stack: Stack::new(),
            keyboard: Keyboard::new(),
            screen: Screen::new(),
            speaker: Speaker::new(audio_subsystem),
        }
    }

    pub fn handle_delay_timer(&mut self) -> () {
        let delay_timer = self.registers.get(DT);
        if delay_timer > 0 {
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            self.registers.set(DT, delay_timer - 1);
        }
    }

    pub fn handle_sound_timer(&mut self) {
        let sound_timer = self.registers.get(ST);
        let status = sound_timer > 0;
        self.speaker.beep(status);
        if status {
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            self.registers.set(ST, sound_timer - 1);
        }
    }

    pub fn load_rom(&mut self, file: &str) -> usize {
        let rom = fs::read(file).expect("Cannot read ROM");
        let rom_length = rom.len();

        if rom_length > memory::MEMORY_SIZE - memory::PROGRAM_LOAD_ADDRESS {
            panic!("ROM too big, aborting")
        }

        for (i, byte) in rom.iter().enumerate() {
            self.memory.set(memory::PROGRAM_LOAD_ADDRESS + i, *byte);
        }

        rom_length
    }

    pub fn exec(&mut self) {
        let pc = self.registers.get(PC);
        let opcode = self.memory.read_opcode(pc as usize);
        println!("{}", opcode);
        self.registers.set(PC, pc + 2);
    }
}
