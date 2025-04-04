## DubOxide: Easy audio splitting that actually makes sense.
A crate made for splitting whatever audio you want (assuming supported format) into chunks based on either time or byte array size.

## Why?
Currently, there aren't really any libraries in the Rust ecosystem that allow for cutting audio files into chunks that make sense (using purely Rust).

You **can** use FFMPEG, but:
1) I don't personally want to use or install ffmpeg just so I can chunk some audio files.
2) If you don't know exactly when the audio gets cut off, you can end up chopping a file mid-sentence. That's bad.

Also, [symphonia is faster than ffmpeg.](https://github.com/pdeljanov/Symphonia/blob/master/BENCHMARKS.md)

## Installation
```bash
cargo add dub-oxide --git https://github.com/joshua-mo-143/dub-oxide.git
```

## Example usage
```rust
use std::time::Duration;
use dub_oxide::{AudioSplitter, WavSplitter, opts::SplitOpts};

fn main() {
    let mut wav_splitter = WavSplitter::from_file_path("input.wav").unwrap();

    let opts = SplitOpts::builder()
        .codec(wav_splitter.codec())
        .silence_threshold(-20.0)
        .split_by_duration(Duration::from_secs(10))
        .build()
        .unwrap();

    let res = wav_splitter.split_audio(opts).unwrap();
    println!("Splitted audio");

    assert_eq!(res.len(), 7);
}
```

That's pretty much it.

## Supported formats

### Decoding
- [x] WAV (fully usable with silence detection)
- [x] AIFF (usable - no split on silence detection)
- [ ] Everything else

### Encoding
- [x] AIFF (somewhat usable - see `encoders/aiff` crate)
- [ ] Everything else

## Roadmap
- Decode every format supported by `symphonia`
- (extremely long term) Encode every format

## License
The entire repo is MIT until I say otherwise. Credits are appreciated if you plan on monetising my work.
