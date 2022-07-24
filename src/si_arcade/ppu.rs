use std::cell::RefCell;
use std::rc::Rc;

use crate::si_arcade::mmu::Mmu;

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
}

impl Ppu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>) -> Ppu {
        Ppu { mmu: Rc::clone(&mmu) }
    }
}
