pub const STACK_DEPTH: usize = 16;

pub struct Stack {
    stack: [u16; STACK_DEPTH],
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; STACK_DEPTH],
        }
    }

    pub fn push(&mut self, stack_pointer: &mut u8, value: u16) -> () {
        assert!((*stack_pointer as usize) < STACK_DEPTH, "stack overflow");
        self.stack[*stack_pointer as usize] = value;
        *stack_pointer += 1;
    }

    pub fn pop(&mut self, stack_pointer: &mut u8) -> u16 {
        assert!((*stack_pointer as usize) > 0, "stack underflow");
        *stack_pointer -= 1;
        assert!((*stack_pointer as usize) < STACK_DEPTH, "stack overflow");
        self.stack[*stack_pointer as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chip8::registers::{Register, Registers};

    #[test]
    fn it_can_push_to_and_pop_from_the_stack() {
        let mut registers = Registers::new();
        assert_eq!(registers.get(Register::SP), 0);

        let mut stack = Stack::new();
        stack.push(&mut registers.sp, 0xff);
        assert_eq!(registers.get(Register::SP), 1);

        stack.push(&mut registers.sp, 0xaa);
        assert_eq!(registers.get(Register::SP), 2);
        assert_eq!(stack.pop(&mut registers.sp), 170);
        assert_eq!(registers.get(Register::SP), 1);
        assert_eq!(stack.pop(&mut registers.sp), 255);
        assert_eq!(registers.get(Register::SP), 0);
    }
}
