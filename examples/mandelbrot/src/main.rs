use bevy::{
    input::ButtonInput,
    math::DVec2,
    prelude::*,
    window::{PresentMode, PrimaryWindow, WindowResolution},
    winit::WinitSettings,
};
use bevy_pixels::prelude::*;

const BUFFER_WIDTH: u32 = 80;
const BUFFER_HEIGHT: u32 = 60;
const WINDOW_WIDTH: u32 = (BUFFER_WIDTH as f32 * SCALE_FACTOR) as u32;
const WINDOW_HEIGHT: u32 = (BUFFER_HEIGHT as f32 * SCALE_FACTOR) as u32;
const SCALE_FACTOR: f32 = 6.0;
const MAX_ITERATIONS: u32 = 96;
const ZOOM_OCTAVES_PER_SECOND: f64 = 1.5;
const MIN_VIEW_WIDTH: f64 = 0.000_01;
const MAX_VIEW_WIDTH: f64 = 3.2;

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
        .insert_resource(WinitSettings::game())
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
            present_mode: PresentMode::AutoVsync,
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
            present_mode: PresentMode::AutoVsync,
            resizable: true,
            ..default()
        }
    }
}

fn zoom_view(
    buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    window: Single<(&Window, &PixelsOptions), With<PrimaryWindow>>,
    mut view: ResMut<MandelbrotView>,
) {
    let zoom_direction =
        if buttons.pressed(MouseButton::Left) && !buttons.pressed(MouseButton::Right) {
            -1.0
        } else if buttons.pressed(MouseButton::Right) && !buttons.pressed(MouseButton::Left) {
            1.0
        } else {
            return;
        };

    let (window, options) = &*window;
    let aspect_ratio = options.height as f64 / options.width as f64;
    let cursor = window
        .cursor_position()
        .unwrap_or(Vec2::new(window.width() * 0.5, window.height() * 0.5));
    let x_ratio = cursor.x as f64 / window.width() as f64;
    let y_ratio = cursor.y as f64 / window.height() as f64;
    let focus = complex_at(&view, x_ratio, y_ratio, aspect_ratio);
    let zoom_factor =
        2.0_f64.powf(zoom_direction * ZOOM_OCTAVES_PER_SECOND * time.delta_secs_f64());
    let new_width = (view.width * zoom_factor).clamp(MIN_VIEW_WIDTH, MAX_VIEW_WIDTH);

    view.center = center_for_focus(focus, x_ratio, y_ratio, new_width, aspect_ratio);
    view.width = new_width;
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

fn center_for_focus(
    focus: DVec2,
    x_ratio: f64,
    y_ratio: f64,
    width: f64,
    aspect_ratio: f64,
) -> DVec2 {
    let height = width * aspect_ratio;
    DVec2::new(
        focus.x - (x_ratio - 0.5) * width,
        focus.y - (0.5 - y_ratio) * height,
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
