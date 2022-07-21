mod cpu;
mod ppu;
mod mmu;
mod inputs;

use std::rc::Rc;

pub struct SpaceInvadersArcade {
    cpu: cpu::Cpu,
    mmu: Rc<mmu::Mmu>,
    ppu: ppu::Ppu,
    // inputs: inputs::Imputs,
}

impl SpaceInvadersArcade {
    pub fn new(roms_path: &str) -> SpaceInvadersArcade {
        let mmu_init = Rc::new(mmu::Mmu::new(roms_path));
        SpaceInvadersArcade {
            cpu: cpu::Cpu::new(&mmu_init),
            mmu: Rc::clone(&mmu_init),
            ppu: ppu::Ppu::new(&mmu_init),
            // inputs: inputs::new()
        }
    }
    fn load_rom(&self) {
        // mmu.load_rom();
    }
    pub fn start(&self) {}
    fn pause_emulation(&self) {}
    fn restart_emulation(&self) {}
    fn save_state(&self) {}
    fn load_state(&self) {}
}
