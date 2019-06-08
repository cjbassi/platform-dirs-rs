# platform-dirs-rs

[![crates.io](https://img.shields.io/crates/v/platform-dirs.svg)](https://crates.io/crates/platform-dirs)

A small Rust library for obtaining platform dependant directory paths for application and user directories.

Note that the directory paths are not guaranteed to exist.

Allows for specifying if an application is CLI or GUI based, since on macOS, CLI applications are expected to conform to the XDG spec, while GUI applications are expected to conform to the [Standard Directories] guidelines.

Uses the following standards:
- Unix (excluding macOS GUI apps): [XDG Base Directory] and [XDG User Directory]
- macOS GUI apps: [Standard Directories]
- Windows: [Known Folder]

[XDG Base Directory]: https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html
[XDG user directory]: https://www.freedesktop.org/wiki/Software/xdg-user-dirs/
[Known Folder]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx
[Standard Directories]: https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6

The API is still being flushed out, so feel free to PR any breaking changes or additions.

## Installation

Add the following to Cargo.toml:

```toml
[dependencies]
platform-dirs = "0.1.2"
```

## Examples

### Displaying paths

```rust
use std::path::PathBuf;

use platform_dirs::{AppDirs, AppUI, UserDirs};

fn main() {
    let app_dirs = AppDirs::new::<PathBuf>(None, AppUI::Graphical).unwrap();
    dbg!(&app_dirs);
    // AppDirs {
    //     cache_dir: "/home/cjbassi/.cache",
    //     config_dir: "/home/cjbassi/.config",
    //     data_dir: "/home/cjbassi/.local/share",
    //     state_dir: "/home/cjbassi/.local/state"
    // }

    let app_dirs = AppDirs::new(Some("program-name"), AppUI::CommandLine).unwrap();
    dbg!(&app_dirs);
    // AppDirs {
    //     cache_dir: "/home/cjbassi/.cache/program-name",
    //     config_dir: "/home/cjbassi/.config/program-name",
    //     data_dir: "/home/cjbassi/.local/share/program-name",
    //     state_dir: "/home/cjbassi/.local/state/program-name"
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

    let home_dir = platform_dirs::home_dir().unwrap();
    dbg!(&home_dir);
    // "/home/cjbassi"
}
```

### Opening config file

```rust
use std::fs::{self, File};

use platform_dirs::{AppDirs, AppUI};

fn main() {
    let app_dirs = AppDirs::new(Some("program-name"), AppUI::CommandLine).unwrap();

    fs::create_dir_all(&app_dirs.config_dir).unwrap();

    let config_file_path = app_dirs.config_dir.join("config-file");
    let f = if config_file_path.exists() {
        File::open(config_file_path).unwrap()
    } else {
        File::create(config_file_path).unwrap()
    };
}
```

## Path list

### AppDirs

Directory  | Windows                                                | Unix (excluding macOS GUI apps)          | macOS (GUI apps)
-----------|--------------------------------------------------------|------------------------------------------|------------------------------------
cache_dir  | `%LOCALAPPDATA%` (`C:\Users\%USERNAME%\AppData\Local`) | `$XDG_CACHE_HOME` (`$HOME/.cache`)       | `$HOME/Library/Caches`
config_dir | `%APPDATA%` (`C:\Users\%USERNAME%\AppData\Roaming`)    | `$XDG_CONFIG_HOME` (`$HOME/.config`)     | `$HOME/Library/Application Support`
data_dir   | `%LOCALAPPDATA%` (`C:\Users\%USERNAME%\AppData\Local`) | `$XDG_DATA_HOME` (`$HOME/.local/share`)  | `$HOME/Library/Application Support`
state_dir  | `%LOCALAPPDATA%` (`C:\Users\%USERNAME%\AppData\Local`) | `$XDG_STATE_HOME` (`$HOME/.local/state`) | `$HOME/Library/Application Support`

### UserDirs

Directory    | Windows                                                   | Unix (excluding macOS)                  | macOS
-------------|-----------------------------------------------------------|-----------------------------------------|------------------
desktop_dir  | `{FOLDERID_Desktop}`  (`C:\Users\%USERNAME%\Desktop`)     | `XDG_DESKTOP_DIR` (`$HOME/Desktop`)     | `$HOME/Desktop`
document_dir | `{FOLDERID_Documents}`  (`C:\Users\%USERNAME%\Documents`) | `XDG_DOCUMENTS_DIR` (`$HOME/Documents`) | `$HOME/Documents`
download_dir | `{FOLDERID_Downloads}`  (`C:\Users\%USERNAME%\Downloads`) | `XDG_DOWNLOAD_DIR` (`$HOME/Downloads`)  | `$HOME/Downloads`
music_dir    | `{FOLDERID_Music}`  (`C:\Users\%USERNAME%\Music`)         | `XDG_MUSIC_DIR` (`$HOME/Music`)         | `$HOME/Music`
picture_dir  | `{FOLDERID_Pictures}` (`C:\Users\%USERNAME%\Pictures`)    | `XDG_PICTURES_DIR` (`$HOME/Pictures`)   | `$HOME/Pictures`
public_dir   | `{FOLDERID_Public}`  (`C:\Users\%USERNAME%\Public`)       | `XDG_PUBLICSHARE_DIR` (`$HOME/Public`)  | `$HOME/Public`
video_dir    | `{FOLDERID_Videos}`  (`C:\Users\%USERNAME%\Videos`)       | `XDG_VIDEOS_DIR` (`$HOME/Videos`)       | `$HOME/Movies`
