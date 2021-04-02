use crate::chip8::keyboard::Keyboard;
use crate::chip8::memory::Memory;
use crate::chip8::registers::Registers;
use crate::chip8::stack::Stack;

pub struct Chip8 {
    pub memory: Memory,
    pub registers: Registers,
    pub stack: Stack,
    pub keyboard: Keyboard,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let chip8 = Chip8 {
            memory: Memory::new(),
            registers: Registers::new(),
            stack: Stack::new(),
            keyboard: Keyboard::new(),
        };
        chip8
    }
}
