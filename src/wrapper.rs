use bevy::prelude::*;
use pixels::Pixels;

/// Wrapper component for underlying [`Pixels`] struct.
#[derive(Component, Debug)]
pub struct PixelsWrapper {
    pub pixels: Pixels<'static>,
}

#[cfg(target_arch = "wasm32")]
// Web builds run this integration on the browser's main thread, so `Pixels` never crosses thread
// boundaries even though `wgpu` marks it as !Send/!Sync on wasm targets.
unsafe impl Send for PixelsWrapper {}

#[cfg(target_arch = "wasm32")]
unsafe impl Sync for PixelsWrapper {}
