use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::binary_lib;
use crate::si_arcade::cpu::register::{Flag, Register};

use super::mmu::Mmu;

mod opcodes;
mod register;

const CLOCK_FREQUENCY: usize = 2_000_000;

pub struct Cpu {
    regs: Register,
    sp: u16,
    pc: u16,
    stat: u16,
    inte: bool,
    halted: bool,
    opcode: u8,
    cycles: u8,
    mmu: Rc<RefCell<Mmu>>,
}

impl Cpu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>) -> Cpu {
        Cpu {
            regs: Register::new(),
            sp: 0,
            pc: 0x100,
            stat: 0,
            inte: false,
            halted: false,
            opcode: 0,
            cycles: 0,
            mmu: Rc::clone(&mmu),
        }
    }

    fn clock(&mut self) {
        if self.cycles == 0 {
            let opcode = self.fetch_byte();
            self.cycles = self.compute_opcode(opcode);
        }
        self.cycles -= 1;
    }

    fn fetch_byte(&mut self) -> u8 {
        let data = self.read(self.pc);
        self.pc += 1;
        data
    }

    fn fetch_word(&mut self) -> u16 {
        (self.fetch_byte() | self.fetch_byte() << 8) as u16
    }

    fn read(&self, address: u16) -> u8 {
        self.mmu.borrow().read(address)
    }

    fn write(&self, address: u16, data: u8) {
        self.mmu.borrow_mut().write(address, data);
    }

    fn compute_opcode(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => self.nop(),
            0x01 => lxi_pr(self.regs.b, self.regs.c),
            // 0x02 => (),
            // 0x03 => (),
            // 0x04 => (),
            // 0x05 => (),
            // 0x06 => (),
            // 0x07 => (),
            // 0x08 => (),
            // 0x09 => (),
            // 0x0A => (),
            // 0x0B => (),
            // 0x0C => (),
            // 0x0D => (),
            // 0x0E => (),
            // 0x0F => (),
            // 0x10 => self.nop(),
            // 0x11 => self.lxi_pr(&mut self.regs.d, &mut self.regs.e),
            // 0x12 => (),
            // 0x13 => (),
            // 0x14 => (),
            // 0x15 => (),
            // 0x16 => (),
            // 0x17 => (),
            // 0x18 => (),
            // 0x19 => (),
            // 0x1A => (),
            // 0x1B => (),
            // 0x1C => (),
            // 0x1D => (),
            // 0x1E => (),
            // 0x1F => (),
            // 0x20 => self.nop(),
            // 0x21 => self.lxi_ps(&mut self.regs.h, &mut self.regs.l),
            // 0x22 => (),
            // 0x23 => (),
            // 0x24 => (),
            // 0x25 => (),
            // 0x26 => (),
            // 0x27 => (),
            // 0x28 => (),
            // 0x29 => (),
            // 0x2A => (),
            // 0x2B => (),
            // 0x2C => (),
            // 0x2D => (),
            // 0x2E => (),
            // 0x2F => (),
            // 0x30 => self.nop(),
            // 0x31 => self.lxi_sp(),
            // 0x32 => (),
            // 0x33 => (),
            // 0x34 => (),
            // 0x35 => (),
            // 0x36 => (),
            // 0x37 => (),
            // 0x38 => (),
            // 0x39 => (),
            // 0x3A => (),
            // 0x3B => (),
            // 0x3C => (),
            // 0x3D => (),
            // 0x3E => (),
            // 0x3F => (),
            // 0x40 => (),
            // 0x41 => (),
            // 0x42 => (),
            // 0x43 => (),
            // 0x44 => (),
            // 0x45 => (),
            // 0x46 => (),
            // 0x47 => (),
            // 0x48 => (),
            // 0x49 => (),
            // 0x4A => (),
            // 0x4B => (),
            // 0x4C => (),
            // 0x4D => (),
            // 0x4E => (),
            // 0x4F => (),
            // 0x50 => (),
            // 0x51 => (),
            // 0x52 => (),
            // 0x53 => (),
            // 0x54 => (),
            // 0x55 => (),
            // 0x56 => (),
            // 0x57 => (),
            // 0x58 => (),
            // 0x59 => (),
            // 0x5A => (),
            // 0x5B => (),
            // 0x5C => (),
            // 0x5D => (),
            // 0x5E => (),
            // 0x5F => (),
            // 0x60 => (),
            // 0x61 => (),
            // 0x62 => (),
            // 0x63 => (),
            // 0x64 => (),
            // 0x65 => (),
            // 0x66 => (),
            // 0x67 => (),
            // 0x68 => (),
            // 0x69 => (),
            // 0x6A => (),
            // 0x6B => (),
            // 0x6C => (),
            // 0x6D => (),
            // 0x6E => (),
            // 0x6F => (),
            // 0x70 => (),
            // 0x71 => (),
            // 0x72 => (),
            // 0x73 => (),
            // 0x74 => (),
            // 0x75 => (),
            // 0x76 => (),
            // 0x77 => (),
            // 0x78 => (),
            // 0x79 => (),
            // 0x7A => (),
            // 0x7B => (),
            // 0x7C => (),
            // 0x7D => (),
            // 0x7E => (),
            // 0x7F => (),
            // 0x80 => (),
            // 0x81 => (),
            // 0x82 => (),
            // 0x83 => (),
            // 0x84 => (),
            // 0x85 => (),
            // 0x86 => (),
            // 0x87 => (),
            // 0x88 => (),
            // 0x89 => (),
            // 0x8A => (),
            // 0x8B => (),
            // 0x8C => (),
            // 0x8D => (),
            // 0x8E => (),
            // 0x8F => (),
            // 0x90 => (),
            // 0x91 => (),
            // 0x92 => (),
            // 0x93 => (),
            // 0x94 => (),
            // 0x95 => (),
            // 0x96 => (),
            // 0x97 => (),
            // 0x98 => (),
            // 0x99 => (),
            // 0x9A => (),
            // 0x9B => (),
            // 0x9C => (),
            // 0x9D => (),
            // 0x9E => (),
            // 0x9F => (),
            // 0xA0 => (),
            // 0xA1 => (),
            // 0xA2 => (),
            // 0xA3 => (),
            // 0xA4 => (),
            // 0xA5 => (),
            // 0xA6 => (),
            // 0xA7 => (),
            // 0xA8 => (),
            // 0xA9 => (),
            // 0xAA => (),
            // 0xAB => (),
            // 0xAC => (),
            // 0xAD => (),
            // 0xAE => (),
            // 0xAF => (),
            // 0xB0 => (),
            // 0xB1 => (),
            // 0xB2 => (),
            // 0xB3 => (),
            // 0xB4 => (),
            // 0xB5 => (),
            // 0xB6 => (),
            // 0xB7 => (),
            // 0xB8 => (),
            // 0xB9 => (),
            // 0xBA => (),
            // 0xBB => (),
            // 0xBC => (),
            // 0xBD => (),
            // 0xBE => (),
            // 0xBF => (),
            // 0xC0 => (),
            // 0xC1 => (),
            // 0xC2 => (),
            // 0xC3 => (),
            // 0xC4 => (),
            // 0xC5 => (),
            // 0xC6 => (),
            // 0xC7 => (),
            // 0xC8 => (),
            // 0xC9 => (),
            // 0xCA => (),
            // 0xCB => (),
            // 0xCC => (),
            // 0xCD => (),
            // 0xCE => (),
            // 0xCF => (),
            // 0xD0 => (),
            // 0xD1 => (),
            // 0xD2 => (),
            // 0xD3 => (),
            // 0xD4 => (),
            // 0xD5 => (),
            // 0xD6 => (),
            // 0xD7 => (),
            // 0xD8 => (),
            // 0xD9 => (),
            // 0xDA => (),
            // 0xDB => (),
            // 0xDC => (),
            // 0xDD => (),
            // 0xDE => (),
            // 0xDF => (),
            // 0xE0 => (),
            // 0xE1 => (),
            // 0xE2 => (),
            // 0xE3 => (),
            // 0xE4 => (),
            // 0xE5 => (),
            // 0xE6 => (),
            // 0xE7 => (),
            // 0xE8 => (),
            // 0xE9 => (),
            // 0xEA => (),
            // 0xEB => (),
            // 0xEC => (),
            // 0xED => (),
            // 0xEE => (),
            // 0xEF => (),
            // 0xF0 => (),
            // 0xF1 => (),
            // 0xF2 => (),
            // 0xF3 => (),
            // 0xF4 => (),
            // 0xF5 => (),
            // 0xF6 => (),
            // 0xF7 => (),
            // 0xF8 => (),
            // 0xF9 => (),
            // 0xFA => (),
            // 0xFB => (),
            // 0xFC => (),
            // 0xFD => (),
            // 0xFE => (),
            // 0xFF => (),
            _ => 10,
        }
    }
}

