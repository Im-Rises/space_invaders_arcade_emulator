use sdl2::mixer;
use sdl2::mixer::{Channel, Chunk, LoaderRWops};
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
    sound_9: Chunk,
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
        sound_9_bytes: &[u8],
    ) -> Self {
        mixer::open_audio(11025, mixer::AUDIO_S8, 1, 256).unwrap();
        mixer::init(mixer::InitFlag::MID).unwrap();
        mixer::allocate_channels(10);

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
            sound_9: RWops::from_bytes(sound_9_bytes).unwrap().load_wav().unwrap(),
        }
    }

    pub fn play_ufo(&self) {
        let channel = 0;
        if !Channel(channel).is_playing() {
            Channel(channel)
                .play(&self.sound_0, 0)
                .expect("Error: Cannot play audio wav file 0");
        }
    }

    pub fn stop_ufo(&self) {
        let channel = 0;
        Channel(channel).halt();
        Channel(channel).set_position(0, 0).expect("Error: Cannot set position");
    }

    pub fn play_shoot(&self) {
        let channel = 1;
        if !Channel(channel).is_playing() {
            Channel(channel)
                .play(&self.sound_1, 0)
                .expect("Error: Cannot play audio wav file 1");
        }
    }

    pub fn stop_shoot(&self) {
        let channel = 1;
        Channel(channel).halt();
        Channel(channel).set_position(0, 0).expect("Error: Cannot set position");
    }

    pub fn play_player_die(&self) {
        let channel = 2;
        if !Channel(channel).is_playing() {
            Channel(channel)
                .play(&self.sound_2, 0)
                .expect("Error: Cannot play audio wav file 2");
        }
    }

    pub fn play_invader_die(&self) {
        let channel = 3;
        if !Channel(channel).is_playing() {
            Channel(channel)
                .play(&self.sound_3, 0)
                .expect("Error: Cannot play audio wav file 3");
        }
    }

    pub fn play_fleet_movement_1(&self) {
        let channel = 4;
        Channel(channel)
            .play(&self.sound_4, 0)
            .expect("Error: Cannot play audio wav file 4");
    }

    pub fn play_fleet_movement_2(&self) {
        let channel = 5;
        Channel(channel)
            .play(&self.sound_5, 0)
            .expect("Error: Cannot play audio wav file 5");
    }

    pub fn play_fleet_movement_3(&self) {
        let channel = 6;
        Channel(channel)
            .play(&self.sound_6, 0)
            .expect("Error: Cannot play audio wav file 6");
    }

    pub fn play_fleet_movement_4(&self) {
        let channel = 7;
        Channel(channel)
            .play(&self.sound_7, 0)
            .expect("Error: Cannot play audio wav file 7");
    }

    pub fn play_ufo_hit(&self) {
        let channel = 8;
        Channel(channel)
            .play(&self.sound_8, 0)
            .expect("Error: Cannot play audio wav file 8");
    }

    pub fn play_extra_ship(&self) {
        let channel = 9;
        Channel(channel)
            .play(&self.sound_9, 0)
            .expect("Error: Cannot play audio wav file 8");
    }
}
