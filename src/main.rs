mod chip8;

use chip8::display;
use chip8::emulator::Chip8;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const EMULATOR_WINDOW_TITLE: &str = "Rust CHIP-8";

fn main() -> Result<(), String> {
    let mut chip8 = Chip8::new();
    chip8.screen.draw_sprite(24, 13, chip8.memory.read(20, 5));
    chip8.screen.draw_sprite(29, 13, chip8.memory.read(10, 5));
    chip8.screen.draw_sprite(34, 13, chip8.memory.read(40, 5));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

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
                    if let Some(vkey) = chip8.keyboard.map(key) {
                        chip8.keyboard.key_down(vkey);
                        println!("key down: {}", vkey);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(vkey) = chip8.keyboard.map(key) {
                        chip8.keyboard.key_up(vkey);
                        println!("key up: {}", vkey);
                    }
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
