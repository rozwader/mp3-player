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

use crate::ui::theme::{ColorType, use_theme};

const TRACK_HEIGHT: f32 = 14.;
const THUMB_WIDTH: f32 = 16.;

pub type Rgb = ColorType;

/// Slider sterowany z zewnątrz. Wartość (0.0..=1.0) trzyma rodzic w `use_state`.
///
/// Kolory thumb/stripe/border biorą się z theme; fill/track można nadpisać builderem
/// (domyślnie: `volume_bg` / `volume_hov_bg`).
///
/// - `on_change` — przy każdym ruchu (np. głośność na żywo)
/// - `on_drag_start` — przy rozpoczęciu przeciągania
/// - `on_change_end` — dopiero po puszczeniu (np. seek utworu)
pub struct Slider {
    value: State<f32>,
    width: Size,
    fill_color: Option<Rgb>,
    track_color: Option<Rgb>,
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
    }
}

impl Slider {
    pub fn new(value: State<f32>, width: Size) -> Self {
        Self {
            value,
            width,
            fill_color: None,
            track_color: None,
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
        self.fill_color = Some(color);
        self
    }

    pub fn track_color(mut self, color: Rgb) -> Self {
        self.track_color = Some(color);
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
        let theme = use_theme()();
        let fill_color = self.fill_color.unwrap_or(theme.volume_bg);
        let track_color = self.track_color.unwrap_or(theme.volume_hov_bg);
        let thumb_color = theme.thumb;
        let thumb_stripe_color = theme.thumb_stripe;
        let border_color = theme.border;

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
            .background(track_color)
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
                    .background(fill_color),
            )
            .child(
                rect()
                    .position(Position::new_absolute().top(0.).left(thumb_x))
                    .layer(1i16)
                    .width(Size::px(THUMB_WIDTH))
                    .height(Size::px(TRACK_HEIGHT))
                    .background(thumb_color)
                    .direction(Direction::Horizontal)
                    .main_align(Alignment::Center)
                    .cross_align(Alignment::Center)
                    .spacing(2.)
                    .child(Self::thumb_stripe(thumb_stripe_color))
                    .child(Self::thumb_stripe(thumb_stripe_color))
                    .child(Self::thumb_stripe(thumb_stripe_color)),
            )
            .border(Border {
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
