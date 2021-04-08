use super::memory::PROGRAM_LOAD_ADDRESS;

const DATA_REGISTERS: usize = 16;

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

use Register::*;

fn data_register_to_index(register: Register) -> usize {
    match register {
        V0 => 0,
        V1 => 1,
        V2 => 2,
        V3 => 3,
        V4 => 4,
        V5 => 5,
        V6 => 6,
        V7 => 7,
        V8 => 8,
        V9 => 9,
        VA => 10,
        VB => 11,
        VC => 12,
        VD => 13,
        VE => 14,
        VF => 15,
        _ => panic!("invalid data register"),
    }
}

pub struct Registers {
    data: [u8; DATA_REGISTERS],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    pub sp: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            data: [0; DATA_REGISTERS],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: PROGRAM_LOAD_ADDRESS as u16,
            sp: 0,
        }
    }

    pub fn set(&mut self, register: Register, value: u16) -> () {
        match register {
            I => self.i = value,
            DT => self.delay_timer = value as u8,
            ST => self.sound_timer = value as u8,
            PC => self.pc = value,
            SP => self.sp = value as u8,
            _ => self.data[data_register_to_index(register)] = value as u8,
        }
    }

    pub fn get(&self, register: Register) -> u16 {
        match register {
            I => self.i,
            DT => self.delay_timer as u16,
            ST => self.sound_timer as u16,
            PC => self.pc,
            SP => self.sp as u16,
            _ => self.data[data_register_to_index(register)] as u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_the_correct_number_of_data_registers() {
        assert_eq!(Registers::new().data.len(), DATA_REGISTERS);
    }

    #[test]
    fn it_can_write_data_registers() {
        let mut registers = Registers::new();
        registers.set(VA, 42);
        assert_eq!(registers.data[10], 42);
    }

    #[test]
    fn it_can_read_data_registers() {
        let mut registers = Registers::new();
        registers.set(VA, 42);
        assert_eq!(registers.get(VA), 42);
    }

    #[test]
    fn it_can_write_special_registers() {
        let mut registers = Registers::new();
        registers.set(PC, 42);
        assert_eq!(registers.pc, 42);
    }

    #[test]
    fn it_can_read_special_registers() {
        let mut registers = Registers::new();
        registers.set(PC, 42);
        assert_eq!(registers.get(PC), 42);
    }
}
