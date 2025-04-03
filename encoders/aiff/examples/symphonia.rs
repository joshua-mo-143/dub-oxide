use std::io::Cursor;

use symphonia::core::{
    formats::{FormatOptions, FormatReader},
    io::{MediaSourceStream, MediaSourceStreamOptions},
};

use aiff::{AiffEncoder, AiffHeader};

fn main() {
    let bytes = std::fs::read("input.wav").unwrap();
    let src = Box::new(Cursor::new(bytes));
    let opts = MediaSourceStreamOptions::default();
    let stream = MediaSourceStream::new(src, opts);

    let mut thing =
        symphonia::default::formats::WavReader::try_new(stream, &FormatOptions::default()).unwrap();

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

    let sample_rate = metadata.sample_rate.unwrap().into();
    let num_channels = metadata.channels.unwrap().count().try_into().unwrap();
    let num_samples = samples.len() as u32;

    let header = AiffHeader::new(sample_rate, num_channels, num_samples);

    let encoder = AiffEncoder::from_samples(samples, header);

    let res = encoder.encode().unwrap();

    std::fs::write("test.aiff", res).unwrap();
}
