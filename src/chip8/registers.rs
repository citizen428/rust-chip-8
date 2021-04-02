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
            pc: 0,
            sp: 0,
        }
    }

    pub fn set(&mut self, register: Register, value: u16) -> () {
        match register {
            Register::I => self.i = value,
            Register::DT => self.delay_timer = value as u8,
            Register::ST => self.sound_timer = value as u8,
            Register::PC => self.pc = value,
            Register::SP => self.sp = value as u8,
            _ => self.data[data_register_to_index(register)] = value as u8,
        }
    }

    pub fn get(&self, register: Register) -> u16 {
        match register {
            Register::I => self.i,
            Register::DT => self.delay_timer as u16,
            Register::ST => self.sound_timer as u16,
            Register::PC => self.pc,
            Register::SP => self.sp as u16,
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
        registers.set(Register::VA, 42);
        assert_eq!(registers.data[10], 42);
    }

    #[test]
    fn it_can_read_data_registers() {
        let mut registers = Registers::new();
        registers.set(Register::VA, 42);
        assert_eq!(registers.get(Register::VA), 42);
    }

    #[test]
    fn it_can_write_special_registers() {
        let mut registers = Registers::new();
        registers.set(Register::PC, 42);
        assert_eq!(registers.pc, 42);
    }

    #[test]
    fn it_can_read_special_registers() {
        let mut registers = Registers::new();
        registers.set(Register::PC, 42);
        assert_eq!(registers.get(Register::PC), 42);
    }
}
