pub mod formats;

use std::time::Duration;

pub use formats::wav::WavSplitter;
use hound::Sample;
use num::{NumCast, ToPrimitive};

pub trait AudioSplitter {
    type ByteSize: num::Num + Sized + ToPrimitive + Clone;
    fn split_by_time_segment(&mut self, duration: Duration) -> Vec<Vec<Self::ByteSize>>;
    fn split_by_size_limit(&mut self, byte_limit: usize) -> Vec<Vec<Self::ByteSize>>;
    fn get_bytes_per_ms(&self) -> u32;

    fn get_frame_size_from_duration(&self, duration: Duration) -> usize {
        let duration_as_millis = duration.as_millis() as usize;

        let bytes_per_ms = self.get_bytes_per_ms() as usize;

        (duration_as_millis * bytes_per_ms) as usize
    }

    fn rms<T>(&self, samples: Vec<T>) -> f32
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

    fn dbfs(&self, rms: f32) -> f32 {
        if rms == 0.0 {
            -100.0
        } else {
            20.0 * rms.log10()
        }
    }
}

type TODO = Box<dyn std::error::Error + 'static>;

fn find_silent_position<T, S>(bytes: &[T], splitter: &S) -> Option<usize>
where
    T: PartialEq + Copy + num::Num + Default + Sample + NumCast,
    S: AudioSplitter,
{
    let s = splitter.get_frame_size_from_duration(Duration::from_millis(50));

    for (i, chunk) in bytes.chunks(s).enumerate().rev() {
        let rms_value = splitter.rms(chunk.to_owned());
        let db = splitter.dbfs(rms_value);

        if db <= -20.0 {
            return Some(i * s);
        }
    }

    Some(bytes.len())
}
