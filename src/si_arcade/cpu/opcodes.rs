use std::mem;

use crate::si_arcade::cpu;
use crate::si_arcade::cpu::register::Flag;

/*---------------MOVE, LOAD AND STORE---------------*/

pub fn mov_a_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.regs.a;
    5
}

pub fn mov_a_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.regs.b;
    5
}

pub fn mov_a_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.regs.c;
    5
}

pub fn mov_a_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.regs.d;
    5
}

pub fn mov_a_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.regs.e;
    5
}

pub fn mov_a_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.regs.h;
    5
}

pub fn mov_a_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.regs.l;
    5
}

pub fn mov_b_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.regs.a;
    5
}

pub fn mov_b_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.regs.b;
    5
}

pub fn mov_b_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.regs.c;
    5
}

pub fn mov_b_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.regs.d;
    5
}

pub fn mov_b_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.regs.e;
    5
}

pub fn mov_b_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.regs.h;
    5
}

pub fn mov_b_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.regs.l;
    5
}

pub fn mov_c_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.regs.a;
    5
}

pub fn mov_c_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.regs.b;
    5
}

pub fn mov_c_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.regs.c;
    5
}

pub fn mov_c_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.regs.d;
    5
}

pub fn mov_c_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.regs.e;
    5
}

pub fn mov_c_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.regs.h;
    5
}

pub fn mov_c_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.regs.l;
    5
}

pub fn mov_d_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.regs.a;
    5
}

pub fn mov_d_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.regs.b;
    5
}

pub fn mov_d_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.regs.c;
    5
}

pub fn mov_d_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.regs.d;
    5
}

pub fn mov_d_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.regs.e;
    5
}

pub fn mov_d_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.regs.h;
    5
}

pub fn mov_d_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.regs.l;
    5
}

pub fn mov_h_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.regs.a;
    5
}

pub fn mov_h_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.regs.b;
    5
}

pub fn mov_h_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.regs.c;
    5
}

pub fn mov_h_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.regs.d;
    5
}

pub fn mov_h_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.regs.e;
    5
}

pub fn mov_h_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.regs.h;
    5
}

pub fn mov_h_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.regs.l;
    5
}

pub fn mov_l_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.regs.a;
    5
}

pub fn mov_l_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.regs.b;
    5
}

pub fn mov_l_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.regs.c;
    5
}

pub fn mov_l_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.regs.d;
    5
}

pub fn mov_l_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.regs.e;
    5
}

pub fn mov_l_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.regs.h;
    5
}

pub fn mov_l_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.regs.l;
    5
}

pub fn mov_e_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.regs.a;
    5
}

pub fn mov_e_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.regs.b;
    5
}

pub fn mov_e_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.regs.c;
    5
}

pub fn mov_e_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.regs.d;
    5
}

pub fn mov_e_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.regs.e;
    5
}

pub fn mov_e_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.regs.h;
    5
}

pub fn mov_e_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.regs.l;
    5
}

pub fn mov_m_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_hl(), cpu.regs.a);
    7
}

pub fn mov_m_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_hl(), cpu.regs.b);
    7
}

pub fn mov_m_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_hl(), cpu.regs.c);
    7
}

pub fn mov_m_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_hl(), cpu.regs.d);
    7
}

pub fn mov_m_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_hl(), cpu.regs.e);
    7
}

pub fn mov_m_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_hl(), cpu.regs.h);
    7
}

pub fn mov_m_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_hl(), cpu.regs.l);
    7
}

pub fn mov_a_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.read(cpu.regs.get_hl());
    7
}

pub fn mov_b_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.read(cpu.regs.get_hl());
    7
}

pub fn mov_c_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.read(cpu.regs.get_hl());
    7
}

pub fn mov_d_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.read(cpu.regs.get_hl());
    7
}

pub fn mov_e_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.read(cpu.regs.get_hl());
    7
}

pub fn mov_h_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.read(cpu.regs.get_hl());
    7
}

pub fn mov_l_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.read(cpu.regs.get_hl());
    7
}

pub fn mvi_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = cpu.fetch_byte();
    7
}

pub fn mvi_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = cpu.fetch_byte();
    7
}

pub fn mvi_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.fetch_byte();
    7
}

pub fn mvi_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = cpu.fetch_byte();
    7
}

pub fn mvi_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.fetch_byte();
    7
}

pub fn mvi_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = cpu.fetch_byte();
    7
}

pub fn mvi_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.fetch_byte();
    7
}

pub fn mvi_m(cpu: &mut cpu::Cpu) -> u8 {
    let data = cpu.fetch_byte(); //???
    cpu.write(cpu.regs.get_hl(), data);
    10
}

