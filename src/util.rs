use bevy::prelude::*;
use std::ops::Sub;

use crate::{Clickable, Fonts, Menu, Shrinking, Temporary};
use rand::Rng;

pub fn color(code: &str) -> Color {
    Color::hex(code).expect("Color was not found.")
}

pub fn rand_range(from: f32, till: f32) -> f32 {
    rand::thread_rng().gen_range(from..till)
}

pub fn real_mouse_pos(window: &Window) -> Vec2 {
    let cursor = window
        .cursor_position()
        .expect("Cursor position not found.");
    let w = window.width();
    let h = window.height();
    let offset = Vec2::new(w / 2.0, h / 2.0);
    cursor.sub(offset)
}

pub fn get_button(
    text: &str,
    style: &TextStyle,
    width: f32,
    height: f32,
) -> (ButtonBundle, TextBundle) {
    (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(height)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: Rect {
                    left: Val::Percent(1.0),
                    right: Val::Percent(1.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                },
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        },
        TextBundle {
            text: Text::with_section(text, style.clone(), Default::default()),
            ..Default::default()
        },
    )
}

pub fn get_text(value: &str, x: f32, y: f32, style: TextStyle) -> TextBundle {
    TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(x),
                bottom: Val::Px(y),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            value,
            style,
            TextAlignment {
                horizontal: HorizontalAlign::Left,
                ..Default::default()
            },
        ),
        ..Default::default()
    }
}

pub fn get_game_over_text(fonts: &mut Res<Fonts>, font_size: f32, value: &str) -> TextBundle {
    TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Relative,
            ..Default::default()
        },
        text: Text::with_section(
            value,
            TextStyle {
                font: fonts.game_over.clone(),
                font_size,
                color: Color::BLACK,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    }
}

pub fn spawn_score_notification(
    commands: &mut Commands,
    fonts: &mut Res<Fonts>,
    windows: &mut Res<Windows>,
    amount: &str,
    positive: bool,
    x: f32,
    y: f32,
) {
    let window = windows.get_primary().unwrap();
    let w = window.width();
    let h = window.height();
    let offset_x = x + w / 2.0;
    let offset_y = y + h / 2.0;
    commands
        .spawn_bundle(get_text(
            amount,
            offset_x,
            offset_y,
            TextStyle {
                font: fonts.score_notification.clone(),
                font_size: 24.0,
                color: color(if positive { "5bd972" } else { "ff2414" }),
            },
        ))
        .insert(Temporary {
            timer: Timer::from_seconds(2.0, false),
            alive: true,
        });
}

pub fn spawn_circle(
    commands: &mut Commands,
    texture: Handle<Image>,
    windows: &mut Res<Windows>,
    clickable: bool,
) {
    let scale_amount = rand_range(0.75, 3.0);
    let scale = Vec3::new(scale_amount, scale_amount, scale_amount);
    let window = windows.get_primary().unwrap();
    let w = window.width();
    let h = window.height();
    let translation = Vec3::new(
        rand_range(-(w / 2.0 - 100.0), w / 2.0 - 100.0),
        rand_range(-(h / 2.0 - 100.0), h / 2.0 - 100.0),
        1.0,
    );
    let transform = Transform::from_translation(translation).with_scale(scale);

    if clickable {
        commands
            .spawn_bundle(SpriteBundle {
                transform,
                texture: texture.clone(),
                ..Default::default()
            })
            .insert(Shrinking)
            .insert(Clickable);
    } else {
        commands
            .spawn_bundle(SpriteBundle {
                transform,
                texture: texture.clone(),
                ..Default::default()
            })
            .insert(Shrinking)
            .insert(Menu);
    }

    if rand_range(0.0, 15.0) < 1.0 {
        spawn_circle(commands, texture, windows, clickable);
    }
}

fn distance_to_point(transform: &Transform, point: Vec2) -> f32 {
    transform.translation.truncate().distance(point)
}

pub fn find_circles_near(
    circles: Query<(Entity, &Transform), (With<Shrinking>, With<Clickable>)>,
    point: Vec2,
    margin_of_error: f32,
) -> Vec<Entity> {
    circles
        .iter()
        .filter(|(_, transform)| {
            distance_to_point(transform, point) < (transform.scale.x * margin_of_error * 100.0)
        })
        .map(|(entity, _)| entity)
        .collect::<Vec<Entity>>()
}
