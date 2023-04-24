pub enum GameInput {
    Coin,
    Player1Start,
    Player2Start,
    Left,
    Right,
    Shot,
    Dip3,
    Dip5,
    Dip6,
    Dip7,
}

pub struct PlayerInputs {
    pub left: bool,
    pub right: bool,
    pub shot: bool,
    // pub start: bool,
}

impl PlayerInputs {
    fn new() -> PlayerInputs {
        PlayerInputs {
            left: false,
            right: false,
            shot: false,
            // start: false,
        }
    }
}

pub struct InputsOutputs {
    pub shift_register: u16,
    pub shift_offset: u8,
    pub coin: bool,
    pub player: PlayerInputs,
    // pub player2: PlayerInputs,
    pub player1_start: bool,
    pub player2_start: bool,
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
            player: PlayerInputs::new(),
            // player2: PlayerInputs::new(),
            player1_start: false,
            player2_start: false,
            dip3: false,
            dip5: false,
            dip6: false,
            dip7: false,
        }
    }

    pub fn update_input(&mut self, input_index: GameInput, value: bool) {
        match input_index {
            GameInput::Coin => self.coin = value,
            GameInput::Player1Start => self.player1_start = value,
            GameInput::Player2Start => self.player2_start = value,
            GameInput::Left => self.player.left = value,
            GameInput::Right => self.player.right = value,
            GameInput::Shot => self.player.shot = value,
            GameInput::Dip3 => self.dip3 = value,
            GameInput::Dip5 => self.dip5 = value,
            GameInput::Dip6 => self.dip6 = value,
            GameInput::Dip7 => self.dip7 = value,
        }
    }
}
