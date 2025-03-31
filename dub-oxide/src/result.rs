use std::{ops::Deref, slice::Iter, vec::IntoIter};

pub struct AudioChunk<T> {
    bytes: Vec<T>,
    idx: usize,
    timestamp_start: usize,
    timestamp_end: usize,
}

impl<T> AudioChunk<T>
where
    T: Clone,
{
    pub fn new(data: &[T], timestamp_start: usize, timestamp_end: usize) -> Self {
        Self {
            bytes: data.to_vec(),
            idx: 0,
            timestamp_start,
            timestamp_end,
        }
    }

    pub fn timestamp_start(&self) -> usize {
        self.timestamp_start
    }

    pub fn timestamp_end(&self) -> usize {
        self.timestamp_end
    }
}

impl AudioChunk<i16> {
    pub fn to_bytes_vec(&self) -> Vec<u8> {
        self.bytes
            .clone()
            .into_iter()
            .flat_map(|x| x.to_le_bytes())
            .collect()
    }
}

impl<T> Deref for AudioChunk<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.bytes.as_slice()
    }
}

impl Iterator for AudioChunk<i16> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.bytes.len() {
            let res = self.bytes[self.idx];
            self.idx += 1;
            Some(res) // Consumes elements
        } else {
            None
        }
    }
}

pub struct SplitResult<T, C> {
    chunks: Vec<AudioChunk<T>>,
    codec_params: C,
}

impl<T, C> SplitResult<T, C> {
    pub fn new(chunks: Vec<AudioChunk<T>>, codec_params: C) -> Self {
        Self {
            chunks,
            codec_params,
        }
    }

    pub fn chunks(&self) -> &[AudioChunk<T>] {
        &self.chunks
    }

    pub fn into_chunks(self) -> Vec<AudioChunk<T>> {
        self.chunks
    }

    pub fn iter(&self) -> Iter<AudioChunk<T>> {
        self.chunks.iter()
    }

    pub fn into_iter(self) -> IntoIter<AudioChunk<T>> {
        self.chunks.into_iter()
    }

    pub fn get_codec(&self) -> &C {
        &self.codec_params
    }

    pub fn get_codec_owned(&self) -> C
    where
        C: Clone,
    {
        self.codec_params.clone()
    }
}

impl<C> SplitResult<i16, C> {
    pub fn into_u8_vec(self) -> Vec<u8> {
        self.chunks
            .into_iter()
            .flat_map(|x| x.to_bytes_vec())
            .collect()
    }
}

impl<T, C> Deref for SplitResult<T, C> {
    type Target = [AudioChunk<T>];

    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}
