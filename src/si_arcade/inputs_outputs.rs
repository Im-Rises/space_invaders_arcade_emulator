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
    pub shift_register: u16,
    pub shift_offset: u8,
    pub coin: bool,
    pub player1: PlayerInputs,
    pub player2: PlayerInputs,
    pub dip3: bool,
    pub dip5: bool,
    pub dip6: bool,
    pub dip7: bool,
    // pub port3_previous_outputs: u8,
    // pub port5_previous_outputs: u8,
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
            // port3_previous_outputs: 0,
            // port5_previous_outputs: 0,
        }
    }
}
