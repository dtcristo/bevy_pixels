use crate::{diagnostic, prelude::*, system};

use bevy::{
    app::MainScheduleOrder,
    diagnostic::{Diagnostic, RegisterDiagnostic},
    ecs::{schedule::ExecutorKind, system::SystemState, world::World},
    prelude::*,
    window::{PrimaryWindow, WindowBackendScaleFactorChanged, WindowResized},
};

/// A [`Plugin`] that defines an integration between Bevy and the [`pixels`](https://github.com/parasyte/pixels)
/// crate. Should be added to app after [`DefaultPlugins`].
pub struct PixelsPlugin {
    /// Configuration for the primary window pixel buffer. This will automatically create a
    /// [`PixelsWrapper`] component (using the provided options) for the primary window entity.
    pub primary_window: Option<PixelsOptions>,
}

fn insert_primary_window_options(world: &mut World, options: PixelsOptions) {
    let mut system_state: SystemState<Query<Entity, With<PrimaryWindow>>> = SystemState::new(world);
    let primary_window = {
        let query = system_state.get(world);
        query.single().ok()
    };

    if let Some(entity) = primary_window {
        world.entity_mut(entity).insert(options);
    }
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
            .add_message::<WindowResized>()
            .add_message::<WindowBackendScaleFactorChanged>()
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
            insert_primary_window_options(app.world_mut(), *options);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::{message::Messages, schedule::ScheduleLabel};

    fn schedule_index(order: &MainScheduleOrder, label: impl ScheduleLabel) -> usize {
        order
            .labels
            .iter()
            .position(|current| (**current).eq(&label))
            .expect("schedule should be present")
    }

    #[test]
    fn default_plugin_uses_default_primary_window_options() {
        assert_eq!(
            PixelsPlugin::default().primary_window,
            Some(PixelsOptions::default())
        );
    }

    #[test]
    fn plugin_adds_draw_and_render_schedules() {
        let mut app = App::new();
        app.add_plugins(PixelsPlugin::default());

        assert!(app.get_schedule(Draw).is_some());
        assert!(app.get_schedule(Render).is_some());
    }

    #[test]
    fn plugin_registers_window_messages() {
        let mut app = App::new();
        app.add_plugins(PixelsPlugin::default());

        assert!(app.world().contains_resource::<Messages<WindowResized>>());
        assert!(
            app.world()
                .contains_resource::<Messages<WindowBackendScaleFactorChanged>>()
        );
    }

    #[test]
    fn plugin_uses_single_threaded_custom_schedules() {
        let mut app = App::new();
        app.add_plugins(PixelsPlugin::default());

        assert_eq!(
            app.get_schedule(Draw).unwrap().get_executor_kind(),
            ExecutorKind::SingleThreaded
        );
        assert_eq!(
            app.get_schedule(Render).unwrap().get_executor_kind(),
            ExecutorKind::SingleThreaded
        );
    }

    #[test]
    fn plugin_inserts_draw_and_render_after_post_update() {
        let mut app = App::new();
        app.add_plugins(PixelsPlugin::default());

        let order = app.world().resource::<MainScheduleOrder>();
        let post_update = schedule_index(&order, PostUpdate);
        let draw = schedule_index(&order, Draw);
        let render = schedule_index(&order, Render);

        assert_eq!(draw, post_update + 1);
        assert_eq!(render, draw + 1);
    }

    #[test]
    fn plugin_inserts_default_options_into_existing_primary_window() {
        let mut app = App::new();
        let window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();

        app.add_plugins(PixelsPlugin::default());

        assert_eq!(
            app.world().get::<PixelsOptions>(window),
            Some(&PixelsOptions::default())
        );
    }

    #[test]
    fn plugin_inserts_custom_options_into_existing_primary_window() {
        let mut app = App::new();
        let window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();
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
    fn plugin_skips_primary_window_insertion_when_disabled() {
        let mut app = App::new();
        let window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();

        app.add_plugins(PixelsPlugin {
            primary_window: None,
        });

        assert!(app.world().get::<PixelsOptions>(window).is_none());
    }

    #[test]
    fn plugin_tolerates_missing_primary_window() {
        let mut app = App::new();

        app.add_plugins(PixelsPlugin::default());

        let mut query = app.world_mut().query::<&PixelsOptions>();
        assert_eq!(query.iter(app.world()).count(), 0);
    }

    #[test]
    fn helper_inserts_options_only_for_primary_window() {
        let mut world = World::new();
        let primary = world.spawn((Window::default(), PrimaryWindow)).id();
        let secondary = world.spawn(Window::default()).id();
        let options = PixelsOptions {
            width: 320,
            height: 180,
            ..PixelsOptions::default()
        };

        insert_primary_window_options(&mut world, options);

        assert_eq!(world.get::<PixelsOptions>(primary), Some(&options));
        assert!(world.get::<PixelsOptions>(secondary).is_none());
    }
}
