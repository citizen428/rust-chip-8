pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const TICKS_PER_FRAME: usize = 10;

const DEFAULT_CHARACTER_SET_SIZE: usize = 80;
const DEFAULT_CHARACTER_SET: [u8; DEFAULT_CHARACTER_SET_SIZE] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0xf0, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];
const INSTRUCTION_LENGTH: u16 = 2;
const NUM_DATA_REGISTERS: usize = 16;
pub const NUM_KEYS: usize = 16;
const PROGRAM_LOAD_ADDRESS: usize = 0x200;
const RAM_SIZE: usize = 4096;
const STACK_DEPTH: usize = 16;

pub trait Speaker {
    fn beep(&mut self, status: bool);
}

pub struct Chip8<'a> {
    pc: u16,
    ram: [u8; RAM_SIZE],
    v_registers: [u8; NUM_DATA_REGISTERS],
    i_register: u16,
    stack: [u16; STACK_DEPTH],
    sp: u8,
    dt: u8,
    st: u8,
    keyboard: [bool; NUM_KEYS],
    screen: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    speaker: Box<dyn Speaker + 'a>,
}

impl<'a> Chip8<'a> {
    // region: Public interface
    pub fn new(speaker: Box<dyn Speaker + 'a>) -> Self {
        let mut chip8 = Chip8 {
            pc: PROGRAM_LOAD_ADDRESS as u16,
            ram: [0; RAM_SIZE],
            v_registers: [0; NUM_DATA_REGISTERS],
            i_register: 0,
            stack: [0; STACK_DEPTH],
            sp: 0,
            dt: 0,
            st: 0,
            keyboard: [false; NUM_KEYS],
            screen: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            speaker,
        };

        // Initialize the default character set in memory
        chip8.ram[..DEFAULT_CHARACTER_SET_SIZE].copy_from_slice(&DEFAULT_CHARACTER_SET);
        chip8
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) -> Result<usize, String> {
        let rom_length = rom.len();

        if rom_length > RAM_SIZE - PROGRAM_LOAD_ADDRESS {
            return Err("ROM too big, aborting".to_string());
        }

        self.ram[PROGRAM_LOAD_ADDRESS..][..rom_length].copy_from_slice(&rom);
        Ok(rom_length)
    }

    pub fn key_down(&mut self, key_idx: usize) {
        self.keyboard[key_idx] = true;
    }

    pub fn key_up(&mut self, key_idx: usize) {
        self.keyboard[key_idx] = false;
    }

