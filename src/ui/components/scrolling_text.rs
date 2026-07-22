use std::rc::Rc;

use freya::{
    animation::{AnimNum, Function, OnCreation, OnFinish, use_animation}, elements::{
        extensions::{ChildrenExt, ContainerSizeExt, ContainerWithContentExt, EventHandlersExt, StyleExt, TextStyleExt}, label::label, rect::rect,
    }, prelude::{Component, EventHandler, IntoElement, Overflow, Size, TextOverflow, use_state},
};

use crate::{audio::AudioPlayer, ui::{components::music_list::Track, theme::use_theme}};

const VIEWPORT_WIDTH: f32 = 400.;
const SCROLL_DURATION_MS: u64 = 5000;

/// Wysokość jednej linii przewijanego tekstu - używana też przy powiększaniu okna.
pub const HEIGHT: f32 = 25.;

#[derive(PartialEq)]
pub struct ScrollingText {
    track: Track,
    player: Option<Rc<AudioPlayer>>,
}

impl ScrollingText {
    pub fn new(track: Track, player: Option<Rc<AudioPlayer>>) -> Self {
        Self { track, player }
    }
}

impl Component for ScrollingText {
    fn render(&self) -> impl IntoElement {
        let theme = use_theme()();
        let mut hovered_state = use_state(|| false);
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            conf.on_finish(OnFinish::restart());
            AnimNum::new(0., -VIEWPORT_WIDTH + 100.)
                .time(SCROLL_DURATION_MS)
                .function(Function::Linear)
        });

        let offset = animation.get().value();
        let duplicated = format!("  {}   {}", self.track.display_title, self.track.title);
        let player = self.player.clone();
        let title = self.track.title.clone();
        let color = if hovered_state() {
            theme.text_hov
        } else {
            theme.text
        };

        rect()
            .width(Size::Fill)
            .height(Size::px(HEIGHT))
            .overflow(Overflow::Clip)
            .on_press(move |_| {
                if let Some(player) = player.as_ref() {
                    player.load_on_demand(title.clone());
                }
            })
            .color(color)
            .on_pointer_enter(EventHandler::new(move |_| {
                *hovered_state.write() = true;
            }))
            .on_pointer_leave(EventHandler::new(move |_| {
                *hovered_state.write() = false;
            }))
            .child(
                rect()
                    .offset_x(offset)
                    .child(
                        label()
                            .text(duplicated)
                            .max_lines(Some(1))
                            .text_overflow(TextOverflow::Clip),
                    ),
            )
    }
}
