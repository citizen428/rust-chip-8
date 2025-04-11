const DATA_REGISTERS: usize = 16;

pub struct Registers {
    v: [u8; DATA_REGISTERS],
    i: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            v: [0; DATA_REGISTERS],
            i: 0,
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
}