    pub fn exec(&mut self) {
        let opcode = self.read_opcode(self.pc as usize);
        let instruction = Instruction::from(opcode);
        self.advance_pc();

        match instruction.nibbles {
            // CLS: clear the display
            (0x00, 0x00, 0x0E, 0x00) => self.clear_screen(),

            // RET: return from subroutine
            (0x00, 0x00, 0x0E, 0x0E) => self.pc = self.stack_pop(),

            // JP addr: jump to location addr
            (0x01, _, _, _) => self.pc = instruction.nnn(),

            // CALL addr: call subroutine at addr
            (0x02, _, _, _) => {
                self.stack_push(self.pc);
                self.pc = instruction.nnn();
            }

            // SE Vx, byte: skip next instruction if Vx = byte
            (0x03, _, _, _) => {
                let x = self.v_registers[instruction.x()];
                if x == instruction.nn() {
                    self.advance_pc();
                }
            }

            // SNE Vx, byte: skip next instruction if Vx != byte
            (0x04, _, _, _) => {
                let x = self.v_registers[instruction.x()];
                if x != instruction.nn() {
                    self.advance_pc();
                }
            }

            // SE Vx, Vy: skip next instruction if Vx = Vy
            (0x05, _, _, 0x00) => {
                let x = self.v_registers[instruction.x()];
                let y = self.v_registers[instruction.y()];

                if x == y {
                    self.advance_pc();
                }
            }

            // LD Vx, byte: set Vx = byte
            (0x06, _, _, _) => self.v_registers[instruction.x()] = instruction.nn(),

            // ADD Vx, byte: set Vx = Vx + byte
            (0x07, _, _, _) => {
                let register = instruction.x();
                let value = self.v_registers[register] as u16;
                let new_value = value + instruction.nn() as u16;
                self.v_registers[register] = new_value as u8;
            }

            // LD Vx, Vy: set Vx = Vy
            (0x08, _, _, 0x00) => {
                self.v_registers[instruction.x()] = self.v_registers[instruction.y()];
            }

            // OR Vx, Vy: set Vx = Vx OR Vy
            (0x08, _, _, 0x01) => {
                let x = self.v_registers[instruction.x()];
                let y = self.v_registers[instruction.y()];
                self.v_registers[instruction.x()] = x | y;
            }

            // AND Vx, Vy: set Vx = V AND Vy
            (0x08, _, _, 0x02) => {
                let x = self.v_registers[instruction.x()];
                let y = self.v_registers[instruction.y()];
                self.v_registers[instruction.x()] = x & y;
            }

            // XOR Vx, Vy: set Vx = Vx XOR Vy
            (0x08, _, _, 0x03) => {
                let x = self.v_registers[instruction.x()];
                let y = self.v_registers[instruction.y()];
                self.v_registers[instruction.x()] = x ^ y;
            }

            // ADD Vx, Vy: set Vx = Vx + Vy, set VF = carry
            (0x08, _, _, 0x04) => {
                let x = self.v_registers[instruction.x()] as u16;
                let y = self.v_registers[instruction.y()] as u16;
                let result = x + y;

                self.set_carry_if(result > 255);
                self.v_registers[instruction.x()] = result as u8;
            }

            // SUB Vx, Vy: set Vx = Vx - Vy, set VF = NOT borrow
            (0x08, _, _, 0x05) => {
                let x = self.v_registers[instruction.x()];
                let y = self.v_registers[instruction.y()];

                self.set_carry_if(x > y);
                self.v_registers[instruction.x()] = x.wrapping_sub(y);
            }

            // SHR Vx {, Vy}: set Vx = Vx SHR 1o
            (0x08, _, _, 0x06) => {
                let x = self.v_registers[instruction.x()];
                self.set_carry_if(x & 1 == 1);
                self.v_registers[instruction.x()] = x >> 1;
            }

            // SUBN Vx, Vy: set Vx = Vy - Vx, set VF = NOT borrow
            (0x08, _, _, 0x07) => {
                let x = self.v_registers[instruction.x()];
                let y = self.v_registers[instruction.y()];

                self.set_carry_if(y > x);
                self.v_registers[instruction.x()] = y.wrapping_sub(x);
            }

            // SHL Vx {, Vy}: set Vx = Vx SHL 1
            (0x08, _, _, 0x0E) => {
                let x = self.v_registers[instruction.x()];
                let msb = (x & 0x80) >> 7; // Extract the MSB (most significant bit)
                self.v_registers[0xF] = msb; // Set VF to the MSB
                self.v_registers[instruction.x()] = x << 1; // Perform the left shift
            }

            // SNE Vx, Vy: skip next instruction if Vx != Vy
            (0x09, _, _, 0x00) => {
                let x = self.v_registers[instruction.x()];
                let y = self.v_registers[instruction.y()];

                if x != y {
                    self.advance_pc();
                }
            }

            // LD I, addr: set I = addr
            (0x0A, _, _, _) => self.i_register = instruction.nnn(),

            // JP V0, addr: jump to location nnn + V0
            (0x0B, _, _, _) => self.pc = self.v_registers[0] as u16 + instruction.nnn(),

            // RND Vx, byte:  et Vx = random byte AND kk.
            (0x0C, _, _, _) => {
                let n: u8 = rand::random();
                self.v_registers[instruction.x()] = n & instruction.nn();
            }

            // DRW Vx, Vy, nibble: display n-byte sprite starting at memory
            // location I at (Vx, Vy), set VF = collision.
            (0x0D, _, _, _) => {
                let x = self.v_registers[instruction.x()] as usize;
                let y = self.v_registers[instruction.y()] as usize;
                let start = self.i_register as usize;
                let sprite: Vec<u8> = self.ram_read(start, instruction.n()).to_vec();

                let collision = self.draw_sprite(x, y, &sprite);
                self.set_carry_if(collision);
            }

            // SKP Vx: skip next instruction if key with the value of Vx is
            // pressed
            (0x0E, _, 0x09, 0x0E) => {
                let x = self.v_registers[instruction.x()] as usize;
                if self.is_key_down(x) {
                    self.advance_pc();
                }
            }

            // SKNP Vx: skip next instruction if key with the value of Vx is
            // not pressed
            (0x0E, _, 0x0A, 0x01) => {
                let x = self.v_registers[instruction.x()] as usize;
                if !self.is_key_down(x) {
                    self.advance_pc();
                }
            }

            // LD Vx, DT: set Vx = delay timer value
            (0x0F, _, 0x00, 0x07) => self.v_registers[instruction.x()] = self.dt,

            // LD Vx, K: wait for a key press, store the value of the key in V
            (0x0F, _, 0x00, 0x0A) => {
                let mut pressed = false;
                for (i, key) in self.keyboard.iter().enumerate() {
                    if *key {
                        self.v_registers[instruction.x()] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // Repeat current instruction
                    self.pc -= INSTRUCTION_LENGTH;
                }
            }

            // LD DT, Vx
            (0x0F, _, 0x01, 0x05) => self.dt = self.v_registers[instruction.x()],

            // LD ST, Vx: set sound timer = Vx
            (0x0F, _, 0x01, 0x08) => self.st = self.v_registers[instruction.x()],

            // ADD I, Vx: set I = I + Vx
            (0x0F, _, 0x01, 0x0E) => {
                let i = self.i_register;
                let x = self.v_registers[instruction.x()] as u16;
                let result = i + x;
                self.i_register = result;
                self.set_carry_if(result > (1 << 15));
            }

            // LD F, Vx: set I = location of sprite for digit Vx
            (0x0F, _, 0x02, 0x09) => {
                let x = self.v_registers[instruction.x()] as u16;
                self.i_register = x * 5;
            }

            // LD B, Vx: store BCD representation of Vx in memory locations I,
            // I+1, and I+2.
            (0x0F, _, 0x03, 0x03) => {
                let x = self.v_registers[instruction.x()] as u16;
                let i = self.i_register as usize;

                self.ram[i] = (x / 100) as u8;
                self.ram[i + 1] = ((x % 100) / 10) as u8;
                self.ram[i + 2] = (x % 10) as u8;
            }

            // LD [I], Vx: store registers V0 through Vx in memory starting at
            // location I.
            (0x0F, _, 0x05, 0x05) => {
                let i = self.i_register as usize;

                for n in 0..=instruction.x() {
                    self.ram[i + n] = self.v_registers[n];
                }
            }

            // LD Vx, [I]: read registers V0 through Vx from memory starting at
            // location I
            (0x0F, _, 0x06, 0x05) => {
                let i = self.i_register as usize;

                for n in 0..=instruction.x() {
                    self.v_registers[n] = self.ram[i + n];
                }
            }

            _ => eprintln!("Invalid instruction"),
        }
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        let status = self.st > 0;
        self.speaker.beep(status);
        if status {
            self.st -= 1;
        }
    }

    pub fn is_pixel_set(&self, x: usize, y: usize) -> bool {
        self.screen[y][x]
    }
    // endregion

    // region: Private functions
    fn advance_pc(&mut self) {
        self.pc += INSTRUCTION_LENGTH;
    }

    fn clear_screen(&mut self) {
        self.screen = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut pixel_collission = false;

        for (ly, b) in sprite.iter().enumerate() {
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

    fn is_key_down(&self, key: usize) -> bool {
        self.keyboard[key]
    }

    fn ram_read(&self, start: usize, bytes: u8) -> &[u8] {
        &self.ram[start..start + bytes as usize]
    }

    fn read_opcode(&self, start: usize) -> u16 {
        let bytes = self.ram_read(start, 2);
        (bytes[0] as u16) << 8 | bytes[1] as u16
    }

    fn set_carry_if(&mut self, condition: bool) {
        self.v_registers[0xf] = if condition { 1 } else { 0 };
    }

    fn stack_push(&mut self, value: u16) {
        if self.sp as usize >= STACK_DEPTH {
            panic!("Stack overflow");
        }
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn stack_pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn toggle_pixel(&mut self, x: usize, y: usize) {
        self.screen[y][x] ^= true;
    }
    // endregion
}

struct Instruction {
    opcode: u16,
    nibbles: (u8, u8, u8, u8),
}

impl Instruction {
    fn x(&self) -> usize {
        self.nibbles.1 as usize
    }

    fn y(&self) -> usize {
        self.nibbles.2 as usize
    }

    fn n(&self) -> u8 {
        self.nibbles.3
    }

    fn nn(&self) -> u8 {
        (self.opcode & 0xFF) as u8
    }

    fn nnn(&self) -> u16 {
        self.opcode & 0xFFF
    }
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        Instruction {
            opcode: instruction,
            nibbles: (
                ((instruction & 0xF000) >> 12) as u8,
                ((instruction & 0x0F00) >> 8) as u8,
                ((instruction & 0x00F0) >> 4) as u8,
                (instruction & 0x000F) as u8,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSpeaker {}
    impl TestSpeaker {
        fn new() -> Self {
            TestSpeaker {}
        }
    }

    impl Speaker for TestSpeaker {
        fn beep(&mut self, _status: bool) {}
    }

    fn new_chip8() -> Chip8<'static> {
        Chip8::new(Box::new(TestSpeaker::new()))
    }

    #[test]
    fn toggle_pixel_can_toggle_a_pixel() {
        let mut chip8 = new_chip8();
        assert!(!chip8.is_pixel_set(5, 5));
        chip8.toggle_pixel(5, 5);
        assert!(chip8.is_pixel_set(5, 5));
        chip8.toggle_pixel(5, 5);
        assert!(!chip8.is_pixel_set(5, 5));
    }

    #[test]
    fn draw_sprite_returns_true_when_overwriting() {
        let mut chip8 = new_chip8();
        assert!(!chip8.draw_sprite(0, 0, &[0xff]));
        assert!(chip8.draw_sprite(0, 0, &[0xff]));
    }

    #[test]
    fn it_can_press_and_release_keys() {
        let mut chip8 = new_chip8();
        assert!(!chip8.is_key_down(1));
        chip8.key_down(1);
        assert!(chip8.is_key_down(1));
        chip8.key_up(1);
        assert!(!chip8.is_key_down(1));
    }

    #[test]
    fn it_can_push_to_and_pop_from_the_stack() {
        let mut chip8 = new_chip8();
        assert_eq!(chip8.sp, 0);
        chip8.stack_push(0xff);
        assert_eq!(chip8.sp, 1);
        assert_eq!(chip8.stack[0], 0xff);

        chip8.stack_push(0xaa);
        assert_eq!(chip8.sp, 2);
        assert_eq!(chip8.stack[1], 0xaa);
        assert_eq!(chip8.stack_pop(), 170);
        assert_eq!(chip8.sp, 1);
        assert_eq!(chip8.stack_pop(), 255);
        assert_eq!(chip8.sp, 0);
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn it_panics_on_stack_overflow() {
        let mut chip8 = new_chip8();
        for _ in 0..=STACK_DEPTH {
            chip8.stack_push(0x1234);
        }
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn it_panics_on_stack_underflow() {
        let mut chip8 = new_chip8();
        chip8.stack_pop();
    }

    #[test]
    fn it_contains_the_default_character_set() {
        let chip8 = new_chip8();
        assert_eq!(
            chip8.ram[..DEFAULT_CHARACTER_SET_SIZE],
            DEFAULT_CHARACTER_SET
        )
    }

    #[test]
    fn test_rom_loading() {
        let mut chip8 = new_chip8();

        // Create a small ROM
        let rom: Vec<u8> = vec![1, 2, 3, 4];

        let result = chip8.load_rom(rom);
        assert!(result.is_ok());

        // Check that the ROM was loaded correctly
        assert_eq!(chip8.ram[PROGRAM_LOAD_ADDRESS], 1);
        assert_eq!(chip8.ram[PROGRAM_LOAD_ADDRESS + 1], 2);
        assert_eq!(chip8.ram[PROGRAM_LOAD_ADDRESS + 2], 3);
        assert_eq!(chip8.ram[PROGRAM_LOAD_ADDRESS + 3], 4);
    }
}
