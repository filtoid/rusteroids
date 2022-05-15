// SDL2 Library Imports
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color; 
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;

use sdl2::rect::Rect;
use sdl2::rect::Point;

use specs::{World, WorldExt, Join, DispatcherBuilder};

use std::time::Duration;
use std::path::Path;
use std::collections::HashMap;

pub mod texture_manager;
pub mod utils;
pub mod components;
pub mod game;
pub mod asteroid; 

const GAME_WIDTH: u32 = 800;
const GAME_HEIGHT: u32 = 600;

fn render(canvas: &mut WindowCanvas, texture_manager: &mut texture_manager::TextureManager<WindowContext>, _texture_creator: &TextureCreator<WindowContext>, _font: &sdl2::ttf::Font, ecs: &World) -> Result<(), String> {  // 
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();
   
    let positions = ecs.read_storage::<components::Position>();
    let renderables = ecs.read_storage::<components::Renderable>();
    
    for (renderable, pos) in (&renderables, &positions).join() {
        let src = Rect::new(0,0,renderable.i_w,renderable.i_h);
        let x: i32 = pos.x as i32;
        let y: i32 = pos.y as i32;
        let dest = Rect::new(x - ((renderable.o_w/2) as i32),y - ((renderable.o_h/2) as i32),renderable.o_w,renderable.o_h);
        
        let center = Point::new( (renderable.o_w/2) as i32, (renderable.o_h/2) as i32);
        let texture = texture_manager.load(&renderable.tex_name)?;
        canvas.copy_ex(
            &texture, 
            src, //source rect 
            dest, // dest rect
            renderable.rot, // angle
            center,  // center
            false, // flip horizontal
            false // flip vertical
        )?;
    }
    
    canvas.present();
    Ok(())
}

struct State { ecs: World }

fn main() -> Result<(), String> {
    println!("Starting Rusteroids");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Rusteroids", GAME_WIDTH, GAME_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let mut tex_man = texture_manager::TextureManager::new(&texture_creator);
    
    // Load the images before the main loop so we don't try and load during gameplay
    tex_man.load("img/space_ship.png")?;
    tex_man.load("img/asteroid.png")?;

    // Prepare fonts
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    let font_path: &Path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
 
    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String, bool> = HashMap::new();
    
    let mut gs = State{
        ecs: World::new()
    };
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Asteroid>();
    
    let mut dispatcher = DispatcherBuilder::new()
                        .with(asteroid::AsteroidMover, "asteroid_mover", &[])
                        .build();

    game::load_world(&mut gs.ecs);

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
        
        game::update(&mut gs.ecs, &mut key_manager);
        dispatcher.dispatch(&gs.ecs);
        render(&mut canvas, &mut tex_man, &texture_creator, &font, &gs.ecs)?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
