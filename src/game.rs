use specs::{World, WorldExt, Builder};
use specs::Join;
use std::collections::HashMap;

const ROTATION_SPEED: f64 = 1.5;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    let mut positions = ecs.write_storage::<crate::components::Position>();
    let players = ecs.read_storage::<crate::components::Player>();
    
    for (_, pos) in (&players, &mut positions).join() {
        if crate::utils::is_key_pressed(&key_manager, "D") {
            pos.rot += ROTATION_SPEED;
        }
        if crate::utils::is_key_pressed(&key_manager, "A") {
            pos.rot -= ROTATION_SPEED;
        }
        if pos.rot > 360.0 {
            pos.rot -= 360.0;
        }
        if pos.rot < 0.0 {
            pos.rot += 360.0;
        }
    }
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
        .with(crate::components::Player {})
        .build();
}
