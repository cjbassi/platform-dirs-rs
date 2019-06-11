use platform_dirs::{AppDirs, AppUI, UserDirs};

fn main() {
    let app_dirs = AppDirs::new(None, AppUI::Graphical).unwrap();
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
