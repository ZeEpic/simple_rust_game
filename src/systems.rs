use crate::util::*;
use crate::Difficulty::{Easy, Hard};
use crate::GameState::{GameOver, Loading, Play};
use crate::{
    find_circles_near, Clickable, Difficulty, Fonts, Game, GameState, GameTimer, Menu, Shrinking,
    Temporary, Textures, HOVERED_BUTTON, NORMAL_BUTTON,
};
use bevy::prelude::*;
use std::time::Duration;

pub fn game_over_system(
    mut commands: Commands,
    circles: Query<(Entity, &mut Transform, &Shrinking)>,
    timer: ResMut<GameTimer>,
    score: ResMut<Game>,
    mut fonts: Res<Fonts>,
    mut game_state: ResMut<State<GameState>>,
) {
    let len = circles.iter().count();
    if *game_state.current() == Play && (len <= 0) && timer.0.finished() {
        let button = get_button(
            "Restart",
            &TextStyle {
                font: fonts.game_over.clone(),
                font_size: 40.0,
                color: color("ffe9ba"),
            },
            150.0,
            65.0,
        );
        let score = &format!("Score: {}", score.score.to_string());
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: color("4a6fab").into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(get_game_over_text(&mut fonts, 104.0, "Game Over!"));
                parent.spawn_bundle(get_game_over_text(&mut fonts, 56.0, score));
                parent
                    .spawn_bundle(button.0)
                    .with_children(|button_parent| {
                        button_parent.spawn_bundle(button.1);
                    });
            });
        game_state.set(GameOver).unwrap();
        return;
    }
}

pub fn shrink_system(
    mut commands: Commands,
    mut circles: Query<(Entity, &mut Transform, &Shrinking)>,
    mut score: ResMut<Game>,
    mut fonts: Res<Fonts>,
    mut windows: Res<Windows>,
    game_state: ResMut<State<GameState>>,
) {
    for (entity, mut trans, _) in circles.iter_mut() {
        trans.scale *= 0.99;
        if trans.scale.x < 0.25 {
            commands.entity(entity).despawn();
            if *game_state.current() == Play {
                score.score -= 1;
                spawn_score_notification(
                    &mut commands,
                    &mut fonts,
                    &mut windows,
                    "-1",
                    false,
                    trans.translation.x,
                    trans.translation.y,
                );
            }
        }
    }
}

pub fn loading_system(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    textures: Res<Textures>,
    mut windows: Res<Windows>,
    query: Query<Entity, With<Menu>>,
    mut timer: ResMut<GameTimer>,
    game: ResMut<Game>,
) {
    if *game_state.current() != Loading {
        return;
    }
    let len = query.iter().count();
    if len == 0 {
        game_state.set(Play).unwrap();
        // timer
        //     .0
        //     .set_duration(Duration::from_secs(match game.difficulty {
        //         Easy(time) => time,
        //         Hard(time) => time,
        //     } as u64));
        timer.0.reset();
        spawn_circle(&mut commands, textures.circle.clone(), &mut windows, true);
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
        circle: asset_server.load("circle.png"),
    });
    commands.insert_resource(Game {
        score: 0,
        difficulty: Easy(30.0),
    });
    commands.insert_resource(Fonts {
        score_notification: asset_server.load("font\\Roboto-Bold.ttf"),
        game_over: asset_server.load("font\\Roboto-BlackItalic.ttf"),
    });

    // Create transparent background rect
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.1, 0.1, 0.4),
                custom_size: Some(Vec2::new(2000.0, 2000.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        })
        .insert(Menu);

    // Create menu
    let font = asset_server.load("font\\Roboto-Medium.ttf");
    let style = TextStyle {
        font,
        font_size: 40.0,
        color: color("ffe9ba"),
    };
    let buttons = vec![
        get_button("30 Seconds Easy", &style, 400.0, 65.0),
        get_button("15 Seconds Easy", &style, 400.0, 65.0),
        get_button("30 Seconds Hard", &style, 400.0, 65.0),
        get_button("15 Seconds Hard", &style, 400.0, 65.0),
    ];
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0., 0., 0., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            for button in buttons.iter() {
                parent
                    .spawn_bundle(button.clone().0)
                    .with_children(|button_parent| {
                        button_parent.spawn_bundle(button.clone().1).insert(Menu);
                    })
                    .insert(Menu);
            }
        })
        .insert(Menu);
}

