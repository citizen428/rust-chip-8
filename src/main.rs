mod chip8;

use chip8::emulator::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};

use debug_print::{debug_eprintln, debug_print, debug_println};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::env;

const WINDOW_TITLE: &str = "Rust CHIP-8";
// Each CHIP-8 pixel gets rendered as a 10x10 square
const SCALE_FACTOR: u32 = 10;
const WINDOW_WIDTH: u32 = DISPLAY_WIDTH as u32 * SCALE_FACTOR;
const WINDOW_HEIGHT: u32 = DISPLAY_HEIGHT as u32 * SCALE_FACTOR;

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

fn run(rom: &str) -> Result<(), String> {
    debug_print!("Initializing SDL: ");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;
    debug_println!("Done");

    let mut chip8 = Chip8::new(&audio_subsystem);
    debug_print!("Loading ROM: {}: ", &rom);
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
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'mainloop;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    chip8.key_down(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    chip8.key_up(key);
                }
                _ => {}
            }
        }

        chip8.handle_delay_timer();
        chip8.handle_sound_timer();
        chip8.exec();

        // Clear the canvas to black before rendering each frame
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Render only the pixels that are set
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
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

fn scaled_rect(x: usize, y: usize) -> Rect {
    Rect::new(
        (x as u32 * SCALE_FACTOR) as i32,
        (y as u32 * SCALE_FACTOR) as i32,
        SCALE_FACTOR,
        SCALE_FACTOR,
    )
}
