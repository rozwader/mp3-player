use std::process::Child;

use freya::{components::SvgViewer, elements::{extensions::{ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt, EventHandlersExt, PressEventData, StyleExt}, image::Image, label::label, rect::rect}, icons::lucide::antenna, prelude::{Alignment, Border, BorderAlignment, BorderWidth, Component, Direction, Element, Event, EventHandler, IntoElement, Size, use_state}};

use freya::icons::*;

pub struct Button {
    text: Option<String>,
    event: EventHandler<Event<PressEventData>>,
    with_icon: bool,
    icon_only: bool,
    icon: Option<Box<dyn Fn() -> Element>>,
    background: (u8,u8,u8),
    hovered_background: (u8,u8,u8),
    border: Option<Border>,
    padding: f32,
}

impl PartialEq for Button {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.with_icon == other.with_icon
            && self.icon_only == other.icon_only
            && self.background == other.background
    }
}

impl Button {
    pub fn new(text: Option<String>, event: EventHandler<Event<PressEventData>>, with_icon: bool, icon_only: bool, icon: Option<Box<dyn Fn() -> Element>>, background: (u8, u8, u8), hovered_background: (u8, u8, u8), border: Option<Border>, padding: f32) -> Self {
        Self {
            text,
            event,
            with_icon,
            icon_only,
            icon,
            background,
            border,
            padding,
            hovered_background
        }
    }

    pub fn icon(event: EventHandler<Event<PressEventData>>, icon: Option<Box<dyn Fn() -> Element>>, background: (u8, u8, u8), hovered_background: (u8, u8, u8)) -> Self {
        Self {
            text: None,
            with_icon: false,
            icon_only: true,
            icon,
            background,
            hovered_background,
            padding: 5.,
            border: None,
            event
        }
    }

    fn get_size(&self) -> Size {
        if self.icon_only {
            return Size::px(32.);
        } else {
            return Size::default();
        }
    }

    fn get_content(&self) -> Element {
        if self.icon_only {
           return (&self.icon.as_ref().clone().unwrap())();
        }

        if self.with_icon {
            return rect()
                .direction(Direction::Horizontal)
                .cross_align(Alignment::Center)
                .spacing(5.)
                .child(label().text(self.text.as_ref().clone().unwrap().to_string()))
                .child(
                    (&self.icon.as_ref().clone().unwrap())()
                )
                .into_element();
        }

        label()
            .text(self.text.as_ref().clone().unwrap().to_string())
            .into_element()
    }

    fn get_border(&self) -> Border {
        self.border.as_ref().clone().unwrap_or(&Border {
            fill: (100, 100, 100).into(),
            alignment: BorderAlignment::Outer,
            width: BorderWidth {top: 0., bottom: 2., left: 0., right: 2.},
        }).clone()
    }

    fn get_current_background(&self, hover_state: &bool) -> (u8, u8, u8) {
        if *hover_state == true {
            return self.hovered_background
        } else {
            return self.background
        }
    }

}

impl Component for Button {
    fn render(&self) -> impl freya::prelude::IntoElement {
        let mut hover_state = use_state(|| false);

        let background = self.get_current_background(&hover_state.read());
        rect()
            .width(self.get_size())
            .height(self.get_size())
            .background(background)
            .on_press(self.event.clone())
            .border(self.get_border())
            .padding(self.padding)
            .on_pointer_enter(EventHandler::new(move |_| {
                *hover_state.write() = true;
            }))
            .on_pointer_leave(EventHandler::new(move |_| {
                *hover_state.write() = false;
            }))
            .child(
                self.get_content()
            )
    }
}
