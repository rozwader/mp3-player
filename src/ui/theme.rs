use freya::prelude::{
    State, WritableUtils, provide_context, try_consume_context, use_consume, use_hook,
};

pub type ColorType = (u8, u8, u8);

#[derive(Clone, Copy, PartialEq)]
pub struct Theme {
    pub bg: ColorType,
    pub secondary_bg: ColorType,
    pub text: ColorType,
    pub text_hov: ColorType,
    pub icon: ColorType,
    pub border: ColorType,
    pub button_bg: ColorType,
    pub button_hov_bg: ColorType,
    pub button_border: ColorType,
    pub volume_bg: ColorType,
    pub volume_hov_bg: ColorType,
    pub music_dur_bg: ColorType,
    pub music_dur_hov_bg: ColorType,
    pub thumb: ColorType,
    pub thumb_stripe: ColorType,
    pub drop_zone_bg: ColorType,
    pub drop_zone_hov_bg: ColorType,
    pub spectrum_theme: SpectrumTheme,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SpectrumTheme {
    pub fill: Option<ColorType>,
    pub border: Option<ColorType>,
    pub width: f32,
}

pub const DEFAULT_SPECTRUM_THEME: SpectrumTheme = SpectrumTheme {
    fill: Some((0, 255, 0)),
    border: None,
    width: 10.,
};

pub const DEFAULT_APP_THEME: Theme = Theme {
    bg: (0, 0, 0),
    secondary_bg: (44, 50, 84),
    text: (0, 255, 0),
    text_hov: (11, 166, 0),
    icon: (200, 200, 200),
    border: (148, 148, 148),
    button_bg: (150, 150, 150),
    button_hov_bg: (120, 120, 120),
    button_border: (100, 100, 100),
    volume_bg: (196, 124, 60),
    volume_hov_bg: (52, 84, 160),
    music_dur_bg: (58, 64, 100),
    music_dur_hov_bg: (28, 32, 54),
    thumb: (188, 188, 188),
    thumb_stripe: (70, 70, 70),
    drop_zone_bg: (20, 20, 20),
    drop_zone_hov_bg: (0, 60, 0),
    spectrum_theme: DEFAULT_SPECTRUM_THEME,
};

pub const LIGHT_APP_THEME: Theme = Theme {
    bg: (255, 255, 255),
    secondary_bg: (219, 219, 219),
    text: (0, 0, 255),
    text_hov: (71, 71, 252),
    icon: (0, 0, 255),
    border: (148, 148, 148),
    button_bg: (230, 230, 230),
    button_hov_bg: (180, 180, 180),
    button_border: (150, 150, 150),
    volume_bg: (196, 124, 60),
    volume_hov_bg: (52, 84, 160),
    music_dur_bg: (58, 64, 100),
    music_dur_hov_bg: (28, 32, 54),
    thumb: (188, 188, 188),
    thumb_stripe: (70, 70, 70),
    drop_zone_bg: (240, 240, 240),
    drop_zone_hov_bg: (220, 220, 220),
    spectrum_theme: LIGHT_SPECTRUM_THEME,
};

pub const LIGHT_SPECTRUM_THEME: SpectrumTheme = SpectrumTheme {
    fill: None,
    border: Some((0, 0, 255)),
    width: 10.,
};

impl Default for Theme {
    fn default() -> Self {
        DEFAULT_APP_THEME
    }
}

/// All selectable app themes, in cycle order.
pub const APP_THEMES: &[Theme] = &[DEFAULT_APP_THEME, LIGHT_APP_THEME];

/// Returns the next theme after `current` (wraps around).
pub fn next_theme(current: Theme) -> Theme {
    let idx = APP_THEMES
        .iter()
        .position(|theme| *theme == current)
        .unwrap_or(0);
    APP_THEMES[(idx + 1) % APP_THEMES.len()]
}

/// Provides the app [`Theme`] as reactive global state.
/// Reuses an existing context if one was already provided higher up.
pub fn use_init_theme(theme_cb: impl FnOnce() -> Theme) -> State<Theme> {
    use_hook(|| {
        if let Some(mut existing) = try_consume_context::<State<Theme>>() {
            existing.set(theme_cb());
            existing
        } else {
            let state = State::create(theme_cb());
            provide_context(state);
            state
        }
    })
}

/// Subscribe to the current app [`Theme`]. Call from any descendant of `use_init_theme`.
pub fn use_theme() -> State<Theme> {
    use_consume::<State<Theme>>()
}
