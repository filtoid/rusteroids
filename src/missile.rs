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

pub struct MissileStriker;

impl<'a> System<'a> for MissileStriker {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Missile>,
        WriteStorage<'a, components::Asteroid>,
        WriteStorage<'a, components::Player>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, rends, missiles, asteroids, _players, entities) = &data;
        
        for (missile_pos, _, _, missile_entity) in (positions, rends, missiles, entities).join() {
            for (asteroid_pos, asteroid_rend, _, asteroid_entity) in (positions, rends, asteroids, entities).join() {
                let diff_x: f64 = (missile_pos.x - asteroid_pos.x).abs();
                let diff_y: f64 = (missile_pos.y - asteroid_pos.y).abs();
                let hyp :f64 = ((diff_x*diff_x) + (diff_y*diff_y)).sqrt();
                if hyp < asteroid_rend.o_w as f64/2.0 { 
                    entities.delete(missile_entity).ok();
                    entities.delete(asteroid_entity).ok();
                }
            }
        }
    }
}