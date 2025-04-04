## A naive AIFF encoder.
This crate aims to provide an implementation for encoding AIFF files entirely in Rust.

Based on [the AIFF 1.3 spec.](https://www.mmsp.ece.mcgill.ca/Documents/AudioFormats/AIFF/AIFF.html)

## Installation
```bash
cargo add aiff --git https://github.com/joshua-mo-143/dub-oxide.git
```

## Example usage
The example below shows how you can use this crate alongside the `symphonia` crate to take a WAV file and successfully encode it into AIFF.

```rust
use std::io::Cursor;

use symphonia::core::{
    formats::{FormatOptions, FormatReader},
    io::{MediaSourceStream, MediaSourceStreamOptions},
};

use crate::{AiffEncoder, AiffHeader};


fn main() {
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
```

That's pretty much it.

## Roadmap
- [x] Successfully encode AIFF files!
- [ ] Instrument chunk
- [ ] Marking chunk
- [ ] Comment chunk
- [ ] AIFF-C spec

## Credits
Credits to [depp](https://github.com/depp/extended-rs) for their implementation of FP80 numbers. The code has been copied into this repo for maintainability purposes (and not needing to reference a crate that has never actually been released), but feel free to take a look!
