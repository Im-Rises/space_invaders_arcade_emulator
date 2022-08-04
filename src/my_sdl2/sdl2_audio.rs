use sdl2::mixer;
use sdl2::mixer::{Channel, Chunk, LoaderRWops};
use sdl2::rwops::RWops;

// SDL2 audio callback Use for future audio custom audio playback ?
//https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-wav.rs

// SDL2 Rust audio mixer
//https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/mixer-demo.rs

// pub enum SiSounds {
//     Ufo = 0,
//     Shot = 1,
//     PlayerDie = 2,
//     InvaderDie = 3,
//     FleetMovement1 = 4,
//     FleetMovement2 = 5,
//     FleetMovement3 = 6,
//     FleetMovement4 = 7,
//     UfoHit = 8,
// }

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
        }
    }

    // pub fn set_volume(&self, volume: u8) {
    //     mixer::Channel::all().set_volume(volume as i32);
    // }

    // pub fn play_audio_sound(&mut self, audio_nbr: i32) {
    //     // println!("{}", audio_nbr);
    //     if !mixer::Channel(audio_nbr).is_playing() {
    //         match audio_nbr {
    //             0 => self.play_ufo(audio_nbr),
    //             1 => self.play_shot_sound(audio_nbr),
    //             2 => self.play_player_die(audio_nbr),
    //             3 => self.play_invader_die(audio_nbr),
    //             4 => self.play_fleet_movement_1(audio_nbr),
    //             5 => self.play_fleet_movement_2(audio_nbr),
    //             6 => self.play_fleet_movement_3(audio_nbr),
    //             7 => self.play_fleet_movement_4(audio_nbr),
    //             8 => self.play_ufo_hit(audio_nbr),
    //             _ => panic!("Error: Unknown audio wav file to play"),
    //         }
    //     }
    //     // if audio_nbr != 0 && audio_nbr != 1 && audio_nbr != 2 && audio_nbr != 3 {
    //     // if audio_nbr == 4 && audio_nbr == 5 && audio_nbr == 6 && audio_nbr == 7 {
    //     //     self.sound_2_enabled = true;
    //     // }
    // }

    // pub fn play_sound(&self, sound: SiSounds) {
    //     if !Channel(sound as i32).is_playing() {
    //         match sound {
    //             SiSounds::Ufo => self.play_ufo(0),
    //             SiSounds::Shot => self.play_shot_sound(1),
    //             SiSounds::PlayerDie => self.play_player_die(2),
    //             SiSounds::InvaderDie => self.play_invader_die(3),
    //             SiSounds::FleetMovement1 => self.play_fleet_movement_1(4),
    //             SiSounds::FleetMovement2 => self.play_fleet_movement_2(5),
    //             SiSounds::FleetMovement3 => self.play_fleet_movement_3(6),
    //             SiSounds::FleetMovement4 => self.play_fleet_movement_4(7),
    //             SiSounds::UfoHit => self.play_ufo_hit(8),
    //         }
    //     }
    // }

    // pub fn play_sound(&self, sound: SiSounds) {
    //     if !Channel(channel).is_playing() {
    //         match sound {
    //             SiSounds::Ufo => self.play_ufo(0),
    //             SiSounds::Shot => self.play_shot_sound(1),
    //             SiSounds::PlayerDie => self.play_player_die(2),
    //             SiSounds::InvaderDie => self.play_invader_die(3),
    //             SiSounds::FleetMovement1 => self.play_fleet_movement_1(4),
    //             SiSounds::FleetMovement1 => self.play_fleet_movement_2(5),
    //             SiSounds::FleetMovement2 => self.play_fleet_movement_3(6),
    //             SiSounds::FleetMovement3 => self.play_fleet_movement_4(7),
    //             SiSounds::FleetMovement4 => self.play_ufo_hit(8),
    //             _ => {}
    //         }
    //     }
    // }

    pub fn play_ufo(&self) {
        let channel = 0;
        if !Channel(channel).is_playing() {
            Channel(channel)
                .play(&self.sound_0, 0)
                .expect("Error: Cannot play audio wav file 0");
        }
    }

    pub fn play_shot(&self) {
        let channel = 1;
        if !Channel(channel).is_playing() {
            Channel(channel)
                .play(&self.sound_1, 0)
                .expect("Error: Cannot play audio wav file 1");
        }
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
}
