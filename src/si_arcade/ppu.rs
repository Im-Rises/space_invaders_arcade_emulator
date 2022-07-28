extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{CanvasBuilder, Texture, WindowCanvas};
use sdl2::video::Window;

use crate::binary_lib::get_bit;
use crate::si_arcade::mmu::Mmu;

pub const SCREEN_FREQUENCY: usize = 60;
const SCREEN_WIDTH: usize = 224;
const SCREEN_HEIGHT: usize = 256;

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    // window: Window,
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
    // texture: Texture,
    pub canvas: WindowCanvas,
}

//https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/renderer-texture.rs

impl Ppu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>) -> Ppu {
        Ppu {
            mmu: Rc::clone(&mmu),
            // window: window_canvas,
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            // texture: (),
            canvas: Ppu::init_video().unwrap(),
        }
    }

    fn init_video() -> Result<WindowCanvas, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Space Invaders Arcade Emulator", 800, 600)
            .position_centered()
            .resizable()
            // .hidden()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(canvas)
    }

    pub fn start_video(&mut self) {
        // self.window.show();
    }

    pub fn clock(&mut self) -> Result<(), String> {
        // self.texture.update(None, self.mmu.borrow().get_vram(), SCREEN_WIDTH)

        let mut index: u32 = 0;
        //Conflict between RefCell's borrow and Borrow's borrow
        // for data in self.mmu.borrow().get_vram() {
        for data in self.mmu.borrow().get_vram() {
            for bit in 0..7 {
                if get_bit(*data, bit as usize) {
                    self.screen[(index * 3 + bit) as usize] = 0xFF;
                    self.screen[(index * 3 + bit + 1) as usize] = 0xFF;
                    self.screen[(index * 3 + bit + 2) as usize] = 0xFF;
                } else {
                    self.screen[(index * 3 + bit) as usize] = 0;
                    self.screen[(index * 3 + bit + 1) as usize] = 0;
                    self.screen[(index * 3 + bit + 2) as usize] = 0;
                }
            }
            index += 1;
        }
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, (SCREEN_WIDTH) as u32, SCREEN_HEIGHT as u32)
            .map_err(|e| e.to_string())?;
        texture
            .update(None, &self.screen, SCREEN_WIDTH * 3)
            .expect("TODO: panic message");

        // self.canvas.clear();
        self.canvas.copy(&texture, None, None)?;
        // self.canvas.copy_ex(&texture, None, None, 90.0, None, false, false)?;
        self.canvas.present();
        Ok(())
    }
}
