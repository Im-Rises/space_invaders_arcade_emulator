use sdl2::mixer;
use sdl2::mixer::{Chunk, LoaderRWops};
use sdl2::rwops::RWops;

// SDL2 audio callback Use for future audio custom audio playback ?
//https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-wav.rs

// SDL2 Rust audio mixer
//https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/mixer-demo.rs

pub struct MySdl2Audio {
    sound_0: Chunk,
    sound_1: Chunk,
    sound_2: Chunk,
    sound_3: Chunk,
    sound_4: Chunk,
    sound_5: Chunk,
    sound_6: Chunk,
    sound_7: Chunk,
    sound_8: Chunk,
    sound_2_enabled: bool,
}

impl MySdl2Audio {
    pub fn new(
        sound_0_bytes: &[u8],
        sound_1_bytes: &[u8],
        sound_2_bytes: &[u8],
        sound_3_bytes: &[u8],
        sound_4_bytes: &[u8],
        sound_5_bytes: &[u8],
        sound_6_bytes: &[u8],
        sound_7_bytes: &[u8],
        sound_8_bytes: &[u8],
    ) -> Self {
        mixer::open_audio(11025, mixer::AUDIO_S8, 1, 256).unwrap();
        mixer::init(mixer::InitFlag::MID).unwrap();
        mixer::allocate_channels(9);

        MySdl2Audio {
            sound_0: RWops::from_bytes(sound_0_bytes).unwrap().load_wav().unwrap(),
            sound_1: RWops::from_bytes(sound_1_bytes).unwrap().load_wav().unwrap(),
            sound_2: RWops::from_bytes(sound_2_bytes).unwrap().load_wav().unwrap(),
            sound_3: RWops::from_bytes(sound_3_bytes).unwrap().load_wav().unwrap(),
            sound_4: RWops::from_bytes(sound_4_bytes).unwrap().load_wav().unwrap(),
            sound_5: RWops::from_bytes(sound_5_bytes).unwrap().load_wav().unwrap(),
            sound_6: RWops::from_bytes(sound_6_bytes).unwrap().load_wav().unwrap(),
            sound_7: RWops::from_bytes(sound_7_bytes).unwrap().load_wav().unwrap(),
            sound_8: RWops::from_bytes(sound_8_bytes).unwrap().load_wav().unwrap(),
            sound_2_enabled: false,
        }
    }

    // pub fn set_volume(&self, volume: u8) {
    //     mixer::Channel::all().set_volume(volume as i32);
    // }

    pub fn play_audio_sound(&mut self, audio_nbr: i32) {
        println!("{}", audio_nbr);
        if !mixer::Channel(audio_nbr).is_playing() {
            match audio_nbr {
                0 => {
                    //OK
                    mixer::Channel(audio_nbr)
                        .play(&self.sound_0, 0)
                        .expect("Error: Cannot play audio wav file 0");
                }
                1 => {
                    mixer::Channel(audio_nbr)
                        .play(&self.sound_1, 0)
                        .expect("Error: Cannot play audio wav file 1");
                }
                2 => {
                    if self.sound_2_enabled {
                        mixer::Channel(audio_nbr)
                            .play(&self.sound_2, 0)
                            .expect("Error: Cannot play audio wav file 2");
                        self.sound_2_enabled = false;
                    }
                    // else {
                    //     self.sound_2_counter -= 1;
                    //     println!("counter {}", self.sound_2_counter);
                    // }
                }
                3 => {
                    //OK
                    mixer::Channel(audio_nbr)
                        .play(&self.sound_3, 0)
                        .expect("Error: Cannot play audio wav file 3");
                }
                4 => {
                    //OK
                    mixer::Channel(audio_nbr)
                        .play(&self.sound_4, 0)
                        .expect("Error: Cannot play audio wav file 4");
                }
                5 => {
                    //OK
                    mixer::Channel(audio_nbr)
                        .play(&self.sound_5, 0)
                        .expect("Error: Cannot play audio wav file 5");
                }
                6 => {
                    //OK
                    mixer::Channel(audio_nbr)
                        .play(&self.sound_6, 0)
                        .expect("Error: Cannot play audio wav file 6");
                }
                7 => {
                    //OK
                    mixer::Channel(audio_nbr)
                        .play(&self.sound_7, 0)
                        .expect("Error: Cannot play audio wav file 7");
                }
                8 => {
                    //OK
                    mixer::Channel(audio_nbr)
                        .play(&self.sound_8, 0)
                        .expect("Error: Cannot play audio wav file 8");
                }
                _ => panic!("Error: Unknown audio wav file to play"),
            }
        }

        if audio_nbr != 2 {
            self.sound_2_enabled = true;
            println!("Reanabling sound2");
        }
    }
}
