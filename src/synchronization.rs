use crate::prelude::*;

use bevy::{
    log::warn,
    prelude::*,
    window::{WindowBackendScaleFactorChanged, WindowResized},
};
use std::collections::HashSet;

#[derive(Default)]
struct DesiredSizes {
    buffer: Option<(u32, u32)>,
    surface: Option<(u32, u32)>,
}

/// Synchronize caller-selected and window-derived sizes with each window's pixel buffer.
pub fn synchronize(
    mut window_resized: MessageReader<WindowResized>,
    mut scale_factor_changed: MessageReader<WindowBackendScaleFactorChanged>,
    mut windows: Query<(
        Entity,
        &mut PixelsOptions,
        &Window,
        Option<&mut PixelsWrapper>,
    )>,
) {
    let resized: HashSet<Entity> = window_resized.read().map(|event| event.window).collect();
    let surface_changed: HashSet<Entity> = scale_factor_changed
        .read()
        .map(|event| event.window)
        .chain(resized.iter().copied())
        .collect();

    for (entity, mut options, window, wrapper) in &mut windows {
        let mut desired = DesiredSizes::default();
        let wrapper_added = wrapper.as_ref().is_some_and(|wrapper| wrapper.is_added());

        if resized.contains(&entity) && options.auto_resize_buffer {
            let (width, height) = buffer_size_for_window(window, options.scale_factor);
            options.width = width;
            options.height = height;
            desired.buffer = Some((width, height));
        } else if options.is_changed() || wrapper_added {
            desired.buffer = Some((options.width, options.height));
        }

        if (surface_changed.contains(&entity) || wrapper_added) && options.auto_resize_surface {
            desired.surface = Some((window.physical_width(), window.physical_height()));
        }

        let Some(mut wrapper) = wrapper else {
            continue;
        };

        if let Some((width, height)) = desired.surface
            && let Err(error) = wrapper.pixels.resize_surface(width, height)
        {
            warn!(?entity, %error, "failed to synchronize pixel surface size");
        }

        if let Some((width, height)) = desired.buffer
            && let Err(error) = wrapper.pixels.resize_buffer(width, height)
        {
            warn!(?entity, %error, "failed to synchronize pixel buffer size");
        }
    }
}

fn buffer_size_for_window(window: &Window, scale_factor: f32) -> (u32, u32) {
    (
        (window.width() / scale_factor).floor() as u32,
        (window.height() / scale_factor).floor() as u32,
    )
}
