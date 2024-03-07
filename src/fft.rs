use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use realfft::{num_complex::Complex, RealFftPlanner, RealToComplex};

use crate::sound::{FloatOut, SAMPLE_RATE};

const FFT_BUFFER_TIME: f32 = 0.1;
const FFT_BUFFER_SIZE: usize = (SAMPLE_RATE as f32 * FFT_BUFFER_TIME) as usize;
static FFT_RESOURCES: Lazy<Mutex<FftResources>> = Lazy::new(|| Default::default());

struct FftResources {
    fft: Arc<dyn RealToComplex<FloatOut>>,
    scratch: [Complex<FloatOut>; FFT_BUFFER_SIZE],
    output: [Complex<FloatOut>; FFT_BUFFER_SIZE],
}

impl Default for FftResources {
    fn default() -> Self {
        FftResources {
            fft: RealFftPlanner::new().plan_fft_forward(FFT_BUFFER_SIZE),
            scratch: [Complex::new(0.0, 0.0); FFT_BUFFER_SIZE],
            output: [Complex::new(0.0, 0.0); FFT_BUFFER_SIZE],
        }
    }
}

pub fn fft(buffer: &mut [FloatOut]) {
    let mut fft_resources = FFT_RESOURCES.lock().unwrap();
    // let fft = &fft_resources.fft;
    // let output = &mut fft_resources.output;
    // let scratch = &mut fft_resources.scratch;
    // fft.process_with_scratch(buffer, output, scratch).unwrap();
}
