use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResizeConstraints,
};
use bevy_pixels::prelude::*;
use rand::prelude::*;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Hello Bevy Pixels".to_string(),
                width: WIDTH as f32,
                height: HEIGHT as f32,
                resize_constraints: WindowResizeConstraints {
                    min_width: WIDTH as f32,
                    min_height: HEIGHT as f32,
                    ..default()
                },
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(PixelsPlugin {
            width: WIDTH,
            height: HEIGHT,
            ..default()
        })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(bounce)
        .add_system(movement.after(bounce))
        .add_system(exit_on_escape)
        .add_system_to_stage(PixelsStage::Draw, draw_background)
        .add_system_to_stage(PixelsStage::Draw, draw_objects.after(draw_background))
        .run();
}

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

fn bounce(mut query: Query<(&Position, &mut Velocity, &Size, &mut Color)>) {
    for (position, mut velocity, size, mut color) in query.iter_mut() {
        let mut bounce = false;
        if position.x == 0 || position.x + size.width > WIDTH {
            velocity.x *= -1;
            bounce = true;
        }
        if position.y == 0 || position.y + size.height > HEIGHT {
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

fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x = (position.x as i16 + velocity.x) as u32;
        position.y = (position.y as i16 + velocity.y) as u32;
    }
}

fn exit_on_escape(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn draw_background(mut pixels_resource: ResMut<PixelsResource>) {
    let frame = pixels_resource.pixels.frame_mut();
    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

fn draw_objects(
    mut pixels_resource: ResMut<PixelsResource>,
    query: Query<(&Position, &Size, &Color)>,
) {
    let frame = pixels_resource.pixels.frame_mut();
    let frame_width_bytes = (WIDTH * 4) as usize;

    for (position, size, color) in query.iter() {
        let x_offset = (position.x * 4) as usize;
        let width_bytes = (size.width * 4) as usize;
        let object_row = &[color.0, color.1, color.2, color.3].repeat(size.width as usize);

        for y in position.y..(position.y + size.height - 1) {
            let y_offset = y as usize * frame_width_bytes;
            let i = y_offset + x_offset;
            let j = i + width_bytes;

            frame[i..j].copy_from_slice(object_row);
        }
    }
}
