# platform-dirs-rs

[![crates.io](https://img.shields.io/crates/v/platform-dirs.svg)](https://crates.io/crates/platform-dirs)

A small Rust library for obtaining platform dependant directory paths for application and user directories.

Note that the directory paths are not guaranteed to exist.

Allows for specifying if an application is CLI or GUI based, since on macOS, CLI applications are expected to conform to the XDG spec, while GUI applications are expected to conform to the [Standard Directories](https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6) guidelines.

The API is still being flushed out, so feel free to PR any breaking changes or additions.

## Example

```rust
use platform_dirs::{AppDirs, AppUI, UserDirs};

if let Some(app_dirs) = AppDirs::new(Some("program-name"), AppUI::Graphical)
    app_dirs.config_dir;
    // Linux:   /home/alice/.config/program-name
    // Windows: C:\Users\Alice\AppData\Roaming\program-name
    // macOS:   /Users/Alice/Library/Application Support/program-name
}

if let Some(app_dirs) = AppDirs::new::<PathBuf>(None, AppUI::CommandLine) {
    app_dirs.cache_dir;
    // Linux:   /home/alice/.cache
    // Windows: C:\Users\alice\AppData\Local
    // macOS:   /home/alice/.cache
}

if let Some(user_dirs) = UserDirs::new() {
    user_dirs.music_dir;
    // Linux:   /home/alice/Music
    // Windows: C:\Users\Alice\Music
    // macOS:   /Users/Alice/Music
}
```

## Installation

Add the following to Cargo.toml:

```toml
[dependencies]
platform-dirs = "0.1.0"
```
