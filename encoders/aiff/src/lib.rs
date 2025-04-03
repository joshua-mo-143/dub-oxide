use byteorder::LittleEndian;
use std::io::Write;

type TODO = Box<dyn std::error::Error>;
use byteorder::{BigEndian, WriteBytesExt};

#[derive(Debug)]
pub struct AiffEncoder {
    samples: Vec<i16>,
    header: AiffHeader,
}

#[derive(Debug)]
pub struct AiffHeader {
    sample_rate: f64,
    num_channels: u16,
    num_samples: u32,
}

impl AiffHeader {
    pub fn new(sample_rate: f64, num_channels: u16, num_samples: u32) -> Self {
        Self {
            sample_rate,
            num_channels,
            num_samples,
        }
    }

    pub fn calculate_form_chunk_size(&self) -> usize {
        self.num_samples as usize * self.num_channels as usize * 2
    }
}

impl AiffEncoder {
    pub fn from_samples(samples: Vec<i16>, header: AiffHeader) -> Self {
        Self { samples, header }
    }

    pub fn convert_to_u8_bytes(&self) -> Vec<u8> {
        self.samples.iter().flat_map(|x| x.to_le_bytes()).collect()
    }

    pub fn comm_chunk(&self) -> CommChunk {
        let AiffHeader {
            sample_rate,
            num_channels,
            ..
        } = self.header;

        CommChunk::new(num_channels, self.samples.len() as u32, 16, sample_rate)
    }

    pub fn sound_chunk(&self) -> SoundChunk {
        let bytes = self.convert_to_u8_bytes();
        SoundChunk::new(bytes)
    }

    pub fn encode(&self) -> Result<Vec<u8>, TODO> {
        let mut buffer = Vec::new();

        buffer.write_all(b"FORM")?;
        // Note that 28 is calculated from the leftover ckID + ckSize
        let len: u32 = 28 + 18 + (self.samples.len() * 2) as u32;
        buffer.write_u32::<BigEndian>(len)?;
        buffer.write_all(b"AIFF")?;

        let comm_chunk = self.comm_chunk();
        comm_chunk.write_bytes(&mut buffer)?;

        let sound_chunk = self.sound_chunk();
        sound_chunk.write_bytes(&mut buffer)?;

        Ok(buffer)
    }
}

pub struct CommChunk<'a> {
    ck_id: &'a [u8; 4],
    ck_size: u32,
    num_channels: u16,
    num_sample_frames: u32,
    sample_size: u16,
    sample_rate: f64,
}

impl<'a> CommChunk<'a> {
    pub fn new(
        num_channels: u16,
        num_sample_frames: u32,
        sample_size: u16,
        sample_rate: f64,
    ) -> Self {
        Self {
            ck_id: b"COMM",
            ck_size: 18,
            num_channels,
            num_sample_frames,
            sample_size,
            sample_rate,
        }
    }

    pub fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), TODO> {
        buffer.write_all(self.ck_id)?;
        buffer.write_u32::<BigEndian>(self.ck_size)?;
        buffer.write_u16::<BigEndian>(self.num_channels)?;
        buffer.write_u32::<BigEndian>(self.num_sample_frames)?;
        buffer.write_u16::<BigEndian>(self.sample_size)?;

        let sample_rate = encode_ieee_754_extended(self.sample_rate);

        buffer.write_all(&sample_rate)?;

        Ok(())
    }
}

/// The sound chunk.
pub struct SoundChunk<'a> {
    /// The ID. This should always be the byte representation of "SSND".
    ck_id: &'a [u8; 4],
    /// The size (should be whatever the sample length is)
    ck_size: u32,
    /// Byte offset to start playing at (should be 0 in most cases)
    offset: u32,
    /// Block size (should be 0 in most cases)
    block_size: u32,
    /// The sound bytes themselves
    sound_data: Vec<u8>,
}

impl<'a> SoundChunk<'a> {
    pub fn new(sound_data: Vec<u8>) -> Self {
        Self {
            ck_id: b"SSND",
            ck_size: sound_data.len() as u32,
            offset: 0,
            block_size: 0,
            sound_data,
        }
    }

    pub fn samples_as_u8_bytes(&self) -> Vec<u8> {
        self.sound_data
            .iter()
            .flat_map(|x| x.to_be_bytes())
            .collect()
    }

    pub fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), TODO> {
        buffer.write_all(self.ck_id)?;
        buffer.write_i32::<BigEndian>(self.ck_size as i32 + 8)?;
        buffer.write_u32::<BigEndian>(self.offset)?;
        buffer.write_u32::<BigEndian>(self.block_size)?;

        let sound_data = self.samples_as_u8_bytes();

        buffer.write_all(&sound_data)?;

        Ok(())
    }
}

/// Creates an 80-bit floating point number (see: Extended type from the Standard Apple Numeric Environment)
fn encode_ieee_754_extended(value: f64) -> [u8; 10] {
    let mut result = [0u8; 10];

    if value == 0.0 {
        return result; // Zero is represented as all zero bytes
    }

    let sign_bit = if value.is_sign_negative() {
        0x8000
    } else {
        0x0000
    };
    let mut v = value.abs();
    let mut exponent: i16 = 16383; // AIFF exponent bias

    // Normalize the value to the range [0.5, 1.0)
    while v >= 1.0 {
        v /= 2.0;
        exponent += 1;
    }
    while v < 0.5 {
        v *= 2.0;
        exponent -= 1;
    }

    // Remove the implicit leading bit
    let mantissa = (v * (1u64 << 63) as f64) as u64;

    // Store exponent and sign in the first 2 bytes
    result[0..2].copy_from_slice(&(sign_bit | (exponent as u16)).to_be_bytes());

    // Store mantissa in the remaining 8 bytes
    result[2..10].copy_from_slice(&mantissa.to_be_bytes());

    result
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use symphonia::core::{
        formats::{FormatOptions, FormatReader},
        io::{MediaSourceStream, MediaSourceStreamOptions},
    };

    use crate::{AiffEncoder, AiffHeader};

    #[test]
    fn test_encoding_works() {
        let bytes = std::fs::read("input.wav").unwrap();
        let src = Box::new(Cursor::new(bytes));
        let opts = MediaSourceStreamOptions::default();
        let stream = MediaSourceStream::new(src, opts);

        let mut thing =
            symphonia::default::formats::WavReader::try_new(stream, &FormatOptions::default())
                .unwrap();

        let metadata = thing.default_track().unwrap().codec_params.clone();

        println!("{metadata:?}");

        let mut samples: Vec<i16> = Vec::new();
        while let Ok(packet) = thing.next_packet() {
            let bytes: Vec<i16> = packet
                .data
                .chunks_exact(2)
                .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();

            samples.extend_from_slice(&bytes);
        }

        let header = AiffHeader {
            sample_rate: metadata.sample_rate.unwrap().into(),
            num_channels: metadata.channels.unwrap().count().try_into().unwrap(),
            num_samples: samples.len() as u32,
        };

        let encoder = AiffEncoder::from_samples(samples, header);

        let res = encoder.encode().unwrap();

        std::fs::write("test.aiff", res).unwrap();
    }
}
