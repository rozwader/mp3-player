use freya::{elements::{extensions::{ChildrenExt, ContainerWithContentExt}, rect::rect}, prelude::{Alignment, Component, Direction, Element, IntoElement}};

#[derive(PartialEq)]
pub struct MusicControls {
    pub controls: Vec<Element>
}

impl MusicControls {
    pub fn new(controls: Vec<Element>) -> Self {
        Self {
            controls
        }
    }
}

impl Component for MusicControls {
    fn render(&self) -> impl IntoElement {
        rect()
            .direction(Direction::Horizontal)
            .cross_align(Alignment::Center)
            .main_align(Alignment::Start)
            .spacing(5.)
            .children(
                self.controls.clone()
            )
    }
}