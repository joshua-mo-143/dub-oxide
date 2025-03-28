## DubOxide: Easy audio splitting that actually makes sense.
A crate made for splitting whatever audio you want (assuming supported format) into chunks based on either time or byte array size.

## Installation
```bash
cargo add dub-oxide --git https://github.com/joshua-mo-143/dub-oxide
```

## Example usage
```rust
use rustdub::{WavSplitter, AudioSplitter};

fn main() {
    let mut wav_splitter = WavSplitter::from_file("input.wav");

    let bytes_chunked = wav_splitter.split_by_time_segments(std::time::Duration::from_secs(10));

    for bytes in bytes_chunked {
        println!("Got bytes: {bytes:?}");
    }
}
```

That's pretty much it.

## Supported formats
- [x] WAV
- [ ] Everything else (this will be updated soon!)

## Why?
Currently, there aren't really any libraries in the Rust ecosystem that allow for cutting audio files into chunks that make sense.

You **can** use FFMPEG, but:
1) I don't personally want to use or install ffmpeg just so I can chunk some audio files.
2) If you don't know exactly when the audio gets cut off, you can end up chopping a file mid-sentence. That's bad.
