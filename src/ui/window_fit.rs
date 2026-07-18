use freya::{
    prelude::{Platform, WinitPlatformExt},
    winit::dpi::LogicalSize,
};

pub const WINDOW_WIDTH: f64 = 600.;
pub const WINDOW_MIN_HEIGHT: f64 = 250.;
pub const WINDOW_MAX_HEIGHT: f64 = 800.;

const APP_PADDING: f32 = 12. * 2.;
const APP_SPACING: f32 = 12.;
/// Timer/title row + controls + progress slider + spacingi w MusicInfo
const MUSIC_INFO_HEIGHT: f32 = 50. + 10. + 32. + 10. + 14.;
const LIST_PADDING: f32 = 8. * 2.;
const LIST_INNER_SPACING: f32 = 8.;
const DROP_ZONE_HEIGHT: f32 = 40.;
const LIST_BORDER: f32 = 2.;
const LIST_MAX_HEIGHT: f32 = 500.;
const TRACK_ROW_HEIGHT: f32 = 25.;
const TRACK_ROW_SPACING: f32 = 2.;

/// Liczy pliki .mp3 w katalogu (bez dekodowania).
pub fn count_mp3(dir: &str) -> usize {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("mp3"))
                })
                .count()
        })
        .unwrap_or(1)
        .max(1)
}

/// Wysokość listy utworów (z limitem ScrollView).
fn list_tracks_height(track_count: usize) -> f32 {
    if track_count == 0 {
        return 0.;
    }
    let content =
        track_count as f32 * TRACK_ROW_HEIGHT + (track_count - 1) as f32 * TRACK_ROW_SPACING;
    content.min(LIST_MAX_HEIGHT)
}

/// Docelowa wysokość okna dla danej liczby utworów.
pub fn height_for_tracks(track_count: usize) -> f64 {
    let music_list = LIST_PADDING
        + LIST_INNER_SPACING
        + list_tracks_height(track_count)
        + DROP_ZONE_HEIGHT
        + LIST_BORDER;
    let total = APP_PADDING + MUSIC_INFO_HEIGHT + APP_SPACING + music_list;
    (total as f64).clamp(WINDOW_MIN_HEIGHT, WINDOW_MAX_HEIGHT)
}

/// Ustawia wysokość okna (szerokość zostaje `WINDOW_WIDTH`).
pub fn resize_window_height(height: f64) {
    let height = height.clamp(WINDOW_MIN_HEIGHT, WINDOW_MAX_HEIGHT);
    Platform::get().with_window(None, move |window| {
        let _ = window.request_inner_size(LogicalSize::new(WINDOW_WIDTH, height));
    });
}
