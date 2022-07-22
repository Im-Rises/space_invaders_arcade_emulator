//Equivalent of https://github.com/Im-Rises/GameBoyEmulator/blob/main/GameBoyEmulator/binaryLib

pub fn get_bit(data: u8, bit: usize) -> bool {
    ((data >> bit) & 0x1) != 0//as bool cannot cast ???
}

pub fn set_bit(data: u8, bit: usize) -> u8 {
    data | (1 << bit)
}

pub fn reset_bit(data: u8, bit: usize) -> u8 {
    data & !(1 << bit)
}
