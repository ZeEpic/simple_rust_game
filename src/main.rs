mod systems;
mod util;

use crate::systems::*;
use crate::util::*;
use bevy::prelude::*;

pub struct GamePlugin;

pub struct Textures {
    circle: Handle<Image>,
}

pub struct Fonts {
    score_notification: Handle<Font>,
    game_over: Handle<Font>,
}

pub struct Game {
    score: i32,
    difficulty: Difficulty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Easy(f32),
    Hard(f32),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Loading,
    Play,
    GameOver,
}

#[derive(Component)]
pub struct Shrinking;

#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct Temporary {
    timer: Timer,
    alive: bool,
}

pub struct MainMenuTimer(Timer);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

fn find_circles_near(
    circles: Query<(Entity, &Transform), (With<Shrinking>, With<Clickable>)>,
    point: Vec2,
    margin_of_error: f32,
) -> Vec<Entity> {
    circles
        .iter()
        .filter(|(_, transform)| {
            transform.translation.truncate().distance(point) < (transform.scale.x * margin_of_error)
        })
        .map(|(entity, _)| entity)
        .collect::<Vec<Entity>>()
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources)
            .add_system(main_menu_circles_system)
            .add_system(game_over_system)
            .add_system(mouse_click_system)
            .add_system(shrink_system)
            .add_system(button_system)
            .add_system(loading_system)
            .add_system(temporary_system)
            .add_system(temporary_un_alive_system)
            .add_state(GameState::MainMenu)
            .insert_resource(ClearColor(color("5c8bd6")))
            .insert_resource(MainMenuTimer(Timer::from_seconds(0.4, true)));
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Circle Clicker Game".to_string(),
            width: 1280.0,
            height: 720.0,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