impl Cpu {
    /*---------------MOVE, LOAD AND STORE---------------*/

    fn mov_r1_r2(&self, r1: &mut u8, r2: u8) -> u8 {
        *r1 = r2;
        5
    }

    fn mov_m_r(&mut self, r: u8) -> u8 {
        self.write(self.regs.get_hl(), r);
        7
    }

    fn mov_r_m(&mut self, r: &mut u8) -> u8 {
        *r = self.read(self.regs.get_hl());
        7
    }

    fn mvi_r(&mut self, r: &mut u8) -> u8 {
        *r = self.fetch_byte();
        7
    }

    fn mvi_m(&mut self) -> u8 {
        let data = self.fetch_byte(); //???
        self.write(self.regs.get_hl(), data);
        10
    }

    fn lxi_pr(&mut self, high_reg: u8, low_reg: u8) -> u8 {
        *low_reg = self.fetch_byte();
        *high_reg = self.fetch_byte();
        10
    }

    fn stax_pr(&mut self, pair_regs: u16) -> u8 {
        self.write(pair_regs, self.regs.a);
        7
    }

    fn ldax_pr(&mut self, pair_regs: u16) {
        self.regs.a = self.read(pair_regs);
    }

    fn sta(&mut self) -> u8 {
        let address = self.fetch_word(); //???
        self.write(address, self.regs.a);
        7
    }

