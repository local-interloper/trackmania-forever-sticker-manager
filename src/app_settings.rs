use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
    process::{self},
};

use serde::{Deserialize, Serialize};

use crate::{constants, utils};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    pub game_directory: PathBuf,
    pub magick_path: PathBuf,
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
            .truncate(false)
            .open(&config_path)
            .unwrap();

        if !config_already_existed {
            return AppSettings::init();
        }

        let Ok(settings) = serde_json::from_reader(config_file) else {
            return AppSettings::init();
        };

        settings
    }

    pub fn save(&self) {
        let config_dir_path = utils::get_config_dir();

        let config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_dir_path.join("config.json"))
            .unwrap();

        serde_json::to_writer_pretty(config_file, &self).unwrap();
    }

    pub fn init() -> Self {
        let mut settings = AppSettings::default();

        rfd::MessageDialog::new()
            .set_title("First time setup")
            .set_description(constants::dialogs::FIRST_TIME_SETUP)
            .show();

        rfd::MessageDialog::new()
            .set_title("Step 1")
            .set_description(constants::dialogs::SELECT_GAME_DIRECTORY)
            .show();

        settings.game_directory = loop {
            match rfd::FileDialog::new()
                .set_title("Select the TrackMania game directory")
                .pick_folder()
            {
                Some(path) => {
                    if !path.join(constants::STICKERS_PATH).exists() {
                        rfd::MessageDialog::new()
                            .set_title("Error")
                            .set_description(constants::dialogs::errors::INCORRECT_GAME_DIRECTORY)
                            .show();
                        continue;
                    }

                    break path;
                }
                None => process::exit(0),
            };
        };

        rfd::MessageDialog::new()
            .set_title("Step 2")
            .set_description(constants::dialogs::SELECT_MAGICK_BINARY)
            .show();

        settings.magick_path = match rfd::FileDialog::new()
            .set_title("Select the magick executable")
            .pick_file()
        {
            Some(path) => path,
            None => process::exit(0),
        };

        settings.save();

        settings
    }
}
