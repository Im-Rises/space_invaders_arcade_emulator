use std::mem;

use crate::binary_lib::*;

use super::super::cpu;
use super::super::cpu::register::{Flag, Register};

/*---------------MOVE, LOAD AND STORE---------------*/
pub fn mov_a_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a = r;
    5
}

pub fn mov_b_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.b = r;
    5
}

pub fn mov_c_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.c = r;
    5
}

pub fn mov_d_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.d = r;
    5
}

pub fn mov_e_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.e = r;
    5
}

pub fn mov_h_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.h = r;
    5
}

pub fn mov_l_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.l = r;
    5
}

pub fn mov_m_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.write(cpu.regs.get_hl(), r);
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
    let data = cpu.fetch_byte();
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

pub fn stax_pr(cpu: &mut cpu::Cpu, pr: u16) -> u8 {
    cpu.write(pr, cpu.regs.a);
    7
}

pub fn ldax_pr(cpu: &mut cpu::Cpu, pr: u16) -> u8 {
    cpu.regs.a = cpu.read(pr);
    7
}

pub fn sta(cpu: &mut cpu::Cpu) -> u8 {
    let address = cpu.fetch_word();
    cpu.write(address, cpu.regs.a);
    13
}

pub fn lda(cpu: &mut cpu::Cpu) -> u8 {
    let address = cpu.fetch_word();
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
    4 //HERE    //? or 5 ?????
}

/*---------------STACK OPS---------------*/

pub fn push(cpu: &mut cpu::Cpu, address: u16) -> u8 {
    let address = Register::unpair_regs(address);
    cpu.write(cpu.sp.wrapping_sub(1), address.0);
    cpu.write(cpu.sp.wrapping_sub(2), address.1);
    cpu.sp = cpu.sp.wrapping_sub(2);
    11
}

pub fn pop_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = cpu.read(cpu.sp);
    cpu.regs.b = cpu.read(cpu.sp + 1);
    cpu.sp = cpu.sp.wrapping_add(2);
    10
}

pub fn pop_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = cpu.read(cpu.sp);
    cpu.regs.d = cpu.read(cpu.sp + 1);
    cpu.sp = cpu.sp.wrapping_add(2);
    10
}

pub fn pop_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = cpu.read(cpu.sp);
    cpu.regs.h = cpu.read(cpu.sp + 1);
    cpu.sp = cpu.sp.wrapping_add(2);
    10
}

pub fn pop_psw(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.f = cpu.read(cpu.sp);
    cpu.regs.a = cpu.read(cpu.sp + 1);
    cpu.sp = cpu.sp.wrapping_add(2);
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

pub fn jmp_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if cpu.regs.get_flag(flag) {
        jmp(cpu)
    } else {
        cpu.pc += 2;
        10
    }
}

pub fn jmp_not_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if !cpu.regs.get_flag(flag) {
        jmp(cpu)
    } else {
        cpu.pc += 2;
        10
    }
}

pub fn pchl(cpu: &mut cpu::Cpu) -> u8 {
    cpu.pc = cpu.regs.get_hl();
    5
}

/*---------------CALL---------------*/

pub fn call(cpu: &mut cpu::Cpu) -> u8 {
    push(cpu, cpu.pc + 2);
    cpu.pc = cpu.fetch_word();
    17
}

pub fn call_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if cpu.regs.get_flag(flag) {
        call(cpu)
    } else {
        cpu.pc += 2;
        11
    }
}

pub fn call_not_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if !cpu.regs.get_flag(flag) {
        call(cpu)
    } else {
        cpu.pc += 2;
        11
    }
}

/*---------------RETURN---------------*/

pub fn ret(cpu: &mut cpu::Cpu) -> u8 {
    cpu.pc = (cpu.read(cpu.sp) as u16 | (cpu.read(cpu.sp + 1) as u16) << 8) as u16;
    cpu.sp += 2;
    10
}

