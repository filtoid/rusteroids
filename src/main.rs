// SDL2 Library Imports
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color; 
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;

use sdl2::rect::Rect;
use sdl2::rect::Point;

use std::time::Duration;
use std::path::Path;
use std::collections::HashMap;

pub mod texture_manager;

const IMAGE_WIDTH:u32 = 100;
const IMAGE_HEIGHT:u32 = 100;
const OUTPUT_WIDTH: u32 = 50;
const OUTPUT_HEIGHT: u32 = 50;
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn render(canvas: &mut WindowCanvas, texture_manager: &mut texture_manager::TextureManager<WindowContext>, _texture_creator: &TextureCreator<WindowContext>, _font: &sdl2::ttf::Font) -> Result<(), String> {  // 
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();
    
    // Draw Space Ship
    let src = Rect::new(0,0,IMAGE_WIDTH,IMAGE_HEIGHT);
    let x: i32 = (SCREEN_WIDTH/2) as i32;
    let y: i32 = (SCREEN_HEIGHT/2) as i32;

    let dest = Rect::new(x - ((OUTPUT_WIDTH/2) as i32),y - ((OUTPUT_HEIGHT/2) as i32),OUTPUT_WIDTH,OUTPUT_HEIGHT);    
    let center = Point::new( (OUTPUT_WIDTH/2) as i32, (OUTPUT_HEIGHT) as i32);

    let texture = texture_manager.load("img/space_ship.png")?;

    canvas.copy_ex(
        &texture, // Texture object
        src,      // source rect
        dest,     // destination rect
        279.0,      // angle (degrees)
        center,   // center
        false,    // flip horizontal
        false     // flip vertical
    )?;

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
    let mut tex_man = texture_manager::TextureManager::new(&texture_creator);
    
    // Load the images before the main loop so we don't try and load during gameplay
    tex_man.load("img/space_ship.png")?;

    // Prepare fonts
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    let font_path: &Path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
 
    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String, bool> = HashMap::new();
    
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
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            utils::key_down(&mut key_manager, key.to_string());
                        }
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            utils::key_up(&mut key_manager, key.to_string());
                        }
                    }
                 },
                
                _ => {} 
            }
        }

        render(&mut canvas, &mut tex_man, &texture_creator, &font)?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
