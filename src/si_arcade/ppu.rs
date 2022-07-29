extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{CanvasBuilder, Texture, WindowCanvas};
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

use crate::binary_lib::get_bit;
use crate::si_arcade::mmu::Mmu;

pub const SCREEN_FREQUENCY: usize = 60;
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 224;

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl Ppu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>) -> Ppu {
        Ppu {
            mmu: Rc::clone(&mmu),
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
        }
    }

    pub fn clock(&mut self) {
        let mut index: usize = 0;
        for data in self.mmu.borrow().get_vram() {
            for bit in 0..8 {
                let color: u8;
                if get_bit(*data, bit) {
                    color = 0xFF;
                } else {
                    color = 0;
                }
                self.screen[index] = color;
                self.screen[index + 1] = color;
                self.screen[index + 2] = color;
                index += 3;
            }
        }
    }

    pub fn get_screen(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3] {
        &self.screen
    }
}