pub fn lxi_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.fetch_byte();
    cpu.regs.b = cpu.fetch_byte();
    10
}

pub fn lxi_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.fetch_byte();
    cpu.regs.d = cpu.fetch_byte();
    10
}

pub fn lxi_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.fetch_byte();
    cpu.regs.h = cpu.fetch_byte();
    10
}

pub fn stax_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_bc(), cpu.regs.a);
    7
}

pub fn stax_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.regs.get_de(), cpu.regs.a);
    7
}

pub fn ldax_b(cpu: &mut cpu::Cpu) {
    cpu.regs.a = cpu.read(cpu.regs.get_bc());
}

pub fn ldax_d(cpu: &mut cpu::Cpu) {
    cpu.regs.a = cpu.read(cpu.regs.get_de());
}

pub fn sta(cpu: &mut cpu::Cpu) -> u8 {
    let address = cpu.fetch_word(); //???
    cpu.write(address, cpu.regs.a);
    7
}

pub fn lda(cpu: &mut cpu::Cpu) -> u8 {
    let address = cpu.fetch_word(); //???
    cpu.regs.a = cpu.read(address);
    13
}

pub fn shld(cpu: &mut cpu::Cpu) -> u8 {
    let address = cpu.fetch_word();
    cpu.write(address, cpu.regs.l);
    cpu.write(address + 1, cpu.regs.h);
    16
}

pub fn lhld(cpu: &mut cpu::Cpu) -> u8 {
    let address = cpu.fetch_word();
    cpu.regs.l = cpu.read(address);
    cpu.regs.h = cpu.read(address + 1);
    16
}

pub fn xchg(cpu: &mut cpu::Cpu) -> u8 {
    mem::swap(&mut cpu.regs.d, &mut cpu.regs.h);
    mem::swap(&mut cpu.regs.e, &mut cpu.regs.l);
    5
}

/*---------------STACK OPS---------------*/

pub fn push_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.sp - 1, cpu.regs.b);
    cpu.write(cpu.sp - 2, cpu.regs.c);
    cpu.sp -= 2;
    11
}

pub fn push_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.sp - 1, cpu.regs.d);
    cpu.write(cpu.sp - 2, cpu.regs.e);
    cpu.sp -= 2;
    11
}

pub fn push_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.sp - 1, cpu.regs.h);
    cpu.write(cpu.sp - 2, cpu.regs.l);
    cpu.sp -= 2;
    11
}

pub fn push_psw(cpu: &mut cpu::Cpu) -> u8 {
    cpu.write(cpu.sp - 1, cpu.regs.a);
    cpu.write(cpu.sp - 2, cpu.regs.f);
    cpu.sp -= 2;
    11
}

pub fn pop_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.read(cpu.sp);
    cpu.regs.b = cpu.read(cpu.sp + 1);
    cpu.sp += 2;
    10
}

pub fn pop_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.read(cpu.sp);
    cpu.regs.d = cpu.read(cpu.sp + 1);
    cpu.sp += 2;
    10
}

pub fn pop_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.read(cpu.sp);
    cpu.regs.h = cpu.read(cpu.sp + 1);
    cpu.sp += 2;
    10
}

pub fn pop_psw(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.f = cpu.read(cpu.sp);
    cpu.regs.a = cpu.read(cpu.sp + 1);
    cpu.sp += 2;
    10
}

pub fn xthl(cpu: &mut cpu::Cpu) -> u8 {
    let temp_l = cpu.regs.l;
    let temp_h = cpu.regs.h;
    cpu.regs.l = cpu.read(cpu.sp);
    cpu.regs.h = cpu.read(cpu.sp + 1);
    cpu.write(cpu.sp, temp_l);
    cpu.write(cpu.sp + 1, temp_h);
    18
}

pub fn sphl(cpu: &mut cpu::Cpu) -> u8 {
    cpu.sp = cpu.regs.get_hl();
    5
}

pub fn lxi_sp(cpu: &mut cpu::Cpu) -> u8 {
    cpu.sp = cpu.fetch_word();
    10
}

pub fn inx_sp(cpu: &mut cpu::Cpu) -> u8 {
    cpu.sp += 1;
    5
}

pub fn dcx_sp(cpu: &mut cpu::Cpu) -> u8 {
    cpu.sp -= 1;
    5
}

/*---------------JUMP---------------*/

pub fn jmp(cpu: &mut cpu::Cpu) -> u8 {
    cpu.pc = cpu.fetch_word();
    10
}

