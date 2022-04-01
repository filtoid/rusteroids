// SDL2 Library Imports
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color; 
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use std::time::Duration;
use std::path::Path;

fn render(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font: &sdl2::ttf::Font) -> Result<(), String> {  // 
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();
    
    // Draw Greeting
    let hello_text: String = "Hello World".to_string();
    let surface = font
        .render(&hello_text)
        .blended(Color::RGBA(255, 0, 0, 128))
        .map_err(|e| e.to_string())?;
    
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(10 as i32, 0 as i32, 200 as u32, 100 as u32);
    canvas.copy(&texture, None, Some(target))?;

    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    println!("Starting Rusteroids");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Rusteroids", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    // Prepare fonts
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    let font_path: &Path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
 
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

        render(&mut canvas, &texture_creator, &font)?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