pub fn ret_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if cpu.regs.get_flag(flag) {
        ret(cpu) + 1
    } else {
        5
    }
}

pub fn ret_not_flag(cpu: &mut cpu::Cpu, flag: Flag) -> u8 {
    if !cpu.regs.get_flag(flag) {
        ret(cpu) + 1
    } else {
        5
    }
}

/*---------------RESTART---------------*/

pub fn rst(cpu: &mut cpu::Cpu, operand: u8) -> u8 {
    push(cpu, cpu.pc + 2);
    cpu.pc = (operand << 3) as u16;
    11
}

/*---------------INCREMENT AND DECREMENT---------------*/

pub fn inr_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = inr_subroutine(cpu, cpu.regs.a);
    5
}

pub fn inr_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = inr_subroutine(cpu, cpu.regs.b);
    5
}

pub fn inr_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = inr_subroutine(cpu, cpu.regs.c);
    5
}

pub fn inr_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = inr_subroutine(cpu, cpu.regs.d);
    5
}

pub fn inr_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = inr_subroutine(cpu, cpu.regs.e);
    5
}

pub fn inr_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = inr_subroutine(cpu, cpu.regs.h);
    5
}

pub fn inr_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = inr_subroutine(cpu, cpu.regs.l);
    5
}

pub fn dcr_a(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.a = dcr_subroutine(cpu, cpu.regs.a);
    5
}

pub fn dcr_b(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.b = dcr_subroutine(cpu, cpu.regs.b);
    5
}

pub fn dcr_c(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.c = dcr_subroutine(cpu, cpu.regs.c);
    5
}

pub fn dcr_d(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.d = dcr_subroutine(cpu, cpu.regs.d);
    5
}

pub fn dcr_e(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.e = dcr_subroutine(cpu, cpu.regs.e);
    5
}

pub fn dcr_h(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.h = dcr_subroutine(cpu, cpu.regs.h);
    5
}

pub fn dcr_l(cpu: &mut cpu::Cpu) -> u8 {
    cpu.regs.l = dcr_subroutine(cpu, cpu.regs.l);
    5
}

pub fn inr_m(cpu: &mut cpu::Cpu) -> u8 {
    let address = cpu.regs.get_hl();
    let result = inr_subroutine(cpu, cpu.read(address));
    cpu.write(address, result);
    10
}

pub fn dcr_m(cpu: &mut cpu::Cpu) -> u8 {
    let address = cpu.regs.get_hl();
    let result = dcr_subroutine(cpu, cpu.read(address));
    cpu.write(address, result);
    10
}

fn inr_subroutine(cpu: &mut cpu::Cpu, data: u8) -> u8 {
    let result = data.overflowing_add(1);
    cpu.regs.set_reset_flag(Flag::C, result.1);
    cpu.regs.update_flag_s(result.0);
    cpu.regs.update_flag_z(result.0);
    cpu.regs.update_flag_p(result.0);
    result.0
}

fn dcr_subroutine(cpu: &mut cpu::Cpu, data: u8) -> u8 {
    let result = data.overflowing_sub(1);
    cpu.regs.set_reset_flag(Flag::C, result.1);
    cpu.regs.update_flag_s(result.0);
    cpu.regs.update_flag_z(result.0);
    cpu.regs.update_flag_p(result.0);
    result.0
}

pub fn inx_b(cpu: &mut cpu::Cpu) -> u8 {
    let word = cpu.regs.get_bc().wrapping_add(1);
    (cpu.regs.b, cpu.regs.c) = Register::unpair_regs(word);
    5
}

pub fn inx_d(cpu: &mut cpu::Cpu) -> u8 {
    let word = cpu.regs.get_de().wrapping_add(1);
    (cpu.regs.d, cpu.regs.e) = Register::unpair_regs(word);
    5
}

pub fn inx_h(cpu: &mut cpu::Cpu) -> u8 {
    let word = cpu.regs.get_hl().wrapping_add(1);
    (cpu.regs.h, cpu.regs.l) = Register::unpair_regs(word);
    5
}

