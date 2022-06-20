use specs::{World, WorldExt, Builder, Join};
use std::collections::HashMap;
use vector2d::Vector2D; 

use crate::components;
use crate::utils;

const ROTATION_SPEED: f64 = 1.5;
const PLAYER_SPEED: f64 = 4.5;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    // Check status of game world
    let mut must_reload_world = false;     
    let mut current_player_position = components::Position{x: 0.0, y: 0.0, rot: 0.0};
    {
        let players = ecs.read_storage::<components::Player>();
        let positions = ecs.read_storage::<components::Position>();

        for (pos, _player) in (&positions,&players).join() {
            current_player_position.x = pos.x;
            current_player_position.y = pos.y;
        }

        if players.join().count() < 1 {
            must_reload_world = true;
        }
    }

    if must_reload_world {
        // Remove all of the previous entities so we can start again
        ecs.delete_all();
        // Reset the world to first state
        load_world(ecs);
    }

    
    // Check if all asteroids are missing
    let mut must_create_asteroid = false;
    {
        let asteroids = ecs.read_storage::<components::Asteroid>();
        if asteroids.join().count() < 1 {
            must_create_asteroid = true;
        }
    }
    if must_create_asteroid {
        if current_player_position.x > (crate::GAME_WIDTH/2).into() && current_player_position.y < (crate::GAME_HEIGHT/2).into() {
            // Player top right 
            current_player_position.x = crate::GAME_WIDTH as f64/4.0;
            current_player_position.y =  crate::GAME_HEIGHT as f64 - (crate::GAME_HEIGHT as f64 /4.0);
            current_player_position.rot = 225.0;
        } else if current_player_position.x < (crate::GAME_WIDTH/2).into() && current_player_position.y < (crate::GAME_HEIGHT/2).into() {
            // Player top left 
            current_player_position.x =  crate::GAME_WIDTH as f64 - (crate::GAME_WIDTH as f64/4.0);
            current_player_position.y =  crate::GAME_HEIGHT as f64 - (crate::GAME_HEIGHT as f64/4.0);
            current_player_position.rot = 135.0;
        } else if current_player_position.x > (crate::GAME_WIDTH/2).into() && current_player_position.y > (crate::GAME_HEIGHT/2).into() {
            // Player bottom right 
            current_player_position.x =  crate::GAME_WIDTH as f64/4.0;
            current_player_position.y =  crate::GAME_HEIGHT as f64/4.0;
            current_player_position.rot = 315.0;
        } else if current_player_position.x < (crate::GAME_WIDTH/2).into() && current_player_position.y > (crate::GAME_HEIGHT/2).into() {
            // Player bottom left 
            current_player_position.x =  crate::GAME_WIDTH as f64 - (crate::GAME_WIDTH as f64/4.0);
            current_player_position.y =  crate::GAME_HEIGHT as f64/4.0;
            current_player_position.rot = 45.0;
        }
        create_asteroid(ecs,current_player_position, 100);
    }

    let mut player_pos = components::Position{x: 0.0, y: 0.0, rot: 0.0};
    let mut must_fire_missile = false;
        
    {
        let mut positions = ecs.write_storage::<components::Position>();
        let mut players = ecs.write_storage::<components::Player>();
        let mut renderables = ecs.write_storage::<components::Renderable>();
        
        for (player, pos, renderable) in (&mut players, &mut positions, &mut renderables).join() {
            if crate::utils::is_key_pressed(&key_manager, "D") {
                pos.rot += ROTATION_SPEED;
            }
            if crate::utils::is_key_pressed(&key_manager, "A") {
                pos.rot -= ROTATION_SPEED;
            }
            update_movement(pos, player);
            if crate::utils::is_key_pressed(&key_manager, "W") {
                let radians = pos.rot.to_radians();

                let move_x = PLAYER_SPEED * radians.sin();
                let move_y = PLAYER_SPEED * radians.cos();
                let move_vec = Vector2D::<f64>::new(move_x, move_y);

                player.impulse += move_vec;
            }

            if pos.rot > 360.0 {
                pos.rot -= 360.0;
            }
            if pos.rot < 0.0 {
                pos.rot += 360.0;
            }

            if pos.x > crate::GAME_WIDTH.into() {
                pos.x -= crate::GAME_WIDTH as f64;
            }
            if pos.x < 0.0 {
                pos.x += crate::GAME_WIDTH as f64;
            }
            if pos.y > crate::GAME_HEIGHT.into() {
                pos.y -= crate::GAME_HEIGHT as f64;
            }
            if pos.y < 0.0 {
                pos.y += crate::GAME_HEIGHT as f64;
            }

            if utils::is_key_pressed(&key_manager, " ") {
                utils::key_up(key_manager, " ".to_string());
                must_fire_missile = true;
                player_pos.x = pos.x;
                player_pos.y = pos.y;
                player_pos.rot = pos.rot;
            }

            // Update the graphic to reflect the rotation
            renderable.rot = pos.rot;
        }
    }

    if must_fire_missile {
        fire_missile(ecs, player_pos);
    }
}

const MAX_SPEED: f64 = 3.5;
const FRICTION: f64 = 0.99;

pub fn update_movement(pos: &mut components::Position, player: &mut components::Player) {
    player.cur_speed *= FRICTION;

    player.cur_speed += player.impulse;
    if player.cur_speed.length() > MAX_SPEED {
        player.cur_speed = player.cur_speed.normalise();
        player.cur_speed = player.cur_speed * MAX_SPEED;
    }

    pos.x += player.cur_speed.x;
    pos.y -= player.cur_speed.y;

    player.impulse = vector2d::Vector2D::new(0.0,0.0);
}

pub fn load_world(ecs: &mut World) {
    ecs.create_entity()
        .with(components::Position{ x:350.0, y:250.0, rot: 0.0})
        .with(components::Renderable{
            tex_name: String::from("img/space_ship.png"),
            i_w: 100,
            i_h: 100,
            o_w: 50,
            o_h: 50,
            frame: 0,
            total_frames: 1,
            rot: 0.0
        })
        .with(components::Player {
            impulse: vector2d::Vector2D::new(0.0,0.0),
            cur_speed: vector2d::Vector2D::new(0.0,0.0)
        })
        .build();
    
    create_asteroid(ecs, components::Position{ x: 400.0, y: 235.0, rot: 45.0}, 50);

    ecs.create_entity()
        .with(components::GameData{score: 0})
        .build();
}

pub fn create_asteroid(ecs: &mut World, position: components::Position, asteroid_size: u32){
    ecs.create_entity()
    .with(position)
    .with(components::Renderable{
        tex_name: String::from("img/asteroid.png"),
        i_w: 100,
        i_h: 100,
        o_w: asteroid_size,
        o_h: asteroid_size,
        frame: 0,
        total_frames: 1,
        rot: 0.0
    })
    .with(crate::components::Asteroid {
        speed: 2.5,
        rot_speed: 0.5
    })
    .build();
}

const MAX_MISSILES: usize = 3;

fn fire_missile(ecs: &mut World, position: components::Position) {
    {
        let missiles = ecs.read_storage::<components::Missile>();
        if missiles.count() > MAX_MISSILES - 1 {
            return;
        }
    }
    ecs.create_entity()
        .with(position)
        .with(components::Renderable{
            tex_name: String::from("img/missile.png"),
            i_w: 50,
            i_h: 100,
            o_w: 10,
            o_h: 20,
            frame: 0,
            total_frames: 1,
            rot: 0.0
        })
        .with(components::Missile {
            speed: 5.0
        })
        .build();
}