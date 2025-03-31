use std::time::Duration;

use crate::{BytesPerMillisecond, error::Error};

pub struct SplitOpts {
    frame_size: usize,
    silence_threshold: Option<f32>,
}

impl SplitOpts {
    pub fn builder<C>() -> SplitOptsBuilder<C>
    where
        C: BytesPerMillisecond,
    {
        SplitOptsBuilder::new()
    }

    pub fn silence_threshold(&self) -> Option<f32> {
        self.silence_threshold
    }

    pub fn frame_size(&self) -> usize {
        self.frame_size
    }
}

pub struct SplitOptsBuilder<C> {
    codec: Option<C>,
    duration_chunk_criteria: Option<Duration>,
    memsize_chunk_criteria: Option<usize>,
    /// The silence threshold. If None, there will be no silence threshold.
    silence_threshold: Option<f32>,
}

impl<C> SplitOptsBuilder<C>
where
    C: BytesPerMillisecond,
{
    fn new() -> Self {
        Self::default()
    }

    pub fn codec(mut self, codec: C) -> Self {
        self.codec = Some(codec);

        self
    }

    pub fn split_by_duration(mut self, duration: Duration) -> Self {
        self.duration_chunk_criteria = Some(duration);

        self
    }

    pub fn split_by_memsize(mut self, size: usize) -> Self {
        self.memsize_chunk_criteria = Some(size);

        self
    }

    pub fn silence_threshold(mut self, threshold: f32) -> Self {
        self.silence_threshold = Some(threshold);

        self
    }

    pub fn build(self) -> Result<SplitOpts, Error> {
        if self.duration_chunk_criteria.is_some() && self.memsize_chunk_criteria.is_some() {
            return Err(Error::incompatible_options(
                "duration_chunk_criteria",
                "memsize_chunk_criteria",
            ));
        }
        let frame_size = {
            if let Some(duration) = self.duration_chunk_criteria {
                let Some(codec) = self.codec else {
                    todo!("write proper error");
                };

                (codec.bytes_per_ms() as u128 * duration.as_millis()) as usize
            } else {
                let Some(memsize) = self.memsize_chunk_criteria else {
                    todo!("write proper error");
                };

                memsize
            }
        };

        Ok(SplitOpts {
            frame_size,
            silence_threshold: self.silence_threshold,
        })
    }
}

impl<C> Default for SplitOptsBuilder<C> {
    fn default() -> Self {
        Self {
            codec: None,
            duration_chunk_criteria: None,
            memsize_chunk_criteria: None,
            silence_threshold: None,
        }
    }
}
