use std::rc::Rc;

use freya::{
    components::SvgViewer,
    elements::{
        extensions::{
            ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt, EventHandlersExt,
            PressEventData, StyleExt, TextStyleExt,
        },
        rect::rect,
    },
    icons::lucide::{arrow_left_to_line, arrow_right_to_line, pause, play, rotate_ccw},
    prelude::{
        Element, Event, EventHandler, IntoElement, Size, SizedEventData, WritableUtils,
        use_provide_context, use_state,
    },
};

use crate::{
    audio::AudioPlayer,
    ui::{
        components::{button::Button, music_list::MusicList},
        music_controls::MusicControls,
        music_info::MusicInfo,
        window_fit::{WINDOW_MAX_HEIGHT, WINDOW_MIN_HEIGHT, resize_window_height},
    },
};

pub fn app() -> Element {
    let player = use_provide_context(|| Rc::new(AudioPlayer::from_dir("assets/music")));
    let mut fitted = use_state(|| false);

    let previous_player = player.clone();
    let play_player = player.clone();
    let pause_player = player.clone();
    let restart_player = player.clone();
    let next_player = player.clone();

    rect()
        .width(Size::Fill)
        .height(Size::Fill)
        .background((44, 50, 84))
        .child(
            rect()
                .width(Size::Fill)
                .height(Size::Inner)
                .padding(12.)
                .spacing(12.)
                .on_sized(move |event: Event<SizedEventData>| {
                    // Po pierwszym layoutcie dopasuj okno do rzeczywistej wysokości treści
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
                            Some(Box::new(|| -> Element {
                                SvgViewer::new(("arrow-left-to-line", arrow_left_to_line()))
                                    .color((200, 200, 200))
                                    .into_element()
                            })),
                            (150, 150, 150),
                            (120, 120, 120),
                        )
                        .into_element(),
                        Button::icon(
                            EventHandler::new(move |_| {
                                play_player.play();
                            }),
                            Some(Box::new(|| -> Element {
                                SvgViewer::new(("play", play()))
                                    .color((200, 200, 200))
                                    .into_element()
                            })),
                            (150, 150, 150),
                            (120, 120, 120),
                        )
                        .into_element(),
                        Button::icon(
                            EventHandler::new(move |_| {
                                pause_player.pause();
                            }),
                            Some(Box::new(|| -> Element {
                                SvgViewer::new(("pause", pause()))
                                    .color((200, 200, 200))
                                    .into_element()
                            })),
                            (150, 150, 150),
                            (120, 120, 120),
                        )
                        .into_element(),
                        Button::icon(
                            EventHandler::new(move |_| {
                                restart_player.restart();
                            }),
                            Some(Box::new(|| -> Element {
                                SvgViewer::new(("rotate_ccw", rotate_ccw()))
                                    .color((200, 200, 200))
                                    .into_element()
                            })),
                            (150, 150, 150),
                            (120, 120, 120),
                        )
                        .into_element(),
                        Button::icon(
                            EventHandler::new(move |_| {
                                next_player.next();
                            }),
                            Some(Box::new(|| -> Element {
                                SvgViewer::new(("arrow-right-to-line", arrow_right_to_line()))
                                    .color((200, 200, 200))
                                    .into_element()
                            })),
                            (150, 150, 150),
                            (120, 120, 120),
                        )
                        .into_element(),
                    ])
                    .into_element(),
                ))
                .child(MusicList {}),
        )
        .into_element()
}
