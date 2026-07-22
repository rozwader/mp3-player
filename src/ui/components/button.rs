use freya::{
    elements::{
        extensions::{
            ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt, EventHandlersExt,
            PressEventData, StyleExt,
        },
        label::label,
        rect::rect,
    },
    prelude::{
        Alignment, Border, BorderAlignment, BorderWidth, Component, Direction, Element, Event,
        EventHandler, IntoElement, Size, use_state,
    },
};

use crate::ui::theme::{ColorType, use_theme};

pub struct Button {
    text: Option<String>,
    event: EventHandler<Event<PressEventData>>,
    with_icon: bool,
    icon_only: bool,
    icon: Option<Box<dyn Fn(ColorType) -> Element>>,
    border: Option<Border>,
    padding: f32,
}

impl PartialEq for Button {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.with_icon == other.with_icon
            && self.icon_only == other.icon_only
    }
}

impl Button {
    pub fn new(
        text: Option<String>,
        event: EventHandler<Event<PressEventData>>,
        with_icon: bool,
        icon_only: bool,
        icon: Option<Box<dyn Fn(ColorType) -> Element>>,
        border: Option<Border>,
        padding: f32,
    ) -> Self {
        Self {
            text,
            event,
            with_icon,
            icon_only,
            icon,
            border,
            padding,
        }
    }

    pub fn icon(
        event: EventHandler<Event<PressEventData>>,
        icon: Option<Box<dyn Fn(ColorType) -> Element>>,
    ) -> Self {
        Self {
            text: None,
            with_icon: false,
            icon_only: true,
            icon,
            padding: 5.,
            border: None,
            event,
        }
    }

    fn get_size(&self) -> Size {
        if self.icon_only {
            Size::px(32.)
        } else {
            Size::default()
        }
    }

    fn get_content(&self, icon_color: ColorType) -> Element {
        if self.icon_only {
            return (self.icon.as_ref().unwrap())(icon_color);
        }

        if self.with_icon {
            return rect()
                .direction(Direction::Horizontal)
                .cross_align(Alignment::Center)
                .spacing(5.)
                .child(label().text(self.text.as_ref().unwrap().clone()))
                .child((self.icon.as_ref().unwrap())(icon_color))
                .into_element();
        }

        label()
            .text(self.text.as_ref().unwrap().clone())
            .into_element()
    }

    fn get_border(&self, border_color: ColorType) -> Border {
        self.border.clone().unwrap_or(Border {
            fill: border_color.into(),
            alignment: BorderAlignment::Outer,
            width: BorderWidth {
                top: 0.,
                bottom: 2.,
                left: 0.,
                right: 2.,
            },
        })
    }
}

impl Component for Button {
    fn render(&self) -> impl IntoElement {
        let theme = use_theme()();
        let mut hover_state = use_state(|| false);

        let background = if hover_state() {
            theme.button_hov_bg
        } else {
            theme.button_bg
        };

        rect()
            .width(self.get_size())
            .height(self.get_size())
            .background(background)
            .on_press(self.event.clone())
            .border(self.get_border(theme.button_border))
            .padding(self.padding)
            .on_pointer_enter(EventHandler::new(move |_| {
                *hover_state.write() = true;
            }))
            .on_pointer_leave(EventHandler::new(move |_| {
                *hover_state.write() = false;
            }))
            .child(self.get_content(theme.icon))
    }
}
