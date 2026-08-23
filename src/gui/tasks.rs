use std::{
    fs::{self},
    path::Path,
    process::Command,
};

use regex::Regex;

use crate::{app_settings::AppSettings, utils};

pub async fn new_sticker(app_settings: &AppSettings) -> Option<String> {
    let result = rfd::FileDialog::new()
        .set_title("Select your sticker image")
        .pick_file();

    let Some(input_path) = result else {
        return None;
    };

    let result = Command::new(&app_settings.magick_path)
        .arg("identify")
        .arg(&input_path)
        .spawn()
        .unwrap()
        .wait();

    if let Ok(result) = result
        && !result.success()
    {
        rfd::MessageDialog::new()
            .set_title("Error")
            .set_description("That's not an image.")
            .show();

        return None;
    }

    let trackmania_stickers_dir_path = utils::get_tm_stickers_dir_path(app_settings);

    let input_file = Path::new(&input_path);

    let Some(file_name) = input_file.file_name() else {
        return None;
    };

    let file_name = file_name.to_string_lossy().to_string();

    let suffix_destroyer = Regex::new(r"\..+$").expect("Failed to create suffix destroyer");

    let sticker_name = suffix_destroyer.replace(&file_name, "").to_string();

    if fs::create_dir(trackmania_stickers_dir_path.join(&sticker_name)).is_err() {
        rfd::MessageDialog::new()
            .set_title("Error")
            .set_description(format!(
                "Sticker with name {} already exists!",
                &sticker_name
            ))
            .show();

        return None;
    }

    let icon_path = trackmania_stickers_dir_path
        .join(&sticker_name)
        .join("Icon.tga");

    let sticker_path = trackmania_stickers_dir_path
        .join(&sticker_name)
        .join("Sticker.tga");

    if let Ok(mut handle) = Command::new(&app_settings.magick_path)
        .arg(&input_path)
        .args(["-resize", "500x500"])
        .arg(&sticker_path)
        .spawn()
    {
        let status = handle.wait().unwrap();
        if !status.success() {
            rfd::MessageDialog::new()
                .set_title("Error")
                .set_description("Failed to convert image. Is it even an image?");

            return None;
        }
    }

    if let Ok(mut handle) = Command::new(&app_settings.magick_path)
        .arg(&input_path)
        .args(["-resize", "64x64"])
        .arg(&icon_path)
        .spawn()
    {
        let status = handle.wait().unwrap();
        if !status.success() {
            rfd::MessageDialog::new()
                .set_title("Error")
                .set_description("Failed to convert image. Is it even an image?");
        }
    }

    Some(sticker_name)
}

pub async fn remove_sticker(app_settings: &AppSettings, sticker_name: String) {
    let tm_stickers_dir_path = utils::get_tm_stickers_dir_path(app_settings);
    let sticker_dir_path = tm_stickers_dir_path.join(&sticker_name);

    for path in [
        utils::get_tm_sticker_asset(app_settings, &sticker_name, utils::StickerAssetType::Icon),
        utils::get_tm_sticker_asset(
            app_settings,
            &sticker_name,
            utils::StickerAssetType::Sticker,
        ),
    ]
    .iter()
    .flat_map(|f| f.clone())
    {
        fs::remove_file(path).ok();
    }

    fs::remove_dir(sticker_dir_path).ok();
}
