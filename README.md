# TrackMania Forever Sticker Manager

A small desktop GUI for managing custom car stickers in **TrackMania Forever**. It lists the stickers currently installed in your game's `GameData/Painter/Stickers/` folder, lets you preview and search them, add new stickers from any image file, and remove ones you no longer want.

Built with Rust and [iced](https://iced.rs/).

## Features

- Browse and search installed stickers
- Preview a sticker's full-size texture
- Add a new sticker from any image (converted and resized automatically)
- Remove an existing sticker
- Remembers your game installation directory between runs

## Requirements

- A TrackMania Forever installation
- [ImageMagick](https://imagemagick.org/) (the `magick` command) available on your `PATH` — used to convert and resize sticker images
- Rust toolchain (if building from source)

## Building & Running

```sh
cargo run --release
```

On first launch, you'll be asked to pick your TrackMania Forever game directory (the folder containing `GameData/`). This is saved to your user config directory for future runs.

## How it works

Each sticker is a subfolder under `GameData/Painter/Stickers/<sticker name>/` containing an `Icon.tga` (used for thumbnails, resized to 64x64) and a `Sticker.tga` (the full texture, resized to 500x500). Adding a sticker picks a source image, creates the folder, and generates both files via ImageMagick. Removing a sticker deletes its icon, texture, and folder.

## Configuration

App settings (currently just the game directory) are stored as JSON in the platform-specific config directory, e.g.:

- Linux: `~/.config/tmsm/config.json`
- macOS: `~/Library/Application Support/me.local-interloper.tmsm/config.json`
- Windows: `%APPDATA%\local-interloper\tmsm\config\config.json`
