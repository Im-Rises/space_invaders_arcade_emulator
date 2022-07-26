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

    // Type the following command to get console output
    //  cargo test -- --nocapture
    #[test]
    fn cpu_test() {
        let mmu_debug = Rc::new(RefCell::new(Mmu::new_debug("debug")));
        let mut cpu_debug = Cpu::new(&mmu_debug);

        let mut cycles_counter: u64 = 0;
        for i in 0..650 {
            print_data_debug(cpu_debug.get_state(), cycles_counter);
            cycles_counter += cpu_debug.clock() as u64;
        }
        print_data_debug(cpu_debug.get_state(), cycles_counter);

        // assert_eq!(4, 2 + 2);
    }
}

pub fn print_data_debug(cpu_state: (u16, u16, u16, u16, u16, u16, u8), cycles_total: u64) {
    println!(
        "PC = {:#X}, AF = {:#X}, BC = {:#X}, DE = {:#X}, HL = {:#X}, SP = {:#X}, Cycles = {}, Total Cycles = {}",
        cpu_state.0, cpu_state.1, cpu_state.2, cpu_state.3, cpu_state.4, cpu_state.5, cpu_state.6, cycles_total
    );
}
