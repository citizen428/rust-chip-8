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

    pub fn map(&self, key: Keycode) -> Option<usize> {
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

    pub fn key_down(&mut self, key: usize) {
        self.keyboard[key] = true
    }

    pub fn key_up(&mut self, key: usize) {
        self.keyboard[key] = false
    }

    pub fn is_key_down(&self, key: usize) -> bool {
        self.keyboard[key]
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
        keyboard.key_down(1);
        assert!(keyboard.is_key_down(1));
        keyboard.key_up(1);
        assert!(!keyboard.is_key_down(1));
    }
}
