use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::si_arcade;

mod sdl2_audio;
mod sdl2_inputs;
mod sdl2_video;

pub struct MySdl2 {
    sdl_context: Sdl,
    window_canvas: WindowCanvas,
    sdl_audio: sdl2_audio::MySdl2Audio,
    sdl_inputs: sdl2_inputs::Sdl2Inputs,
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
            sdl_inputs: sdl2_inputs::Sdl2Inputs::new(),
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
        }
    }

    fn mysdl2_init() -> Result<Sdl, String> {
        sdl2::init()
    }

    pub fn update_screen(&mut self, si_arcade: &mut si_arcade::SpaceInvadersArcade) {
        sdl2_video::update_screen(si_arcade, &mut self.window_canvas).expect("Error: Cannot update SDL video window");
    }

    pub fn get_window_active(&mut self, si_arcade: &mut si_arcade::SpaceInvadersArcade) -> bool {
        self.sdl_inputs
            .get_window_active(si_arcade, &self.sdl_context)
            .expect("Error: Cannot fetch keyboard state")
    }

    pub fn play_audio_sound(&mut self, audio_song_nbr: i32) {
        self.sdl_audio.play_audio_sound(audio_song_nbr);
    }
}