fn jmp_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if cpu.regs.get_flag(flag) {
        jmp(cpu)
    } else {
        cpu.pc += 2;
        10
    }
}

fn jmp_not_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if !cpu.regs.get_flag(flag) {
        jmp(cpu)
    } else {
        cpu.pc += 2;
        10
    }
}

/*pub fn jc(cpu: &mut cpu::Cpu) -> u8 {
    if cpu.regs.get_flag(Flag::C) {
        jmp(cpu)
    } else {
        cpu.pc += 1;
        10
    }
}

pub fn jnc(cpu: &mut cpu::Cpu) -> u8 {
    if !cpu.regs.get_flag(Flag::C) {
        jmp(cpu)
    } else {
        cpu.pc += 1;
        10
    }
}

pub fn jz(cpu: &mut cpu::Cpu) -> u8 {
    if cpu.regs.get_flag(Flag::Z) {
        jmp(cpu)
    } else {
        cpu.pc += 1;
        10
    }
}

pub fn jnz(cpu: &mut cpu::Cpu) -> u8 {
    if !cpu.regs.get_flag(Flag::Z) {
        jmp(cpu)
    } else {
        cpu.pc += 1;
        10
    }
}

pub fn jp(cpu: &mut cpu::Cpu) -> u8 {
    if !cpu.regs.get_flag(Flag::S) {
        jmp(cpu)
    } else {
        cpu.pc += 1;
        10
    }
}

pub fn jm(cpu: &mut cpu::Cpu) -> u8 {
    if cpu.regs.get_flag(Flag::S) {
        jmp(cpu)
    } else {
        cpu.pc += 1;
        10
    }
}

pub fn jpe(cpu: &mut cpu::Cpu) -> u8 {
    if cpu.regs.get_flag(Flag::P) {
        jmp(cpu)
    } else {
        cpu.pc += 1;
        10
    }
}

pub fn jpo(cpu: &mut cpu::Cpu) -> u8 {
    if !cpu.regs.get_flag(Flag::P) {
        jmp(cpu)
    } else {
        cpu.pc += 1;
        10
    }
}*/

pub fn pchl(cpu: &mut cpu::Cpu) -> u8 {
    cpu.pc = cpu.regs.get_hl();
    5
}
//////////
/*
/*---------------CALL---------------*/

pub fn call(cpu: &mut cpu::Cpu) -> u8 {
    cpu.push_word(cpu.pc + 2);
    cpu.pc = cpu.fetch_word();
    17
}

pub fn call_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if cpu.regs.get_flag(flag) {
        cpu.call()
    } else {
        cpu.pc += 2;
        11
    }
}

pub fn call_not_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if !cpu.regs.get_flag(flag) {
        cpu.call()
    } else {
        cpu.pc += 2;
        11
    }
}

/*---------------RETURN---------------*/

pub fn ret(cpu: &mut cpu::Cpu) -> u8 {
    cpu.pc = (cpu.read(cpu.sp) | cpu.read(cpu.sp + 1) << 8) as u16;
    cpu.sp += 2;
    10
}

pub fn ret_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if cpu.regs.get_flag(flag) {
        cpu.ret() + 1
    } else {
        5
    }
}

pub fn ret_not_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if !cpu.regs.get_flag(flag) {
        cpu.ret() + 1
    } else {
        5
    }
}

/*---------------RESTART---------------*/

pub fn rst(cpu: &mut cpu::Cpu, operand: u8) -> u8 {
    cpu.push_word(cpu.pc + 2);
    cpu.pc = operand as u16;
    11
}

/*---------------INCREMENT AND DECREMENT---------------*/

pub fn inr_r(&self, r: &mut u8) -> u8 {
    *r += 1;
    5
}

pub fn dcr_r(&self, r: &mut u8) -> u8 {
    *r -= 1;
    5
}

pub fn inr_m(&self) -> u8 {
    let address = cpu.regs.get_hl();
    cpu.write(address, cpu.read(address) + 1);
    10
}

pub fn dcr_m(&self) -> u8 {
    let address = cpu.regs.get_hl();
    cpu.write(address, cpu.read(address) - 1);
    10
}

pub fn inx_pr(&self, high_reg: &mut u8, low_reg: &mut u8) -> u8 {
    let word = register::Register::pair_regs(*high_reg, *low_reg) + 1;
    *high_reg = ((word & 0xFF00) >> 8) as u8;
    *low_reg = (word & 0x00FF) as u8;
    5
}

pub fn dcx_pr(&self, high_reg: &mut u8, low_reg: &mut u8) -> u8 {
    let word = register::Register::pair_regs(*high_reg, *low_reg) - 1;
    *high_reg = ((word & 0xFF00) >> 8) as u8;
    *low_reg = (word & 0x00FF) as u8;
    5
}

