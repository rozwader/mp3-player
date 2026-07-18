use std::rc::Rc;

use freya::{
    components::ScrollView,
    elements::{
        extensions::{
            ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt,
            EventHandlersExt, KeyExt, StyleExt,
        },
        label::label,
        rect::rect,
    },
    prelude::{
        Alignment, Border, BorderAlignment, BorderWidth, Component, Direction, Element, Event,
        FileEventData, IntoElement, Platform, Size, WritableUtils, use_consume, use_state,
    },
};

use crate::{
    audio::AudioPlayer,
    ui::{
        components::scrolling_text::{HEIGHT as TRACK_ROW_HEIGHT, ScrollingText},
        window_fit::resize_window_height,
    },
};

const DROP_ZONE_COLOR: (u8, u8, u8) = (20, 20, 20);
const DROP_ZONE_HOVER_COLOR: (u8, u8, u8) = (0, 60, 0);

fn track_labels(player: &AudioPlayer) -> Vec<String> {
    player
        .tracks()
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let secs = track.duration.as_secs();
            format!("{}: {} ({}:{:02})", index + 1, track.title, secs / 60, secs % 60)
        })
        .collect()
}

#[derive(PartialEq)]
pub struct MusicList {}

impl Component for MusicList {
    fn render(&self) -> impl IntoElement {
        let player = use_consume::<Rc<AudioPlayer>>();

        let mut tracks = use_state({
            let player = player.clone();
            move || track_labels(&player)
        });
        let mut file_hovering = use_state(|| false);

        let drop_player = player.clone();

        let items: Vec<Element> = tracks
            .read()
            .iter()
            .enumerate()
            .map(|(index, track_label)| {
                rect()
                    .key(index)
                    .width(Size::Fill)
                    .child(ScrollingText::new(track_label.clone()))
                    .into_element()
            })
            .collect();

        rect()
            .width(Size::Fill)
            .background((0, 0, 0))
            .color((0, 255, 0))
            .border(Border {
                fill: (148, 148, 148).into(),
                alignment: BorderAlignment::Outer,
                width: BorderWidth { top: 0., bottom: 2., left: 0., right: 2. },
            })
            .padding(8.)
            .spacing(8.)
            .child(
                ScrollView::new()
                    .height(Size::Inner)
                    .max_height(Size::px(500.))
                    .spacing(2.)
                    .children(items),
            )
            .child(
                rect()
                    .width(Size::Fill)
                    .height(Size::px(40.))
                    .main_align(Alignment::Center)
                    .cross_align(Alignment::Center)
                    .background(if file_hovering() {
                        DROP_ZONE_HOVER_COLOR
                    } else {
                        DROP_ZONE_COLOR
                    })
                    .border(Border {
                        fill: (148, 148, 148).into(),
                        alignment: BorderAlignment::Outer,
                        width: BorderWidth { top: 0., bottom: 2., left: 0., right: 2. },
                    })
                    .on_global_file_hover(move |_| {
                        file_hovering.set_if_modified(true);
                    })
                    .on_global_file_hover_cancelled(move |_| {
                        file_hovering.set_if_modified(false);
                    })
                    .on_file_drop(move |event: Event<FileEventData>| {
                        file_hovering.set(false);
                        if let Some(path) = event.file_path.clone() {
                            if drop_player.add_track(path) {
                                tracks.set(track_labels(&drop_player));

                                // Powiększ okno o wysokość jednej linii listy
                                let size = *Platform::get().root_size.peek();
                                resize_window_height((size.height + TRACK_ROW_HEIGHT) as f64);
                            }
                        }
                    })
                    .child(label().text("Przeciągnij tutaj pliki .mp3")),
            )
    }
}
