use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PixelsSet {
    /// Runs in [`Update`] schedule. Use this set for simulation logic before draw.
    Update,
    /// Runs in [`PostUpdate`] schedule. Use this set for logic that draws to the buffer.
    Draw,
    /// Runs in [`Last`] schedule. This set is used internally for rendering the buffer to
    /// the surface.
    Render,
}
