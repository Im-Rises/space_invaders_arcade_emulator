extern crate core;

mod binary_lib;
mod si_arcade;

// Add audio
// Correct CPU
// Add interupt delay ???

fn main() {
    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new();
    space_invaders_arcade.start();
}
