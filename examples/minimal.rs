use bevy::app::AppExit;
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

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Velocity {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Color(u8, u8, u8, u8);

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Hello Bevy Pixels".to_string(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            ..Default::default()
        })
        .insert_resource(PixelsOptions {
            width: WIDTH,
            height: HEIGHT,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_startup_system(setup_system.system())
        .add_system(bounce_system.system())
        .add_system(movement_system.system())
        .add_system(exit_on_escape_system.system())
        .add_stage_after(
            bevy_pixels::stage::DRAW,
            "draw_background",
            SystemStage::parallel(),
        )
        .add_stage_after("draw_background", "draw_objects", SystemStage::parallel())
        .add_system_to_stage("draw_background", draw_background_system.system())
        .add_system_to_stage("draw_objects", draw_objects_system.system())
        .run();
}

fn setup_system(commands: &mut Commands) {
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

fn bounce_system(mut query: Query<(&Position, &mut Velocity, &Size, &mut Color)>) {
    for (position, mut velocity, size, mut color) in query.iter_mut() {
        let mut bounce = false;
        if position.x <= 0 || position.x + size.width > WIDTH {
            velocity.x *= -1;
            bounce = true;
        }
        if position.y <= 0 || position.y + size.height > HEIGHT {
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

fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x = (position.x as i16 + velocity.x) as u32;
        position.y = (position.y as i16 + velocity.y) as u32;
    }
}

fn exit_on_escape_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn draw_background_system(mut pixels_resource: ResMut<PixelsResource>) {
    let frame = pixels_resource.pixels.get_frame();
    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

fn draw_objects_system(
    mut pixels_resource: ResMut<PixelsResource>,
    query: Query<(&Position, &Size, &Color)>,
) {
    let frame = pixels_resource.pixels.get_frame();
    for (position, size, color) in query.iter() {
        // TODO: Make this way more efficient
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i as u32 % WIDTH;
            let y = i as u32 / WIDTH;

            let inside_object = x >= position.x
                && x < position.x + size.width
                && y >= position.y
                && y < position.y + size.height;

            if inside_object {
                pixel.copy_from_slice(&[color.0, color.1, color.2, color.3]);
            }
        }
    }
}
