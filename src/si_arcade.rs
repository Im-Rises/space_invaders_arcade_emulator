use std::cell::RefCell;
use std::rc::Rc;

mod cpu;
mod inputs_outputs;
mod mmu;
mod ppu;

pub struct SpaceInvadersArcade {
    cpu: cpu::Cpu,
    mmu: Rc<RefCell<mmu::Mmu>>,
    ppu: ppu::Ppu,
    // inputs: inputs::Imputs,
}

impl SpaceInvadersArcade {
    pub fn new(roms_path: &str) -> SpaceInvadersArcade {
        let mmu_init = Rc::new(RefCell::new(mmu::Mmu::new(roms_path)));
        SpaceInvadersArcade {
            cpu: cpu::Cpu::new(&mmu_init),
            mmu: Rc::clone(&mmu_init),
            ppu: ppu::Ppu::new(&mmu_init),
            // inputs: inputs::new()
        }
    }
    pub fn start(&mut self) {
        loop {
            // self.inputs.readInputs();
            self.cpu.clock();
            // self.ppu.clock();
        }
    }

    // fn load_rom() {
    //     // mmu.load_rom();
    // }
    // fn pause_emulation(&self) {}
    // fn restart_emulation(&self) {}
    // fn save_state(&self) {}
    fn load_state(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::si_arcade::cpu::Cpu;
    use crate::si_arcade::mmu::Mmu;

    use super::*;

    #[test]
    fn cpu_test() {
        println!("here");
        // assert_eq!(4, 2 + 2);

        let mmu_debug = Rc::new(RefCell::new(Mmu::new_debug("debug")));
        let mut cpu_debug = Cpu::new(&mmu_debug);

        let mut cycles_counter: u64 = 0;
        for i in 0..650 {
            cpu_debug.print_data_debug();
            cycles_counter += cpu_debug.clock() as u64;
        }
        cpu_debug.print_data_debug();
        println!("Final state: {:#0X?} {}", cpu_debug.get_state(), cycles_counter);
    }
}
