use bevy::{
    app::MainScheduleOrder,
    ecs::message::Messages,
    prelude::*,
    window::{PrimaryWindow, WindowBackendScaleFactorChanged, WindowResized},
};
use bevy_pixels::prelude::*;

#[derive(Resource, Default, Debug, PartialEq, Eq)]
struct ExecutionTrace(Vec<&'static str>);

#[derive(Resource, Default, Debug, PartialEq, Eq)]
struct ScheduleCounts {
    draw: u32,
    render: u32,
}

fn record_post_update(mut trace: ResMut<ExecutionTrace>) {
    trace.0.push("post_update");
}

fn record_draw(mut trace: ResMut<ExecutionTrace>) {
    trace.0.push("draw");
}

fn record_render(mut trace: ResMut<ExecutionTrace>) {
    trace.0.push("render");
}

fn record_last(mut trace: ResMut<ExecutionTrace>) {
    trace.0.push("last");
}

fn count_draw(mut counts: ResMut<ScheduleCounts>) {
    counts.draw += 1;
}

fn count_render(mut counts: ResMut<ScheduleCounts>) {
    counts.render += 1;
}

fn add_primary_window(app: &mut App) -> Entity {
    app.world_mut()
        .spawn((Window::default(), PrimaryWindow))
        .id()
}

#[test]
fn public_api_inserts_default_primary_window_options() {
    let mut app = App::new();
    let window = add_primary_window(&mut app);

    app.add_plugins(PixelsPlugin::default());

    assert_eq!(
        app.world().get::<PixelsOptions>(window),
        Some(&PixelsOptions::default())
    );
}

#[test]
fn public_api_inserts_custom_primary_window_options() {
    let mut app = App::new();
    let window = add_primary_window(&mut app);
    let options = PixelsOptions {
        width: 320,
        height: 240,
        scale_factor: 2.0,
        auto_resize_buffer: false,
        auto_resize_surface: false,
    };

    app.add_plugins(PixelsPlugin {
        primary_window: Some(options),
    });

    assert_eq!(app.world().get::<PixelsOptions>(window), Some(&options));
}

#[test]
fn only_primary_window_receives_auto_inserted_options() {
    let mut app = App::new();
    let primary = add_primary_window(&mut app);
    let secondary = app.world_mut().spawn(Window::default()).id();

    app.add_plugins(PixelsPlugin::default());

    assert_eq!(
        app.world().get::<PixelsOptions>(primary),
        Some(&PixelsOptions::default())
    );
    assert!(app.world().get::<PixelsOptions>(secondary).is_none());
}

#[test]
fn plugin_with_primary_window_none_updates_without_inserting_options() {
    let mut app = App::new();
    let window = add_primary_window(&mut app);

    app.add_plugins(PixelsPlugin {
        primary_window: None,
    });
    app.update();

    assert!(app.world().get::<PixelsOptions>(window).is_none());
}

#[test]
fn draw_and_render_schedules_run_in_expected_frame_order() {
    let mut app = App::new();
    app.init_resource::<ExecutionTrace>();
    app.add_plugins(PixelsPlugin {
        primary_window: None,
    });
    app.add_systems(PostUpdate, record_post_update);
    app.add_systems(Draw, record_draw);
    app.add_systems(Render, record_render);
    app.add_systems(Last, record_last);

    app.update();

    assert_eq!(
        app.world().resource::<ExecutionTrace>().0,
        vec!["post_update", "draw", "render", "last"]
    );
}

#[test]
fn draw_and_render_schedules_run_every_frame() {
    let mut app = App::new();
    app.init_resource::<ScheduleCounts>();
    app.add_plugins(PixelsPlugin {
        primary_window: None,
    });
    app.add_systems(Draw, count_draw);
    app.add_systems(Render, count_render);

    app.update();
    app.update();
    app.update();

    assert_eq!(
        *app.world().resource::<ScheduleCounts>(),
        ScheduleCounts { draw: 3, render: 3 }
    );
}

#[test]
fn public_schedule_order_resource_matches_custom_schedules() {
    let mut app = App::new();
    app.add_plugins(PixelsPlugin {
        primary_window: None,
    });

    let order = app.world().resource::<MainScheduleOrder>();
    let labels = &order.labels;
    let draw_index = labels
        .iter()
        .position(|label| (**label).eq(&Draw))
        .expect("draw schedule should exist");
    let render_index = labels
        .iter()
        .position(|label| (**label).eq(&Render))
        .expect("render schedule should exist");
    let post_update_index = labels
        .iter()
        .position(|label| (**label).eq(&PostUpdate))
        .expect("post update schedule should exist");
    let last_index = labels
        .iter()
        .position(|label| (**label).eq(&Last))
        .expect("last schedule should exist");

    assert_eq!(draw_index, post_update_index + 1);
    assert_eq!(render_index, draw_index + 1);
    assert!(render_index < last_index);
}

#[test]
fn plugin_registers_window_messages_for_public_apps() {
    let mut app = App::new();
    app.add_plugins(PixelsPlugin {
        primary_window: None,
    });

    assert!(app.world().contains_resource::<Messages<WindowResized>>());
    assert!(
        app.world()
            .contains_resource::<Messages<WindowBackendScaleFactorChanged>>()
    );
}

#[test]
fn window_resize_updates_each_automatic_buffer_size_without_a_pixels_wrapper() {
    let mut app = App::new();
    app.add_plugins(PixelsPlugin {
        primary_window: None,
    });

    let automatic = app
        .world_mut()
        .spawn((
            Window {
                resolution: bevy::window::WindowResolution::new(640, 480),
                ..default()
            },
            PixelsOptions {
                scale_factor: 2.0,
                ..default()
            },
        ))
        .id();
    let manual = app
        .world_mut()
        .spawn((
            Window {
                resolution: bevy::window::WindowResolution::new(900, 600),
                ..default()
            },
            PixelsOptions {
                width: 160,
                height: 90,
                scale_factor: 3.0,
                auto_resize_buffer: false,
                ..default()
            },
        ))
        .id();
    let fractional = app
        .world_mut()
        .spawn((
            Window {
                resolution: bevy::window::WindowResolution::new(641, 479),
                ..default()
            },
            PixelsOptions {
                scale_factor: 2.0,
                ..default()
            },
        ))
        .id();
    let high_dpi = app
        .world_mut()
        .spawn((
            Window {
                resolution: bevy::window::WindowResolution::new(1280, 720)
                    .with_scale_factor_override(2.0),
                ..default()
            },
            PixelsOptions {
                scale_factor: 2.0,
                ..default()
            },
        ))
        .id();

    app.world_mut().write_message(WindowResized {
        window: automatic,
        width: 640.0,
        height: 480.0,
    });
    app.world_mut().write_message(WindowResized {
        window: manual,
        width: 900.0,
        height: 600.0,
    });
    app.world_mut().write_message(WindowResized {
        window: fractional,
        width: 641.0,
        height: 479.0,
    });
    app.world_mut().write_message(WindowResized {
        window: high_dpi,
        width: 640.0,
        height: 360.0,
    });
    app.update();

    let automatic = app.world().get::<PixelsOptions>(automatic).unwrap();
    assert_eq!((automatic.width, automatic.height), (320, 240));
    let manual = app.world().get::<PixelsOptions>(manual).unwrap();
    assert_eq!((manual.width, manual.height), (160, 90));
    let fractional = app.world().get::<PixelsOptions>(fractional).unwrap();
    assert_eq!((fractional.width, fractional.height), (320, 239));
    let high_dpi = app.world().get::<PixelsOptions>(high_dpi).unwrap();
    assert_eq!((high_dpi.width, high_dpi.height), (320, 180));
}
