use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_lib::get_bit;

use super::mmu::Mmu;

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
            mmu: Rc::clone(mmu),
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
        }
    }

    pub fn clock(&mut self) {
        let mut index: usize = 0;
        for data in self.mmu.borrow().get_vram() {
            for bit in 0..8 {
                let color: u8 = if get_bit(*data, bit) { 0xFF } else { 0 };
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
