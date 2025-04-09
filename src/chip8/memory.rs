pub const MEMORY_SIZE: usize = 4096;
pub const PROGRAM_LOAD_ADDRESS: usize = 0x200;

// Storing the character set at 0x50 – 0x09f is a sort of convention.
const DEFAULT_CHARACTER_SET_START: usize = 0x50;

static DEFAULT_CHARACTER_SET: [u8; 80] = [
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

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

fn copy_default_char_set(memory: &mut Memory) {
    let mut index = DEFAULT_CHARACTER_SET_START;
    for byte in DEFAULT_CHARACTER_SET.iter() {
        memory.set(index, *byte);
        index += 1;
    }
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Memory {
            memory: [0; MEMORY_SIZE],
        };
        copy_default_char_set(&mut memory);
        memory
    }

    pub fn set(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }

    pub fn get(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn read(&self, start: usize, bytes: u8) -> &[u8] {
        &self.memory[start..start + bytes as usize]
    }

    pub fn read_opcode(&self, start: usize) -> u16 {
        let bytes = self.read(start, 2);
        (bytes[0] as u16) << 8 | bytes[1] as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_the_correct_memory_size() {
        assert_eq!(Memory::new().memory.len(), MEMORY_SIZE);
    }

    #[test]
    fn it_can_write_the_memory() {
        let mut memory = Memory::new();
        memory.set(200, 42);
        assert_eq!(memory.memory[200..=202], [42, 0, 0]);
    }

    #[test]
    fn it_can_read_the_memory() {
        let mut memory = Memory::new();
        memory.set(2, 42);
        assert_eq!(memory.get(2), 42);
    }

    #[test]
    fn it_contains_the_default_character_set() {
        let memory = Memory::new();
        assert_eq!(memory.memory[0x50..0x55], [0xf0, 0x90, 0x90, 0x90, 0xf0])
    }

    #[test]
    fn it_returns_a_slice_of_memory() {
        let memory = Memory::new();
        let slice = memory.read(0x50, 5);
        assert_eq!(slice.len(), 5);
        assert_eq!(slice, [0xf0, 0x90, 0x90, 0x90, 0xf0]);
    }
}
