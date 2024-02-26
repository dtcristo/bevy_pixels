use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{WindowResizeConstraints, WindowResolution},
};
use bevy_pixels::prelude::*;
use rand::prelude::*;

const INITIAL_WIDTH: u32 = 320;
const INITIAL_HEIGHT: u32 = 240;
const SCALE_FACTOR: f32 = 2.0;

#[derive(Bundle, Debug)]
struct ObjectBundle {
    position: Position,
    velocity: Velocity,
    size: Size,
    color: Color,
}

#[derive(Component, Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: i16,
    y: i16,
}

#[derive(Component, Debug)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Component, Debug)]
struct Color(u8, u8, u8, u8);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Hello Bevy Pixels".to_string(),
                    resolution: WindowResolution::new(
                        INITIAL_WIDTH as f32 * SCALE_FACTOR,
                        INITIAL_HEIGHT as f32 * SCALE_FACTOR,
                    ),
                    resize_constraints: WindowResizeConstraints {
                        min_width: INITIAL_WIDTH as f32 * SCALE_FACTOR,
                        min_height: INITIAL_HEIGHT as f32 * SCALE_FACTOR,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
            PixelsPlugin {
                primary_window: Some(PixelsOptions {
                    width: INITIAL_WIDTH,
                    height: INITIAL_HEIGHT,
                    scale_factor: SCALE_FACTOR,
                    ..default()
                }),
            },
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (bevy::window::close_on_esc, (bounce, movement).chain()),
        )
        .add_systems(Draw, (draw_background, draw_objects).chain())
        .run();
}

/// Spawn object.
fn setup(mut commands: Commands) {
    let box_object = ObjectBundle {
        position: Position { x: 24, y: 16 },
        velocity: Velocity { x: 1, y: 1 },
        size: Size {
            width: 64,
            height: 64,
        },
        color: Color(0x5e, 0x48, 0xe8, 0xff),
    };
    commands.spawn(box_object);
}

/// Bounce object off edges of buffer.
fn bounce(
    options_query: Query<&PixelsOptions>,
    mut query: Query<(&Position, &mut Velocity, &Size, &mut Color)>,
) {
    let Ok(options) = options_query.get_single() else {
        return;
    };

    for (position, mut velocity, size, mut color) in &mut query {
        let mut bounce = false;
        if position.x == 0 && velocity.x < 0 {
            velocity.x *= -1;
            bounce = true;
        }
        if position.x + size.width == options.width && velocity.x > 0 {
            velocity.x *= -1;
            bounce = true;
        }
        if position.y == 0 && velocity.y < 0 {
            velocity.y *= -1;
            bounce = true;
        }
        if position.y + size.height == options.height && velocity.y > 0 {
            velocity.y *= -1;
            bounce = true;
        }
        if bounce {
            color.0 = random();
            color.1 = random();
            color.2 = random();
        }
    }
}

/// Move object based on current velocity.
fn movement(
    options_query: Query<&PixelsOptions>,
    mut query: Query<(&mut Position, &Velocity, &Size)>,
) {
    let Ok(options) = options_query.get_single() else {
        return;
    };

    for (mut position, velocity, size) in &mut query {
        position.x = ((position.x as i16 + velocity.x) as u32).clamp(0, options.width - size.width);
        position.y =
            ((position.y as i16 + velocity.y) as u32).clamp(0, options.height - size.height);
    }
}

/// Draw solid background to buffer.
fn draw_background(mut wrapper_query: Query<&mut PixelsWrapper>) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else {
        return;
    };
    let frame = wrapper.pixels.frame_mut();

    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

/// Draw objects to buffer.
fn draw_objects(
    mut wrapper_query: Query<(&mut PixelsWrapper, &PixelsOptions)>,
    query: Query<(&Position, &Size, &Color)>,
) {
    let Ok((mut wrapper, options)) = wrapper_query.get_single_mut() else {
        return;
    };
    let frame = wrapper.pixels.frame_mut();
    let frame_width_bytes = (options.width * 4) as usize;

    for (position, size, color) in &query {
        let x_offset = (position.x * 4) as usize;
        let width_bytes = (size.width * 4) as usize;
        let object_row = &[color.0, color.1, color.2, color.3].repeat(size.width as usize);

        for y in position.y..(position.y + size.height) {
            let y_offset = y as usize * frame_width_bytes;
            let i = y_offset + x_offset;
            let j = i + width_bytes;

            frame[i..j].copy_from_slice(object_row);
        }
    }
}
