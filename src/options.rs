use bevy::prelude::*;

/// Defines the sizing and behavior of the pixel buffer and surface texture.
#[derive(Component, Debug, Copy, Clone)]
pub struct PixelsOptions {
    /// Width of the pixel buffer. Changing this after initialization will resize the buffer.
    pub width: u32,
    /// Height of the pixel buffer. Changing this after initialization will resize the buffer.
    pub height: u32,
    /// Scale factor between logical window size and buffer size. Only used when
    /// `auto_resize_buffer` is enabled.
    pub scale_factor: f32,
    /// Should the buffer automatically be resized when the window changes?
    pub auto_resize_buffer: bool,
    /// Should the surface texture automatically be resized when the window changes?
    pub auto_resize_surface: bool,
}

impl Default for PixelsOptions {
    fn default() -> Self {
        PixelsOptions {
            width: 1280,
            height: 720,
            scale_factor: 1.0,
            auto_resize_buffer: true,
            auto_resize_surface: true,
        }
    }
}