    fn lda(&mut self) -> u8 {
        let address = self.fetch_word(); //???
        self.regs.a = self.read(address);
        13
    }

    fn shld(&mut self) -> u8 {
        let address = self.fetch_word();
        self.write(address, self.regs.l);
        self.write(address + 1, self.regs.h);
        16
    }

    fn lhld(&mut self) -> u8 {
        let address = self.fetch_word();
        self.regs.l = self.read(address);
        self.regs.h = self.read(address + 1);
        16
    }

    fn xchg(&mut self) -> u8 {
        mem::swap(&mut self.regs.d, &mut self.regs.h);
        mem::swap(&mut self.regs.e, &mut self.regs.l);
        5
    }

    /*---------------STACK OPS---------------*/

    fn push_hi_lo(&mut self, high: u8, low: u8) -> u8 {
        self.write(self.sp - 1, high);
        self.write(self.sp - 2, low);
        self.sp -= 2;
        11
    }

    fn push_word(&mut self, data: u16) -> u8 {
        self.push_hi_lo(((data & 0xFF00) >> 8) as u8, (data & 0x00FF) as u8)
    }

    fn pop(&mut self, high_reg: &mut u8, low_reg: &mut u8) -> u8 {
        *low_reg = self.read(self.sp);
        *high_reg = self.read(self.sp + 1);
        self.sp += 2;
        10
    }

    fn xthl(&mut self) -> u8 {
        let temp_l = self.regs.l;
        let temp_h = self.regs.h;
        self.regs.l = self.read(self.sp);
        self.regs.h = self.read(self.sp + 1);
        self.write(self.sp, temp_l);
        self.write(self.sp + 1, temp_h);
        18
    }

    fn sphl(&mut self) -> u8 {
        self.sp = self.regs.get_hl();
        5
    }

    fn lxi_sp(&mut self) -> u8 {
        self.sp = self.fetch_word();
        10
    }

    fn inx_sp(&mut self) -> u8 {
        self.sp += 1;
        5
    }

    fn dcx_sp(&mut self) -> u8 {
        self.sp -= 1;
        5
    }

    /*---------------JUMP---------------*/

    fn jmp(&mut self) -> u8 {
        self.pc = self.fetch_word();
        10
    }

    // /*JC Instructions*/
    //
    // fn jc(&mut self) {
    //     if self.regs.get_flag(Flag::C) {
    //         self.jmp();
    //     } else {
    //         pc += 1;
    //     }
    // }
    //
    // /*JNC Instructions*/
    //
    // fn jnc(&mut self) {
    //     if !self.regs.get_flag(Flag::C) {
    //         self.jmp();
    //     } else {
    //         pc += 1;
    //     }
    // }
    //
    // /*JZ Instructions*/
    //
    // fn jz(&mut self) {
    //     if self.regs.get_flag(Flag::Z) {
    //         self.jmp();
    //     } else {
    //         pc += 1;
    //     }
    // }
    //
    // /*JNZ Instructions*/
    //
    // fn jnz(&mut self) {
    //     if !self.regs.get_flag(Flag::Z) {
    //         self.jmp();
    //     } else {
    //         pc += 1;
    //     }
    // }
    //
    // /*JP Instructions*/
    //
    // fn jp(&mut self) {
    //     if !self.regs.get_flag(Flag::S) {
    //         self.jmp();
    //     } else {
    //         pc += 1;
    //     }
    // }
    //
    // /*JM Instructions*/
    //
    // fn jm(&mut self) {
    //     if self.regs.get_flag(Flag::S) {
    //         self.jmp();
    //     } else {
    //         pc += 1;
    //     }
    // }
    //
    // /*JPE Instructions*/
    //
    // fn jpe(&mut self) {
    //     if self.regs.get_flag(Flag::P) {
    //         self.jmp();
    //     } else {
    //         pc += 1;
    //     }
    // }
    //
    // /*JPO Instructions*/
    //
    // fn jpo(&mut self) {
    //     if !self.regs.get_flag(Flag::P) {
    //         self.jmp();
    //     } else {
    //         pc += 1;
    //     }
    // }

