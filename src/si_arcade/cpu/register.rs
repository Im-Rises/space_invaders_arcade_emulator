use crate::binary_lib;

pub enum Flag {
    //Three bits are unused
    S = 7,
    Z = 6,
    A = 4,
    P = 2,
    C = 0,
}

pub struct Register {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Register {
    pub fn new() -> Register {
        Register {
            a: 0,
            f: 0b0000_0010,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }
}

impl Register {
    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
    }

    pub fn set_af(&mut self, data: u16) {
        self.a = (data >> 8) as u8;
        self.f = (data & 0x00ff) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        u16::from(self.b) << 8 | u16::from(self.c)
    }

    pub fn set_bc(&mut self, data: u16) {
        self.b = (data >> 8) as u8;
        self.c = (data & 0x00ff) as u8;
    }

    pub fn get_de(&self) -> u16 {
        u16::from(self.d) << 8 | u16::from(self.e)
    }

    pub fn set_de(&mut self, data: u16) {
        self.d = (data >> 8) as u8;
        self.e = (data & 0x00ff) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        u16::from(self.h) << 8 | u16::from(self.l)
    }

    pub fn set_hl(&mut self, data: u16) {
        self.h = (data >> 8) as u8;
        self.l = (data & 0x00ff) as u8;
    }

    pub fn pair_regs(hi: u8, lo: u8) -> u16 {
        ((hi as u16) << 8 | lo as u16) as u16
    }

    pub fn unpair_regs(pr: u16) -> (u8, u8) {
        (((pr & 0xFF00) >> 8) as u8, (pr & 0x00FF) as u8)
    }
}

/*
S - Sign Flag
Z - Zero Flag
0 - Not used, always zero
A - also called AC, Auxiliary Carry Flag
0 - Not used, always zero
P - Parity Flag
1 - Not used, always one
C - Carry Flag
*/

impl Register {
    pub fn get_flag(&self, f: Flag) -> bool {
        binary_lib::get_bit(self.f, f as usize)
    }

    pub fn set_reset_flag(&mut self, f: Flag, bit: bool) {
        if bit {
            self.f = binary_lib::set_bit(self.f, f as usize)
        } else {
            self.f = binary_lib::reset_bit(self.f, f as usize)
        }
    }

    pub fn update_flag_s(&mut self, value: u8) {
        self.set_reset_flag(Flag::S, value & 0x80 > 0);
    }

    pub fn update_flag_z(&mut self, value: u8) {
        self.set_reset_flag(Flag::Z, value == 0);
    }

    pub fn update_flag_p(&mut self, value: u8) {
        self.set_reset_flag(Flag::P, (value.count_ones() % 2) == 0);
    }

    pub fn update_flag_c(&mut self, operand1: u8, operand2: u8) {
        self.set_reset_flag(Flag::C, operand1 as u16 + operand2 as u16 > 0xFF);
    }

    pub fn update_flag_a(&mut self, operand1: u8, operand2: u8) {
        self.set_reset_flag(Flag::A, ((operand1 & 0xF) + (operand2 & 0xF)) > 0xF);
    }
}
