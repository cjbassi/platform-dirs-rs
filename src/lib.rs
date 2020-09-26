use std::env;
use std::path::{Path, PathBuf};

use dirs_next::home_dir;

#[derive(Clone, Debug)]
pub struct AppDirs {
    pub cache_dir: PathBuf,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub state_dir: PathBuf,
}

#[derive(Clone, Debug)]
pub struct UserDirs {
    pub desktop_dir: PathBuf,
    pub document_dir: PathBuf,
    pub download_dir: PathBuf,
    pub music_dir: PathBuf,
    pub picture_dir: PathBuf,
    pub public_dir: PathBuf,
    pub video_dir: PathBuf,
}

fn is_absolute_path(path: impl AsRef<Path>) -> Option<PathBuf> {
    let path = path.as_ref();

    if path.is_absolute() {
        Some(path.to_path_buf())
    } else {
        None
    }
}

impl AppDirs {
    pub fn new(name: Option<&str>, use_xdg_on_macos: bool) -> Option<Self> {
        if cfg!(target_os = "macos") && !use_xdg_on_macos {
            if home_dir().is_some() {
                let mut cache_dir = dirs_next::cache_dir().expect("home directory is set");
                let mut data_dir = dirs_next::data_dir().expect("home directory is set");

                if let Some(name) = name {
                    cache_dir.push(&name);
                    data_dir.push(&name);
                }

                let config_dir = data_dir.clone();
                let state_dir = data_dir.clone();

                Some(AppDirs {
                    cache_dir,
                    config_dir,
                    data_dir,
                    state_dir,
                })
            } else {
                None
            }
        } else if cfg!(target_os = "windows") {
            // TODO document why we need to check data_dir and data_local_dir
            if let (Some(_home_dir), Some(data_dir), Some(data_local_dir)) = (
                home_dir(),
                dirs_next::data_dir(),
                dirs_next::data_local_dir(),
            ) {
                let mut cache_dir = data_local_dir.clone();
                let mut config_dir = data_dir.clone();
                let mut data_dir = data_local_dir.clone();

                if let Some(name) = name {
                    cache_dir.push(&name);
                    config_dir.push(&name);
                    data_dir.push(&name);
                }

                let state_dir = data_dir.clone();

                Some(AppDirs {
                    cache_dir,
                    config_dir,
                    data_dir,
                    state_dir,
                })
            } else {
                None
            }
        } else if let Some(home_dir) = home_dir() {
            let mut cache_dir = env::var_os("XDG_CACHE_HOME")
                .and_then(is_absolute_path)
                .unwrap_or_else(|| home_dir.join(".cache"));
            let mut config_dir = env::var_os("XDG_CONFIG_HOME")
                .and_then(is_absolute_path)
                .unwrap_or_else(|| home_dir.join(".config"));
            let mut data_dir = env::var_os("XDG_DATA_HOME")
                .and_then(is_absolute_path)
                .unwrap_or_else(|| home_dir.join(".local/share"));
            let mut state_dir = env::var_os("XDG_STATE_HOME")
                .and_then(is_absolute_path)
                .unwrap_or_else(|| home_dir.join(".local/state"));

            if let Some(name) = name {
                cache_dir.push(&name);
                config_dir.push(&name);
                data_dir.push(&name);
                state_dir.push(&name);
            }

            Some(AppDirs {
                cache_dir,
                config_dir,
                data_dir,
                state_dir,
            })
        } else {
            None
        }
    }
}

impl UserDirs {
    pub fn new() -> Option<Self> {
        if let Some(home_dir) = home_dir() {
            Some(UserDirs {
                desktop_dir: dirs_next::desktop_dir().unwrap_or_else(|| home_dir.join("Desktop")),
                document_dir: dirs_next::document_dir()
                    .unwrap_or_else(|| home_dir.join("Documents")),
                download_dir: dirs_next::download_dir()
                    .unwrap_or_else(|| home_dir.join("Downloads")),
                music_dir: dirs_next::audio_dir().unwrap_or_else(|| home_dir.join("Music")),
                picture_dir: dirs_next::picture_dir().unwrap_or_else(|| home_dir.join("Pictures")),
                public_dir: dirs_next::public_dir().unwrap_or_else(|| home_dir.join("Public")),
                video_dir: dirs_next::video_dir().unwrap_or_else(|| {
                    if cfg!(target_os = "macos") {
                        home_dir.join("Movies")
                    } else {
                        home_dir.join("Videos")
                    }
                }),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_xdg() {
        let home_dir = home_dir().unwrap();
        let config_env = env::var_os("XDG_CONFIG_HOME");

        env::set_var("XDG_CONFIG_HOME", "");
        let app_dirs = AppDirs::new(None, AppUI::CommandLine).unwrap();
        assert!(app_dirs.config_dir == home_dir.join(".config"));

        env::set_var("XDG_CONFIG_HOME", "/home/cjbassi/foo");
        let app_dirs = AppDirs::new(Some("bar"), AppUI::CommandLine).unwrap();
        assert!(app_dirs.config_dir == home_dir.join("/home/cjbassi/foo/bar"));

        if let Some(config_env) = config_env {
            env::set_var("XDG_CONFIG_HOME", config_env);
        }
    }

    #[test]
    fn test_config_dir() {
        if cfg!(target_os = "macos") {
            let home_dir = home_dir().unwrap();
            let app_dirs = AppDirs::new(Some("foo"), AppUI::Graphical).unwrap();
            assert_eq!(
                app_dirs.config_dir,
                home_dir
                    .join("Library")
                    .join("Application Support")
                    .join("foo"),
            );
            test_xdg();
        } else if cfg!(target_os = "windows") {
            let home_dir = home_dir().unwrap();
            let app_dirs = AppDirs::new(Some("foo"), AppUI::Graphical).unwrap();
            assert_eq!(
                app_dirs.config_dir,
                home_dir.join("AppData").join("Roaming").join("foo")
            );
        } else {
            test_xdg();
        }
    }

    #[test]
    fn test_music_dir() {
        let music_dir = home_dir().unwrap().join("Music");
        let user_dirs = UserDirs::new().unwrap();
        assert!(user_dirs.music_dir == music_dir);
    }
}
