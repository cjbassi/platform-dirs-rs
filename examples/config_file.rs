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
