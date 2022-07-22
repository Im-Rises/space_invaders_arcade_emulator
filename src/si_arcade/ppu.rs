use std::rc::Rc;

use crate::si_arcade::mmu::Mmu;

pub struct Ppu {
    mmu: Rc<Mmu>,
}

impl Ppu {
    pub fn new(mmu: &Rc<Mmu>) -> Ppu {
        Ppu {
            mmu: Rc::clone(&mmu),
        }
    }
}
