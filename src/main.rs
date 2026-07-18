use freya::prelude::{Bytes, LaunchConfig, WindowConfig, launch};

use crate::app::app;

mod app;
mod audio;
mod ui;

fn main() {
    launch(
        LaunchConfig::new()
            .with_font("Bytesized", Bytes::from_static(include_bytes!("../assets/fonts/Bytesized.ttf")))
            .with_font("JetBrainsMono", Bytes::from_static(include_bytes!("../assets/fonts/JetBrainsMono.ttf")))
            .with_default_font("JetBrainsMono")
            .with_window(
                WindowConfig::new(app)
                    .with_size(600., 300.)
                    .with_max_size(600., 800.)
                    .with_min_size(600., 250.)
                    .with_title("MP3 Player")
            )
        );
}
