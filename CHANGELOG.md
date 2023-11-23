# Changelog

## [0.12.0] - 2023-11-11

### Changed
- Updated `bevy` to 0.12.

## [0.11.0] - 2023-07-18

### Changed
- Updated `bevy` to 0.11.
- Updated `pixels` to 0.12.
- Replaced `PixelsSet` with custom `Draw` and `Render` schedules defined in
  `bevy_pixels::schedule` and re-exported in `bevy_pixels::prelude`.

## [0.10.0] - 2023-05-01

### Added
- Added support for defining a custom render system by disabling default `render` cargo feature. Use
  `default_features = "false"` in Cargo.toml.

### Changed
- Internally refactored crate into modules.
- Diagnostic `PixelsPlugin::RENDER_TIME` has been moved to `bevy_pixels::diagnostic::RENDER_TIME`
  module.

## [0.9.0] - 2023-03-29

### Added

- Added support support for multiple windows. Made possible by the move from `PixelsResource` to
  `PixelsWrapper` described below.
- Added `multiple_windows` example demonstrating support for multiple windows.
- Added `scale_factor` option to control scale factor between logical window size and buffer size
  when using `auto_resize_buffer`.
- Added `auto_resize_buffer` option to control automatic resizing of the buffer when the window
  changes.
- Added `auto_resize_surface` option to control automatic resizing of the surface when the window
  changes.

### Changed

- Updated `bevy` to 0.10.
- Updated `pixels` to 0.12.
- Configuration of buffer size has been moved from `PixelsPlugin` to `PixelsOptions`.
- Primary window buffer is created by providing `Some(PixelsOptions { ... })` to the
  `primary_window` when creating `PixelsPlugin`. This works the same was as Bevy's own configuration
  of primary window in the `WindowPlugin`.
- Resouce `PixelsResource` has been replaced with `PixelsWrapper` component that is automatically
  added to `Window` entities with the `PixelsOptions` component.
- Diagnostic `PixelsPlugin::RENDER_TIME` is now recorded in miliseconds instead of seconds.
- Updated `minimal` example to demonstrate `auto_resize_buffer` feature.

## [0.8.0] - 2022-12-20

### Changed

- Updated `pixels` to 0.11.

## [0.7.0] - 2022-11-14

### Changed

- Updated `bevy` to 0.9.
- Updated `PixelsPlugin` to take configuration in Bevy 0.9 style. `PixelsOptions` resource is only
  for internal use now.

## [0.6.0] - 2022-11-03

### Added

- Added support for WASM builds.

### Changed

- Updated `bevy` to 0.8.
- Updated `pixels` to 0.10.

## [0.5.0] - 2022-06-28

### Changed

- Updated `bevy` to 0.7.
- Simplify minimal example.
- Initialize `PixelsResource` in `StartupStage::PreStartup` instead of `StartupStage::Startup`.
- Relicense under dual MIT or Apache-2.0 license.

### Fixed

- Fixed window resize on high DPI displays.

## [0.4.0] - 2022-03-01

### Added

- Added display server protocol features.

### Changed

- Updated `bevy` to 0.6.1.

## [0.3.0] - 2022-02-08

### Changed

- Updated to Rust 2021 Edition.
- Updated `bevy` to 0.6.
- Updated `pixels` to 0.9.

## [0.2.0] - 2021-12-16

### Changed

- Updated `pixels` to 0.8.

## [0.1.1] - 2021-05-30

### Removed

- Removed logo. Will be used for [official purposes](https://github.com/bevyengine/bevy/issues/2279) instead.

## [0.1.0] - 2021-05-29

Initial release.

[unreleased]: https://github.com/dtcristo/bevy_pixels/compare/v0.12.0...HEAD
[0.12.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.12.0
[0.11.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.11.0
[0.10.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.10.0
[0.9.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.9.0
[0.8.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.8.0
[0.7.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.7.0
[0.6.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.6.0
[0.5.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.5.0
[0.4.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.4.0
[0.3.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.3.0
[0.2.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.2.0
[0.1.1]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.1.1
[0.1.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.1.0
