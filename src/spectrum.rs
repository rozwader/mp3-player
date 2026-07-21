//! Spectrum analysis of the audio currently flowing through the player.
//!
//! Pipeline: rodio Source (samples) -> SampleTap ring buffer -> FFT -> 32 bands.
//! We tap the live playback stream instead of decoding MP3 twice, so the bars
//! stay in sync with what you hear (including seeks and pauses).

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

use realfft::{RealFftPlanner, RealToComplex};
use rodio::{source::SeekError, ChannelCount, Sample, SampleRate, Source};

pub const BAND_COUNT: usize = 32;
pub const FFT_SIZE: usize = 1024;
const RING_CAPACITY: usize = FFT_SIZE * 2;
const MIN_FREQ_HZ: f32 = 40.;

fn forward_fft() -> &'static Arc<dyn RealToComplex<f32>> {
    static FFT: OnceLock<Arc<dyn RealToComplex<f32>>> = OnceLock::new();
    FFT.get_or_init(|| {
        let mut planner = RealFftPlanner::<f32>::new();
        planner.plan_fft_forward(FFT_SIZE)
    })
}

/// Shared mono sample ring filled by the audio thread.
#[derive(Clone)]
pub struct SampleTap {
    inner: Arc<Mutex<SampleTapInner>>,
}

struct SampleTapInner {
    samples: VecDeque<f32>,
    capacity: usize,
    /// EMA of the last computed bands (attack/release smoothing).
    smoothed: [f32; BAND_COUNT],
    /// Adaptive peak for AGC — rises fast, falls slowly so bars stay relative.
    peak: f32,
}

impl SampleTap {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SampleTapInner {
                samples: VecDeque::with_capacity(RING_CAPACITY),
                capacity: RING_CAPACITY,
                smoothed: [0.; BAND_COUNT],
                peak: 0.02,
            })),
        }
    }

    pub fn clear(&self) {
        let mut inner = self.inner.lock().expect("sample tap poisoned");
        inner.samples.clear();
        inner.smoothed = [0.; BAND_COUNT];
        inner.peak = 0.02;
    }

    pub fn push_mono(&self, sample: f32) {
        let mut inner = self.inner.lock().expect("sample tap poisoned");
        if inner.samples.len() >= inner.capacity {
            inner.samples.pop_front();
        }
        inner.samples.push_back(sample);
    }

    /// Runs FFT on the latest window and returns 32 dynamically scaled amplitudes.
    pub fn bands(&self) -> [f32; BAND_COUNT] {
        let mut inner = self.inner.lock().expect("sample tap poisoned");

        if inner.samples.len() < FFT_SIZE {
            Self::decay_smoothed(&mut inner.smoothed, 0.85);
            inner.peak = (inner.peak * 0.95).max(0.02);
            return inner.smoothed;
        }

        let start = inner.samples.len() - FFT_SIZE;
        let mut windowed = vec![0f32; FFT_SIZE];
        for (i, sample) in inner.samples.iter().skip(start).enumerate() {
            // Hann window reduces spectral leakage.
            let hann = 0.5
                * (1. - (2. * std::f32::consts::PI * i as f32 / (FFT_SIZE as f32 - 1.)).cos());
            windowed[i] = sample * hann;
        }

        // 44.1kHz is a safe default for band edges; relative spacing still looks fine at 48kHz.
        let sample_rate = 44_100f32;
        let raw = fft_to_bands(&windowed, sample_rate);

        // AGC: skaluj względem rolling peak, żeby nic nie „przyklejało się” do sufitu.
        let frame_peak = raw.iter().copied().fold(0f32, f32::max);
        if frame_peak > inner.peak {
            inner.peak = frame_peak;
        } else {
            inner.peak = inner.peak * 0.92 + frame_peak * 0.08;
        }
        inner.peak = inner.peak.max(0.02);
        let gain = 0.92 / inner.peak;

        for (smoothed, &raw_band) in inner.smoothed.iter_mut().zip(raw.iter()) {
            let scaled = raw_band * gain;
            // Faster attack, slower release -> punchy bars that fall smoothly.
            let factor = if scaled > *smoothed { 0.6 } else { 0.22 };
            *smoothed += (scaled - *smoothed) * factor;
            if *smoothed < 0.002 {
                *smoothed = 0.;
            }
        }

        inner.smoothed
    }

    /// Soft fall toward silence (used while paused / stopped).
    pub fn decay(&self) -> [f32; BAND_COUNT] {
        let mut inner = self.inner.lock().expect("sample tap poisoned");
        Self::decay_smoothed(&mut inner.smoothed, 0.82);
        inner.peak = (inner.peak * 0.9).max(0.02);
        inner.smoothed
    }

    fn decay_smoothed(smoothed: &mut [f32; BAND_COUNT], factor: f32) {
        for band in smoothed {
            *band *= factor;
            if *band < 0.002 {
                *band = 0.;
            }
        }
    }
}

fn fft_to_bands(windowed: &[f32], sample_rate: f32) -> [f32; BAND_COUNT] {
    let fft = forward_fft();
    let mut input = windowed.to_vec();
    let mut spectrum = fft.make_output_vec();
    fft.process(&mut input, &mut spectrum).expect("fft");

    let bin_hz = sample_rate / FFT_SIZE as f32;
    let nyquist = sample_rate * 0.5;
    let mut bands = [0f32; BAND_COUNT];

    // Pełne spektrum logarytmiczne: ~40 Hz → Nyquist, 32 pasma.
    for band in 0..BAND_COUNT {
        let t0 = band as f32 / BAND_COUNT as f32;
        let t1 = (band + 1) as f32 / BAND_COUNT as f32;
        let freq_lo = MIN_FREQ_HZ * (nyquist / MIN_FREQ_HZ).powf(t0);
        let freq_hi = MIN_FREQ_HZ * (nyquist / MIN_FREQ_HZ).powf(t1);

        let bin_lo = ((freq_lo / bin_hz).floor() as usize).max(1);
        let bin_hi = ((freq_hi / bin_hz).ceil() as usize).min(spectrum.len() - 1);

        let mut sum = 0f32;
        let mut count = 0usize;
        for bin in bin_lo..=bin_hi {
            let re = spectrum[bin].re;
            let im = spectrum[bin].im;
            sum += (re * re + im * im).sqrt();
            count += 1;
        }
        let avg = if count > 0 { sum / count as f32 } else { 0. };
        // Bez clamp do 1 — AGC w `bands()` skaluje względnie do peaku.
        bands[band] = avg.sqrt().max(0.);
    }

    bands
}

/// Wraps a rodio [`Source`] and copies mono-downmixed samples into a [`SampleTap`].
pub struct TappedSource<S> {
    inner: S,
    tap: SampleTap,
    channels: u16,
    channel_pos: u16,
    frame_sum: f32,
}

impl<S: Source> TappedSource<S> {
    pub fn new(inner: S, tap: SampleTap) -> Self {
        let channels = inner.channels().max(1);
        Self {
            inner,
            tap,
            channels,
            channel_pos: 0,
            frame_sum: 0.,
        }
    }
}

impl<S: Source> Iterator for TappedSource<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next()?;
        self.frame_sum += sample;
        self.channel_pos += 1;
        if self.channel_pos >= self.channels {
            self.tap.push_mono(self.frame_sum / self.channels as f32);
            self.frame_sum = 0.;
            self.channel_pos = 0;
        }
        Some(sample)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<S: Source> Source for TappedSource<S> {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }

    fn channels(&self) -> ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }

    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.tap.clear();
        self.channel_pos = 0;
        self.frame_sum = 0.;
        self.inner.try_seek(pos)
    }
}
