use crate::{diagnostic, prelude::*, system};

use bevy::{
    app::MainScheduleOrder,
    diagnostic::{Diagnostic, RegisterDiagnostic},
    ecs::{schedule::ExecutorKind, system::SystemState},
    prelude::*,
    window::PrimaryWindow,
};

/// A [`Plugin`] that defines an integration between Bevy and the [`pixels`](https://github.com/parasyte/pixels)
/// crate. Should be added to app after [`DefaultPlugins`].
pub struct PixelsPlugin {
    /// Configuration for the primary window pixel buffer. This will automatically create a
    /// [`PixelsWrapper`] component (using the provided options) for the primary window entity.
    pub primary_window: Option<PixelsOptions>,
}

impl Default for PixelsPlugin {
    fn default() -> Self {
        PixelsPlugin {
            primary_window: Some(PixelsOptions::default()),
        }
    }
}

impl Plugin for PixelsPlugin {
    fn build(&self, app: &mut App) {
        let mut draw_schedule = Schedule::new(Draw);
        draw_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

        let mut render_schedule = Schedule::new(Render);
        render_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        #[cfg(feature = "render")]
        render_schedule.add_systems(system::render);

        app.register_diagnostic(Diagnostic::new(diagnostic::RENDER_TIME).with_suffix("ms"))
            .add_schedule(draw_schedule)
            .add_schedule(render_schedule)
            .add_systems(First, system::create_pixels)
            .add_systems(
                PreUpdate,
                (
                    system::window_change,
                    system::window_resize,
                    system::resize_buffer.after(system::window_resize),
                ),
            );

        // Ensure `Draw` and `Render` schedules execute at the correct moment.
        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
        order.insert_after(PostUpdate, Draw);
        order.insert_after(Draw, Render);

        // If supplied, attach the primary window [`PixelsOptions`] component to the [`Window`]
        // entity with the [`PrimaryWindow`] marker component (if it exists). This will trigger
        // [`create_pixels`] system for this entity which will initialize the [`Pixels`] buffer.
        if let Some(options) = &self.primary_window {
            let mut system_state: SystemState<Query<Entity, With<PrimaryWindow>>> =
                SystemState::new(&mut app.world_mut());
            let query = system_state.get(&app.world());

            if let Ok(entity) = query.get_single() {
                app.world_mut().entity_mut(entity).insert(*options);
            };
        }
    }
}