pub fn dcx_b(cpu: &mut cpu::Cpu) -> u8 {
    let word = cpu.regs.get_bc().wrapping_sub(1);
    (cpu.regs.b, cpu.regs.c) = Register::unpair_regs(word);
    5
}

pub fn dcx_d(cpu: &mut cpu::Cpu) -> u8 {
    let word = cpu.regs.get_de().wrapping_sub(1);
    (cpu.regs.d, cpu.regs.e) = Register::unpair_regs(word);
    5
}

pub fn dcx_h(cpu: &mut cpu::Cpu) -> u8 {
    let word = cpu.regs.get_hl().wrapping_sub(1);
    (cpu.regs.h, cpu.regs.l) = Register::unpair_regs(word);
    5
}

/*---------------ADD---------------*/

pub fn add_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a = add_subroutine_function(cpu, cpu.regs.a, r);
    4
}

pub fn add_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.read(cpu.regs.get_hl());
    cpu.regs.a = add_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

pub fn adi_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.fetch_byte();
    cpu.regs.a = add_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

fn add_subroutine_function(cpu: &mut cpu::Cpu, operand1: u8, operand2: u8) -> u8 {
    let result = operand1.overflowing_add(operand2);
    cpu.regs.set_reset_flag(Flag::C, result.1);
    cpu.regs.update_flag_a(operand1, operand2);
    cpu.regs.update_flag_s(result.0);
    cpu.regs.update_flag_z(result.0);
    cpu.regs.update_flag_p(result.0);
    result.0
}

pub fn adc_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a = adc_subroutine_function(cpu, cpu.regs.a, r);
    4
}

pub fn adc_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.read(cpu.regs.get_hl());
    cpu.regs.a = adc_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

pub fn aci_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.fetch_byte();
    cpu.regs.a = adc_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

fn adc_subroutine_function(cpu: &mut cpu::Cpu, operand1: u8, operand2: u8) -> u8 {
    let result_u16: u16 = operand1 as u16 + operand2 as u16 + cpu.regs.get_flag(Flag::C) as u16;
    let result_u8 = (result_u16 & 0x00FF) as u8;
    cpu.regs.set_reset_flag(Flag::C, result_u16 > 0xFF);
    cpu.regs.update_flag_s(result_u8);
    cpu.regs.update_flag_z(result_u8);
    cpu.regs.update_flag_p(result_u8);
    result_u8
}

pub fn dad_word(cpu: &mut cpu::Cpu, word: u16) -> u8 {
    let result: u32 = cpu.regs.get_hl() as u32 + word as u32;
    cpu.regs.set_reset_flag(Flag::C, result > 0xFFFF);
    (cpu.regs.h, cpu.regs.l) = Register::unpair_regs((result & 0x0000FFFF) as u16);
    10
}

/*---------------SUBTRACT---------------*/

pub fn sub_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a = sub_subroutine_function(cpu, cpu.regs.a, r);
    4
}

pub fn sub_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.read(cpu.regs.get_hl());
    cpu.regs.a = sub_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

pub fn sui(cpu: &mut cpu::Cpu) -> u8 {
    let operand2 = cpu.fetch_byte();
    cpu.regs.a = sub_subroutine_function(cpu, cpu.regs.a, operand2);
    7
}

fn sub_subroutine_function(cpu: &mut cpu::Cpu, operand1: u8, operand2: u8) -> u8 {
    let operand2 = (!operand2).wrapping_add(1);
    let result = add_subroutine_function(cpu, operand1, operand2);
    cpu.regs.set_reset_flag(Flag::C, !cpu.regs.get_flag(Flag::C));
    result
}

pub fn sbb_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a = sbb_subroutine_function(cpu, cpu.regs.a, r);
    4
}

pub fn sbb_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.read(cpu.regs.get_hl());
    cpu.regs.a = sbb_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

