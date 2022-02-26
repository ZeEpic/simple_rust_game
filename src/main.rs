mod util;

use std::ops::Sub;
use crate::util::*;
use bevy::prelude::*;

pub struct GamePlugin;

struct Textures {
    circle: Handle<Image>,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Circle Clicker Game".to_string(),
            width: 1000.0,
            height: 600.0,
            ..Default::default()
        })
        .add_startup_system(setup_resources)
        .add_startup_system(setup_circles)
        .add_system(mouse_click_system)
        .add_system(shrink_system)
        .insert_resource(ClearColor(color("5c8bd6")));
    }
}

#[derive(Component)]
struct Shrinking;

fn shrink_system(mut commands: Commands, mut circles: Query<(Entity, &mut Transform, &Shrinking)>) {
    for (entity, mut trans, _) in circles.iter_mut() {
        trans.scale *= 0.99;
        if trans.scale.x < 0.25 {
            commands.entity(entity).despawn()
        }
    }
}

fn spawn_circle(mut commands: Commands, texture: Handle<Image>, windows: Res<Windows>) {
    let scale_amount = rand_range(0.75, 3.0);
    let scale = Vec3::new(scale_amount, scale_amount, scale_amount);
    let window = windows.get_primary().unwrap();
    let w = window.width();
    let h = window.height();
    let translation = Vec3::new(rand_range(-(w / 2.0 - 100.0), w / 2.0 - 100.0), rand_range(-(h / 2.0 - 100.0), h / 2.0 - 100.0), 1.0);


    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(translation).with_scale(scale),
            texture: texture.clone(),
            ..Default::default()
        })
        .insert(Shrinking);

    if rand_range(0.0, 15.0) < 1.0 {
        spawn_circle(commands, texture, windows)
    }
}

fn setup_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add resources
    commands.insert_resource(Textures {
        circle: asset_server.load("circle.png")
    });
}

fn setup_circles(commands: Commands,
                 asset_server: Res<AssetServer>,
                 windows: Res<Windows>) {
    spawn_circle(commands, asset_server.load("circle.png"), windows);
}

fn find_circles_near(circles: Query<(Entity, &Transform, &Shrinking)>, point: Vec2) -> Vec<Entity> {
    println!("-> {:?}", point);
    circles
        .iter()
        .filter(|(_, transform, _)| {
            transform.translation.truncate().distance(point) < (transform.scale.x * 100.0)
        })
        .map(|(entity, _, _)| entity)
        .collect::<Vec<Entity>>()
}

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    circles: Query<(Entity, &Transform, &Shrinking)>,
    windows: Res<Windows>,
    textures: Res<Textures>,
    mut commands: Commands,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().expect("No window was found.");
        let cursor = window.cursor_position().expect("Cursor position not found.");
        let w = window.width();
        let h = window.height();
        let window_offset = Vec2::new(w / 2.0, h / 2.0);
        let result = find_circles_near(circles, cursor.sub(window_offset));
        match result.get(0) {
            Some(thing) => {
                commands.entity(*thing).despawn();
                spawn_circle(commands, textures.circle.clone(), windows);
            }
            None => {}
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
