use crate::chip8::registers::Registers;

pub const STACK_DEPTH: usize = 16;

pub struct Stack {
    stack: [u16; STACK_DEPTH],
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0; STACK_DEPTH],
        }
    }

    pub fn push(&mut self, registers: &mut Registers, value: u16) {
        let stack_pointer = registers.get_sp() as usize;
        assert!(stack_pointer < STACK_DEPTH, "stack overflow");
        self.stack[stack_pointer] = value;
        registers.inc_sp();
    }

    pub fn pop(&mut self, registers: &mut Registers) -> u16 {
        let stack_pointer = registers.get_sp() as usize;
        assert!(stack_pointer > 0, "stack underflow");
        registers.dec_sp();
        assert!(stack_pointer < STACK_DEPTH, "stack overflow");
        self.stack[registers.get_sp() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_push_to_and_pop_from_the_stack() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_sp(), 0);

        let mut stack = Stack::new();
        stack.push(&mut registers, 0xff);
        assert_eq!(registers.get_sp(), 1);
        assert_eq!(stack.stack[0], 0xff);

        stack.push(&mut registers, 0xaa);
        assert_eq!(registers.get_sp(), 2);
        assert_eq!(stack.stack[1], 0xaa);
        assert_eq!(stack.pop(&mut registers), 170);
        assert_eq!(registers.get_sp(), 1);
        assert_eq!(stack.pop(&mut registers), 255);
        assert_eq!(registers.get_sp(), 0);
    }
}
