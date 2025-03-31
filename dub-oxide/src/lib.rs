pub mod error;
pub mod formats;
pub mod opts;
pub mod result;

use std::time::Duration;

use error::Error;
pub use formats::wav::WavSplitter;
use hound::{Sample, WavSpec};
use num::{NumCast, ToPrimitive};
use opts::SplitOpts;
pub use result::AudioChunk;

pub trait AudioSplitter {
    type ByteSize: num::Num + Sized + ToPrimitive + Clone;
    fn split_audio(&mut self, opts: SplitOpts) -> Result<Vec<AudioChunk<Self::ByteSize>>, Error>;
}

pub trait BytesPerMillisecond {
    fn bytes_per_ms(&self) -> usize;
}

impl BytesPerMillisecond for WavSpec {
    fn bytes_per_ms(&self) -> usize {
        let sample_rate = self.sample_rate as u32;
        let channels = self.channels as u32;

        // Calculate bytes per millisecond
        ((sample_rate * channels) / 1000) as usize
    }
}
