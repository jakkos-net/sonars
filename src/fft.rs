use itertools::Itertools;
use spectrum_analyzer::{
    samples_fft_to_spectrum, scaling::divide_by_N_sqrt, windows::hann_window, FrequencyLimit,
};

use crate::sound::{FloatOut, SAMPLE_RATE};

// needs to be power of 2
pub const FFT_BUFFER_SIZE: usize = 16384;

pub fn fft(buffer: &[FloatOut]) -> anyhow::Result<impl Iterator<Item = FreqMag>> {
    let hann_window = hann_window(buffer);
    // calc spectrum
    let spectrum_hann_window = samples_fft_to_spectrum(
        // (windowed) samples
        &hann_window,
        // sampling rate
        SAMPLE_RATE,
        // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
        FrequencyLimit::All,
        // optional scale
        Some(&divide_by_N_sqrt),
    )
    .unwrap();

    let output = spectrum_hann_window
        .data()
        .into_iter()
        .cloned()
        .collect_vec();

    Ok(output.into_iter().map(|(f, m)| FreqMag {
        freq: f.val(),
        mag: m.val(),
    }))
}

#[derive(Clone, Debug)]
pub struct FreqMag {
    pub freq: f32,
    pub mag: f32,
}
