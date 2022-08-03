use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::si_arcade;

mod sdl2_audio;
mod sdl2_inputs;
mod sdl2_video;

pub struct MySdl2 {
    sdl_context: Sdl,
    window_canvas: WindowCanvas,
}

impl MySdl2 {
    pub fn new() -> MySdl2 {
        let sdl_init = MySdl2::mysdl2_init().unwrap();
        MySdl2 {
            window_canvas: sdl2_video::init_video(&sdl_init).unwrap(),
            sdl_context: sdl_init,
        }
    }

    fn mysdl2_init() -> Result<Sdl, String> {
        Ok(sdl2::init()?)
    }

    pub fn update_screen(&mut self, si_arcade: &mut si_arcade::SpaceInvadersArcade) {
        sdl2_video::update_screen(si_arcade, &mut self.window_canvas).expect("Error: Cannot update SDL video window");
    }

    pub fn get_window_active(&self, si_arcade: &mut si_arcade::SpaceInvadersArcade) -> bool {
        sdl2_inputs::get_window_active(si_arcade, &self.sdl_context).expect("Error: Cannot fetch keyboard state")
    }

    pub fn emulate_audio_sound(&self, audio_song_nbr: u8) {}
}
