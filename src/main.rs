mod chip8;

use chip8::display;
use chip8::emulator::Chip8;

use debug_print::{debug_eprintln, debug_print, debug_println};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::env;

const EMULATOR_WINDOW_TITLE: &str = "Rust CHIP-8";

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
        .window(
            EMULATOR_WINDOW_TITLE,
            display::WINDOW_WIDTH,
            display::WINDOW_HEIGHT,
        )
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;

    'mainloop: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

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
                    chip8.keyboard.key_down(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    chip8.keyboard.key_up(key);
                }
                _ => {}
            }
        }

        canvas.present();
        chip8.handle_delay_timer();
        chip8.handle_sound_timer();
        chip8.exec();

        for y in 0..display::HEIGHT {
            for x in 0..display::WIDTH {
                if chip8.screen.is_pixel_set(x, y) {
                    canvas.fill_rect(Rect::new(
                        (x as u32 * display::SCALE_FACTOR) as i32,
                        (y as u32 * display::SCALE_FACTOR) as i32,
                        display::SCALE_FACTOR,
                        display::SCALE_FACTOR,
                    ))?;
                }
            }
        }
    }

    Ok(())
}
