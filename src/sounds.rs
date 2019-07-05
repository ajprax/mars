use std::boxed::Box;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Read;
use std::io::Cursor;

use rodio::Device;
use rodio::Sink;
use rodio::Source;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Sound {
    TapMuted,
    Holst,
}

lazy_static! {
    static ref SOUND_DEVICE: Device = rodio::default_output_device().unwrap();
    static ref SOUNDS: HashMap<Sound, Vec<u8>> = load_sounds();
}

fn load_sounds() -> HashMap<Sound, Vec<u8>> {
    let sounds_dir = find_folder::Search::ParentsThenKids(3, 3).for_folder("sounds").unwrap();
    let mut sounds: HashMap<Sound, Vec<u8>> = HashMap::new();
    for (sound, filename) in [
        (Sound::TapMuted, "tap-muted.wav"),
        (Sound::Holst, "holst.mp3"),
    ].iter() {
        let mut bytes: Vec<u8> = Vec::new();
        File::open(sounds_dir.join(filename)).unwrap().read_to_end(&mut bytes);
        sounds.insert(*sound, bytes);
    }
    sounds
}

pub fn play_sound(sound: Sound, volume: f32) {
    let file: Cursor<&[u8]> = Cursor::new(&SOUNDS[&sound]);
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sink = Sink::new(&SOUND_DEVICE);
    sink.set_volume(volume);
    sink.append(source);
    sink.play();
    sink.detach();
}