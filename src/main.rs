extern crate core;

mod binary_lib;
mod si_arcade;

// Add audio
// Correct CPU error, changing wrong flag... Swap between C and A linke in INR

fn main() {
    println!("+----------------------------------+");
    println!("|    Space Invaders Arcade Game    |");
    println!("|----------------------------------|");
    println!("|     Press C to insert a coin     |");
    println!("|     Press 1 to start player 1    |");
    println!("|     Press 2 to start player 2    |");
    println!("|     Press → to move P1 left      |");
    println!("|     Press ← to move P1 right     |");
    println!("|     Press ↑ to shoot with P1     |");
    println!("|     Press S to move P2 left      |");
    println!("|     Press F to move P2 right     |");
    println!("|     Press E to shoot with P2     |");
    println!("|                                  |");
    println!("|    Prepare for the invasion!     |");
    println!("+----------------------------------+");

    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new();
    space_invaders_arcade.start();
}
