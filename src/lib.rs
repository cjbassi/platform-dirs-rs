use std::env;
use std::path::{Path, PathBuf};

pub use dirs::home_dir;

#[derive(PartialEq)]
pub enum AppUI {
    CommandLine,
    Graphical,
}

pub struct AppDirs {
    pub cache_dir: PathBuf,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub state_dir: PathBuf,
}

pub struct UserDirs {
    pub desktop_dir: PathBuf,
    pub document_dir: PathBuf,
    pub download_dir: PathBuf,
    pub music_dir: PathBuf,
    pub picture_dir: PathBuf,
    pub public_dir: PathBuf,
    pub video_dir: PathBuf,
}

pub fn is_absolute_path<P>(path: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.is_absolute() {
        Some(path.to_path_buf())
    } else {
        None
    }
}

impl AppDirs {
    pub fn new<P>(prefix: Option<P>, app_ui: AppUI) -> Option<Self>
    where
        P: AsRef<Path>,
    {
        if cfg!(target_os = "macos") && app_ui == AppUI::Graphical {
            if home_dir().is_some() {
                let mut cache_dir = dirs::cache_dir().unwrap();
                let mut data_dir = dirs::data_dir().unwrap();

                if let Some(prefix) = prefix {
                    cache_dir.push(&prefix);
                    data_dir.push(&prefix);
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
            if let (Some(_home_dir), Some(data_dir), Some(data_local_dir)) =
                (home_dir(), dirs::data_dir(), dirs::data_local_dir())
            {
                let mut cache_dir = data_local_dir.clone();
                let mut config_dir = data_dir.clone();
                let mut data_dir = data_local_dir.clone();

                if let Some(prefix) = prefix {
                    cache_dir.push(&prefix);
                    config_dir.push(&prefix);
                    data_dir.push(&prefix);
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

            if let Some(prefix) = prefix {
                cache_dir.push(&prefix);
                config_dir.push(&prefix);
                data_dir.push(&prefix);
                state_dir.push(&prefix);
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
                desktop_dir: dirs::desktop_dir().unwrap_or_else(|| home_dir.join("Desktop")),
                document_dir: dirs::document_dir().unwrap_or_else(|| home_dir.join("Documents")),
                download_dir: dirs::download_dir().unwrap_or_else(|| home_dir.join("Downloads")),
                music_dir: dirs::audio_dir().unwrap_or_else(|| home_dir.join("Music")),
                picture_dir: dirs::picture_dir().unwrap_or_else(|| home_dir.join("Pictures")),
                public_dir: dirs::public_dir().unwrap_or_else(|| home_dir.join("Public")),
                video_dir: dirs::video_dir().unwrap_or_else(|| {
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

    #[test]
    fn test_config_dir() {
        let config_dir = home_dir().unwrap().join(".config");
        env::set_var("XDG_CONFIG_HOME", &config_dir);
        let app_dirs = AppDirs::new::<PathBuf>(None, AppUI::CommandLine).unwrap();

        if cfg!(target_os = "macos") {
            assert!(app_dirs.config_dir == config_dir);
        } else if cfg!(target_os = "windows") {
            assert!(app_dirs.config_dir == config_dir);
        } else {
            assert!(app_dirs.config_dir == config_dir);
        }
    }

    #[test]
    fn test_music_dir() {
        let music_dir = home_dir().unwrap().join("Music");
        let user_dirs = UserDirs::new().unwrap();

        if cfg!(target_os = "macos") {
            assert!(user_dirs.music_dir == music_dir);
        } else if cfg!(target_os = "windows") {
            assert!(user_dirs.music_dir == music_dir);
        } else {
            assert!(user_dirs.music_dir == music_dir);
        }
    }
}
