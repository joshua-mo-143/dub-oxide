use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek},
    path::Path,
};

use crate::{
    BytesPerMillisecond,
    error::Error,
    formats::common::{bytes_to_timestamp, find_silent_position},
};

use hound::{WavReader, WavSpec};

use crate::{AudioChunk, AudioSplitter, SplitOpts};

pub struct WavSplitter<R> {
    reader: WavReader<R>,
}

impl WavSplitter<BufReader<File>> {
    pub fn from_file_path<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let reader = WavReader::open(path)?;

        Ok(Self { reader })
    }

    pub fn codec(&self) -> WavSpec {
        self.reader.spec()
    }
}

impl<'a> WavSplitter<Cursor<&'a [u8]>> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, Error> {
        let reader = WavReader::new(Cursor::new(bytes))?;

        Ok(Self { reader })
    }

    pub fn codec(&self) -> WavSpec {
        self.reader.spec()
    }
}

impl<R> WavSplitter<R>
where
    R: Read + Seek,
{
    fn get_bytes(&mut self) -> Result<Vec<i16>, Error> {
        let samples: Vec<i16> = self
            .reader
            .samples::<i16>()
            .filter_map(|x| x.ok())
            .collect();

        if samples.len() as u32 != self.reader.len() {
            return Err(Error::inconsistent_byte_length(
                samples.len(),
                self.reader.len() as usize,
            ));
        }
        let samples_len = samples.len();

        #[cfg(feature = "tracing")]
        tracing::trace!("{samples_len} samples loaded.");

        Ok(samples)
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        self.reader.seek(0)?;

        Ok(())
    }
}

impl<R> AudioSplitter for WavSplitter<R>
where
    R: Read + Seek,
{
    type ByteSize = i16;

    fn split_audio(&mut self, opts: SplitOpts) -> Result<Vec<AudioChunk<Self::ByteSize>>, Error> {
        let byte_limit = opts.frame_size();
        let bytes_per_ms = self.reader.spec().bytes_per_ms() as usize;

        let mut bigvec = Vec::new();

        let mut offset: usize = 0;

        let bytes = self.get_bytes()?;

        #[cfg(feature = "tracing")]
        tracing::trace!("Bytes length:{}", bytes.len());

        loop {
            if offset >= bytes.len() {
                break;
            }
            if bytes[offset..bytes.len()].len() <= byte_limit {
                let timestamp_start = bytes_to_timestamp(offset, bytes_per_ms);
                let timestamp_end = bytes_to_timestamp(bytes.len(), bytes_per_ms);

                let audio_chunk =
                    AudioChunk::new(&bytes[offset..bytes.len()], timestamp_start, timestamp_end);

                bigvec.push(audio_chunk);
                offset = bytes.len();
                break;
            }

            let end_pos = if offset + byte_limit >= bytes.len() {
                bytes.len()
            } else {
                offset + byte_limit
            };

            let pos = if let Some(threshold) = opts.silence_threshold() {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "Searching for a chunk between position {offset} and position {end_pos}"
                );

                let pos = if let Some(pos) = find_silent_position::<_, Self>(
                    &bytes[offset..end_pos],
                    bytes_per_ms * 50,
                    threshold,
                ) {
                    pos + offset
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::info!("Could not find chunk between X and X");

                    offset + byte_limit
                };
                pos
            } else {
                offset + byte_limit
            };

            let timestamp_start = bytes_to_timestamp(offset, bytes_per_ms);
            let timestamp_end = bytes_to_timestamp(pos, bytes_per_ms);

            let audiochunk = AudioChunk::new(&bytes[offset..pos], timestamp_start, timestamp_end);

            #[cfg(feature = "tracing")]
            tracing::debug!("Created chunk at timestamp {timestamp_start}ms to {timestamp_end}ms");

            bigvec.push(audiochunk);
            offset = pos;
        }

        Ok(bigvec)
    }
}
