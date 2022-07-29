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