    fn jmp_flag(&mut self, flag: Flag) -> u8 {
        if self.regs.get_flag(flag) {
            self.jmp()
        } else {
            self.pc += 2;
            10
        }
    }

    fn jmp_not_flag(&mut self, flag: Flag) -> u8 {
        if !self.regs.get_flag(flag) {
            self.jmp()
        } else {
            self.pc += 2;
            10
        }
    }

    fn pchl(&mut self) -> u8 {
        self.pc = self.regs.get_hl();
        5
    }

    /*---------------CALL---------------*/

    fn call(&mut self) -> u8 {
        self.push_word(self.pc + 2);
        self.pc = self.fetch_word();
        17
    }

    fn call_flag(&mut self, flag: Flag) -> u8 {
        if self.regs.get_flag(flag) {
            self.call()
        } else {
            self.pc += 2;
            11
        }
    }

    fn call_not_flag(&mut self, flag: Flag) -> u8 {
        if !self.regs.get_flag(flag) {
            self.call()
        } else {
            self.pc += 2;
            11
        }
    }

    /*---------------RETURN---------------*/

    fn ret(&mut self) -> u8 {
        self.pc = (self.read(self.sp) | self.read(self.sp + 1) << 8) as u16;
        self.sp += 2;
        10
    }

    fn ret_flag(&mut self, flag: Flag) -> u8 {
        if self.regs.get_flag(flag) {
            self.ret() + 1
        } else {
            5
        }
    }

    fn ret_not_flag(&mut self, flag: Flag) -> u8 {
        if !self.regs.get_flag(flag) {
            self.ret() + 1
        } else {
            5
        }
    }

    /*---------------RESTART---------------*/

    fn rst(&mut self, operand: u8) -> u8 {
        self.push_word(self.pc + 2);
        self.pc = operand as u16;
        11
    }

    /*---------------INCREMENT AND DECREMENT---------------*/

    fn inr_r(&self, r: &mut u8) -> u8 {
        *r += 1;
        5
    }

    fn dcr_r(&self, r: &mut u8) -> u8 {
        *r -= 1;
        5
    }

    fn inr_m(&self) -> u8 {
        let address = self.regs.get_hl();
        self.write(address, self.read(address) + 1);
        10
    }

    fn dcr_m(&self) -> u8 {
        let address = self.regs.get_hl();
        self.write(address, self.read(address) - 1);
        10
    }

    fn inx_pr(&self, high_reg: &mut u8, low_reg: &mut u8) -> u8 {
        let word = register::Register::pair_regs(*high_reg, *low_reg) + 1;
        *high_reg = ((word & 0xFF00) >> 8) as u8;
        *low_reg = (word & 0x00FF) as u8;
        5
    }

    fn dcx_pr(&self, high_reg: &mut u8, low_reg: &mut u8) -> u8 {
        let word = register::Register::pair_regs(*high_reg, *low_reg) - 1;
        *high_reg = ((word & 0xFF00) >> 8) as u8;
        *low_reg = (word & 0x00FF) as u8;
        5
    }

    /*---------------ADD---------------*/

    fn add_r(&mut self, r: u8) -> u8 {
        self.regs.a += r;
        4
    }

    fn adc_r(&mut self, r: u8) -> u8 {
        self.regs.a += r + self.regs.get_flag(Flag::C) as u8;
        4
    }

    fn add_m(&mut self) -> u8 {
        self.regs.a += self.read(self.regs.get_hl());
        7
    }

    fn adc_m(&mut self) -> u8 {
        self.regs.a += self.read(self.regs.get_hl()) + self.regs.get_flag(Flag::C) as u8;
        7
    }

    fn adi_m(&mut self) -> u8 {
        self.regs.a += self.fetch_byte();
        7
    }

    fn aci_m(&mut self) -> u8 {
        self.regs.a += self.fetch_byte() + self.regs.get_flag(Flag::C) as u8;
        7
    }

