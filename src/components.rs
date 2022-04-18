use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub rot: f64
}

/// A renderable item and details about image
#[derive(Component)]
pub struct Renderable {
    /// The name of the texture to be rendered
    pub tex_name: String,
    // Width of src
    pub i_w: u32,
    // Height of src
    pub i_h: u32,
    // Width of dest
    pub o_w: u32,
    // Height of dest
    pub o_h: u32,
    // Offset number of widths to crop
    pub frame: u32,
    // Max frame offset before 
    pub total_frames: u32,
    // Rotation of imagae to display
    pub rot: f64
}

#[derive(Component)]
pub struct Player {}