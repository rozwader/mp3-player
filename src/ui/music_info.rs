use std::{rc::Rc, time::Duration};

use freya::{
    elements::{
        extensions::{ChildrenExt, ContainerExt, ContainerSizeExt, ContainerWithContentExt}, rect::rect,
    }, prelude::{
        Alignment, Component, Direction, Element, IntoElement, Size, WritableUtils, spawn, use_consume, use_hook, use_state,
    },
};

use crate::{audio::AudioPlayer, ui::components::{music_timer::MusicTimer, music_title::MusicTitle, slider::Slider}};

const PROGRESS_FILL_COLOR: (u8, u8, u8) = (58, 64, 100);
const PROGRESS_TRACK_COLOR: (u8, u8, u8) = (28, 32, 54);
const PROGRESS_THUMB_COLOR: (u8, u8, u8) = (224, 176, 66);
const PROGRESS_THUMB_STRIPE_COLOR: (u8, u8, u8) = (120, 90, 30);
const PROGRESS_BORDER_COLOR: (u8, u8, u8) = (10, 12, 24);

#[derive(PartialEq)]
pub struct MusicInfo {
    controls: Element,
}

impl MusicInfo {
    pub fn new(controls: Element) -> Self {
        Self { controls }
    }
}

impl Component for MusicInfo {
    fn render(&self) -> impl IntoElement {
        let player = use_consume::<Rc<AudioPlayer>>();

        let mut title = use_state({
            let player = player.clone();
            move || player.title()
        });
        let mut total_secs = use_state({
            let player = player.clone();
            move || player.duration().as_secs().max(1) as u16
        });

        let length_text = format!("({}:{:02})", total_secs() / 60, total_secs() % 60);

        let mut music_timer_current = use_state(|| 0u16);
        let mut currently_playing = use_state(|| false);

        let volume = use_state(|| 1.0f32);
        let mut music_progress = use_state(|| 0.0f32);

        use_hook({
            let player = player.clone();
            move || {
                spawn(async move {
                    loop {
                        async_io::Timer::after(Duration::from_millis(250)).await;

                        // Koniec utworu - przeskocz do następnego (next() zapętla playlistę)
                        if player.is_finished() {
                            player.next();
                        }

                        // Tytuł i długość mogą się zmienić po przełączeniu utworu przyciskami
                        title.set_if_modified(player.title());
                        total_secs.set_if_modified(player.duration().as_secs().max(1) as u16);

                        let position_secs = player.position().as_secs() as u16;
                        music_timer_current.set_if_modified(position_secs);
                        music_progress
                            .set_if_modified(position_secs as f32 / total_secs() as f32);
                        currently_playing.set_if_modified(player.is_playing());
                    }
                });
            }
        });

        let seek_player = player.clone();
        let volume_player = player.clone();

        rect()
            .width(Size::Fill)
            .direction(Direction::Vertical)
            .spacing(10.)
            .child(
                rect()
                    .width(Size::Fill)
                    .height(Size::px(50.))
                    .direction(Direction::Horizontal)
                    .spacing(5.)
                    .cross_align(Alignment::SpaceBetween)
                    .main_align(Alignment::SpaceBetween)
                    .child(MusicTimer::new(
                        music_timer_current(),
                        currently_playing(),
                    ))
                    .child(
                        rect()
                            .width(Size::Fill)
                            .direction(Direction::Vertical)
                            .main_align(Alignment::Start)
                            .cross_align(Alignment::Start)
                            .spacing(5.)
                            .child(MusicTitle::new((&*title.read().clone().to_string()).to_string(), length_text))
                            .child(
                                Slider::new(volume, Size::Fill)
                                    .on_change(move |new_volume| {
                                        volume_player.set_volume(new_volume);
                                    })
                            )
                    )
            )
            .child(self.controls.clone())
            .child(
                Slider::new(music_progress, Size::Fill)
                    .fill_color(PROGRESS_FILL_COLOR)
                    .track_color(PROGRESS_TRACK_COLOR)
                    .thumb_color(PROGRESS_THUMB_COLOR)
                    .thumb_stripe_color(PROGRESS_THUMB_STRIPE_COLOR)
                    .on_change(move |progress| {
                        let target = Duration::from_secs_f32(progress * total_secs() as f32);
                        seek_player.seek(target);
                        music_timer_current.set((progress * total_secs() as f32) as u16);
                    })
            )
    }
}
