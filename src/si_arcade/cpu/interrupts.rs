use crate::si_arcade::cpu::Cpu;

use super::super::cpu;

pub fn interrupt(mut cpu: &mut Cpu, interrupt_num: u8) {
    cpu.write(cpu.sp - 1, ((cpu.pc & 0xFF00) >> 8) as u8);
    cpu.write(cpu.sp - 2, (cpu.pc & 0x00FF) as u8);
    cpu.sp -= 2;
    cpu.pc = (interrupt_num * 8) as u16;
    cpu.halted = false;
}
