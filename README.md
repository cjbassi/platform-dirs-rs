# platform-dirs-rs

[![crates.io](https://img.shields.io/crates/v/platform-dirs.svg)](https://crates.io/crates/platform-dirs)
[![docs.rs](https://docs.rs/platform-dirs/badge.svg)](https://docs.rs/platform-dirs)

A Rust library for obtaining platform dependent directory paths for application and user directories.

Uses the following standards:
- Linux/*BSD: [XDG Base Directories] and [XDG User Directories]
- macOS: [Standard Directories]
- Windows: [Known Folder]

[XDG Base Directories]: https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html
[XDG user directories]: https://www.freedesktop.org/wiki/Software/xdg-user-dirs/
[Known Folder]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx
[Standard Directories]: https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
platform-dirs = "0.3.0"
```

## Examples

### Obtaining paths

```rust
use platform_dirs::{AppDirs, UserDirs};

fn main() {
    let app_dirs = AppDirs::new(Some("name"), false).unwrap();
    dbg!(&app_dirs);
    // AppDirs {
    //     cache_dir: "/home/cjbassi/.cache/name",
    //     config_dir: "/home/cjbassi/.config/name",
    //     data_dir: "/home/cjbassi/.local/share/name",
    //     state_dir: "/home/cjbassi/.local/state/name"
    // }

    let user_dirs = UserDirs::new().unwrap();
    dbg!(&user_dirs);
    // UserDirs {
    //     desktop_dir: "/home/cjbassi/Desktop",
    //     document_dir: "/home/cjbassi/Documents",
    //     download_dir: "/home/cjbassi/Downloads",
    //     music_dir: "/home/cjbassi/Music",
    //     picture_dir: "/home/cjbassi/Pictures",
    //     public_dir: "/home/cjbassi/Public",
    //     video_dir: "/home/cjbassi/Videos"
    // }
}
```

### Opening config file

```rust
use std::fs::{self, File};

use platform_dirs::AppDirs;

fn main() {
    let app_dirs = AppDirs::new(Some("name"), true).unwrap();
    let config_file_path = app_dirs.config_dir.join("config-file");

    fs::create_dir_all(&app_dirs.config_dir).unwrap();

    let file = if config_file_path.exists() {
        File::open(config_file_path).unwrap()
    } else {
        File::create(config_file_path).unwrap()
    };
}
```

## Path list

### AppDirs

Directory  | Windows                                                | Linux/*BSD                           | macOS
-----------|--------------------------------------------------------|--------------------------------------|------------------------------------
cache_dir  | `%LOCALAPPDATA%` (`C:\Users\%USERNAME%\AppData\Local`) | `$XDG_CACHE_HOME` (`~/.cache`)       | `~/Library/Caches`
config_dir | `%APPDATA%` (`C:\Users\%USERNAME%\AppData\Roaming`)    | `$XDG_CONFIG_HOME` (`~/.config`)     | `~/Library/Application Support`
data_dir   | `%LOCALAPPDATA%` (`C:\Users\%USERNAME%\AppData\Local`) | `$XDG_DATA_HOME` (`~/.local/share`)  | `~/Library/Application Support`
state_dir  | `%LOCALAPPDATA%` (`C:\Users\%USERNAME%\AppData\Local`) | `$XDG_STATE_HOME` (`~/.local/state`) | `~/Library/Application Support`

### UserDirs

Directory    | Windows                                                   | Linux/*BSD                          | macOS
-------------|-----------------------------------------------------------|-------------------------------------|------------------
desktop_dir  | `{FOLDERID_Desktop}`  (`C:\Users\%USERNAME%\Desktop`)     | `$XDG_DESKTOP_DIR` (`~/Desktop`)     | `~/Desktop`
document_dir | `{FOLDERID_Documents}`  (`C:\Users\%USERNAME%\Documents`) | `$XDG_DOCUMENTS_DIR` (`~/Documents`) | `~/Documents`
download_dir | `{FOLDERID_Downloads}`  (`C:\Users\%USERNAME%\Downloads`) | `$XDG_DOWNLOAD_DIR` (`~/Downloads`)  | `~/Downloads`
music_dir    | `{FOLDERID_Music}`  (`C:\Users\%USERNAME%\Music`)         | `$XDG_MUSIC_DIR` (`~/Music`)         | `~/Music`
picture_dir  | `{FOLDERID_Pictures}` (`C:\Users\%USERNAME%\Pictures`)    | `$XDG_PICTURES_DIR` (`~/Pictures`)   | `~/Pictures`
public_dir   | `{FOLDERID_Public}`  (`C:\Users\%USERNAME%\Public`)       | `$XDG_PUBLICSHARE_DIR` (`~/Public`)  | `~/Public`
video_dir    | `{FOLDERID_Videos}`  (`C:\Users\%USERNAME%\Videos`)       | `$XDG_VIDEOS_DIR` (`~/Videos`)       | `~/Movies`

## Comparisons

platform-dirs differs from [dirs-rs](https://github.com/soc/dirs-rs) and [directories-rs](https://github.com/soc/directories-rs) in several ways:

- allows for using the XDG spec on macOS for CLI apps
- changes the config directory on macOS from `Library/Preferences` to `Library/Application Support`
    - `Library/Preferences` is supposed to be used for macOS unique plist preferences: [info](https://www.reddit.com/r/rust/comments/8hbzyx/can_people_here_give_the_dirs_and_directories/dyj4qtk/)
- only includes directories that are cross platform
    - `AppDirs`:
        - removes `data_local_dir`
    - `UserDirs`:
        - removes `runtime_dir`, `executable_dir`
- provides a simpler API than directories-rs
    - the fields of `UserDirs` are no longer `Options`
    - the struct fields are now publicly accessible
    - combines the `ProjectDirs` struct into `AppDirs`
- adds `state_dir` to `AppDirs`
    - documentation can be found [here](https://wiki.debian.org/XDGBaseDirectorySpecification) at the bottom of the page
    - used for stateful application data like logs, history, etc
- on Linux, returns default platforms values for the `UserDirs` if they are not set instead of returning `None`
