use std::{rc::Rc, time::Duration};

use freya::{
    components::SvgViewer,
    elements::{
        extensions::{
            ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt, EventHandlersExt,
            StyleExt,
        },
        rect::rect,
    },
    icons::lucide::{arrow_left_to_line, arrow_right_to_line, pause, play, rotate_ccw},
    prelude::{
        Element, Event, EventHandler, IntoElement, Size, SizedEventData, WritableUtils, spawn,
        use_hook, use_provide_context, use_state,
    },
};

use crate::{
    audio::AudioPlayer, spectrum::BAND_COUNT, ui::{
        components::{button::Button, music_list::MusicList, spectrum_analyzer::SpectrumAnalyzer}, music_controls::MusicControls, music_info::MusicInfo, theme::{DEFAULT_APP_THEME, LIGHT_APP_THEME, use_init_theme}, window_fit::{
            SPECTRUM_HEIGHT, WINDOW_MAX_HEIGHT, WINDOW_MIN_HEIGHT, resize_window_height,
        },
    },
};

pub fn app() -> Element {
    let theme = use_init_theme(|| LIGHT_APP_THEME);
    let player = use_provide_context(|| Rc::new(AudioPlayer::from_dir("assets/music")));
    let mut fitted = use_state(|| false);
    let mut spectrum = use_state(|| [0f32; BAND_COUNT]);

    let theme_colors = theme();

    use_hook({
        let player = player.clone();
        move || {
            spawn(async move {
                loop {
                    // ~30 FPS spectrum refresh
                    async_io::Timer::after(Duration::from_millis(33)).await;
                    spectrum.set(player.spectrum_bands());
                }
            });
        }
    });

    let previous_player = player.clone();
    let play_player = player.clone();
    let pause_player = player.clone();
    let restart_player = player.clone();
    let next_player = player.clone();

    rect()
        .width(Size::Fill)
        .height(Size::Fill)
        .background(theme_colors.secondary_bg)
        .child(
            rect()
                .width(Size::Fill)
                .height(Size::Inner)
                .padding(12.)
                .spacing(12.)
                .on_sized(move |event: Event<SizedEventData>| {
                    if !fitted() {
                        fitted.set(true);
                        let height = (event.area.height() as f64)
                            .clamp(WINDOW_MIN_HEIGHT, WINDOW_MAX_HEIGHT);
                        resize_window_height(height);
                    }
                })
                .child(MusicInfo::new(
                    MusicControls::new(vec![
                        Button::icon(
                            EventHandler::new(move |_| {
                                previous_player.previous();
                            }),
                            Some(Box::new(|color| {
                                SvgViewer::new(("arrow-left-to-line", arrow_left_to_line()))
                                    .color(color)
                                    .into_element()
                            })),
                        )
                        .into_element(),
                        Button::icon(
                            EventHandler::new(move |_| {
                                play_player.play();
                            }),
                            Some(Box::new(|color| {
                                SvgViewer::new(("play", play()))
                                    .color(color)
                                    .into_element()
                            })),
                        )
                        .into_element(),
                        Button::icon(
                            EventHandler::new(move |_| {
                                pause_player.pause();
                            }),
                            Some(Box::new(|color| {
                                SvgViewer::new(("pause", pause()))
                                    .color(color)
                                    .into_element()
                            })),
                        )
                        .into_element(),
                        Button::icon(
                            EventHandler::new(move |_| {
                                restart_player.restart();
                            }),
                            Some(Box::new(|color| {
                                SvgViewer::new(("rotate_ccw", rotate_ccw()))
                                    .color(color)
                                    .into_element()
                            })),
                        )
                        .into_element(),
                        Button::icon(
                            EventHandler::new(move |_| {
                                next_player.next();
                            }),
                            Some(Box::new(|color| {
                                SvgViewer::new(("arrow-right-to-line", arrow_right_to_line()))
                                    .color(color)
                                    .into_element()
                            })),
                        )
                        .into_element(),
                    ])
                    .into_element(),
                ))
                .child(SpectrumAnalyzer::new(spectrum()).max_height(SPECTRUM_HEIGHT))
                .child(MusicList {}),
        )
        .into_element()
}