/*---------------ADD---------------*/

pub fn add_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a += r;
    4
}

pub fn adc_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a += r + cpu.regs.get_flag(Flag::C) as u8;
    4
}

pub fn add_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a += cpu.read(cpu.regs.get_hl());
    7
}

pub fn adc_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a += cpu.read(cpu.regs.get_hl()) + cpu.regs.get_flag(Flag::C) as u8;
    7
}

pub fn adi_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a += cpu.fetch_byte();
    7
}

pub fn aci_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a += cpu.fetch_byte() + cpu.regs.get_flag(Flag::C) as u8;
    7
}

pub fn dad_word(cpu: &mut cpu::Cpu, word: u16) -> u8 {
    (cpu.regs.h, cpu.regs.l) = Register::unpair_regs(cpu.regs.get_hl() + word);
    10
}

/*---------------SUBTRACT---------------*/

pub fn sub_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a -= r;
    4
}

pub fn sbb_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    let subtract = r + cpu.regs.get_flag(Flag::C) as u8;
    cpu.regs.a -= !subtract + 1;
    4
}

pub fn sub_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a -= cpu.read(cpu.regs.get_hl());
    7
}

pub fn sbb_m(cpu: &mut cpu::Cpu) -> u8 {
    let subtract = cpu.read(cpu.regs.get_hl()) + cpu.regs.get_flag(Flag::C) as u8;
    cpu.regs.a -= !subtract + 1;
    7
}

pub fn sui(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a -= cpu.fetch_byte();
    7
}

pub fn sbi(cpu: &mut cpu::Cpu) -> u8 {
    let subtract = cpu.fetch_byte() + cpu.regs.get_flag(Flag::C) as u8;
    cpu.regs.a -= !subtract + 1;
    7
}

/*---------------LOGICAL---------------*/

pub fn ana_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a &= r;
    4
}

pub fn xra_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a ^= r;
    4
}

pub fn ora_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a |= r;
    4
}

pub fn cmp_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a == r;
    4
}

pub fn ana_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a &= cpu.read(cpu.regs.get_hl());
    7
}

pub fn xra_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a ^= cpu.read(cpu.regs.get_hl());
    7
}

pub fn ora_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a |= cpu.read(cpu.regs.get_hl());
    7
}

pub fn cmp_m(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a == cpu.read(cpu.regs.get_hl());
    7
}

pub fn ani(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a &= cpu.fetch_byte();
    7
}

pub fn xri(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a ^= cpu.fetch_byte();
    7
}

pub fn ori(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a |= cpu.fetch_byte();
    7
}

pub fn cpi(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a == cpu.fetch_byte();
    7
}

/*---------------ROTATE---------------*/

pub fn rlc(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a <<= 1;
    4
}

pub fn rrc(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a >>= 1;
    4
}

pub fn ral(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.set_reset_flag(Flag::C, binary_lib::get_bit(cpu.regs.a, 7));
    cpu.regs.a <<= 1;
    4
}

pub fn rar(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.set_reset_flag(Flag::C, binary_lib::get_bit(cpu.regs.a, 0));
    cpu.regs.a >>= 1;
    4
}

/*---------------SPECIALS---------------*/

pub fn cma(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = !cpu.regs.a;
    4
}

pub fn stc(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.set_reset_flag(Flag::C, true);
    4
}

pub fn cmc(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.set_reset_flag(Flag::C, !cpu.regs.get_flag(Flag::C));
    4
}

pub fn daa(cpu: &mut cpu::Cpu) -> u8 {
    if cpu.regs.a & 0x0F > 9 || cpu.regs.get_flag(Flag::A) {
        cpu.regs.a += 0x06;
    };

    if ((cpu.regs.a & 0xF0) >> 8) > 9 || cpu.regs.get_flag(Flag::C) {
        cpu.regs.a += 0x60;
    };
    4
}

/*---------------INPUT/OUTPUT---------------*/

pub fn input_in(&self) -> u8 {
    10
}

pub fn output_out(&self) -> u8 {
    10
}

/*---------------CONTROL---------------*/

pub fn ei(cpu: &mut cpu::Cpu) -> u8 {
    cpu.inte = true;
    4
}

pub fn di(cpu: &mut cpu::Cpu) -> u8 {
    cpu.inte = false;
    4
}

pub fn nop(&self) -> u8 {
    4
}

pub fn hlt(cpu: &mut cpu::Cpu) -> u8 {
    cpu.halted = true;
    7
}
*/
