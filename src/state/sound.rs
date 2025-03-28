use std::{thread, fs::File, io::BufReader};
use rodio::{Decoder, OutputStream, Sink};

pub fn play_threaded(path: String) {
    thread::spawn(|| {
        play(path);
    });
}

pub fn play(path: String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}