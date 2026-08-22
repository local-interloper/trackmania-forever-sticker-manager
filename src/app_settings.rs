use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub game_directory: PathBuf,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            game_directory: PathBuf::from("."),
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let config_dir_path = utils::get_config_dir();

        if !config_dir_path.exists() {
            fs::create_dir(&config_dir_path).unwrap();
        }

        let config_path = config_dir_path.join("config.json");

        let config_already_existed = config_path.exists();

        let config_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&config_path)
            .unwrap();

        if !config_already_existed {
            loop {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let new_settings = AppSettings {
                        game_directory: path,
                        ..Self::default()
                    };

                    new_settings.save();

                    return new_settings;
                }
            }
        }

        

        serde_json::from_reader(config_file).unwrap()
    }

    pub fn save(&self) {
        let config_dir_path = utils::get_config_dir();

        let config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_dir_path.join("config.json"))
            .unwrap();

        serde_json::to_writer_pretty(config_file, &self).unwrap();
    }
}
