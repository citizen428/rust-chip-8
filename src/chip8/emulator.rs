use crate::chip8::audio::Speaker;
use crate::chip8::instruction::Instruction;
use crate::chip8::memory::{self, Memory};
use crate::chip8::registers::Registers;

use debug_print::debug_println;
use rand;
use sdl2::AudioSubsystem;
use sdl2::keyboard::Keycode;
use std::{fs, thread, time::Duration};

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

const KEYS: usize = 16;
const STACK_DEPTH: usize = 16;

pub struct Chip8 {
    pub memory: Memory,
    pub registers: Registers,
    stack: [u16; STACK_DEPTH],
    sp: u8,
    pub keyboard: [bool; KEYS],
    screen: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub speaker: Speaker,
}

// Sleep duration for a 60 Hz refresh rate
const REFRESH_DURATION: Duration = Duration::from_millis(1000 / 60);

impl Chip8 {
    pub fn new(audio_subsystem: &AudioSubsystem) -> Self {
        Chip8 {
            memory: Memory::new(),
            registers: Registers::new(),
            stack: [0; STACK_DEPTH],
            sp: 0,
            keyboard: [false; KEYS],
            screen: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            speaker: Speaker::new(audio_subsystem),
        }
    }

    pub fn handle_delay_timer(&mut self) {
        if self.registers.get_dt() > 0 {
            thread::sleep(REFRESH_DURATION);
            self.registers.dec_dt();
        }
    }

    pub fn handle_sound_timer(&mut self) {
        let status = self.registers.get_st() > 0;
        self.speaker.beep(status);
        if status {
            thread::sleep(REFRESH_DURATION);
            self.registers.dec_st();
        }
    }

    pub fn load_rom(&mut self, file: &str) -> Result<usize, String> {
        let rom = fs::read(file).map_err(|e| format!("Cannot read ROM: {}", e))?;
        let rom_length = rom.len();

        if rom_length > memory::MEMORY_SIZE - memory::PROGRAM_LOAD_ADDRESS {
            return Err("ROM too big, aborting".to_string());
        }

        for (i, byte) in rom.iter().enumerate() {
            self.memory.set(memory::PROGRAM_LOAD_ADDRESS + i, *byte);
        }

        Ok(rom_length)
    }