pub fn sbi(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.fetch_byte();
    cpu.regs.a = sbb_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

fn sbb_subroutine_function(cpu: &mut cpu::Cpu, operand1: u8, operand2: u8) -> u8 {
    let operand2 = operand2.wrapping_add(cpu.regs.get_flag(Flag::C) as u8); // Add carry
    let operand2 = (!operand2).wrapping_add(1); // Two's complement
    let result = add_subroutine_function(cpu, operand1, operand2);
    cpu.regs.set_reset_flag(Flag::C, !cpu.regs.get_flag(Flag::C));
    result
}

/*---------------LOGICAL---------------*/

pub fn ana_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a = and_subroutine_function(cpu, cpu.regs.a, r);
    4
}

pub fn ana_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.read(cpu.regs.get_hl());
    cpu.regs.a = and_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

pub fn ani(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.fetch_byte();
    cpu.regs.a = and_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

fn and_subroutine_function(cpu: &mut cpu::Cpu, operand1: u8, operand2: u8) -> u8 {
    let result = operand1 & operand2;
    cpu.regs.set_reset_flag(Flag::C, false);
    cpu.regs.set_reset_flag(Flag::A, true);
    cpu.regs.update_flag_s(result);
    cpu.regs.update_flag_z(result);
    cpu.regs.update_flag_p(result);
    result
}

pub fn xra_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a = xor_subroutine_function(cpu, cpu.regs.a, r);
    4
}

pub fn xri(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.fetch_byte();
    cpu.regs.a = xor_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

pub fn xra_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.read(cpu.regs.get_hl());
    cpu.regs.a = xor_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

fn xor_subroutine_function(cpu: &mut cpu::Cpu, operand1: u8, operand2: u8) -> u8 {
    let result = operand1 ^ operand2;
    cpu.regs.set_reset_flag(Flag::C, false);
    cpu.regs.set_reset_flag(Flag::A, false);
    cpu.regs.update_flag_s(result);
    cpu.regs.update_flag_z(result);
    cpu.regs.update_flag_p(result);
    result
}

pub fn ora_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    cpu.regs.a = or_subroutine_function(cpu, cpu.regs.a, r);
    4
}

