use freya::{
    elements::{
        extensions::{
            ChildrenExt, ContainerPositionExt, ContainerSizeExt, ContainerWithContentExt,
            EventHandlersExt, LayerExt, StyleExt,
        },
        rect::rect,
    },
    prelude::{
        Alignment, Border, BorderAlignment, BorderWidth, Component, Direction, Element, Event,
        EventHandler, IntoElement, PointerEventData, Position, Size, SizedEventData, State,
        WritableUtils, use_state,
    },
};

const TRACK_HEIGHT: f32 = 14.;
const THUMB_WIDTH: f32 = 16.;

pub type Rgb = (u8, u8, u8);

const DEFAULT_FILL_COLOR: Rgb = (196, 124, 60);
const DEFAULT_TRACK_COLOR: Rgb = (52, 84, 160);
const DEFAULT_THUMB_COLOR: Rgb = (188, 188, 188);
const DEFAULT_THUMB_STRIPE_COLOR: Rgb = (70, 70, 70);
const DEFAULT_BORDER_COLOR: Rgb = (148, 148, 148);

/// Slider sterowany z zewnątrz. Wartość (0.0..=1.0) trzyma rodzic w `use_state`.
///
/// - `on_change` — przy każdym ruchu (np. głośność na żywo)
/// - `on_drag_start` — przy rozpoczęciu przeciągania
/// - `on_change_end` — dopiero po puszczeniu (np. seek utworu)
pub struct Slider {
    value: State<f32>,
    width: Size,
    fill_color: Rgb,
    track_color: Rgb,
    thumb_color: Rgb,
    thumb_stripe_color: Rgb,
    border_color: Rgb,
    on_change: Option<EventHandler<f32>>,
    on_drag_start: Option<EventHandler<()>>,
    on_change_end: Option<EventHandler<f32>>,
}

impl PartialEq for Slider {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.width == other.width
            && self.fill_color == other.fill_color
            && self.track_color == other.track_color
            && self.thumb_color == other.thumb_color
            && self.thumb_stripe_color == other.thumb_stripe_color
            && self.border_color == other.border_color
    }
}

impl Slider {
    pub fn new(value: State<f32>, width: Size) -> Self {
        Self {
            value,
            width,
            fill_color: DEFAULT_FILL_COLOR,
            track_color: DEFAULT_TRACK_COLOR,
            thumb_color: DEFAULT_THUMB_COLOR,
            thumb_stripe_color: DEFAULT_THUMB_STRIPE_COLOR,
            border_color: DEFAULT_BORDER_COLOR,
            on_change: None,
            on_drag_start: None,
            on_change_end: None,
        }
    }

    /// Wywoływany przy każdej zmianie wartości przez użytkownika (klik/przeciąganie).
    pub fn on_change(mut self, handler: impl FnMut(f32) + 'static) -> Self {
        self.on_change = Some(EventHandler::new(handler));
        self
    }

    /// Wywoływany raz, gdy użytkownik zaczyna przeciągać suwak.
    pub fn on_drag_start(mut self, handler: impl FnMut(()) + 'static) -> Self {
        self.on_drag_start = Some(EventHandler::new(handler));
        self
    }

    /// Wywoływany raz, gdy użytkownik puści suwak — z finalną wartością.
    pub fn on_change_end(mut self, handler: impl FnMut(f32) + 'static) -> Self {
        self.on_change_end = Some(EventHandler::new(handler));
        self
    }

    pub fn fill_color(mut self, color: Rgb) -> Self {
        self.fill_color = color;
        self
    }

    pub fn track_color(mut self, color: Rgb) -> Self {
        self.track_color = color;
        self
    }

    pub fn thumb_color(mut self, color: Rgb) -> Self {
        self.thumb_color = color;
        self
    }

    pub fn thumb_stripe_color(mut self, color: Rgb) -> Self {
        self.thumb_stripe_color = color;
        self
    }

    pub fn border_color(mut self, color: Rgb) -> Self {
        self.border_color = color;
        self
    }

    fn thumb_stripe(color: Rgb) -> Element {
        rect()
            .width(Size::px(2.))
            .height(Size::px(TRACK_HEIGHT - 6.))
            .background(color)
            .into_element()
    }
}

impl Component for Slider {
    fn render(&self) -> impl IntoElement {
        let mut value = self.value;
        let on_change_down = self.on_change.clone();
        let on_change_move = self.on_change.clone();
        let on_drag_start = self.on_drag_start.clone();
        let on_change_end = self.on_change_end.clone();

        let mut is_dragging = use_state(|| false);
        let mut track_origin_x = use_state(|| 0.0f32);
        let mut track_width = use_state(|| 0.0f32);

        let progress = self.value.read().clamp(0., 1.);
        let thumb_x = (track_width() * progress - THUMB_WIDTH / 2.)
            .clamp(0., (track_width() - THUMB_WIDTH).max(0.));

        rect()
            .width(self.width.clone())
            .height(Size::px(TRACK_HEIGHT))
            .background(self.track_color)
            .on_sized(move |event: Event<SizedEventData>| {
                track_origin_x.set(event.area.min_x());
                track_width.set(event.area.width());
            })
            .on_pointer_down(move |event: Event<PointerEventData>| {
                if event.is_primary() && track_width() > 0. {
                    is_dragging.set(true);
                    if let Some(on_drag_start) = &on_drag_start {
                        on_drag_start.call(());
                    }

                    let x = event.element_location().x as f32;
                    let new_value = (x / track_width()).clamp(0., 1.);
                    value.set(new_value);
                    if let Some(on_change) = &on_change_down {
                        on_change.call(new_value);
                    }
                }
            })
            .on_global_pointer_move(move |event: Event<PointerEventData>| {
                if is_dragging() && track_width() > 0. {
                    let x = event.global_location().x as f32 - track_origin_x();
                    let new_value = (x / track_width()).clamp(0., 1.);
                    value.set(new_value);
                    if let Some(on_change) = &on_change_move {
                        on_change.call(new_value);
                    }
                }
            })
            .on_global_pointer_press(move |_| {
                if is_dragging() {
                    is_dragging.set(false);
                    if let Some(on_change_end) = &on_change_end {
                        on_change_end.call(value().clamp(0., 1.));
                    }
                }
            })
            .child(
                rect()
                    .width(Size::percent(progress * 100.))
                    .height(Size::Fill)
                    .background(self.fill_color),
            )
            .child(
                rect()
                    .position(Position::new_absolute().top(0.).left(thumb_x))
                    .layer(1i16)
                    .width(Size::px(THUMB_WIDTH))
                    .height(Size::px(TRACK_HEIGHT))
                    .background(self.thumb_color)
                    .direction(Direction::Horizontal)
                    .main_align(Alignment::Center)
                    .cross_align(Alignment::Center)
                    .spacing(2.)
                    .child(Self::thumb_stripe(self.thumb_stripe_color))
                    .child(Self::thumb_stripe(self.thumb_stripe_color))
                    .child(Self::thumb_stripe(self.thumb_stripe_color)),
            )
            .border(Border {
                fill: self.border_color.into(),
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