    pub fn exec(&mut self) {
        let pc = self.registers.get_pc();
        let opcode = self.memory.read_opcode(pc as usize);
        let instruction = Instruction::from(opcode);
        self.registers.advance_pc();

        match instruction.nibbles {
            // CLS: clear the display
            (0x00, 0x00, 0x0E, 0x00) => self.clear_screen(),

            // RET: return from subroutine
            (0x00, 0x00, 0x0E, 0x0E) => {
                let new_pc = self.stack_pop();
                self.registers.set_pc(new_pc);
            }

            // JP addr: jump to location addr
            (0x01, _, _, _) => self.registers.set_pc(instruction.addr),

            // CALL addr: call subroutine at addr
            (0x02, _, _, _) => {
                let pc = self.registers.get_pc();
                self.stack_push(pc);
                self.registers.set_pc(instruction.addr);
            }

            // SE Vx, byte: skip next instruction if Vx = byte
            (0x03, _, _, _) => {
                let x = self.registers.get_v(instruction.x);
                if x == instruction.byte {
                    self.registers.advance_pc();
                }
            }

            // SNE Vx, byte: skip next instruction if Vx != byte
            (0x04, _, _, _) => {
                let x = self.registers.get_v(instruction.x);
                if x != instruction.byte {
                    self.registers.advance_pc();
                }
            }

            // SE Vx, Vy: skip next instruction if Vx = Vy
            (0x05, _, _, 0x00) => {
                let x = self.registers.get_v(instruction.x);
                let y = self.registers.get_v(instruction.y);

                if x == y {
                    self.registers.advance_pc();
                }
            }

            // LD Vx, byte: set Vx = byte
            (0x06, _, _, _) => {
                let register = instruction.x;
                let value = instruction.byte;
                self.registers.set_v(register, value);
            }

            // ADD Vx, byte: set Vx = Vx + byte
            (0x07, _, _, _) => {
                let register = instruction.x;
                let value = self.registers.get_v(register) as u16;
                let new_value = value + instruction.byte as u16;
                self.registers.set_v(register, new_value as u8);
            }

            // LD Vx, Vy: set Vx = Vy
            (0x08, _, _, 0x00) => {
                let y = self.registers.get_v(instruction.y);
                self.registers.set_v(instruction.x, y);
            }

            // OR Vx, Vy: set Vx = Vx OR Vy
            (0x08, _, _, 0x01) => {
                let x = self.registers.get_v(instruction.x);
                let y = self.registers.get_v(instruction.y);
                self.registers.set_v(instruction.x, x | y);
            }

            // AND Vx, Vy: set Vx = V AND Vy
            (0x08, _, _, 0x02) => {
                let x = self.registers.get_v(instruction.x);
                let y = self.registers.get_v(instruction.y);
                self.registers.set_v(instruction.x, x & y);
            }

            // XOR Vx, Vy: set Vx = Vx XOR Vy
            (0x08, _, _, 0x03) => {
                let x = self.registers.get_v(instruction.x);
                let y = self.registers.get_v(instruction.y);
                self.registers.set_v(instruction.x, x ^ y);
            }

            // ADD Vx, Vy: set Vx = Vx + Vy, set VF = carry
            (0x08, _, _, 0x04) => {
                let x = self.registers.get_v(instruction.x) as u16;
                let y = self.registers.get_v(instruction.y) as u16;
                let result = x + y;

                self.registers.set_carry_if(result > 255);
                self.registers.set_v(instruction.x, result as u8);
            }

            // SUB Vx, Vy: set Vx = Vx - Vy, set VF = NOT borrow
            (0x08, _, _, 0x05) => {
                let x = self.registers.get_v(instruction.x);
                let y = self.registers.get_v(instruction.y);

                self.registers.set_carry_if(x > y);
                self.registers.set_v(instruction.x, x.wrapping_sub(y));
            }

            // SHR Vx {, Vy}: set Vx = Vx SHR 1o
            (0x08, _, _, 0x06) => {
                let x = self.registers.get_v(instruction.x);
                self.registers.set_carry_if(x & 1 == 1);
                self.registers.set_v(instruction.x, x >> 1);
            }

            // SUBN Vx, Vy: set Vx = Vy - Vx, set VF = NOT borrow
            (0x08, _, _, 0x07) => {
                let x = self.registers.get_v(instruction.x);
                let y = self.registers.get_v(instruction.y);

                self.registers.set_carry_if(y > x);
                self.registers.set_v(instruction.x, y.wrapping_sub(x));
            }

            // SHL Vx {, Vy}: set Vx = Vx SHL 1
            (0x08, _, _, 0x0E) => {
                let x = self.registers.get_v(instruction.x);
                let msb = (x & 0x80) >> 7; // Extract the MSB (most significant bit)
                self.registers.set_v(0xF, msb); // Set VF to the MSB
                self.registers.set_v(instruction.x, x << 1); // Perform the left shift
            }

            // SNE Vx, Vy: skip next instruction if Vx != Vy
            (0x09, _, _, 0x00) => {
                let x = self.registers.get_v(instruction.x);
                let y = self.registers.get_v(instruction.y);

                if x != y {
                    self.registers.advance_pc();
                }
            }

            // LD I, addr: set I = addr
            (0x0A, _, _, _) => self.registers.set_i(instruction.addr),

            // JP V0, addr: jump to location nnn + V0
            (0x0B, _, _, _) => {
                self.registers
                    .set_pc(self.registers.get_v(0) as u16 + instruction.addr);
            }

            // RND Vx, byte:  et Vx = random byte AND kk.
            (0x0C, _, _, _) => {
                let n: u8 = rand::random();
                self.registers.set_v(instruction.x, n & instruction.byte);
            }

            // DRW Vx, Vy, nibble: display n-byte sprite starting at memory
            // location I at (Vx, Vy), set VF = collision.
            (0x0D, _, _, _) => {
                let x = self.registers.get_v(instruction.x) as usize;
                let y = self.registers.get_v(instruction.y) as usize;
                let start = self.registers.get_i() as usize;
                let sprite: Vec<u8> = self.memory.read(start, instruction.nibble).to_vec();

                let collission = self.draw_sprite(x, y, &sprite);
                self.registers.set_carry_if(collission);
            }

            // SKP Vx: skip next instruction if key with the value of Vx is
            // pressed
            (0x0E, _, 0x09, 0x0E) => {
                let x = self.registers.get_v(instruction.x) as usize;
                if self.is_key_down(x) {
                    self.registers.advance_pc();
                }
            }

            // SKNP Vx: skip next instruction if key with the value of Vx is
            // not pressed
            (0x0E, _, 0x0A, 0x01) => {
                let x = self.registers.get_v(instruction.x) as usize;
                if !self.is_key_down(x) {
                    self.registers.advance_pc();
                }
            }

            // LD Vx, DT: set Vx = delay timer value
            (0x0F, _, 0x00, 0x07) => {
                self.registers.set_v(instruction.x, self.registers.get_dt());
            }

            // LD Vx, K: wait for a key press, store the value of the key in V
            (0x0F, _, 0x00, 0x0A) => {
                // TODO
            }

            // LD DT, Vx
            (0x0F, _, 0x01, 0x05) => {
                self.registers.set_dt(self.registers.get_v(instruction.x));
            }

            // LD ST, Vx: set sound timer = Vx
            (0x0F, _, 0x01, 0x08) => {
                self.registers.set_st(self.registers.get_v(instruction.x));
            }

            // ADD I, Vx: set I = I + Vx
            (0x0F, _, 0x01, 0x0E) => {
                let i = self.registers.get_i();
                let x = self.registers.get_v(instruction.x) as u16;
                let result = (i + x) as u16;
                self.registers.set_i(result);
                self.registers.set_carry_if(result > (1 << 15));
            }

            // LD F, Vx: set I = location of sprite for digit Vx
            (0x0F, _, 0x02, 0x09) => {
                let x = self.registers.get_v(instruction.x) as u16;
                self.registers.set_i(x * 5);
            }

            // LD B, Vx: store BCD representation of Vx in memory locations I,
            // I+1, and I+2.
            (0x0F, _, 0x03, 0x03) => {
                let x = self.registers.get_v(instruction.x) as u16;
                let i = self.registers.get_i() as usize;

                self.memory.set(i, (x / 100) as u8);
                self.memory.set(i + 1, ((x % 100) / 10) as u8);
                self.memory.set(i + 2, (x % 10) as u8)
            }

            // LD [I], Vx: store registers V0 through Vx in memory starting at
            // location I.
            (0x0F, _, 0x05, 0x05) => {
                let i = self.registers.get_i() as usize;

                for n in 0..=instruction.x {
                    self.memory.set(i + n, self.registers.get_v(n));
                }
            }

            // LD Vx, [I]: read registers V0 through Vx from memory starting at
            // location I
            (0x0F, _, 0x06, 0x05) => {
                let i = self.registers.get_i() as usize;

                for n in 0..=instruction.x {
                    let value = self.memory.get(i + n);
                    self.registers.set_v(n, value);
                }
            }

            _ => eprintln!("Invalid instruction"),
        }
    }

