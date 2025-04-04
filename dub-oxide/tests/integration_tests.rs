use std::time::Duration;
use tracing_subscriber::filter::LevelFilter;

use dub_oxide::{AudioSplitter, WavSplitter, opts::SplitOpts};

#[test]
fn chunking_by_time_works() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    #[cfg(feature = "tracing")]
    tracing::info!("Tracing loaded");

    let mut wav_splitter = WavSplitter::from_file_path("../test_files/test.wav").unwrap();

    #[cfg(feature = "tracing")]
    tracing::info!("Loaded file path");

    let opts = SplitOpts::builder()
        .codec(wav_splitter.codec())
        .silence_threshold(-20.0)
        .split_by_duration(Duration::from_secs(10))
        .build()
        .unwrap();

    let res = wav_splitter.split_audio(opts).unwrap();

    #[cfg(feature = "tracing")]
    tracing::info!("Splitted audio");

    assert_eq!(res.len(), 7);
}

#[test]
fn chunking_by_memsize_works() {
    #[cfg(feature = "tracing")]
    tracing::info!("Tracing loaded");

    let mut wav_splitter = WavSplitter::from_file_path("../test_files/test.wav").unwrap();

    #[cfg(feature = "tracing")]
    tracing::info!("Loaded file path");

    let opts = SplitOpts::builder()
        .codec(wav_splitter.codec())
        .silence_threshold(-20.0)
        .split_by_memsize(1048576)
        .build()
        .unwrap();

    let res = wav_splitter.split_audio(opts).unwrap();

    #[cfg(feature = "tracing")]
    tracing::info!("Splitted audio");

    assert_eq!(res.len(), 6);
}

#[cfg(feature = "aiff")]
#[test]
fn aiff_decoding_works() {
    use crate::formats::aiff::AiffSplitter;
    let mut aiff_splitter = AiffSplitter::from_file_path("../test_files/test.aiff").unwrap();

    let opts = SplitOpts::builder()
        .codec(aiff_splitter.codec())
        .silence_threshold(-20.0)
        .split_by_duration(Duration::from_secs(10))
        .build()
        .unwrap();

    let res = aiff_splitter.split_audio(opts).unwrap();

    assert_eq!(res.len(), 5);
}
