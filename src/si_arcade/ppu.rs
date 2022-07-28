extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::video::Window;
use sdl2::Error;

use crate::si_arcade::mmu::Mmu;

pub const SCREEN_FREQUENCY: usize = 60;
const SCREEN_WIDTH: usize = 224;
const SCREEN_HEIGHT: usize = 256;

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    window: Window,
    // texture: Texture,
}

impl Ppu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>) -> Ppu {
        Ppu {
            mmu: Rc::clone(&mmu),
            window: Ppu::init_video().unwrap(),
            // texture: (),
        }
    }

    fn init_video() -> Result<(Window), String> {
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

        // let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        // let texture_creator = canvas.texture_creator();

        Ok((window))
    }

    pub fn start_video(&mut self) {
        self.window.show();
    }

    pub fn clock(&self) {
        // self.texture.update(None, self.mmu.borrow().get_vram(), SCREEN_WIDTH)
        // println!("{:?}", self.mmu.borrow().get_vram().len())

        for i in self.mmu.borrow().get_vram() {}
    }
}
