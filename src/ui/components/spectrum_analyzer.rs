use freya::{
    elements::{
        extensions::{
            ChildrenExt, ContainerSizeExt, ContainerWithContentExt, StyleExt,
        },
        rect::rect,
    }, prelude::{
        Alignment, Border, BorderAlignment, BorderWidth, Component, CornerRadius, Direction, IntoElement, Size,
    },
};

use crate::spectrum::BAND_COUNT;

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
        let max_height = self.max_height;

        rect()
            .width(Size::Fill)
            .height(Size::px(max_height))
            .background((0, 0, 0))
            .direction(Direction::Horizontal)
            .main_align(Alignment::SpaceAround)
            .cross_align(Alignment::End)
            .spacing(2.)
            .border(Border {
                fill: (148, 148, 148).into(),
                alignment: BorderAlignment::Outer,
                width: BorderWidth {
                    top: 0.,
                    bottom: 2.,
                    left: 0.,
                    right: 2.,
                },
            })
            .children(self.values.iter().map(|value| {
                // Wartości są już skalowane AGC; lekki headroom żeby peak nie „kleił” się do góry.
                let height = (value.max(0.) * max_height * 0.95).max(1.);
                rect()
                    .width(Size::px(10.))
                    .height(Size::px(height))
                    .background((0, 255, 0))
                    .corner_radius(CornerRadius {
                        top_left: 3.,
                        top_right: 3.,
                        bottom_left: 0.,
                        bottom_right: 0.,
                        smoothing: 2.
                    })
                    .into_element()
            }))
    }
}