    fn dad_word(&mut self, word: u16) -> u8 {
        (self.regs.h, self.regs.l) = Register::unpair_regs(self.regs.get_hl() + word);
        10
    }

    /*---------------SUBTRACT---------------*/

    fn sub_r(&mut self, r: u8) -> u8 {
        self.regs.a -= r;
        4
    }

    fn sbb_r(&mut self, r: u8) -> u8 {
        let subtract = r + self.regs.get_flag(Flag::C) as u8;
        self.regs.a -= !subtract + 1;
        4
    }

    fn sub_m(&mut self) -> u8 {
        self.regs.a -= self.read(self.regs.get_hl());
        7
    }

    fn sbb_m(&mut self) -> u8 {
        let subtract = self.read(self.regs.get_hl()) + self.regs.get_flag(Flag::C) as u8;
        self.regs.a -= !subtract + 1;
        7
    }

    fn sui(&mut self) -> u8 {
        self.regs.a -= self.fetch_byte();
        7
    }

    fn sbi(&mut self) -> u8 {
        let subtract = self.fetch_byte() + self.regs.get_flag(Flag::C) as u8;
        self.regs.a -= !subtract + 1;
        7
    }

    /*---------------LOGICAL---------------*/

    fn ana_r(&mut self, r: u8) -> u8 {
        self.regs.a &= r;
        4
    }

    fn xra_r(&mut self, r: u8) -> u8 {
        self.regs.a ^= r;
        4
    }

    fn ora_r(&mut self, r: u8) -> u8 {
        self.regs.a |= r;
        4
    }

    fn cmp_r(&mut self, r: u8) -> u8 {
        self.regs.a == r;
        4
    }

    fn ana_m(&mut self) -> u8 {
        self.regs.a &= self.read(self.regs.get_hl());
        7
    }

    fn xra_m(&mut self) -> u8 {
        self.regs.a ^= self.read(self.regs.get_hl());
        7
    }

    fn ora_m(&mut self) -> u8 {
        self.regs.a |= self.read(self.regs.get_hl());
        7
    }

    fn cmp_m(&mut self) -> u8 {
        self.regs.a == self.read(self.regs.get_hl());
        7
    }

    fn ani(&mut self) -> u8 {
        self.regs.a &= self.fetch_byte();
        7
    }

    fn xri(&mut self) -> u8 {
        self.regs.a ^= self.fetch_byte();
        7
    }

    fn ori(&mut self) -> u8 {
        self.regs.a |= self.fetch_byte();
        7
    }

    fn cpi(&mut self) -> u8 {
        self.regs.a == self.fetch_byte();
        7
    }

    /*---------------ROTATE---------------*/

    fn rlc(&mut self) -> u8 {
        self.regs.a <<= 1;
        4
    }

    fn rrc(&mut self) -> u8 {
        self.regs.a >>= 1;
        4
    }

    fn ral(&mut self) -> u8 {
        self.regs.set_reset_flag(Flag::C, binary_lib::get_bit(self.regs.a, 7));
        self.regs.a <<= 1;
        4
    }

    fn rar(&mut self) -> u8 {
        self.regs.set_reset_flag(Flag::C, binary_lib::get_bit(self.regs.a, 0));
        self.regs.a >>= 1;
        4
    }

    /*---------------SPECIALS---------------*/

    fn cma(&mut self) -> u8 {
        self.regs.a = !self.regs.a;
        4
    }

    fn stc(&mut self) -> u8 {
        self.regs.set_reset_flag(Flag::C, true);
        4
    }

    fn cmc(&mut self) -> u8 {
        self.regs.set_reset_flag(Flag::C, !self.regs.get_flag(Flag::C));
        4
    }

    fn daa(&mut self) -> u8 {
        if self.regs.a & 0x0F > 9 || self.regs.get_flag(Flag::A) {
            self.regs.a += 0x06;
        };

        if ((self.regs.a & 0xF0) >> 8) > 9 || self.regs.get_flag(Flag::C) {
            self.regs.a += 0x60;
        };
        4
    }

    /*---------------INPUT/OUTPUT---------------*/

    fn input_in(&self) -> u8 {
        10
    }

    fn output_out(&self) -> u8 {
        10
    }

    /*---------------CONTROL---------------*/

    fn ei(&mut self) -> u8 {
        self.inte = true;
        4
    }

    fn di(&mut self) -> u8 {
        self.inte = false;
        4
    }

    fn nop(&self) -> u8 {
        4
    }

    fn hlt(&mut self) -> u8 {
        self.halted = true;
        7
    }
}
