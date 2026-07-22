use freya::{
    components::SvgViewer,
    elements::{
        extensions::{ChildrenExt, ContainerSizeExt, ContainerWithContentExt, StyleExt},
        label::label,
        rect::rect,
    },
    icons::lucide::{pause, play},
    prelude::{
        Alignment, Border, BorderAlignment, BorderWidth, Component, Direction, Element, FontSize,
        IntoElement, Size,
    },
};

use crate::ui::theme::use_theme;

#[derive(PartialEq)]
pub struct MusicTimer {
    current_time: u16,
    current_playing: bool,
}

impl MusicTimer {
    pub fn new(current_time: u16, current_playing: bool) -> Self {
        Self {
            current_time,
            current_playing,
        }
    }

    fn playing_icon(&self, color: (u8, u8, u8)) -> Element {
        if self.current_playing {
            SvgViewer::new(("play", play()))
                .color(color)
                .width(Size::px(20.))
                .height(Size::px(20.))
                .into_element()
        } else {
            SvgViewer::new(("pause", pause()))
                .color(color)
                .width(Size::px(20.))
                .height(Size::px(20.))
                .into_element()
        }
    }
}

const VIEWPORT_HEIGHT: f32 = 50.;
const VIEWPORT_WIDTH: f32 = 150.;

impl Component for MusicTimer {
    fn render(&self) -> impl IntoElement {
        let theme = use_theme()();

        rect()
            .height(Size::px(VIEWPORT_HEIGHT))
            .width(Size::px(VIEWPORT_WIDTH))
            .background(theme.bg)
            .color(theme.text)
            .direction(Direction::Horizontal)
            .main_align(Alignment::SpaceAround)
            .cross_align(Alignment::Center)
            .font_size(FontSize::from(40.))
            .child(self.playing_icon(theme.text))
            .border(Border {
                fill: theme.border.into(),
                alignment: BorderAlignment::Outer,
                width: BorderWidth {
                    top: 0.,
                    bottom: 2.,
                    left: 0.,
                    right: 2.,
                },
            })
            .child(label().text(format!(
                "{:02}:{:02}",
                self.current_time / 60,
                self.current_time % 60
            )))
    }
}
