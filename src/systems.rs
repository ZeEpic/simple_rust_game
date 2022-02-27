use bevy::app::AppExit;
use crate::util::*;
use bevy::prelude::*;
use crate::GameState::{Loading, Play};
use crate::{Clickable, find_circles_near, Fonts, Game, GameState, HOVERED_BUTTON, MainMenuTimer, Menu, NORMAL_BUTTON, Shrinking, Temporary, Textures};

pub fn shrink_system(mut commands: Commands,
                 mut exit: EventWriter<AppExit>,
                 mut circles: Query<(Entity, &mut Transform, &Shrinking)>,
                 timer: Res<MainMenuTimer>,
                 mut score: ResMut<Game>,
                 mut fonts: Res<Fonts>,
                 mut windows: Res<Windows>,
                 game_state: Res<State<GameState>>,) {
    let len = circles.iter().count();
    if *game_state.current() == Play && (len <= 0) && timer.0.just_finished() {
        // exit.send(AppExit)
        println!("{}", score.score)
    }
    for (entity, mut trans, _) in circles.iter_mut() {
        trans.scale *= 0.99;
        if trans.scale.x < 0.25 {
            commands.entity(entity).despawn();
            if *game_state.current() == Play {
                score.score -= 1;
                spawn_score_notification(&mut commands, &mut fonts, &mut windows, "-1", false, trans.translation.x, trans.translation.y);
            }
        }
    }
}

pub fn loading_system(mut commands: Commands, mut game_state: ResMut<State<GameState>>, query: Query<Entity, With<Menu>>) {
    if *game_state.current() != Loading { return }
    let len = query.iter().count();
    if len == 0 {
        game_state.set(Play).unwrap();
    } else {
        query.for_each(|entity| commands.entity(entity).despawn());
    }
}

pub fn setup_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Add resources
    commands.insert_resource(Textures {
        circle: asset_server.load("circle.png")
    });
    commands.insert_resource(Game { score: 0 });
    commands.insert_resource(Fonts {
        score_notification: asset_server.load("font\\Roboto-Bold.ttf")
    });

    // Create transparent background rect
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.1, 0.1, 0.1, 0.4),
            custom_size: Some(Vec2::new(2000.0, 2000.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..Default::default()
    }).insert(Menu);

    // Create menu
    add_button(commands,
               Size::new(Val::Px(150.0), Val::Px(65.0)),
               NORMAL_BUTTON,
               "Play",
               TextStyle {
                   font: asset_server.load("font\\Roboto-Medium.ttf"),
                   font_size: 40.0,
                   color: color("ffe9ba"),
               }
    )

}

pub fn main_menu_circles_system(mut commands: Commands,
                            mut windows: Res<Windows>,
                            mut timer: ResMut<MainMenuTimer>,
                            textures: Res<Textures>,
                            circles: Query<&Shrinking>,
                            time: Res<Time>,
                            game_state: ResMut<State<GameState>>,) {
    if circles.iter().count() > 30 { return }
    if timer.0.tick(time.delta()).just_finished() {
        spawn_circle(&mut commands, textures.circle.clone(), &mut windows, *game_state.current() == Play);
    }
}

pub fn temporary_system(time: Res<Time>, mut query: Query<&mut Temporary>) {
    for mut temporary in query.iter_mut() {
        if temporary.timer.tick(time.delta()).just_finished() {
            temporary.alive = false
        }
    }
}

pub fn temporary_un_alive_system(mut commands: Commands, mut query: Query<(Entity, &Temporary, &mut Text)>) {
    for (entity, temporary, mut text) in query.iter_mut() {
        let a = text.sections[0].style.color.a();
        text.sections[0].style.color.set_a(a - 0.01);
        if !temporary.alive {
            commands.entity(entity).despawn();
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor,),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
    mut timer: ResMut<MainMenuTimer>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                timer.0.set_repeating(false);
                timer.0.reset();
                game_state.set(GameState::Loading).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    circles: Query<(Entity, &Transform), (With<Shrinking>, With<Clickable>)>,
    mut windows: Res<Windows>,
    textures: Res<Textures>,
    mut commands: Commands,
    mut score: ResMut<Game>,
    mut fonts: Res<Fonts>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.current() != GameState::Play { return }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().expect("No window was found.");
        let cursor = real_mouse_pos(window);
        let result = find_circles_near(circles, cursor);
        match result.get(0) {
            Some(thing) => {
                commands.entity(*thing).despawn();
                spawn_circle(&mut commands, textures.circle.clone(), &mut windows, true);
                println!("Score increased!");
                score.score += 1;
                spawn_score_notification(&mut commands, &mut fonts, &mut windows, "+1", true, cursor.x, cursor.y);
            }
            None => {
                score.score -= 1;
                spawn_score_notification(&mut commands, &mut fonts, &mut windows, "-1", false, cursor.x, cursor.y);
            }
        }
    }
}
