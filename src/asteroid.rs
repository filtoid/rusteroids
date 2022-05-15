use specs::{System, WriteStorage, Join};

pub struct AsteroidMover;

use crate::components;

impl<'a> System<'a> for AsteroidMover {
    type SystemData = (
        WriteStorage<'a, components::Position>, 
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Asteroid>
    );
    
    fn run(&mut self, mut data: Self::SystemData) {
        for (pos, rend, asteroid) in (&mut data.0, &mut data.1, &data.2).join() {
            let radians = pos.rot.to_radians();

            pos.x += asteroid.speed * radians.sin();
            pos.y -= asteroid.speed * radians.cos();
            
            let half_width = (rend.o_w/2) as u32;
            let half_height = (rend.o_h/2) as u32;

            if pos.x > (crate::GAME_WIDTH - half_width).into() 
                || pos.x < half_width.into() {
                    pos.rot = 360.0 - pos.rot;
            } else if pos.y > (crate::GAME_HEIGHT - half_height).into()
                || pos.y < half_height.into() {
                    if pos.rot > 180.0 {
                        pos.rot = 540.0 - pos.rot;
                    } else {
                        pos.rot = 180.0 - pos.rot;
                    }
            }
        
            rend.rot += asteroid.rot_speed;
            if rend.rot > 360.0 {
                rend.rot -= 360.0;
            }
            if rend.rot < 0.0 {
                rend.rot += 360.0;
            }
        }
    }
}
