use std::cell::RefCell;
use std::rc::Rc;

mod cpu;
mod inputs_outputs;
mod mmu;
mod ppu;

const INTERRUPT_VBLANK_COUNTER: usize = cpu::CLOCK_FREQUENCY / ppu::SCREEN_FREQUENCY;
const INTERRUPT_MIDDLE_VBLANK: usize = INTERRUPT_VBLANK_COUNTER / 2;

pub struct SpaceInvadersArcade {
    cpu: cpu::Cpu,
    mmu: Rc<RefCell<mmu::Mmu>>,
    ppu: ppu::Ppu,
    // inputs: inputs::Imputs,
}

impl SpaceInvadersArcade {
    pub fn new() -> SpaceInvadersArcade {
        let mmu_init = Rc::new(RefCell::new(mmu::Mmu::new()));
        SpaceInvadersArcade {
            cpu: cpu::Cpu::new(&mmu_init, 0),
            mmu: Rc::clone(&mmu_init),
            ppu: ppu::Ppu::new(&mmu_init),
            // inputs: inputs::new()
        }
    }
    pub fn start(&mut self) {
        self.ppu.start_video();
        let mut frequency_counter: usize = 0;
        let mut last_frequency_counter: usize = 0;
        while self.ppu.get_window_active().unwrap() {
            self.cpu.clock();
            frequency_counter += 1;
            if self.cpu.get_inte() {
                if frequency_counter >= INTERRUPT_MIDDLE_VBLANK && last_frequency_counter < INTERRUPT_MIDDLE_VBLANK {
                    cpu::interrupts::interrupt(&mut self.cpu, 1);
                }
                if frequency_counter >= INTERRUPT_VBLANK_COUNTER {
                    cpu::interrupts::interrupt(&mut self.cpu, 2);
                    frequency_counter = 0;
                    self.ppu.clock().expect("Error : Panic happend in PPU");
                }
            } else {
                frequency_counter = 0;
            }

            last_frequency_counter = frequency_counter;
        }
    }

    // fn pause_emulation(&self) {}
    // fn restart_emulation(&self) {}
    // fn save_state(&self) {}
    // fn load_state(&self) {}

    fn clock_ppu(&self) {}

    fn handle_inputs(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::si_arcade::cpu::Cpu;
    use crate::si_arcade::mmu::Mmu;

    use super::*;

    #[test]
    fn cpu_test() {
        let mmu_debug = Rc::new(RefCell::new(Mmu::new_debug()));
        let mut cpu_debug = Cpu::new(&mmu_debug, 0x100);

        let mut cycles_counter: u64 = 0;
        let mut opcode: u8 = 0;
        for i in 0..650 {
            print_data_debug(cpu_debug.get_state(), cycles_counter);
            let cycles_opcode = cpu_debug.clock_debug();
            cycles_counter += cycles_opcode.0 as u64;
            opcode = cycles_opcode.1;
        }
        let result = cpu_debug.get_state();
        print_data_debug(cpu_debug.get_state(), cycles_counter);
        assert_eq!(result.0, 0); //Verify we reach pc = 0x0 after 651 operations
    }
}

pub fn print_data_debug(cpu_state: (u16, u16, u16, u16, u16, u16, u8), cycles_total: u64) {
    println!(
        "PC = {:#X}, AF = {:#X}, BC = {:#X}, DE = {:#X}, HL = {:#X}, SP = {:#X}, Cycles = {}, Total Cycles = {}",
        cpu_state.0, cpu_state.1, cpu_state.2, cpu_state.3, cpu_state.4, cpu_state.5, cpu_state.6, cycles_total
    );
}
