use std::{env, fs};

use debug_print::{debug_eprintln, debug_print, debug_println};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod chip8;
mod sdl_speaker;

const WINDOW_TITLE: &str = "Rust CHIP-8";
// Each CHIP-8 pixel gets rendered as a 10x10 square
const SCALE_FACTOR: u32 = 10;
const WINDOW_WIDTH: u32 = chip8::DISPLAY_WIDTH as u32 * SCALE_FACTOR;
const WINDOW_HEIGHT: u32 = chip8::DISPLAY_HEIGHT as u32 * SCALE_FACTOR;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <path to ROM>", &args[0]);
        std::process::exit(0);
    }

    std::process::exit(match run(&args[1]) {
        Ok(_) => 0,
        Err(err) => {
            debug_eprintln!("ERROR: {:?}", err);
            1
        }
    });
}

fn run(rom_path: &str) -> Result<(), String> {
    debug_print!("Initializing SDL: ");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;
    debug_println!("Done");

    let speaker = sdl_speaker::SDLSpeaker::new(&audio_subsystem);
    let mut chip8 = chip8::Chip8::new(Box::new(speaker));
    debug_print!("Loading ROM: {}: ", rom_path);
    let rom = fs::read(rom_path).map_err(|e| format!("Cannot read ROM: {}", e))?;
    let bytes = chip8.load_rom(rom)?;
    debug_println!("Done ({} bytes)", bytes);

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas");

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => {
                    break 'mainloop;
                }
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    if let Some(key) = map_scancode_to_key(sc) {
                        debug_println!("key down: {}", key);
                        chip8.key_down(key);
                    }
                }
                Event::KeyUp {
                    scancode: Some(sc), ..
                } => {
                    if let Some(key) = map_scancode_to_key(sc) {
                        debug_println!("key up: {}", key);
                        chip8.key_up(key);
                    }
                }
                _ => {}
            }
        }

        chip8.exec();
        chip8.update_timers();

        // Clear the canvas to black before rendering each frame
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Render only the pixels that are set
        for y in 0..chip8::DISPLAY_HEIGHT {
            for x in 0..chip8::DISPLAY_WIDTH {
                if chip8.is_pixel_set(x, y) {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(scaled_rect(x, y))?;
                }
            }
        }

        canvas.present();
    }

    Ok(())
}

fn map_scancode_to_key(sc: Scancode) -> Option<usize> {
    match sc {
        Scancode::Num1 => Some(1),
        Scancode::Num2 => Some(2),
        Scancode::Num3 => Some(3),
        Scancode::Num4 => Some(12),
        Scancode::Q => Some(4),
        Scancode::W => Some(5),
        Scancode::E => Some(6),
        Scancode::R => Some(13),
        Scancode::A => Some(7),
        Scancode::S => Some(8),
        Scancode::D => Some(9),
        Scancode::F => Some(14),
        Scancode::Z => Some(10),
        Scancode::X => Some(0),
        Scancode::C => Some(11),
        Scancode::V => Some(15),
        _ => None,
    }
}

fn scaled_rect(x: usize, y: usize) -> Rect {
    Rect::new(
        (x as u32 * SCALE_FACTOR) as i32,
        (y as u32 * SCALE_FACTOR) as i32,
        SCALE_FACTOR,
        SCALE_FACTOR,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_maps_physical_keys_to_virtual_ones() {
        assert_eq!(map_scancode_to_key(Scancode::A), Some(7));
        assert_eq!(map_scancode_to_key(Scancode::X), Some(0));
        assert_eq!(map_scancode_to_key(Scancode::M), None);
    }
}
