use std::cell::RefCell;
use std::rc::Rc;

use crate::si_arcade::mmu::Mmu;

pub const SCREEN_FREQUENCY: usize = 60;
const SCREEN_WIDTH: usize = 224;
const SCREEN_HEIGHT: usize = 256;

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
}

impl Ppu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>) -> Ppu {
        Ppu { mmu: Rc::clone(&mmu) }
    }
}
