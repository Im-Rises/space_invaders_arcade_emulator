use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

use crate::si_arcade::cpu::interrupts::interrupt;
use crate::si_arcade::cpu::opcodes::*;
use crate::si_arcade::cpu::register::{Flag, Register};
use crate::si_arcade::inputs_outputs::InputsOutputs;

use super::mmu::Mmu;

pub mod interrupts;
mod opcodes;
mod register;

pub const CLOCK_FREQUENCY: usize = 1_996_800;

pub struct Cpu {
    regs: Register,
    sp: u16,
    pc: u16,
    inte: bool,
    halted: bool,
    cycles: u8,
    opcode: u8,
    mmu: Rc<RefCell<Mmu>>,
    inputs_outputs: InputsOutputs,
}

impl Cpu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>, ini_pc: u16) -> Cpu {
        Cpu {
            regs: Register::new(),
            sp: 0,
            pc: ini_pc,
            inte: false,
            halted: false,
            cycles: 0,
            opcode: 0,
            mmu: Rc::clone(&mmu),
            inputs_outputs: InputsOutputs::new(),
        }
    }

    pub fn clock(&mut self) {
        if !self.halted {
            if self.cycles == 0 {
                self.fetch_compute();
                // self.opcode = self.fetch_byte();
                // self.cycles = self.compute_opcode(self.opcode);
            }
            self.cycles -= 1;
        }
    }

    pub fn fetch_compute(&mut self) -> (u8, u8) {
        self.opcode = self.fetch_byte();
        self.cycles = self.compute_opcode(self.opcode);
        (self.cycles, self.opcode)
    }

    fn fetch_byte(&mut self) -> u8 {
        let data = self.read(self.pc);
        self.pc += 1;
        data
    }

    fn fetch_word(&mut self) -> u16 {
        (self.fetch_byte() as u16 | (self.fetch_byte() as u16) << 8) as u16
    }

    fn read(&self, address: u16) -> u8 {
        self.mmu.borrow().read(address)
    }

    fn write(&self, address: u16, data: u8) {
        self.mmu.borrow_mut().write(address, data);
    }

    fn compute_opcode(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => nop(),
            0x01 => lxi_b(self),
            0x02 => stax_pr(self, self.regs.get_bc()),
            0x03 => inx_b(self),
            0x04 => inr_b(self),
            0x05 => dcr_b(self),
            0x06 => mvi_b(self),
            0x07 => rlc(self),
            0x08 => nop(),
            0x09 => dad_word(self, self.regs.get_bc()),
            0x0A => ldax_pr(self, self.regs.get_bc()),
            0x0B => dcx_b(self),
            0x0C => inr_c(self),
            0x0D => dcr_c(self),
            0x0E => mvi_c(self),
            0x0F => rrc(self),
            0x10 => nop(),
            0x11 => lxi_d(self),
            0x12 => stax_pr(self, self.regs.get_de()),
            0x13 => inx_d(self),
            0x14 => inr_d(self),
            0x15 => dcr_d(self),
            0x16 => mvi_d(self),
            0x17 => ral(self),
            0x18 => nop(),
            0x19 => dad_word(self, self.regs.get_de()),
            0x1A => ldax_pr(self, self.regs.get_de()),
            0x1B => dcx_d(self),
            0x1C => inr_e(self),
            0x1D => dcr_e(self),
            0x1E => mvi_e(self),
            0x1F => rar(self),
            0x20 => nop(),
            0x21 => lxi_h(self),
            0x22 => shld(self),
            0x23 => inx_h(self),
            0x24 => inr_h(self),
            0x25 => dcr_h(self),
            0x26 => mvi_h(self),
            0x27 => daa(self),
            0x28 => nop(),
            0x29 => dad_word(self, self.regs.get_hl()),
            0x2A => lhld(self),
            0x2B => dcx_h(self),
            0x2C => inr_l(self),
            0x2D => dcr_l(self),
            0x2E => mvi_l(self),
            0x2F => cma(self),
            0x30 => nop(),
            0x31 => lxi_sp(self),
            0x32 => sta(self),
            0x33 => inx_sp(self),
            0x34 => inr_m(self),
            0x35 => dcr_m(self),
            0x36 => mvi_m(self),
            0x37 => stc(self),
            0x38 => nop(),
            0x39 => dad_word(self, self.sp),
            0x3A => lda(self),
            0x3B => dcx_sp(self),
            0x3C => inr_a(self),
            0x3D => dcr_a(self),
            0x3E => mvi_a(self),
            0x3F => cmc(self),
            0x40 => mov_b_r(self, self.regs.b),
            0x41 => mov_b_r(self, self.regs.c),
            0x42 => mov_b_r(self, self.regs.d),
            0x43 => mov_b_r(self, self.regs.e),
            0x44 => mov_b_r(self, self.regs.h),
            0x45 => mov_b_r(self, self.regs.l),
            0x46 => mov_b_m(self),
            0x47 => mov_b_r(self, self.regs.a),
            0x48 => mov_c_r(self, self.regs.b),
            0x49 => mov_c_r(self, self.regs.c),
            0x4A => mov_c_r(self, self.regs.d),
            0x4B => mov_c_r(self, self.regs.e),
            0x4C => mov_c_r(self, self.regs.h),
            0x4D => mov_c_r(self, self.regs.l),
            0x4E => mov_c_m(self), //HERE
            0x4F => mov_c_r(self, self.regs.a),
            0x50 => mov_d_r(self, self.regs.b),
            0x51 => mov_d_r(self, self.regs.c),
            0x52 => mov_d_r(self, self.regs.d),
            0x53 => mov_d_r(self, self.regs.e),
            0x54 => mov_d_r(self, self.regs.h),
            0x55 => mov_d_r(self, self.regs.l),
            0x56 => mov_d_m(self),
            0x57 => mov_d_r(self, self.regs.a),
            0x58 => mov_e_r(self, self.regs.b),
            0x59 => mov_e_r(self, self.regs.c),
            0x5A => mov_e_r(self, self.regs.d),
            0x5B => mov_e_r(self, self.regs.e),
            0x5C => mov_e_r(self, self.regs.h),
            0x5D => mov_e_r(self, self.regs.l),
            0x5E => mov_e_m(self),
            0x5F => mov_e_r(self, self.regs.a),
            0x60 => mov_h_r(self, self.regs.b),
            0x61 => mov_h_r(self, self.regs.c),
            0x62 => mov_h_r(self, self.regs.d),
            0x63 => mov_h_r(self, self.regs.e),
            0x64 => mov_h_r(self, self.regs.h),
            0x65 => mov_h_r(self, self.regs.l),
            0x66 => mov_h_m(self),
            0x67 => mov_h_r(self, self.regs.a),
            0x68 => mov_l_r(self, self.regs.b),
            0x69 => mov_l_r(self, self.regs.c),
            0x6A => mov_l_r(self, self.regs.d),
            0x6B => mov_l_r(self, self.regs.e),
            0x6C => mov_l_r(self, self.regs.h),
            0x6D => mov_l_r(self, self.regs.l),
            0x6E => mov_l_m(self),
            0x6F => mov_l_r(self, self.regs.a),
            0x70 => mov_m_r(self, self.regs.b),
            0x71 => mov_m_r(self, self.regs.c),
            0x72 => mov_m_r(self, self.regs.d),
            0x73 => mov_m_r(self, self.regs.e),
            0x74 => mov_m_r(self, self.regs.h),
            0x75 => mov_m_r(self, self.regs.l),
            0x76 => hlt(self),
            0x77 => mov_m_r(self, self.regs.a),
            0x78 => mov_a_r(self, self.regs.b),
            0x79 => mov_a_r(self, self.regs.c),
            0x7A => mov_a_r(self, self.regs.d),
            0x7B => mov_a_r(self, self.regs.e),
            0x7C => mov_a_r(self, self.regs.h),
            0x7D => mov_a_r(self, self.regs.l),
            0x7E => mov_a_m(self),
            0x7F => mov_a_r(self, self.regs.a),
            0x80 => add_r(self, self.regs.b),
            0x81 => add_r(self, self.regs.c),
            0x82 => add_r(self, self.regs.d),
            0x83 => add_r(self, self.regs.e),
            0x84 => add_r(self, self.regs.h),
            0x85 => add_r(self, self.regs.l),
            0x86 => add_m(self),
            0x87 => add_r(self, self.regs.a),
            0x88 => adc_r(self, self.regs.b),
            0x89 => adc_r(self, self.regs.c),
            0x8A => adc_r(self, self.regs.d),
            0x8B => adc_r(self, self.regs.e),
            0x8C => adc_r(self, self.regs.h),
            0x8D => adc_r(self, self.regs.l),
            0x8E => adc_m(self),
            0x8F => adc_r(self, self.regs.a),
            0x90 => sub_r(self, self.regs.b),
            0x91 => sub_r(self, self.regs.c),
            0x92 => sub_r(self, self.regs.d),
            0x93 => sub_r(self, self.regs.e),
            0x94 => sub_r(self, self.regs.h),
            0x95 => sub_r(self, self.regs.l),
            0x96 => sub_m(self),
            0x97 => sub_r(self, self.regs.a),
            0x98 => sbb_r(self, self.regs.b),
            0x99 => sbb_r(self, self.regs.c),
            0x9A => sbb_r(self, self.regs.d),
            0x9B => sbb_r(self, self.regs.e),
            0x9C => sbb_r(self, self.regs.h),
            0x9D => sbb_r(self, self.regs.l),
            0x9E => sbb_m(self),
            0x9F => sbb_r(self, self.regs.a),
            0xA0 => ana_r(self, self.regs.b),
            0xA1 => ana_r(self, self.regs.c),
            0xA2 => ana_r(self, self.regs.d),
            0xA3 => ana_r(self, self.regs.e),
            0xA4 => ana_r(self, self.regs.h),
            0xA5 => ana_r(self, self.regs.l),
            0xA6 => ana_m(self),
            0xA7 => ana_r(self, self.regs.a),
            0xA8 => xra_r(self, self.regs.b),
            0xA9 => xra_r(self, self.regs.c),
            0xAA => xra_r(self, self.regs.d),
            0xAB => xra_r(self, self.regs.e),
            0xAC => xra_r(self, self.regs.h),
            0xAD => xra_r(self, self.regs.l),
            0xAE => xra_m(self),
            0xAF => xra_r(self, self.regs.a),
            0xB0 => ora_r(self, self.regs.b),
            0xB1 => ora_r(self, self.regs.c),
            0xB2 => ora_r(self, self.regs.d),
            0xB3 => ora_r(self, self.regs.e),
            0xB4 => ora_r(self, self.regs.h),
            0xB5 => ora_r(self, self.regs.l),
            0xB6 => ora_m(self),
            0xB7 => ora_r(self, self.regs.a),
            0xB8 => cmp_r(self, self.regs.b),
            0xB9 => cmp_r(self, self.regs.c),
            0xBA => cmp_r(self, self.regs.d),
            0xBB => cmp_r(self, self.regs.e),
            0xBC => cmp_r(self, self.regs.h),
            0xBD => cmp_r(self, self.regs.l),
            0xBE => cmp_m(self),
            0xBF => cmp_r(self, self.regs.a),
            0xC0 => ret_not_flag(self, Flag::Z),
            0xC1 => pop_b(self),
            0xC2 => jmp_not_flag(self, Flag::Z),
            0xC3 => jmp(self),
            0xC4 => call_not_flag(self, Flag::Z),
            0xC5 => push(self, self.regs.get_bc()),
            0xC6 => adi_m(self),
            0xC7 => rst(self, 0),
            0xC8 => ret_flag(self, Flag::Z),
            0xC9 => ret(self),
            0xCA => jmp_flag(self, Flag::Z),
            0xCB => jmp(self),
            0xCC => call_flag(self, Flag::Z),
            0xCD => call(self),
            0xCE => aci_m(self),
            0xCF => rst(self, 1),
            0xD0 => ret_not_flag(self, Flag::C),
            0xD1 => pop_d(self),
            0xD2 => jmp_not_flag(self, Flag::C),
            0xD3 => output_out(self),
            0xD4 => call_not_flag(self, Flag::C),
            0xD5 => push(self, self.regs.get_de()),
            0xD6 => sui(self),
            0xD7 => rst(self, 2),
            0xD8 => ret_flag(self, Flag::C),
            0xD9 => ret(self),
            0xDA => jmp_flag(self, Flag::C),
            0xDB => input_in(self),
            0xDC => call_flag(self, Flag::C),
            0xDD => call(self),
            0xDE => sbi(self),
            0xDF => rst(self, 3),
            0xE0 => ret_not_flag(self, Flag::P),
            0xE1 => pop_h(self),
            0xE2 => jmp_not_flag(self, Flag::P),
            0xE3 => xthl(self),
            0xE4 => call_not_flag(self, Flag::P),
            0xE5 => push(self, self.regs.get_hl()),
            0xE6 => ani(self),
            0xE7 => rst(self, 4),
            0xE8 => ret_flag(self, Flag::P),
            0xE9 => pchl(self),
            0xEA => jmp_flag(self, Flag::P),
            0xEB => xchg(self),
            0xEC => call_flag(self, Flag::P),
            0xED => call(self),
            0xEE => xri(self),
            0xEF => rst(self, 5),
            0xF0 => ret_not_flag(self, Flag::S),
            0xF1 => pop_psw(self),
            0xF2 => jmp_not_flag(self, Flag::S),
            0xF3 => di(self),
            0xF4 => call_not_flag(self, Flag::S),
            0xF5 => push(self, self.regs.get_af()),
            0xF6 => ori(self),
            0xF7 => rst(self, 6),
            0xF8 => ret_flag(self, Flag::S),
            0xF9 => sphl(self),
            0xFA => jmp_flag(self, Flag::S),
            0xFB => ei(self),
            0xFC => call_flag(self, Flag::S),
            0xFD => call(self),
            0xFE => cpi(self),
            0xFF => rst(self, 7),
            _ => {
                println!("Error: unknown opcode");
                exit(1);
            }
        }
    }

    pub fn get_inte(&self) -> bool {
        self.inte
    }

    pub fn print_regs(&self, cycles_total: u64) {
        println!(
            "PC = {:#X}, AF = {:#X}, BC = {:#X}, DE = {:#X}, HL = {:#X}, SP = {:#X}, Cycles = {}, Total Cycles = {}",
            self.pc,
            self.regs.get_af(),
            self.regs.get_bc(),
            self.regs.get_de(),
            self.regs.get_hl(),
            self.sp,
            self.cycles,
            cycles_total
        );
    }
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
            cpu_debug.print_regs(cycles_counter);
            let cycles_opcode = cpu_debug.fetch_compute();
            cycles_counter += cycles_opcode.0 as u64;
            opcode = cycles_opcode.1;
        }
        cpu_debug.print_regs(cycles_counter);
        assert_eq!(cpu_debug.pc, 0); //Verify we reach pc = 0x0 after 651 operations
    }
}
