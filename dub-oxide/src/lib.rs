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
use symphonia::core::codecs::CodecParameters;

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
        let sample_rate = self.sample_rate;
        let channels = self.channels as u32;

        // Calculate bytes per millisecond
        ((sample_rate * channels) / 1000) as usize
    }
}

impl BytesPerMillisecond for CodecParameters {
    fn bytes_per_ms(&self) -> usize {
        let sample_rate = self.sample_rate.unwrap();
        let channels = self.channels.unwrap().count();
        let bits_per_sample = self.bits_per_sample.unwrap();

        (sample_rate as usize * channels) / 1000
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::BytesPerMillisecond;
    use hound::{SampleFormat, WavSpec};
    use symphonia::core::formats::FormatReader;
    use symphonia::core::{
        formats::FormatOptions,
        io::{MediaSourceStream, MediaSourceStreamOptions},
    };

    #[test]
    fn wavspec_bytes_per_ms_works() {
        let wavspec = WavSpec {
            sample_rate: 44100,
            sample_format: SampleFormat::Int,
            bits_per_sample: 16,
            channels: 2,
        };

        assert_eq!(wavspec.bytes_per_ms(), 88);
    }

    #[test]
    fn codec_params_bytes_per_ms_works() {
        let bytes = std::fs::read("../test_files/test.wav").unwrap();
        let src = Box::new(Cursor::new(bytes));
        let opts = MediaSourceStreamOptions::default();
        let stream = MediaSourceStream::new(src, opts);

        let wav_reader =
            symphonia::default::formats::WavReader::try_new(stream, &FormatOptions::default())
                .unwrap();

        let metadata = wav_reader.default_track().unwrap().codec_params.clone();

        assert_eq!(metadata.sample_rate.unwrap(), 44100);
        assert_eq!(metadata.bits_per_sample.unwrap(), 16);
        assert_eq!(metadata.channels.unwrap().count(), 2);

        assert_eq!(metadata.bytes_per_ms(), 88);
    }
}
