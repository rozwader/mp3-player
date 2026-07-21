use freya::{prelude::{Bytes, LaunchConfig, WindowConfig, launch}, winit::window::Icon};

use crate::{
    app::app,
    ui::window_fit::{
        WINDOW_MAX_HEIGHT, WINDOW_MIN_HEIGHT, WINDOW_WIDTH, count_mp3, height_for_tracks,
    },
};

mod app;
mod audio;
mod spectrum;
mod ui;

fn main() {
    let track_count = count_mp3("assets/music");
    let height = height_for_tracks(track_count);

    launch(
        LaunchConfig::new()
            .with_font(
                "Bytesized",
                Bytes::from_static(include_bytes!("../assets/fonts/Bytesized.ttf")),
            )
            .with_font(
                "JetBrainsMono",
                Bytes::from_static(include_bytes!("../assets/fonts/JetBrainsMono.ttf")),
            )
            .with_default_font("JetBrainsMono")
            .with_window(
                WindowConfig::new(app)
                    .with_size(WINDOW_WIDTH, height)
                    .with_max_size(WINDOW_WIDTH, WINDOW_MAX_HEIGHT)
                    .with_min_size(WINDOW_WIDTH, WINDOW_MIN_HEIGHT)
                    .with_title("MP3 Player")
                    .with_icon(LaunchConfig::window_icon(include_bytes!("../assets/app_icon.png"))),
            ),
    );
}