pub fn main_menu_circles_system(
    mut commands: Commands,
    mut windows: Res<Windows>,
    mut timer: ResMut<GameTimer>,
    textures: Res<Textures>,
    circles: Query<&Shrinking>,
    time: Res<Time>,
    game_state: ResMut<State<GameState>>,
) {
    if circles.iter().count() > 30 {
        return;
    }
    if timer.0.tick(time.delta()).just_finished() && *game_state.current() != Play {
        spawn_circle(&mut commands, textures.circle.clone(), &mut windows, false);
    }
}

pub fn temporary_system(time: Res<Time>, mut query: Query<&mut Temporary>) {
    for mut temporary in query.iter_mut() {
        if temporary.timer.tick(time.delta()).just_finished() {
            temporary.alive = false
        }
    }
}

pub fn temporary_un_alive_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Temporary, &mut Text)>,
) {
    for (entity, temporary, mut text) in query.iter_mut() {
        let a = text.sections[0].style.color.a();
        text.sections[0].style.color.set_a(a - 0.01);
        if !temporary.alive {
            commands.entity(entity).despawn();
        }
    }
}

fn load_game(
    game: &mut ResMut<Game>,
    game_state: &mut ResMut<State<GameState>>,
    timer: &mut ResMut<GameTimer>,
    difficulty: Difficulty,
) {
    timer.0.set_repeating(false);
    timer.0.reset();
    game.difficulty = difficulty;
    game_state.set(GameState::Loading).unwrap();
}

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
    everything: Query<Entity>,
    mut game: ResMut<Game>,
    mut game_state: ResMut<State<GameState>>,
    mut timer: ResMut<GameTimer>,
    asset_server: Res<AssetServer>,
) {
    let (interaction, mut color, children) = match interaction_query.get_single_mut() {
        Ok(thing) => thing,
        Err(_) => return
    };
    let _ = match *interaction {
        Interaction::Clicked => {
            let text = text_query.get(children[0]).unwrap();
            match &*text.sections[0].value {
                "15 Seconds Easy" => load_game(
                    &mut game,
                    &mut game_state,
                    &mut timer,
                    Difficulty::Easy(15.0),
                ),
                "30 Seconds Easy" => load_game(
                    &mut game,
                    &mut game_state,
                    &mut timer,
                    Difficulty::Easy(30.0),
                ),
                "15 Seconds Hard" => load_game(
                    &mut game,
                    &mut game_state,
                    &mut timer,
                    Difficulty::Hard(15.0),
                ),
                "30 Seconds Hard" => load_game(
                    &mut game,
                    &mut game_state,
                    &mut timer,
                    Difficulty::Hard(30.0),
                ),
                "Restart" => {
                    game_state.set(GameState::MainMenu).unwrap();
                    everything
                        .iter()
                        .for_each(|entity| commands.entity(entity).despawn());
                    timer.0.reset();
                    timer.0.set_repeating(true);
                    timer.0.set_duration(Duration::from_secs_f32(0.4));
                    setup_resources(commands, asset_server);
                }
                _ => {}
            };
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into();
        }
    };
}

pub fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    circles: Query<(Entity, &Transform), (With<Shrinking>, With<Clickable>)>,
    mut windows: Res<Windows>,
    textures: Res<Textures>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut fonts: Res<Fonts>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.current() != GameState::Play {
        return;
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().expect("No window was found.");
        let cursor = real_mouse_pos(window);
        let error = match game.difficulty.clone() {
            Difficulty::Easy(_) => 1.0,
            Difficulty::Hard(_) => 0.5,
        };
        let result = find_circles_near(circles, cursor, error);
        match result.get(0) {
            Some(thing) => {
                commands.entity(*thing).despawn();
                spawn_circle(&mut commands, textures.circle.clone(), &mut windows, true);
                game.score += 1;
                spawn_score_notification(
                    &mut commands,
                    &mut fonts,
                    &mut windows,
                    "+1",
                    true,
                    cursor.x,
                    cursor.y,
                );
            }
            None => {
                game.score -= 1;
                spawn_score_notification(
                    &mut commands,
                    &mut fonts,
                    &mut windows,
                    "-1",
                    false,
                    cursor.x,
                    cursor.y,
                );
            }
        }
    }
}
