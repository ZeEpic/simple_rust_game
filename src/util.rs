use bevy::prelude::*;

use rand::Rng;

pub fn color(code: &str) -> Color {
    Color::hex(code).expect("Color was not found.")
}

pub fn rand_range(from: f32, till: f32) -> f32 {
    rand::thread_rng().gen_range(from..till)
}
