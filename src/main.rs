mod binary_lib;
mod my_sdl2;
mod si_arcade;

fn main() {
    println!("+--------------------------------------+");
    println!("|      Space Invaders Arcade Game      |");
    println!("|--------------------------------------|");
    println!("|       C to insert a coin             |");
    println!("|       1 to start 1 player mode       |");
    println!("|       2 to start 2 player mode       |");
    println!("|       → to move left                 |");
    println!("|       ← to move right                |");
    println!("|       ↑ to shoot                     |");
    println!("|       K to get 1 extra life          |");
    println!("|       L to get 2 extra lives         |");
    println!("|       M extra ship at 1000 points    |");
    println!("|                                      |");
    println!("|      Prepare for the invasion!       |");
    println!("+--------------------------------------+");

    println!("\nClick the Space Invaders Window to play.");
    println!("\nLogs:");

    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new();
    space_invaders_arcade.start();
}
