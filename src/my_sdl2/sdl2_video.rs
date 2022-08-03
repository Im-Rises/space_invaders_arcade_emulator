// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
// use sdl2::render::{Canvas, WindowCanvas};
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::si_arcade;

const WINDOW_WIDTH: usize = 600;
const WINDOW_HEIGHT: usize = 600;

pub fn init_video(sdl_context: &Sdl) -> Result<WindowCanvas, String> {
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Space Invaders Arcade Emulator",
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    canvas
        .set_logical_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .expect("Error: Cannot create canvas");
    Ok(canvas)
}

pub fn update_screen(si_arcade: &mut si_arcade::SpaceInvadersArcade, canvas: &mut WindowCanvas) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let si_arcade_screen = si_arcade.get_si_arcade_screen_width_height();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            si_arcade_screen.0 as u32,
            si_arcade_screen.1 as u32,
        )
        .map_err(|e| e.to_string())?;
    texture
        .update(None, si_arcade.get_screen(), si_arcade_screen.0 * 3)
        .expect("Error: Cannot create texture");

    canvas.copy_ex(&texture, None, None, -90.0, None, false, false)?;
    canvas.present();
    Ok(())
}
