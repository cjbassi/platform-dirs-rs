# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Types of changes**:
>
> - **Added**: for new features.
> - **Changed**: for changes in existing functionality.
> - **Deprecated**: for soon-to-be removed features.
> - **Removed**: for now removed features.
> - **Fixed**: for any bug fixes.
> - **Security**: in case of vulnerabilities.

## [Unreleased]

## [v0.3.0] - 2020-09-26

### Changed

- change `AppUI` to `use_xdg_on_macos`

### Removed

- remove `home_dir` and `is_absolute_path` from public API

## [0.2.0] - 2019-06-10

### Changed

- Change prefix parameter from `AsRef<Path>` to `&str`

## [0.1.2] - 2019-06-08

### Added

- Derive `Clone` and `Debug` for `AppDirs` and `UserDirs`

## [0.1.1] - 2019-06-08

### Fixed

- Fixed app name prefix for `AppDirs`

## [0.1.0] - 2019-06-08

Initial release

[Unreleased]: https://github.com/cjbassi/platform-dirs-rs/compare/v0.3.0...HEAD
[v0.3.0]: https://github.com/cjbassi/platform-dirs-rs/compare/0.2.0...v0.3.0
[0.2.0]: https://github.com/cjbassi/platform-dirs-rs/compare/0.1.2...0.2.0
[0.1.2]: https://github.com/cjbassi/platform-dirs-rs/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/cjbassi/platform-dirs-rs/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/cjbassi/platform-dirs-rs/compare/4afc9b7218db1f2847203951ff3e1493b3d9ef38...0.1.0
