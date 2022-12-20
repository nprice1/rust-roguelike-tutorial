use rodio::{source::Source, Decoder, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct SoundSystem {
    background_sink: Sink,
    effects_sink: Sink,
}

impl SoundSystem {
    pub fn new(stream_handle: &OutputStreamHandle) -> SoundSystem {
        let background_sink = Sink::try_new(stream_handle).unwrap();
        let effects_sink = Sink::try_new(stream_handle).unwrap();
        let file = BufReader::new(File::open("resources/sounds/background.mp3").unwrap());
        let source = Decoder::new(file).unwrap().repeat_infinite();
        background_sink.append(source);
        return SoundSystem {
            background_sink,
            effects_sink,
        };
    }

    pub fn play_sound_effects(&self, file_names: Vec<String>) {
        for file_name in file_names {
            let file_path = format!("resources/sounds/{}", file_name);
            let file = BufReader::new(File::open(file_path).expect("Sound effect not found"));
            let source = Decoder::new(file).unwrap();
            self.effects_sink.append(source);
        }
    }
}
