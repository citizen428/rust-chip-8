pub const INSTRUCTION_LENGTH: u16 = 2;

pub struct Instruction {
    pub nibbles: (u8, u8, u8, u8),
    pub addr: u16,
    pub byte: u8,
    pub x: usize,
    pub y: usize,
    pub nibble: u8,
}

impl Instruction {
    pub fn parse(instruction: u16) -> Self {
        let nibbles = (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        );

        Instruction {
            nibbles,
            addr: (instruction & 0x0FFF),
            nibble: nibbles.3,
            byte: (instruction & 0x00FF) as u8,
            x: nibbles.1 as usize,
            y: nibbles.2 as usize,
        }
    }
}
