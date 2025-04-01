use crate::chip8::audio::Speaker;
use crate::chip8::display::Screen;
use crate::chip8::instruction::Instruction;
use crate::chip8::keyboard::Keyboard;
use crate::chip8::memory::{self, Memory};
use crate::chip8::registers::Registers;
use crate::chip8::stack::Stack;

use rand;
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
        if self.registers.get_dt() > 0 {
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            self.registers.dec_dt();
        }
    }

    pub fn handle_sound_timer(&mut self) {
        let status = self.registers.get_st() > 0;
        self.speaker.beep(status);
        if status {
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            self.registers.dec_st();
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
        let pc = self.registers.get_pc();
        let opcode = self.memory.read_opcode(pc as usize);
        let instruction = Instruction::parse(opcode);
        self.registers.advance_pc();

        match instruction.nibbles {
            // CLS: clear the display
            (0x00, 0x00, 0x0E, 0x00) => self.screen.clear(),

            // RET: return from subroutine
            (0x00, 0x00, 0x0E, 0x0E) => {
                let new_pc = self.stack.pop(&mut self.registers);
                self.registers.set_pc(new_pc);
            }

            // JP addr: jump to location addr
            (0x01, _, _, _) => self.registers.set_pc(instruction.addr),

            // CALL addr: call subroutine at addr
            (0x02, _, _, _) => {
                let pc = self.registers.get_pc();
                self.stack.push(&mut self.registers, pc);
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
                let msb = 1 << 7;

                self.registers.set_carry_if(x & msb == 1);
                self.registers.set_v(instruction.x, x << 1);
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

            //DRW Vx, Vy, nibble: display n-byte sprite starting at memory
            // location I at (Vx, Vy), set VF = collision.
            (0x0D, _, _, _) => {
                let x = self.registers.get_v(instruction.x) as usize;
                let y = self.registers.get_v(instruction.y) as usize;
                let start = self.registers.get_i() as usize;
                let sprite = &self.memory.read(start, instruction.nibble);

                let collission = self.screen.draw_sprite(x, y, sprite);
                self.registers.set_carry_if(collission);
            }

            // SKP Vx: skip next instruction if key with the value of Vx is
            // pressed
            (0x0E, _, 0x09, 0x0E) => {
                let x = self.registers.get_v(instruction.x) as usize;
                if self.keyboard.is_key_down(x) {
                    self.registers.advance_pc();
                }
            }

            // SKNP Vx: skip next instruction if key with the value of Vx is
            // not pressed
            (0x0E, _, 0x0A, 0x01) => {
                let x = self.registers.get_v(instruction.x) as usize;
                if !self.keyboard.is_key_down(x) {
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
                let x = self.registers.get_v(instruction.x) as usize;
                let i = self.registers.get_i() as usize;

                for n in 0..=x {
                    self.memory.set(i + n, self.registers.get_v(n));
                }
            }

            // LD Vx, [I]: read registers V0 through Vx from memory starting at
            // location I
            (0x0F, _, 0x06, 0x05) => {
                let x = self.registers.get_v(instruction.x) as usize;
                let i = self.registers.get_i() as usize;

                for n in 0..x {
                    let value = self.memory.get(i + n);
                    self.registers.set_v(n, value);
                }
            }

            _ => eprintln!("Invalid instruction"),
        }
    }
}
