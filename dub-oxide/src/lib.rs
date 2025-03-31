pub mod error;
pub mod formats;
pub mod opts;
pub mod result;

use error::Error;
pub use formats::wav::WavSplitter;
use hound::WavSpec;
use num::ToPrimitive;
use opts::SplitOpts;
pub use result::AudioChunk;
use result::SplitResult;

pub trait AudioSplitter {
    type ByteSize: num::Num + Sized + ToPrimitive + Clone;
    type CodecParams: BytesPerMillisecond;
    fn split_audio(
        &mut self,
        opts: SplitOpts,
    ) -> Result<SplitResult<Self::ByteSize, Self::CodecParams>, Error>;
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
