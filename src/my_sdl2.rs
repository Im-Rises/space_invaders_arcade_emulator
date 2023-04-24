use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::binary_lib::get_bit;
use crate::si_arcade;

mod sdl2_audio;
mod sdl2_inputs;
mod sdl2_video;

pub struct MySdl2 {
    sdl_context: Sdl,
    window_canvas: WindowCanvas,
    sdl_audio: sdl2_audio::MySdl2Audio,
    port3_previous_outputs: u8,
    port5_previous_outputs: u8,
}

impl MySdl2 {
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
    ) -> MySdl2 {
        let sdl_init = MySdl2::mysdl2_init().unwrap();
        MySdl2 {
            window_canvas: sdl2_video::init_video(&sdl_init).unwrap(),
            sdl_context: sdl_init,
            port3_previous_outputs: 0,
            sdl_audio: sdl2_audio::MySdl2Audio::new(
                sound_0_bytes,
                sound_1_bytes,
                sound_2_bytes,
                sound_3_bytes,
                sound_4_bytes,
                sound_5_bytes,
                sound_6_bytes,
                sound_7_bytes,
                sound_8_bytes,
            ),
            port5_previous_outputs: 0,
        }
    }

    fn mysdl2_init() -> Result<Sdl, String> {
        sdl2::init()
    }

    pub fn update_screen(&mut self, si_arcade: &mut si_arcade::SpaceInvadersArcade) {
        sdl2_video::update_screen(si_arcade, &mut self.window_canvas).expect("Error: Cannot update SDL video window");
    }

    pub fn get_window_active(&mut self, si_arcade: &mut si_arcade::SpaceInvadersArcade) -> bool {
        sdl2_inputs::get_window_active(si_arcade, &self.sdl_context).expect("Error: Cannot fetch keyboard state")
    }

    pub fn play_audio_sound(&mut self, port: u8, data: u8) {
        match port {
            3 => {
                if get_bit(data, 0) {
                    self.sdl_audio.play_ufo();
                }
                if get_bit(data, 1) && !get_bit(self.port3_previous_outputs, 1) {
                    self.sdl_audio.play_shot();
                }
                if get_bit(data, 2) && !get_bit(self.port3_previous_outputs, 2) {
                    self.sdl_audio.play_player_die();
                }
                if get_bit(data, 3) && !get_bit(self.port3_previous_outputs, 3) {
                    self.sdl_audio.play_invader_die();
                }
                self.port3_previous_outputs = data;
            }
            5 => {
                if get_bit(data, 0) {
                    self.sdl_audio.play_fleet_movement_1();
                }
                if get_bit(data, 1) {
                    self.sdl_audio.play_fleet_movement_2();
                }
                if get_bit(data, 2) {
                    self.sdl_audio.play_fleet_movement_3();
                }
                if get_bit(data, 3) {
                    self.sdl_audio.play_fleet_movement_4();
                }
                if get_bit(data, 4) && !get_bit(self.port5_previous_outputs, 4) {
                    self.sdl_audio.play_ufo_hit();
                }
                self.port5_previous_outputs = data;
            }
            _ => {
                panic!("Error: Trying to use port {} as audio port", port)
            }
        }
    }
}
