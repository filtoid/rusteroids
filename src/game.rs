use specs::{World, WorldExt, Builder};
use specs::Join;

use std::collections::HashMap;
use vector2d::Vector2D;

const ROTATION_SPEED: f64 = 1.5;
const PLAYER_SPEED: f64 = 4.5;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    let mut positions = ecs.write_storage::<crate::components::Position>();
    let mut players = ecs.write_storage::<crate::components::Player>();
    
    for (player, pos) in (&mut players, &mut positions).join() {
        if crate::utils::is_key_pressed(&key_manager, "D") {
            pos.rot += ROTATION_SPEED;
        }
        if crate::utils::is_key_pressed(&key_manager, "A") {
            pos.rot -= ROTATION_SPEED;
        }

        // Update the movement then calculate next impulse 
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
    }
}

const MAX_SPEED: f64 = 3.5;
const FRICTION: f64 = 0.99;

pub fn update_movement(pos: &mut crate::components::Position, player: &mut crate::components::Player) {
    player.cur_speed *= FRICTION;

    player.cur_speed += player.impulse;
    if player.cur_speed.length() > MAX_SPEED {
        player.cur_speed = player.cur_speed.normalise();
        player.cur_speed = player.cur_speed * MAX_SPEED;
    }

    // Move the player position entity
    pos.x += player.cur_speed.x;
    pos.y -= player.cur_speed.y;

    // Reset the impulse to 0,0
    player.impulse = vector2d::Vector2D::new(0.0,0.0);
}

pub fn load_world(ecs: &mut World) {
    ecs.create_entity()
        .with(crate::components::Position{ x:350.0, y:250.0, rot: 0.0})
        .with(crate::components::Renderable{
            tex_name: String::from("img/space_ship.png"),
            i_w: 100,
            i_h: 100,
            o_w: 100,
            o_h: 100,
            frame: 0,
            total_frames: 1,
            rot: 0.0
        })
        .with(crate::components::Player {
            impulse: vector2d::Vector2D::new(0.0,0.0),
            cur_speed: vector2d::Vector2D::new(0.0,0.0) 
        })
        .build();
}
