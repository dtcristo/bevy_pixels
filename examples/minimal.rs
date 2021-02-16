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
    x: i16,
    y: i16,
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
        let mut bounced = false;
        if position.x <= 0 || position.x + size.width as i16 > WIDTH as i16 {
            velocity.x *= -1;
            bounced = true;
        }
        if position.y <= 0 || position.y + size.height as i16 > HEIGHT as i16 {
            velocity.y *= -1;
            bounced = true;
        }
        if bounced {
            color.0 = random();
            color.1 = random();
            color.2 = random();
        }
    }
}

fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
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
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_object = x >= position.x
                && x < position.x + size.width as i16
                && y >= position.y
                && y < position.y + size.height as i16;

            if inside_object {
                pixel.copy_from_slice(&[color.0, color.1, color.2, color.3]);
            }
        }
    }
}
