const MEMORY_SIZE: usize = 4096;
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub const SCALE_FACTOR: u32 = 10;
pub const WINDOW_WIDTH: u32 = WIDTH * SCALE_FACTOR;
pub const WINDOW_HEIGHT: u32 = HEIGHT * SCALE_FACTOR;

type Memory = [u8; MEMORY_SIZE];

pub struct Chip8 {
    memory: Memory,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn memory_set(&mut self, index: usize, value: u8) -> () {
        self.memory[index] = value;
    }

    pub fn memory_get(&self, index: usize) -> u8 {
        self.memory[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_the_correct_size() {
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
}
