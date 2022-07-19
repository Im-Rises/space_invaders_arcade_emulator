//private imports
mod cpu;
mod ppu;
mod mmu;
mod inputs;

pub struct SpaceInvadersArcade {
    cpu: cpu::Cpu,
    mmu: mmu::Mmu,
    // ppu: ppu::Ppu,
    // inputs: inputs::Imputs,
}

impl SpaceInvadersArcade {
    // fn new() -> SpaceInvadersArcade {
    // }
    fn load_rom(&self) {
        // mmu.load_rom();
    }
    fn start(&self) {}
    fn pause_emulation(&self) {}
    fn restart_emulation(&self) {}
    fn save_state(&self) {}
    fn load_state(&self) {}
}