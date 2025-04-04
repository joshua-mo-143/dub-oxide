use std::{io::Cursor, path::Path, time::Duration};

use symphonia::core::{
    codecs::CodecParameters,
    conv::IntoSample,
    formats::{FormatOptions, FormatReader},
    io::{MediaSourceStream, MediaSourceStreamOptions},
};

use symphonia::default::formats::AiffReader;

use crate::{AudioChunk, error::Error, opts::SplitOpts, result::SplitResult};

pub struct AiffSplitter {
    reader: AiffReader,
}

impl AiffSplitter {
    pub fn codec(&self) -> CodecParameters {
        self.reader.default_track().unwrap().codec_params.clone()
    }
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = std::fs::read(path)?;
        let src = Box::new(Cursor::new(bytes));
        let opts = MediaSourceStreamOptions::default();
        let stream = MediaSourceStream::new(src, opts);

        let reader =
            symphonia::default::formats::AiffReader::try_new(stream, &FormatOptions::default())?;

        Ok(Self { reader })
    }

    pub fn from_u8_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        let src = Box::new(Cursor::new(bytes));
        let opts = MediaSourceStreamOptions::default();
        let stream = MediaSourceStream::new(src, opts);

        let reader =
            symphonia::default::formats::AiffReader::try_new(stream, &FormatOptions::default())?;

        Ok(Self { reader })
    }
}

impl crate::AudioSplitter for AiffSplitter {
    type ByteSize = u8;
    type CodecParams = CodecParameters;

    fn split_audio(
        &mut self,
        opts: SplitOpts,
    ) -> Result<SplitResult<Self::ByteSize, Self::CodecParams>, Error> {
        let byte_limit = opts.frame_size();

        let mut bigvec: Vec<AudioChunk<Self::ByteSize>> = Vec::new();

        let mut current_vec: Vec<u8> = Vec::new();

        while let Ok(packet) = self.reader.next_packet() {
            let packet_buf = packet.buf();
            if (packet_buf.len() + current_vec.len()) > byte_limit {
                let limit = byte_limit - current_vec.len();
                current_vec.extend(&packet_buf[0..limit]);
                let audio_chunk = AudioChunk::new(&current_vec, 0, 0);
                bigvec.push(audio_chunk);
                current_vec.clear();
                current_vec.extend(&packet_buf[0..limit]);
            } else {
                current_vec.extend(packet.buf());
            }
        }

        Ok(SplitResult::new(bigvec, self.codec()))
    }
}
