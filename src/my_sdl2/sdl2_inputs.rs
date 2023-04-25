use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use crate::si_arcade;
use crate::si_arcade::GameInput;

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

            Event::KeyDown {
                keycode: Some(keycode), ..
            } => match keycode {
                Keycode::C => {
                    si_arcade.inputs_outputs.coin = true;
                }
                Keycode::Left => {
                    si_arcade.inputs_outputs.player.left = true;
                }
                Keycode::Right => {
                    si_arcade.inputs_outputs.player.right = true;
                }
                Keycode::Up => {
                    si_arcade.inputs_outputs.player.shot = true;
                }
                Keycode::Num1 => {
                    si_arcade.inputs_outputs.player1_start = true;
                }
                Keycode::Num2 => {
                    si_arcade.inputs_outputs.player2_start = true;
                }
                _ => {}
            },

            Event::KeyUp {
                keycode: Some(keycode), ..
            } => match keycode {
                Keycode::C => {
                    si_arcade.inputs_outputs.coin = false;
                }
                Keycode::Left => {
                    si_arcade.inputs_outputs.player.left = false;
                }
                Keycode::Right => {
                    si_arcade.inputs_outputs.player.right = false;
                }
                Keycode::Up => {
                    si_arcade.inputs_outputs.player.shot = false;
                }
                Keycode::Num1 => {
                    si_arcade.inputs_outputs.player1_start = false;
                }
                Keycode::Num2 => {
                    si_arcade.inputs_outputs.player2_start = false;
                }
                Keycode::K => {
                    si_arcade.inputs_outputs.dip3 = !si_arcade.inputs_outputs.dip3;
                    if si_arcade.inputs_outputs.dip3 {
                        println!("- 1 additional life");
                    } else {
                        println!("- 1 additional life disabled");
                    }
                }
                Keycode::L => {
                    si_arcade.inputs_outputs.dip5 = !si_arcade.inputs_outputs.dip5;
                    if si_arcade.inputs_outputs.dip5 {
                        println!("- 2 additional lives");
                    } else {
                        println!("- 2 additional lives disabled");
                    }
                }
                Keycode::M => {
                    si_arcade.inputs_outputs.dip6 = !si_arcade.inputs_outputs.dip6;
                    if si_arcade.inputs_outputs.dip6 {
                        println!("- Extra ship at 1500 points");
                    } else {
                        println!("- Extra ship at 1000 points");
                    }
                }
                Keycode::O => {
                    si_arcade.inputs_outputs.dip7 = !si_arcade.inputs_outputs.dip7;
                    if si_arcade.inputs_outputs.dip7 {
                        println!("- Coin info displayed in demo screen");
                    } else {
                        println!("- Coin info not displayed in demo screen");
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(window_active)
}
