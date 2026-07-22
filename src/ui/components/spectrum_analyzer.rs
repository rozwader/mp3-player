use freya::{
    components::SvgViewer,
    elements::{
        extensions::{ChildrenExt, ContainerSizeExt, ContainerWithContentExt, StyleExt},
        rect::rect,
    },
    icons::lucide::paintbrush_vertical,
    prelude::{
        Alignment, Border, BorderAlignment, BorderWidth, Component, CornerRadius, Direction,
        EventHandler, IntoElement, Size, WritableUtils,
    },
};

use crate::{
    spectrum::BAND_COUNT,
    ui::{
        components::button::Button,
        theme::{next_theme, use_theme},
    },
};

const DEFAULT_MAX_HEIGHT: f32 = 80.;

#[derive(PartialEq)]
pub struct SpectrumAnalyzer {
    values: [f32; BAND_COUNT],
    max_height: f32,
}

impl SpectrumAnalyzer {
    pub fn new(values: [f32; BAND_COUNT]) -> Self {
        Self {
            values,
            max_height: DEFAULT_MAX_HEIGHT,
        }
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }
}

impl Component for SpectrumAnalyzer {
    fn render(&self) -> impl IntoElement {
        let mut theme_state = use_theme();
        let theme = theme_state();
        let max_height = self.max_height;
        let spectrum = theme.spectrum_theme;
        let bar_fill = spectrum.fill.unwrap_or(theme.text);
        let bar_width = spectrum.width;

        rect()
            .width(Size::Fill)
            .height(Size::Inner)
            .direction(Direction::Vertical)
            .spacing(10.)
            .child(
                rect().width(Size::Fill).direction(Direction::Horizontal).child(
                    Button::icon(
                        EventHandler::new(move |_| {
                            theme_state.set(next_theme(theme_state()));
                        }),
                        Some(Box::new(|color| {
                            SvgViewer::new(("paintbrush-vertical", paintbrush_vertical()))
                                .color(color)
                                .into_element()
                        })),
                    ),
                ),
            )
            .child(
                rect()
                    .width(Size::Fill)
                    .height(Size::px(max_height))
                    .background(theme.bg)
                    .direction(Direction::Horizontal)
                    .main_align(Alignment::SpaceAround)
                    .cross_align(Alignment::End)
                    .spacing(2.)
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
                    .children(self.values.iter().map(|value| {
                        let height = (value.max(0.) * max_height * 0.95).max(1.);
                        let mut bar = rect()
                            .width(Size::px(bar_width))
                            .height(Size::px(height))
                            .corner_radius(CornerRadius {
                                top_left: 3.,
                                top_right: 3.,
                                bottom_left: 0.,
                                bottom_right: 0.,
                                smoothing: 2.,
                            });

                        if let Some(border) = spectrum.border {
                            bar = bar.border(Border {
                                fill: border.into(),
                                alignment: BorderAlignment::Inner,
                                width: BorderWidth {
                                    top: 1.,
                                    right: 1.,
                                    bottom: 0.,
                                    left: 1.,
                                },
                            });
                        } else {
                            bar = bar.background(bar_fill);
                        }

                        bar.into_element()
                    })),
            )
    }
}
