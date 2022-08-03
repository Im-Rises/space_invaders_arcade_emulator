use crate::binary_lib::{get_bit, reset_bit, set_bit};

pub enum Flag {
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
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn set_af(&mut self, data: u16) {
        self.a = (data >> 8) as u8;
        self.f = (data & 0x00ff) as u8;
        self.f = reset_bit(self.f, 5);
        self.f = reset_bit(self.f, 3);
        self.f = set_bit(self.f, 1);
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, data: u16) {
        self.b = (data >> 8) as u8;
        self.c = (data & 0x00ff) as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, data: u16) {
        self.d = (data >> 8) as u8;
        self.e = (data & 0x00ff) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, data: u16) {
        self.h = (data >> 8) as u8;
        self.l = (data & 0x00ff) as u8;
    }

    // pub fn pair_regs(hi: u8, lo: u8) -> u16 {
    //     ((hi as u16) << 8 | lo as u16) as u16
    // }

    // pub fn unpair_regs(pr: u16) -> (u8, u8) {
    //     (((pr & 0xFF00) >> 8) as u8, (pr & 0x00FF) as u8)
    // }
}

impl Register {
    pub fn get_flag(&self, f: Flag) -> bool {
        get_bit(self.f, f as usize)
    }

    pub fn set_reset_flag(&mut self, flag: Flag, bit: bool) {
        if bit {
            self.f = set_bit(self.f, flag as usize)
        } else {
            self.f = reset_bit(self.f, flag as usize)
        }
    }

    pub fn update_flags_szp(&mut self, value: u8) {
        self.update_flag_s(value);
        self.update_flag_z(value);
        self.update_flag_p(value);
    }

    pub fn update_flag_s(&mut self, value: u8) {
        self.set_reset_flag(Flag::S, get_bit(value, 7));
    }

    pub fn update_flag_z(&mut self, value: u8) {
        self.set_reset_flag(Flag::Z, value == 0);
    }

    pub fn update_flag_p(&mut self, value: u8) {
        self.set_reset_flag(Flag::P, (value.count_ones() & 1) == 0);
    }

    pub fn carry(&self, operand1: u8, operand2: u8, cy: bool, bit_index: usize) -> bool {
        let result: u16 = operand1 as u16 + operand2 as u16 + cy as u16;
        let carry: u16 = result ^ (operand1 as u16) ^ (operand2 as u16);
        ((carry >> bit_index) & 0x1) != 0
    }
}
