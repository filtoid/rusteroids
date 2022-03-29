// SDL2 Library Imports
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

fn main() -> Result<(), String> {
    println!("Starting Rusteroids");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _window = video_subsystem.window("Rusteroids", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {} 
            }
        }

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
