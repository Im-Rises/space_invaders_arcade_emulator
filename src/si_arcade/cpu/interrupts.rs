use super::super::cpu;

pub fn interrupt(mut cpu: cpu::Cpu, address: u16) {
    cpu.write(cpu.sp - 2, pc & 0x00FF);
    cpu.write(cpu.sp - 1, (pc & 0xFF00) >> 8);
    cpu.sp -= 2;
    pc = address;
}
