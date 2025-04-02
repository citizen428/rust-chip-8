use debug_print::debug_println;
use sdl2::keyboard::Keycode;

const KEYS: usize = 16;

pub struct Keyboard {
    keyboard: [bool; KEYS],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keyboard: [false; KEYS],
        }
    }

    pub fn key_down(&mut self, key: Keycode) {
        self.toggle_key(key, true);
    }

    pub fn key_up(&mut self, key: Keycode) {
        self.toggle_key(key, false);
    }

    pub fn toggle_key(&mut self, key: Keycode, is_down: bool) {
        // Ignore all keys that aren't mapped to the CHIP-8 hex keyboard.
        if let Some(key_index) = self.map(key) {
            self.keyboard[key_index] = is_down;
            if is_down {
                debug_println!("key down: {}", key_index);
            } else {
                debug_println!("key up: {}", key_index);
            }
        }
    }

    pub fn is_key_down(&self, key: usize) -> bool {
        self.keyboard[key]
    }

    fn map(&self, key: Keycode) -> Option<usize> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_maps_physical_keys_to_virtual_ones() {
        let keyboard = Keyboard::new();
        assert_eq!(keyboard.map(Keycode::A), Some(7));
        assert_eq!(keyboard.map(Keycode::X), Some(0));
        assert_eq!(keyboard.map(Keycode::M), None);
    }
    #[test]
    fn it_can_press_and_release_keys() {
        let mut keyboard = Keyboard::new();
        assert!(!keyboard.is_key_down(1));
        keyboard.key_down(Keycode::Num1);
        assert!(keyboard.is_key_down(1));
        keyboard.key_up(Keycode::Num1);
        assert!(!keyboard.is_key_down(1));
    }
}
