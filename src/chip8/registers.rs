use super::instruction;
use super::memory;

const DATA_REGISTERS: usize = 16;

pub struct Registers {
    v: [u8; DATA_REGISTERS],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            v: [0; DATA_REGISTERS],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: memory::PROGRAM_LOAD_ADDRESS as u16,
            sp: 0,
        }
    }
}

impl Registers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_v(&self, n: usize) -> u8 {
        self.v[n]
    }

    pub fn set_v(&mut self, n: usize, value: u8) {
        self.v[n] = value;
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn set_i(&mut self, addr: u16) {
        self.i = addr;
    }

    pub fn get_dt(&self) -> u8 {
        self.delay_timer
    }

    pub fn set_dt(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn dec_dt(&mut self) {
        self.delay_timer -= 1;
    }

    pub fn get_st(&self) -> u8 {
        self.sound_timer
    }

    pub fn set_st(&mut self, value: u8) {
        self.sound_timer = value;
    }

    pub fn dec_st(&mut self) {
        self.sound_timer -= 1;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn advance_pc(&mut self) {
        self.pc += instruction::INSTRUCTION_LENGTH;
    }

    pub fn get_sp(&self) -> u8 {
        self.sp
    }

    pub fn inc_sp(&mut self) {
        self.sp += 1;
    }

    pub fn dec_sp(&mut self) {
        self.sp -= 1;
    }

    pub fn set_carry_if(&mut self, condition: bool) {
        self.v[0xf] = if condition { 1 } else { 0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_the_correct_number_of_data_registers() {
        assert_eq!(Registers::new().v.len(), DATA_REGISTERS);
    }

    #[test]
    fn it_can_write_data_registers() {
        let mut registers = Registers::new();
        registers.set_v(0xA, 42);
        assert_eq!(registers.v[0xA], 42);
    }

    #[test]
    fn it_can_read_data_registers() {
        let mut registers = Registers::new();
        registers.set_v(0xA, 42);
        assert_eq!(registers.get_v(0xA), 42);
    }

    #[test]
    fn it_can_write_special_registers() {
        let mut registers = Registers::new();
        registers.set_pc(42);
        assert_eq!(registers.pc, 42);
    }

    #[test]
    fn it_can_read_special_registers() {
        let mut registers = Registers::new();
        registers.set_pc(42);
        assert_eq!(registers.get_pc(), 42);
    }
}
