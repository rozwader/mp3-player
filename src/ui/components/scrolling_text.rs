use freya::{
    animation::{use_animation, AnimNum, Function, OnCreation, OnFinish},
    elements::{
        extensions::{ChildrenExt, ContainerSizeExt, ContainerWithContentExt, TextStyleExt},
        label::label,
        rect::rect,
    },
    prelude::{Component, IntoElement, Overflow, Size, TextOverflow},
};

const VIEWPORT_WIDTH: f32 = 400.;
const SCROLL_DURATION_MS: u64 = 5000;

/// Wysokość jednej linii przewijanego tekstu - używana też przy powiększaniu okna.
pub const HEIGHT: f32 = 25.;

#[derive(PartialEq)]
pub struct ScrollingText {
    text: String,
}

impl ScrollingText {
    pub fn new(text: String) -> Self {
        ScrollingText { text }
    }
}

impl Component for ScrollingText {
    fn render(&self) -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            conf.on_finish(OnFinish::restart());
            AnimNum::new(0., -VIEWPORT_WIDTH + 100.)
                .time(SCROLL_DURATION_MS)
                .function(Function::Linear)
        });

        let offset = animation.get().value();
        let duplicated = format!("{}    {}", self.text, self.text);

        rect()
            .width(Size::Fill)
            .height(Size::px(HEIGHT))
            .overflow(Overflow::Clip)
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
