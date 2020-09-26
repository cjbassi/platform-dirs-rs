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