pub fn ora_m(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.read(cpu.regs.get_hl());
    cpu.regs.a = or_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

pub fn ori(cpu: &mut cpu::Cpu) -> u8 {
    let operand = cpu.fetch_byte();
    cpu.regs.a = or_subroutine_function(cpu, cpu.regs.a, operand);
    7
}

fn or_subroutine_function(cpu: &mut cpu::Cpu, operand1: u8, operand2: u8) -> u8 {
    let result = operand1 | operand2;
    cpu.regs.set_reset_flag(Flag::C, false);
    cpu.regs.set_reset_flag(Flag::A, false);
    cpu.regs.update_flag_s(result);
    cpu.regs.update_flag_z(result);
    cpu.regs.update_flag_p(result);
    result
}

pub fn cmp_r(cpu: &mut cpu::Cpu, r: u8) -> u8 {
    subroutine_logical_compare(cpu, cpu.regs.a, r);
    4
}

pub fn cmp_m(cpu: &mut cpu::Cpu) -> u8 {
    subroutine_logical_compare(cpu, cpu.regs.a, cpu.read(cpu.regs.get_hl()));
    7
}

pub fn cpi(cpu: &mut cpu::Cpu) -> u8 {
    let operand2 = cpu.fetch_byte();
    subroutine_logical_compare(cpu, cpu.regs.a, operand2);
    7
}

pub fn subroutine_logical_compare(cpu: &mut cpu::Cpu, operand1: u8, operand2: u8) {
    let result = operand1.wrapping_sub(operand2);
    cpu.regs.set_reset_flag(Flag::C, !(operand1 >= operand2));
    cpu.regs.update_flag_s(result);
    cpu.regs.set_reset_flag(Flag::Z, operand1 == operand2);
    cpu.regs.update_flag_p(result);
    cpu.regs.update_flag_a(operand1, operand2);
}

/*---------------ROTATE---------------*/

pub fn rlc(cpu: &mut cpu::Cpu) -> u8 {
    let high_order_bit = get_bit(cpu.regs.a, 7);
    cpu.regs.set_reset_flag(Flag::C, high_order_bit);
    cpu.regs.a = cpu.regs.a.overflowing_shl(1).0;
    cpu.regs.a = set_reset_bit(cpu.regs.a, 0, high_order_bit);
    4
}

pub fn rrc(cpu: &mut cpu::Cpu) -> u8 {
    let low_order_bit = get_bit(cpu.regs.a, 0);
    cpu.regs.set_reset_flag(Flag::C, low_order_bit);
    cpu.regs.a = cpu.regs.a.overflowing_shr(1).0;
    cpu.regs.a = set_reset_bit(cpu.regs.a, 7, low_order_bit);
    4
}

pub fn ral(cpu: &mut cpu::Cpu) -> u8 {
    let high_order_bit = get_bit(cpu.regs.a, 7);
    let carry = cpu.regs.get_flag(Flag::C);
    cpu.regs.set_reset_flag(Flag::C, high_order_bit);
    cpu.regs.a = cpu.regs.a.overflowing_shl(1).0;
    cpu.regs.a = set_reset_bit(cpu.regs.a, 0, carry);
    4
}

pub fn rar(cpu: &mut cpu::Cpu) -> u8 {
    let low_order_bit = get_bit(cpu.regs.a, 0);
    let carry = cpu.regs.get_flag(Flag::C);
    cpu.regs.set_reset_flag(Flag::C, low_order_bit);
    cpu.regs.a = cpu.regs.a.overflowing_shr(1).0;
    cpu.regs.a = set_reset_bit(cpu.regs.a, 7, carry);
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
    /*
    (p15/16)
    If a carry out of the least significant four bits occurs
    during Step (1), the Auxiliary Carry bit is set; otherwise it is
    reset. Likewise, if a carry out of the most significant four
    bits occurs during Step (2). the normal Carry bit is set;
    otherwise, it is unaffected.
    */

    // Step 1
    if cpu.regs.a & 0x0F > 9 || cpu.regs.get_flag(Flag::A) {
        cpu.regs.a = cpu.regs.a.wrapping_add(0x06);
        cpu.regs.update_flag_a(cpu.regs.a, 0x06);
    };

    // Step 2
    if ((cpu.regs.a & 0xF0) >> 4) > 9 || cpu.regs.get_flag(Flag::C) {
        let result = cpu.regs.a.overflowing_add(0x60);
        if result.1 {
            cpu.regs.set_reset_flag(Flag::C, true);
        }
        cpu.regs.a = result.0;
    };

    cpu.regs.update_flag_s(cpu.regs.a);
    cpu.regs.update_flag_z(cpu.regs.a);
    cpu.regs.update_flag_p(cpu.regs.a);

    4
}

/*---------------INPUT/OUTPUT---------------*/

pub fn input_in(cpu: &mut cpu::Cpu) -> u8 {
    // let port = cpu.fetch_byte();
    // cpu.regs.a = cpu.inputs_outputs.borrow_mut().inputs(port, cpu.regs.a);
    // panic!("Error: Input opcode handling from opcode.rs file instead of trap handling in si_arcade.rs");
    10
}

pub fn output_out(cpu: &mut cpu::Cpu) -> u8 {
    // let port = cpu.fetch_byte();
    // cpu.inputs_outputs.borrow_mut().outputs(port, cpu.regs.a);
    // panic!("Error: Output opcode handling from opcode.rs file instead of trap handling in si_arcade.rs");
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

pub fn nop() -> u8 {
    4
}

pub fn hlt(cpu: &mut cpu::Cpu) -> u8 {
    cpu.halted = true;
    7
}
