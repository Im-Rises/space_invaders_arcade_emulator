use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use crate::si_arcade;

// extraship_btn_last_state: false,

pub fn get_window_active(si_arcade: &mut si_arcade::SpaceInvadersArcade, sdl_context: &Sdl) -> Result<bool, String> {
    let mut event_pump = sdl_context.event_pump()?;
    let mut window_active = true;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => window_active = false,

            //Insert Coin KeyDown
            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => si_arcade.inputs_outputs.coin = true,
            //Insert Coin KeyUp
            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => si_arcade.inputs_outputs.coin = false,

            //Player 1 KeyDown
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => si_arcade.inputs_outputs.player1.left = true,
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => si_arcade.inputs_outputs.player1.right = true,
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => si_arcade.inputs_outputs.player1.shot = true,
            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => si_arcade.inputs_outputs.player1.start = true,
            //Player 1 KeyUp
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => si_arcade.inputs_outputs.player1.left = false,
            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => si_arcade.inputs_outputs.player1.right = false,
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => si_arcade.inputs_outputs.player1.shot = false,
            Event::KeyUp {
                keycode: Some(Keycode::Num1),
                ..
            } => si_arcade.inputs_outputs.player1.start = false,

            //Player 2 KeyDown
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => si_arcade.inputs_outputs.player2.left = true,
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => si_arcade.inputs_outputs.player2.right = true,
            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => si_arcade.inputs_outputs.player2.shot = true,
            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => si_arcade.inputs_outputs.player2.start = true,
            //Player 2 KeyUp
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => si_arcade.inputs_outputs.player2.left = false,
            Event::KeyUp {
                keycode: Some(Keycode::F),
                ..
            } => si_arcade.inputs_outputs.player2.right = false,
            Event::KeyUp {
                keycode: Some(Keycode::E),
                ..
            } => si_arcade.inputs_outputs.player2.shot = false,
            Event::KeyUp {
                keycode: Some(Keycode::Num2),
                ..
            } => si_arcade.inputs_outputs.player2.start = false,

            //DIP 3
            Event::KeyDown {
                keycode: Some(Keycode::K),
                ..
            } => si_arcade.inputs_outputs.dip3 = true,
            Event::KeyUp {
                keycode: Some(Keycode::K),
                ..
            } => si_arcade.inputs_outputs.dip3 = false,

            //DIP 5
            Event::KeyDown {
                keycode: Some(Keycode::L),
                ..
            } => si_arcade.inputs_outputs.dip5 = true,
            Event::KeyUp {
                keycode: Some(Keycode::L),
                ..
            } => si_arcade.inputs_outputs.dip5 = false,

            // //DIP 6
            // Event::KeyDown {
            //     keycode: Some(Keycode::M),
            //     ..
            // } => {
            //     if !si_arcade.extraship_btn_last_state {
            //         si_arcade.inputs_outputs.dip6 = !si_arcade.inputs_outputs.dip6;
            //         si_arcade.extraship_btn_last_state = true;
            //         if si_arcade.inputs_outputs.dip6 {
            //             println!("Extra ship at 1000 points");
            //         } else {
            //             println!("Extra ship at 1500 points");
            //         }
            //     }
            // }
            // Event::KeyUp {
            //     keycode: Some(Keycode::M),
            //     ..
            // } => {
            //     si_arcade.extraship_btn_last_state = false;
            // }

            // //DIP7 Coin info displayed in demo screen 0=ON
            // Event::KeyDown {
            //     keycode: Some(Keycode::O),
            //     ..
            // } => si_arcade.inputs_outputs.borrow_mut().dip7 = true,
            // Event::KeyUp {
            //     keycode: Some(Keycode::O),
            //     ..
            // } => si_arcade.inputs_outputs.borrow_mut().dip7 = false,

            // Default
            _ => window_active = true,
        }
    }
    Ok(window_active)
}
