use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn main() {
    // Get the audio playback stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    // Add a tone.
    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);
    sink.append(source);
    
    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
