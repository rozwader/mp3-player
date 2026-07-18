use freya::{elements::{extensions::{ChildrenExt, ContainerSizeExt, StyleExt}, rect::rect}, prelude::{Border, BorderAlignment, BorderWidth, Component, IntoElement, Size}};

use crate::ui::components::{music_list::Track, scrolling_text::ScrollingText};

#[derive(PartialEq)]
pub struct MusicTitle {
    title: String,
    length: String,
}

impl MusicTitle {
    pub fn new(title: String, length: String) -> Self {
        Self {
            title,
            length
        }
    }
}

impl Component for MusicTitle {
    fn render(&self) -> impl IntoElement {
        let full_text = format!("{} - {}", self.title, self.length);

        rect()
            .width(Size::Fill)
            .background((0,0,0))
            .color((0,255,0))
            .border(Border {
                fill: (148, 148, 148).into(),
                alignment: BorderAlignment::Outer,
                width: BorderWidth {top: 0., bottom: 2., left: 0., right: 2.},
            })
            .child(
                ScrollingText::new(Track {
                    display_title: full_text,
                    title: self.title.clone(),
                }, None)
            )
    }
}