# Changelog

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

[unreleased]: https://github.com/dtcristo/bevy_pixels/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.8.0
[0.7.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.7.0
[0.6.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.6.0
[0.5.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.5.0
[0.4.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.4.0
[0.3.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.3.0
[0.2.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.2.0
[0.1.1]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.1.1
[0.1.0]: https://github.com/dtcristo/bevy_pixels/releases/tag/v0.1.0
