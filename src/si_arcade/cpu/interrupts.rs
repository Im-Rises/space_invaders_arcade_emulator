use super::super::cpu;

pub fn interrupt(mut cpu: cpu::Cpu, address: u16) {
    cpu.write(cpu.sp - 2, (cpu.pc & 0x00FF) as u8);
    cpu.write(cpu.sp - 1, ((cpu.pc & 0xFF00) >> 8) as u8);
    cpu.sp -= 2;
    cpu.pc = address;
}
