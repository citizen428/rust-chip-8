pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SCALE_FACTOR: u32 = 10;
pub const WINDOW_WIDTH: u32 = WIDTH as u32 * SCALE_FACTOR;
pub const WINDOW_HEIGHT: u32 = HEIGHT as u32 * SCALE_FACTOR;

pub struct Screen {
    pixels: [[bool; WIDTH]; HEIGHT],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn pixel_set(&mut self, x: usize, y: usize) -> () {
        self.pixels[y][x] ^= true;
    }

    pub fn is_pixel_set(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut pixel_collission = false;

        for ly in 0..sprite.len() {
            let b = sprite[ly];
            for lx in 0..8 {
                if b & 0b10000000 >> lx > 0 {
                    let dx = (x + lx) % WIDTH;
                    let dy = (y + ly) % HEIGHT;
                    if !pixel_collission && self.is_pixel_set(dx, dy) {
                        pixel_collission = true;
                    }

                    self.pixel_set(dx, dy);
                }
            }
        }

        pixel_collission
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

    #[test]
    fn it_returns_true_when_overwriting() {
        let mut screen = Screen::new();
        assert!(!screen.draw_sprite(0, 0, &[0xff]));
        assert!(screen.draw_sprite(0, 0, &[0xff]));
    }
}
