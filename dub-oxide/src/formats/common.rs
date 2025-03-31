use hound::Sample;
use num::NumCast;

use crate::AudioSplitter;

pub fn dbfs(rms: f32) -> f32 {
    if rms == 0.0 {
        -100.0
    } else {
        20.0 * rms.log10()
    }
}

pub fn rms<T>(samples: Vec<T>) -> f32
where
    T: num::NumCast + Clone + hound::Sample,
{
    let sum_sq: f32 = samples
        .iter()
        .cloned()
        .map(|s| {
            let sample: f32 = NumCast::from(s).unwrap();
            (sample / i16::MAX as f32).powi(2)
        })
        .sum();
    (sum_sq / samples.len() as f32).sqrt()
}

pub fn find_silent_position<T, S>(
    bytes: &[T],
    frame_size: usize,
    volume_threshold: f32,
) -> Option<usize>
where
    T: PartialEq + Copy + num::Num + Default + Sample + NumCast,
    S: AudioSplitter,
{
    for (i, chunk) in bytes.chunks(frame_size).enumerate().rev() {
        let rms_value = rms::<T>(chunk.to_owned());
        let db = dbfs(rms_value);

        if db <= volume_threshold {
            return Some(i * frame_size);
        }
    }

    Some(bytes.len())
}

pub fn bytes_to_timestamp(offset: usize, bytes_per_ms: usize) -> usize {
    let timestamp = ((offset / bytes_per_ms) as f64).floor();
    timestamp as usize
}
