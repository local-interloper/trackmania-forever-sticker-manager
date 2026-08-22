use std::{fs, panic, path::PathBuf};

use iced::widget::image;

use crate::{app_settings::AppSettings, constants};

pub enum StickerAssetType {
    Icon,
    Sticker,
}

pub fn get_tm_stickers_dir_path(app_settings: &AppSettings) -> PathBuf {
    app_settings.game_directory.join(constants::STICKERS_PATH)
}

pub fn get_tm_sticker_asset(
    app_settings: &AppSettings,
    sticker_name: &String,
    asset_type: StickerAssetType,
) -> Option<PathBuf> {
    let sticker_dir_path = get_tm_stickers_dir_path(app_settings).join(sticker_name);
    let Ok(files) = fs::read_dir(sticker_dir_path) else {
        return None;
    };

    let base = match asset_type {
        StickerAssetType::Icon => "Icon",
        StickerAssetType::Sticker => "Sticker",
    };

    files
        .filter_map(|f| f.ok())
        .find(|f| f.file_name().to_string_lossy().to_string().contains(base))
        .map(|f| f.path())
}

pub fn get_tm_sticker_image(
    app_settings: &AppSettings,
    sticker_name: &String,
    asset_type: StickerAssetType,
) -> image::Handle {
    match get_tm_sticker_asset(app_settings, sticker_name, asset_type) {
        Some(path) => image::Handle::from_path(path),
        None => image::Handle::from_bytes(constants::MISSING_TEXTURE.to_vec()),
    }
}

pub fn handle_with_dialog<T, E>(result: Result<T, E>, message: &str) {
    if result.is_err() {
        rfd::MessageDialog::new()
            .set_title("Error")
            .set_description(message)
            .show();
    }
}

pub fn get_sticker_names(app_settings: &AppSettings) -> Vec<String> {
    let stickers_dir_path = get_tm_stickers_dir_path(app_settings);

    let Ok(entries) = fs::read_dir(&stickers_dir_path) else {
        return vec![];
    };

    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().expect("Failed to read metadata").is_dir())
        .map(|e| e.file_name().to_str().unwrap_or("Unknown").to_string())
        .collect()
}

pub fn get_config_dir() -> PathBuf {
    let Some(project_dirs) = directories::ProjectDirs::from("me", "local.interloper", "tmsm")
    else {
        panic!("Failed to compute project dirs");
    };

    project_dirs.config_dir().to_path_buf()
}