    // region: Display functions
    pub fn is_pixel_set(&self, x: usize, y: usize) -> bool {
        self.screen[y][x]
    }

    fn toggle_pixel(&mut self, x: usize, y: usize) {
        self.screen[y][x] ^= true;
    }

    fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut pixel_collission = false;

        for ly in 0..sprite.len() {
            let b = sprite[ly];
            for lx in 0..8 {
                if b & 0b10000000 >> lx > 0 {
                    let dx = (x + lx) % DISPLAY_WIDTH;
                    let dy = (y + ly) % DISPLAY_HEIGHT;
                    pixel_collission = pixel_collission || self.is_pixel_set(dx, dy);

                    self.toggle_pixel(dx, dy);
                }
            }
        }

        pixel_collission
    }

    fn clear_screen(&mut self) {
        self.screen = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }
    // endregion

    // region: Keyboard functions
    pub fn key_down(&mut self, key: Keycode) {
        self.toggle_key(key, true);
    }

    pub fn key_up(&mut self, key: Keycode) {
        self.toggle_key(key, false);
    }

    fn toggle_key(&mut self, key: Keycode, is_down: bool) {
        // Ignore all keys that aren't mapped to the CHIP-8 hex keyboard.
        if let Some(key_index) = self.map(key) {
            self.keyboard[key_index] = is_down;
            if is_down {
                debug_println!("key down: {}", key_index);
            } else {
                debug_println!("key up: {}", key_index);
            }
        }
    }

    fn is_key_down(&self, key: usize) -> bool {
        self.keyboard[key]
    }

    fn map(&self, key: Keycode) -> Option<usize> {
        match key {
            Keycode::Num1 => Some(1),
            Keycode::Num2 => Some(2),
            Keycode::Num3 => Some(3),
            Keycode::Num4 => Some(12),
            Keycode::Q => Some(4),
            Keycode::W => Some(5),
            Keycode::E => Some(6),
            Keycode::R => Some(13),
            Keycode::A => Some(7),
            Keycode::S => Some(8),
            Keycode::D => Some(9),
            Keycode::F => Some(14),
            Keycode::Z => Some(10),
            Keycode::X => Some(0),
            Keycode::C => Some(11),
            Keycode::V => Some(15),
            _ => None,
        }
    }
    // endregion

    // region: Stack functions
    fn get_sp(&self) -> u8 {
        self.sp
    }

    fn inc_sp(&mut self) {
        self.sp += 1;
    }

    fn dec_sp(&mut self) {
        self.sp -= 1;
    }

    fn stack_push(&mut self, value: u16) {
        let stack_pointer = self.get_sp() as usize;
        assert!(stack_pointer < STACK_DEPTH, "stack overflow");
        self.stack[stack_pointer] = value;
        self.inc_sp();
    }

    fn stack_pop(&mut self) -> u16 {
        let stack_pointer = self.get_sp() as usize;
        assert!(stack_pointer > 0, "stack underflow");
        self.dec_sp();
        assert!(stack_pointer < STACK_DEPTH, "stack overflow");
        self.stack[self.get_sp() as usize]
    }
    // endregion
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_chip8() -> Chip8 {
        let sdl_context = sdl2::init().unwrap();
        Chip8::new(&sdl_context.audio().unwrap())
    }

    #[test]
    fn toggle_pixel_can_toggle_a_pixel() {
        let mut chip8 = new_chip8();
        assert_eq!(chip8.is_pixel_set(5, 5), false);
        chip8.toggle_pixel(5, 5);
        assert_eq!(chip8.is_pixel_set(5, 5), true);
        chip8.toggle_pixel(5, 5);
        assert_eq!(chip8.is_pixel_set(5, 5), false);
    }

    #[test]
    fn draw_sprite_returns_true_when_overwriting() {
        let mut chip8 = new_chip8();
        assert_eq!(chip8.draw_sprite(0, 0, &[0xff]), false);
        assert_eq!(chip8.draw_sprite(0, 0, &[0xff]), true);
    }

    #[test]
    fn it_maps_physical_keys_to_virtual_ones() {
        let chip8 = new_chip8();
        assert_eq!(chip8.map(Keycode::A), Some(7));
        assert_eq!(chip8.map(Keycode::X), Some(0));
        assert_eq!(chip8.map(Keycode::M), None);
    }

    #[test]
    fn it_can_press_and_release_keys() {
        let mut chip8 = new_chip8();
        assert_eq!(chip8.is_key_down(1), false);
        chip8.key_down(Keycode::Num1);
        assert_eq!(chip8.is_key_down(1), true);
        chip8.key_up(Keycode::Num1);
        assert_eq!(chip8.is_key_down(1), false);
    }

    #[test]
    fn it_can_push_to_and_pop_from_the_stack() {
        let mut chip8 = new_chip8();
        assert_eq!(chip8.get_sp(), 0);
        chip8.stack_push(0xff);
        assert_eq!(chip8.get_sp(), 1);
        assert_eq!(chip8.stack[0], 0xff);

        chip8.stack_push(0xaa);
        assert_eq!(chip8.get_sp(), 2);
        assert_eq!(chip8.stack[1], 0xaa);
        assert_eq!(chip8.stack_pop(), 170);
        assert_eq!(chip8.get_sp(), 1);
        assert_eq!(chip8.stack_pop(), 255);
        assert_eq!(chip8.get_sp(), 0);
    }
}
