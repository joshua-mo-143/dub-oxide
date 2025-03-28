use std::{
    io::{Cursor, Read, Seek},
    path::Path,
    time::Duration,
};

use hound::WavReader;

use crate::{AudioSplitter, TODO, find_silent_position};
pub struct WavSplitter<R>(WavReader<R>);

impl WavSplitter<std::io::BufReader<std::fs::File>> {
    pub fn from_file_path<P>(path: P) -> Result<Self, TODO>
    where
        P: AsRef<Path>,
    {
        let reader = WavReader::open(path)?;

        Ok(Self(reader))
    }
}

impl<'a> WavSplitter<Cursor<&'a [u8]>> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, TODO> {
        let res = WavReader::new(Cursor::new(bytes))?;
        Ok(Self(res))
    }
}

impl<R> WavSplitter<R>
where
    R: Read + Seek,
{
    fn get_bytes(&mut self) -> Result<Vec<i16>, TODO> {
        let bytes: Vec<i16> = self.0.samples::<i16>().filter_map(|x| x.ok()).collect();

        if bytes.len() as u32 != self.0.len() {
            return Err("There was an encoding error".into());
        }

        Ok(bytes)
    }

    pub fn reset(&mut self) -> Result<(), TODO> {
        self.0.seek(0)?;

        Ok(())
    }
}

impl<R> AudioSplitter for WavSplitter<R>
where
    R: Read + Seek,
{
    type ByteSize = i16;

    fn split_by_time_segment(
        &mut self,
        duration: Duration,
    ) -> Result<Vec<Vec<Self::ByteSize>>, TODO> {
        let byte_limit = self.get_frame_size_from_duration(duration);

        let mut bigvec: Vec<Vec<Self::ByteSize>> = Vec::new();

        let mut offset: usize = 0;

        let bytes = self.get_bytes()?;

        #[cfg(feature = "tracing")]
        tracing::trace!("Bytes length:{}", bytes.len());

        loop {
            if bytes[offset..bytes.len()].len() <= byte_limit {
                bigvec.push(bytes[offset..bytes.len()].to_vec());
                return Ok(bigvec);
            }

            let end_pos = if offset + byte_limit >= bytes.len() {
                bytes.len()
            } else {
                offset + byte_limit
            };

            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Searching for a chunk between position {offset} and position {end_pos}"
            );

            let Some(pos) = find_silent_position(&bytes[offset..end_pos], self) else {
                return Err("Could not find an area of silence.".into());
            };

            let pos = pos + offset;

            #[cfg(feature = "tracing")]
            tracing::trace!("Position: {pos}");

            if pos >= bytes.len() {
                break;
            }

            bigvec.push(bytes[offset..pos].to_vec());
            offset = pos;
        }

        Ok(bigvec)
    }

    fn split_by_size_limit(&mut self, byte_limit: usize) -> Result<Vec<Vec<Self::ByteSize>>, TODO> {
        let mut bigvec: Vec<Vec<Self::ByteSize>> = Vec::new();

        let mut offset: usize = 0;

        let bytes = self.get_bytes().unwrap();

        #[cfg(feature = "tracing")]
        tracing::trace!("Bytes length:{}", bytes.len());

        loop {
            if bytes[offset..bytes.len()].len() <= byte_limit {
                bigvec.push(bytes[offset..bytes.len()].to_vec());
                return Ok(bigvec);
            }

            let end_pos = if offset + byte_limit >= bytes.len() {
                bytes.len()
            } else {
                offset + byte_limit
            };

            #[cfg(feature = "tracing")]
            tracing::trace!(
                "Searching for a chunk between position {offset} and position {end_pos}"
            );

            let Some(pos) = find_silent_position(&bytes[offset..end_pos], self) else {
                return Err("Could not find an area of silence.".into());
            };

            let pos = pos + offset;

            #[cfg(feature = "tracing")]
            println!("{pos}");

            if pos >= bytes.len() {
                break;
            }

            bigvec.push(bytes[offset..pos].to_vec());
            offset = pos;
        }

        Ok(bigvec)
    }

    fn get_bytes_per_ms(&self) -> u32 {
        let spec = self.0.spec();
        let sample_rate = spec.sample_rate as u32;
        let channels = spec.channels as u32;

        // Calculate bytes per millisecond
        (sample_rate * channels) / 1000
    }
}
