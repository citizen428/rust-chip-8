use sdl2::keyboard::Keycode;

const MEMORY_SIZE: usize = 4096;
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const DATA_REGISTERS: usize = 16;
const STACK_DEPTH: usize = 16;
const KEYS: usize = 16;

pub const SCALE_FACTOR: u32 = 10;
pub const WINDOW_WIDTH: u32 = WIDTH * SCALE_FACTOR;
pub const WINDOW_HEIGHT: u32 = HEIGHT * SCALE_FACTOR;

type Memory = [u8; MEMORY_SIZE];

pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
    I,
    DT,
    ST,
    PC,
    SP,
}

struct Registers {
    data: [u8; DATA_REGISTERS],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u8,
}

type Stack = [u16; STACK_DEPTH];

type Keyboard = [bool; KEYS];

pub struct Chip8 {
    memory: Memory,
    registers: Registers,
    stack: Stack,
    keyboard: Keyboard,
}

fn data_register_to_index(register: Register) -> usize {
    match register {
        Register::V0 => 0,
        Register::V1 => 1,
        Register::V2 => 2,
        Register::V3 => 3,
        Register::V4 => 4,
        Register::V5 => 5,
        Register::V6 => 6,
        Register::V7 => 7,
        Register::V8 => 8,
        Register::V9 => 9,
        Register::VA => 10,
        Register::VB => 11,
        Register::VC => 12,
        Register::VD => 13,
        Register::VE => 14,
        Register::VF => 15,
        _ => panic!("invalid data register"),
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; MEMORY_SIZE],
            registers: Registers {
                data: [0; DATA_REGISTERS],
                i: 0,
                delay_timer: 0,
                sound_timer: 0,
                pc: 0,
                sp: 0,
            },
            stack: [0; STACK_DEPTH],
            keyboard: [false; KEYS],
        }
    }

    pub fn memory_set(&mut self, index: usize, value: u8) -> () {
        self.memory[index] = value;
    }

    pub fn memory_get(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn register_set(&mut self, register: Register, value: u16) -> () {
        match register {
            Register::I => self.registers.i = value,
            Register::DT => self.registers.delay_timer = value as u8,
            Register::ST => self.registers.sound_timer = value as u8,
            Register::PC => self.registers.pc = value,
            Register::SP => self.registers.sp = value as u8,
            _ => self.registers.data[data_register_to_index(register)] = value as u8,
        }
    }

    pub fn register_get(&self, register: Register) -> u16 {
        match register {
            Register::I => self.registers.i,
            Register::DT => self.registers.delay_timer as u16,
            Register::ST => self.registers.sound_timer as u16,
            Register::PC => self.registers.pc,
            Register::SP => self.registers.sp as u16,
            _ => self.registers.data[data_register_to_index(register)] as u16,
        }
    }

    pub fn stack_push(&mut self, value: u16) -> () {
        assert!((self.registers.sp as usize) < STACK_DEPTH, "stack overflow");
        self.stack[self.registers.sp as usize] = value;
        self.registers.sp += 1;
    }

    pub fn stack_pop(&mut self) -> u16 {
        assert!(self.registers.sp > 0, "stack underflow");
        self.registers.sp -= 1;
        assert!((self.registers.sp as usize) < STACK_DEPTH, "stack overflow");
        self.stack[self.registers.sp as usize]
    }

    pub fn keyboard_map(&self, key: Keycode) -> Option<usize> {
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

    pub fn key_down(&mut self, key: usize) -> () {
        self.keyboard[key] = true
    }

    pub fn key_up(&mut self, key: usize) -> () {
        self.keyboard[key] = false
    }

    pub fn is_key_down(&self, key: usize) -> bool {
        self.keyboard[key]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_the_correct_memory_size() {
        assert_eq!(Chip8::new().memory.len(), MEMORY_SIZE);
    }

    #[test]
    fn it_can_write_the_memory() {
        let mut chip8 = Chip8::new();
        chip8.memory_set(2, 42);
        assert_eq!(chip8.memory[0..3], [0, 0, 42]);
    }

    #[test]
    fn it_can_read_the_memory() {
        let mut chip8 = Chip8::new();
        chip8.memory_set(2, 42);
        assert_eq!(chip8.memory_get(2), 42);
    }

    #[test]
    fn it_has_the_correct_number_of_data_registers() {
        assert_eq!(Chip8::new().registers.data.len(), DATA_REGISTERS);
    }

    #[test]
    fn it_can_write_data_registers() {
        let mut chip8 = Chip8::new();
        chip8.register_set(Register::VA, 42);
        assert_eq!(chip8.registers.data[10], 42);
    }

    #[test]
    fn it_can_read_data_registers() {
        let mut chip8 = Chip8::new();
        chip8.register_set(Register::VA, 42);
        assert_eq!(chip8.register_get(Register::VA), 42);
    }

    #[test]
    fn it_can_write_special_registers() {
        let mut chip8 = Chip8::new();
        chip8.register_set(Register::PC, 42);
        assert_eq!(chip8.registers.pc, 42);
    }

    #[test]
    fn it_can_read_special_registers() {
        let mut chip8 = Chip8::new();
        chip8.register_set(Register::PC, 42);
        assert_eq!(chip8.register_get(Register::PC), 42);
    }

    #[test]
    fn it_can_push_to_and_pop_from_the_stack() {
        let mut chip8 = Chip8::new();
        assert_eq!(chip8.register_get(Register::SP), 0);
        chip8.stack_push(0xff);
        assert_eq!(chip8.register_get(Register::SP), 1);
        chip8.stack_push(0xaa);
        assert_eq!(chip8.register_get(Register::SP), 2);
        assert_eq!(chip8.stack_pop(), 170);
        assert_eq!(chip8.register_get(Register::SP), 1);
        assert_eq!(chip8.stack_pop(), 255);
        assert_eq!(chip8.register_get(Register::SP), 0);
    }

    #[test]
    fn it_maps_physical_keys_to_virtual_ones() {
        let mut chip8 = Chip8::new();
        assert_eq!(chip8.keyboard_map(Keycode::A), Some(7));
        assert_eq!(chip8.keyboard_map(Keycode::X), Some(0));
        assert_eq!(chip8.keyboard_map(Keycode::M), None);
    }

    #[test]
    fn it_can_press_and_release_keys() {
        let mut chip8 = Chip8::new();
        chip8.key_down(1);
        assert!(chip8.is_key_down(1));
        chip8.key_up(1);
        assert!(!chip8.is_key_down(1));
    }
}
