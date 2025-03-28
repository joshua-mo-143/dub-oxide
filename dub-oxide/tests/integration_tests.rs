use std::time::Duration;

use dub_oxide::{AudioSplitter, WavSplitter};

#[test]
fn chunking_by_time_works() {
    let mut wav_splitter = WavSplitter::from_file_path("tests/files/input.wav").unwrap();

    let res = wav_splitter.split_by_time_segment(Duration::from_secs(10));

    assert_eq!(res.len(), 7);
}
