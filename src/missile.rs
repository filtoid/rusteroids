use specs::prelude::*;
use specs::{Entities,Join};

use crate::components;

pub struct MissileMover;

impl<'a> System<'a> for MissileMover {
    type SystemData = (
        WriteStorage<'a, components::Position>, 
        WriteStorage<'a, components::Renderable>,
        ReadStorage<'a, components::Missile>,
        Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut renderables, missiles, entities) = data;
        
        for (pos, rend, missile, entity) in (&mut positions, &mut renderables, &missiles, &entities).join() {
            let radian = pos.rot.to_radians();
    
            let move_x = missile.speed * radian.sin();
            let move_y = missile.speed * radian.cos();
            pos.x += move_x;
            pos.y -= move_y;
            if pos.x > crate::GAME_WIDTH.into() || pos.x < 0.0 || pos.y > crate::GAME_HEIGHT.into() || pos.y < 0.0 {
                entities.delete(entity).ok();
            } 
            
            rend.rot = pos.rot;
        }       
    }
}

