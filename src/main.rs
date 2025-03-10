use aws_config::BehaviorVersion;
use aws_sdk_polly::{
    Client,
    types::{Engine, OutputFormat, VoiceId},
};
use chrono::Local;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::Cursor;
use std::io::Write;

#[tokio::main]
async fn main() {
    // Load AWS configuration
    let config = aws_config::defaults(BehaviorVersion::latest())
        // .region("us-west-2")
        .load()
        .await;

    // Create Polly client
    let client = Client::new(&config);

    let text = String::from("Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications. It gives the flexibility to target a wide range of systems, from large servers with dozens of cores to small embedded devices.

    At a high level, Tokio provides a few major components:

    A multi-threaded runtime for executing asynchronous code.
    An asynchronous version of the standard library.
    A large ecosystem of libraries.");

    // Synthesize speech
    let output = client
        .synthesize_speech()
        .engine(Engine::Neural)
        .voice_id(VoiceId::Matthew)
        .output_format(OutputFormat::Mp3)
        .text(text)
        .send()
        .await
        .unwrap();

    // Set up audio output
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Get the audio stream and play it
    let audio_data = output
        .audio_stream
        .collect()
        .await
        .unwrap()
        .into_bytes()
        .to_vec();

    // Save to MP3 file
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("output_{}.mp3", timestamp);

    let mut file = File::create(&filename).unwrap();
    file.write_all(&audio_data).unwrap();

    // Create cursor and play the audio
    let cursor = Cursor::new(audio_data);
    let decoder = Decoder::new(cursor).unwrap();
    sink.append(decoder);
    sink.sleep_until_end();
}
