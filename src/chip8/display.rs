pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SCALE_FACTOR: u32 = 10;
pub const WINDOW_WIDTH: u32 = WIDTH as u32 * SCALE_FACTOR;
pub const WINDOW_HEIGHT: u32 = HEIGHT as u32 * SCALE_FACTOR;

pub struct Screen {
    pixels: [[bool; WIDTH]; HEIGHT],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn pixel_set(&mut self, x: usize, y: usize) -> () {
        self.pixels[y][x] = true;
    }

    pub fn is_pixel_set(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_set_a_pixel() {
        let mut screen = Screen::new();
        screen.pixel_set(5, 5);
        assert!(screen.is_pixel_set(5, 5));
    }
}
