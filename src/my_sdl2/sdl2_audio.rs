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
            sound_2_enabled: true,
        }
    }

    // pub fn set_volume(&self, volume: u8) {
    //     mixer::Channel::all().set_volume(volume as i32);
    // }

    pub fn play_audio_sound(&mut self, audio_nbr: i32) {
        // println!("{}", audio_nbr);
        if !mixer::Channel(audio_nbr).is_playing() {
            match audio_nbr {
                0 => self.play_ufo(audio_nbr),
                1 => {
                    println!("Play sound");
                    // self.play_shot_sound(audio_nbr);
                }
                2 => {
                    if self.sound_2_enabled {
                        self.play_player_die(audio_nbr);
                        self.sound_2_enabled = false;
                    }
                }
                3 => self.play_invader_die(audio_nbr),
                4 => self.play_fleet_movement_1(audio_nbr),
                5 => self.play_fleet_movement_2(audio_nbr),
                6 => self.play_fleet_movement_3(audio_nbr),
                7 => self.play_fleet_movement_4(audio_nbr),
                8 => self.play_ufo_hit(audio_nbr),
                _ => panic!("Error: Unknown audio wav file to play"),
            }
        }
        if audio_nbr != 0 && audio_nbr != 2 {
            self.sound_2_enabled = true;
        }
    }

    fn play_ufo(&self, channel: i32) {
        mixer::Channel(channel)
            .play(&self.sound_0, 0)
            .expect("Error: Cannot play audio wav file 0");
    }

    pub fn play_shot_sound(&self, channel: i32) {
        if !mixer::Channel(channel).is_playing() {
            mixer::Channel(channel)
                .play(&self.sound_1, 0)
                .expect("Error: Cannot play audio wav file 1");
        }
    }

    fn play_player_die(&self, channel: i32) {
        mixer::Channel(channel)
            .play(&self.sound_2, 0)
            .expect("Error: Cannot play audio wav file 2");
    }

    fn play_invader_die(&self, channel: i32) {
        mixer::Channel(channel)
            .play(&self.sound_3, 0)
            .expect("Error: Cannot play audio wav file 3");
    }

    fn play_fleet_movement_1(&self, channel: i32) {
        mixer::Channel(channel)
            .play(&self.sound_4, 0)
            .expect("Error: Cannot play audio wav file 4");
    }

    fn play_fleet_movement_2(&self, channel: i32) {
        mixer::Channel(channel)
            .play(&self.sound_5, 0)
            .expect("Error: Cannot play audio wav file 5");
    }

    fn play_fleet_movement_3(&self, channel: i32) {
        mixer::Channel(channel)
            .play(&self.sound_6, 0)
            .expect("Error: Cannot play audio wav file 6");
    }

    fn play_fleet_movement_4(&self, channel: i32) {
        mixer::Channel(channel)
            .play(&self.sound_7, 0)
            .expect("Error: Cannot play audio wav file 7");
    }

    fn play_ufo_hit(&self, channel: i32) {
        mixer::Channel(channel)
            .play(&self.sound_8, 0)
            .expect("Error: Cannot play audio wav file 8");
    }
}
