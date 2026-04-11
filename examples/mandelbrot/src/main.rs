use bevy::{
    input::ButtonInput,
    math::DVec2,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_pixels::prelude::*;

const BUFFER_WIDTH: u32 = 320;
const BUFFER_HEIGHT: u32 = 240;
const WINDOW_WIDTH: u32 = (BUFFER_WIDTH as f32 * SCALE_FACTOR) as u32;
const WINDOW_HEIGHT: u32 = (BUFFER_HEIGHT as f32 * SCALE_FACTOR) as u32;
const SCALE_FACTOR: f32 = 3.0;
const MAX_ITERATIONS: u32 = 96;

#[derive(Resource, Debug)]
struct MandelbrotView {
    center: DVec2,
    width: f64,
}

impl Default for MandelbrotView {
    fn default() -> Self {
        Self {
            center: DVec2::new(-0.6, 0.0),
            width: 3.2,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(MandelbrotView::default())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(primary_window()),
                ..default()
            }),
            PixelsPlugin {
                primary_window: Some(PixelsOptions {
                    width: BUFFER_WIDTH,
                    height: BUFFER_HEIGHT,
                    scale_factor: SCALE_FACTOR,
                    auto_resize_buffer: true,
                    auto_resize_surface: true,
                }),
            },
        ))
        .add_systems(Update, zoom_view)
        .add_systems(Draw, draw)
        .run();
}

fn primary_window() -> Window {
    #[cfg(target_arch = "wasm32")]
    {
        return Window {
            title: "Mandelbrot".to_string(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: true,
            fit_canvas_to_parent: true,
            ..default()
        };
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Window {
            title: "Mandelbrot".to_string(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: true,
            ..default()
        }
    }
}

fn zoom_view(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<(&Window, &PixelsOptions), With<PrimaryWindow>>,
    mut view: ResMut<MandelbrotView>,
) {
    let zoom = if buttons.just_pressed(MouseButton::Left) {
        0.5
    } else if buttons.just_pressed(MouseButton::Right) {
        2.0
    } else {
        return;
    };

    let (window, options) = &*window;
    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let aspect_ratio = options.height as f64 / options.width as f64;

    view.center = complex_at(
        &view,
        cursor.x as f64 / window.width() as f64,
        cursor.y as f64 / window.height() as f64,
        aspect_ratio,
    );
    view.width *= zoom;
}

fn draw(mut wrapper: Single<(&mut PixelsWrapper, &PixelsOptions)>, view: Res<MandelbrotView>) {
    let (wrapper, options) = &mut *wrapper;
    let frame = wrapper.pixels.frame_mut();
    let width = options.width as usize;
    let aspect_ratio = options.height as f64 / options.width as f64;

    for y in 0..options.height {
        for x in 0..options.width {
            let point = complex_at(
                &view,
                (x as f64 + 0.5) / options.width as f64,
                (y as f64 + 0.5) / options.height as f64,
                aspect_ratio,
            );
            let color = mandelbrot_color(point);
            let index = ((y as usize * width) + x as usize) * 4;
            frame[index..index + 4].copy_from_slice(&color);
        }
    }
}

fn complex_at(view: &MandelbrotView, x_ratio: f64, y_ratio: f64, aspect_ratio: f64) -> DVec2 {
    let height = view.width * aspect_ratio;
    DVec2::new(
        view.center.x + (x_ratio - 0.5) * view.width,
        view.center.y + (0.5 - y_ratio) * height,
    )
}

fn mandelbrot_color(c: DVec2) -> [u8; 4] {
    let mut z = DVec2::ZERO;
    let mut iter = 0;

    while iter < MAX_ITERATIONS && z.length_squared() <= 4.0 {
        z = DVec2::new(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        iter += 1;
    }

    if iter == MAX_ITERATIONS {
        return [6, 4, 18, 255];
    }

    let t = iter as f32 / MAX_ITERATIONS as f32;
    let red = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
    let green = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
    let blue = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;

    [red, green, blue, 255]
}
