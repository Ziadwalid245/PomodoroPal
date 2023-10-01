use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
pub fn play_sound(file_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(file_path).unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
