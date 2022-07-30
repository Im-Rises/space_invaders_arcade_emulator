use crate::binary_lib::{set_bit, set_reset_bit};

pub struct PlayerInputs {
    pub left: bool,
    pub right: bool,
    pub shot: bool,
    pub start: bool,
}

impl PlayerInputs {
    fn new() -> PlayerInputs {
        PlayerInputs {
            left: false,
            right: false,
            shot: false,
            start: false,
        }
    }
}

pub struct InputsOutputs {
    shift_register: u16,
    shift_offset: u8,
    pub coin: bool,
    pub player1: PlayerInputs,
    pub player2: PlayerInputs,
    pub dip3: bool,
    pub dip5: bool,
    pub dip6: bool,
    pub dip7: bool,
}

impl InputsOutputs {
    pub fn new() -> Self {
        InputsOutputs {
            shift_register: 0,
            shift_offset: 0,
            coin: false,
            player1: PlayerInputs::new(),
            player2: PlayerInputs::new(),
            dip3: false,
            dip5: false,
            dip6: false,
            dip7: false,
        }
    }

    pub fn inputs(&mut self, port: u8, mut data: u8) -> u8 {
        match port {
            0 => {
                data = 0b0000_1110;
            } //INPUTS (Mapped in hardware but never used by the code)
            1 => {
                data = 0b0000_1000;
                data = set_reset_bit(data, 0, self.coin);
                data = set_reset_bit(data, 1, self.player2.start);
                data = set_reset_bit(data, 2, self.player1.start);
                data = set_reset_bit(data, 4, self.player1.shot);
                data = set_reset_bit(data, 5, self.player1.left);
                data = set_reset_bit(data, 6, self.player1.right);
            } //INPUTS
            2 => {
                data = 0b0000_0000;
                data = set_reset_bit(data, 0, self.dip3);
                data = set_reset_bit(data, 1, self.dip5);
                data = set_reset_bit(data, 3, self.dip6);
                data = set_reset_bit(data, 4, self.player2.shot);
                data = set_reset_bit(data, 5, self.player2.left);
                data = set_reset_bit(data, 6, self.player2.right);
                data = set_reset_bit(data, 7, self.dip7);
            } //INPUTS
            3 => data = ((self.shift_register >> (8 - self.shift_offset)) & 0xFF) as u8,
            6 => (), //WATCHDOG
            _ => {
                println!(
                    "Error: Writing to port not implemented at port {} with data {}",
                    port, data
                );
            }
        }
        // self.coin = false;
        // self.player1 = PlayerInputs::new();
        // self.player2 = PlayerInputs::new();

        data
    }

    pub fn set_coin(&mut self) {
        self.coin = true;
    }

    pub fn outputs(&mut self, port: u8, data: u8) {
        match port {
            2 => self.shift_offset = data & 0b0000_0111,
            3 => (), //Sound bit
            4 => self.shift_register = self.shift_register >> 8 | (data as u16) << 8,
            5 => (), //Sound bit
            6 => (), //Watch dog
            _ => {
                println!(
                    "Error: Reading from port not implemented at port {} with data {}",
                    port, data
                );
            }
        }
    }
}
