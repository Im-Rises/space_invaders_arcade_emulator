// Get here for emulator and game inputs

pub struct InputsOutputs {
    shift_register: u16,
    shift_offset: u8,
}

impl InputsOutputs {
    pub fn new() -> Self {
        InputsOutputs {
            shift_register: 0,
            shift_offset: 0,
        }
    }

    pub fn inputs(&self, port: u8, mut data: u8) -> u8 {
        match port {
            0 => (), //INPUTS (Mapped in hardware but never used by the code)
            1 => (), //INPUTS
            2 => (), //INPUTS
            3 => data = ((self.shift_register >> (8 - self.shift_offset)) & 0xFF) as u8,
            6 => (), //WATCHDOG
            _ => {
                println!(
                    "Error: Writing to port not implemented at port {} with data {}",
                    port, data
                );
            }
        }
        data
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
