pub const MISSING_TEXTURE: &[u8] = include_bytes!("../assets/images/missing.png");

pub const STICKERS_PATH: &str = "GameData/Painter/Stickers/";

pub mod package_info {
    pub const QUALIFIER: &str = "me";
    pub const ORGANIZATION: &str = "local.interloper";
    pub const APPLICATION: &str = "trackmania-forever-sticker-manager";
}

pub mod dialogs {
    pub const FIRST_TIME_SETUP: &str = "Welcome to Trackmania Forever Sticker Manager.\n\
                                        We are going to perform the first time setup now.";

    pub const SELECT_GAME_DIRECTORY: &str = "First, select your TrackMania game directory";

    pub const SELECT_MAGICK_BINARY: &str = "Now select your magick executable. If you do\n\
                                            not have one, obtain it.";

    pub mod errors {
        pub const INCORRECT_GAME_DIRECTORY: &str = "That is not the TrackMania directory.\n\
                                                    Select the same directory where TmForever.exe sits.";
    }
}
