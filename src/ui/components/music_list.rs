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
        theme::use_theme,
        window_fit::resize_window_height,
    },
};

#[derive(PartialEq)]
pub struct Track {
    pub title: String,
    pub display_title: String,
}

fn get_tracks(player: &AudioPlayer) -> Vec<Track> {
    player
        .tracks()
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let secs = track.duration.as_secs();
            Track {
                title: track.title.clone(),
                display_title: format!("{}: {} ({}:{:02})", index + 1, track.title, secs / 60, secs % 60)
            }
        })
        .collect()
}

#[derive(PartialEq)]
pub struct MusicList {}

impl Component for MusicList {
    fn render(&self) -> impl IntoElement {
        let player = use_consume::<Rc<AudioPlayer>>();
        let theme = use_theme()();

        let mut tracks = use_state({
            let player = player.clone();
            move || get_tracks(&player)
        });
        let mut file_hovering = use_state(|| false);

        let drop_player = player.clone();
        
        let items: Vec<Element> = tracks
            .read()
            .iter()
            .enumerate()
            .map(|(index, track)| {
                rect()
                    .key(index)
                    .width(Size::Fill)
                    .child(ScrollingText::new(Track {
                        display_title: track.display_title.clone(),
                        title: track.title.clone(),
                    }, Some(player.clone())))
                    .into_element()
            })
            .collect();

        rect()
            .width(Size::Fill)
            .background(theme.bg)
            .color(theme.text)
            .border(Border {
                fill: theme.border.into(),
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
                        theme.drop_zone_hov_bg
                    } else {
                        theme.drop_zone_bg
                    })
                    .border(Border {
                        fill: theme.border.into(),
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
                                tracks.set(get_tracks(&drop_player));

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
