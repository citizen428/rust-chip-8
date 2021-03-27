use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod chip8;

const EMULATOR_WINDOW_TITLE: &str = "Rust CHIP-8";

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window(EMULATOR_WINDOW_TITLE, chip8::WINDOW_WIDTH, chip8::WINDOW_HEIGHT)
    .position_centered().build()
    .expect("Could not initialize video subsystem");

  let mut canvas = window.into_canvas().build().expect("Could not make a canvas");

  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();

  let mut event_pump = sdl_context.event_pump()?;
  let mut i = 0;

  'mainloop: loop {
    i = (i + 1) % 255;
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.fill_rect(Rect::new(10, 10, 620, 300))?;

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} |
          Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            break 'mainloop;
          },
        _ => {}
      }
    }

    canvas.present();
    ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
  }

  Ok(())
}
