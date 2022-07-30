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
        WriteStorage<'a, components::GameData>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, rends, missiles, asteroids, _players, _, entities) = &data;
        let mut asteroid_creation = Vec::<components::PendingAsteroid>::new();        
        let mut score: u32 = 0;

        for (missile_pos, _, _, missile_entity) in (positions, rends, missiles, entities).join() {
            for (asteroid_pos, asteroid_rend, _, asteroid_entity) in (positions, rends, asteroids, entities).join() {
                let diff_x: f64 = (missile_pos.x - asteroid_pos.x).abs();
                let diff_y: f64 = (missile_pos.y - asteroid_pos.y).abs();
                let hyp :f64 = ((diff_x*diff_x) + (diff_y*diff_y)).sqrt();
                if hyp < asteroid_rend.o_w as f64/2.0 { 
                    score += 10;
                    entities.delete(missile_entity).ok();
                    entities.delete(asteroid_entity).ok();
                    let new_size = asteroid_rend.o_w/2;
                    if new_size >= 25 {
                        asteroid_creation.push(components::PendingAsteroid{x: asteroid_pos.x, y: asteroid_pos.y, rot: asteroid_pos.rot - 90.0, size: new_size});
                        asteroid_creation.push(components::PendingAsteroid{x: asteroid_pos.x, y: asteroid_pos.y, rot: asteroid_pos.rot + 90.0, size: new_size});
                    }
                }
            }
        }
    
        
        let (mut positions, mut rends, _, mut asteroids, _, _, entities) = data;
        for new_asteroid in asteroid_creation {
            let new_ast = entities.create();
            positions.insert(new_ast,components::Position{x: new_asteroid.x, y: new_asteroid.y, rot: new_asteroid.rot}).ok();
            asteroids.insert(new_ast, components::Asteroid{speed: 2.5, rot_speed: 0.5}).ok();
            rends.insert(new_ast, components::Renderable{
                tex_name: String::from("img/asteroid.png"),
                i_w: 100,
                i_h: 100,
                o_w: new_asteroid.size,
                o_h: new_asteroid.size,
                frame: 0,
                total_frames: 1,
                rot: 0.0
            }).ok();
        }

        let (_, _, _, _, _, mut gamedatas, _) = data;
        for mut gamedata in (&mut gamedatas).join() {
            gamedata.score += score;
            let mut gamestate = crate::GAMESTATE.lock().unwrap();
            if gamedata.score > gamestate.high_score {
                gamestate.high_score = gamedata.score;
            }
        }
    }
}