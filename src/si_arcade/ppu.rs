extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::video::Window;

use crate::binary_lib::get_bit;
use crate::si_arcade::mmu::Mmu;

pub const SCREEN_FREQUENCY: usize = 60;
const SCREEN_WIDTH: usize = 224;
const SCREEN_HEIGHT: usize = 256;

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    window: Window,
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    // texture: Texture,
}

impl Ppu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>) -> Ppu {
        Ppu {
            mmu: Rc::clone(&mmu),
            window: Ppu::init_video().unwrap(),
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            // texture: (),
        }
    }

    fn init_video() -> Result<Window, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Space Invaders Arcade Emulator", 800, 600)
            .position_centered()
            .resizable()
            .hidden()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(window)
    }

    pub fn start_video(&mut self) {
        self.window.show();
    }

    pub fn clock(&mut self) /*-> Result<(), String>*/
    {
        // self.texture.update(None, self.mmu.borrow().get_vram(), SCREEN_WIDTH)

        let mut index: u32 = 0;
        for data in self.mmu.borrow().get_vram() {
            for bit in 0..7 {
                if get_bit(*data, bit as usize) {
                    self.screen[(index * 8 + bit) as usize] = 1
                } else {
                    self.screen[(index * 8 + bit) as usize] = 0;
                }
            }
            index += 1;
        }

        /*        let mut canvas = *self.window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.borrow().texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .map_err(|e| e.to_string())?;
        // Create a red-green gradient
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            }
        })?;

        canvas.clear();
        canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
        canvas.copy_ex(
            &texture,
            None,
            Some(Rect::new(450, 100, 256, 256)),
            30.0,
            None,
            false,
            false,
        )?;
        canvas.present();

        Ok(())*/
    }
}
