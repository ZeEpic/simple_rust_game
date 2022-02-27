use std::ops::Sub;
use bevy::prelude::*;

use rand::Rng;
use crate::{Clickable, Fonts, Menu, Shrinking, Temporary};

pub fn color(code: &str) -> Color {
    Color::hex(code).expect("Color was not found.")
}

pub fn rand_range(from: f32, till: f32) -> f32 {
    rand::thread_rng().gen_range(from..till)
}

pub fn real_mouse_pos(window: &Window) -> Vec2 {
    let cursor= window.cursor_position().expect("Cursor position not found.");
    let w = window.width();
    let h = window.height();
    let offset = Vec2::new(w / 2.0, h / 2.0);
    cursor.sub(offset)
}

pub fn add_button(mut commands: Commands, size: Size<Val>, color: Color, text: &str, style: TextStyle) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size,
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: color.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(text, style, Default::default(), ),
                ..Default::default()
            }).insert(Menu);
        }).insert(Menu);
}

pub fn get_text(value: &str, x: f32, y: f32, style: TextStyle) -> TextBundle {
    TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(x),
                bottom: Val::Px(y),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            value,
            style,
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    }
}

pub fn spawn_score_notification(commands: &mut Commands, fonts: &mut Res<Fonts>, windows: &mut Res<Windows>, amount: &str, positive: bool, x: f32, y: f32) {
    let window = windows.get_primary().unwrap();
    let w = window.width();
    let h = window.height();
    let offset_x = x + w / 2.0;
    let offset_y = y + h / 2.0;
    commands.spawn_bundle(get_text(amount, offset_x, offset_y, TextStyle {
        font: fonts.score_notification.clone(),
        font_size: 24.0,
        color: color(if positive { "5bd972" } else { "d96a5d" })
    })).insert(Temporary {
        timer: Timer::from_seconds(2.0, false),
        alive: true
    });
}

pub fn spawn_circle(commands: &mut Commands, texture: Handle<Image>, windows: &mut Res<Windows>, clickable: bool) {
    let scale_amount = rand_range(0.75, 3.0);
    let scale = Vec3::new(scale_amount, scale_amount, scale_amount);
    let window = windows.get_primary().unwrap();
    let w = window.width();
    let h = window.height();
    let translation = Vec3::new(rand_range(-(w / 2.0 - 100.0), w / 2.0 - 100.0),
                                rand_range(-(h / 2.0 - 100.0), h / 2.0 - 100.0),
                                1.0);
    let transform = Transform::from_translation(translation).with_scale(scale);

    if clickable {
        commands.spawn_bundle(SpriteBundle { transform, texture: texture.clone(), ..Default::default() })
            .insert(Shrinking)
            .insert(Clickable);
    } else {
        commands.spawn_bundle(SpriteBundle { transform, texture: texture.clone(), ..Default::default() })
            .insert(Shrinking)
            .insert(Menu);
    }

    if rand_range(0.0, 15.0) < 1.0 {
        spawn_circle(commands, texture, windows, clickable);
    }
}
