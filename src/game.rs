use specs::{World, WorldExt, Builder, Join};
use std::collections::HashMap;
use vector2d::Vector2D; 
use rand::Rng; 

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
    let mut number_asteroids: u32 = 0;
    {
        let asteroids = ecs.read_storage::<components::Asteroid>();
        if asteroids.join().count() < 1 {
            must_create_asteroid = true;

            let mut gamedatas = ecs.write_storage::<components::GameData>();
            for mut gamedata in (&mut gamedatas).join() {
                gamedata.level += 1;
                number_asteroids = (gamedata.level / 3) + 1;
            }
        }
    }
    
    if must_create_asteroid {        
        let mut asteroid_count: u32 = 0;
        while asteroid_count < number_asteroids {
            let mut rng = rand::thread_rng();
            let next_x = rng.gen_range(50.0..(crate::GAME_WIDTH as f64 - 50.0) );
            let next_y = rng.gen_range(50.0..(crate::GAME_HEIGHT as f64 - 50.0) );
            let next_rot = rng.gen_range(0.0..360.0);

            let diff_x = (current_player_position.x - next_x).abs();
            let diff_y = (current_player_position.y - next_y).abs();
            if ((diff_x*diff_x) + (diff_y*diff_y)).sqrt() < 150.0 {
                // We are too close to the player
                continue;
            }
            asteroid_count += 1;
            let new_asteroid = components::Position {
                x: next_x,
                y: next_y,
                rot: next_rot   
            };    
            create_asteroid(ecs,new_asteroid, 100);
        }
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
        .with(components::GameData{score: 0, level: 1})
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

const MAX_MISSILES: usize = 5;

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