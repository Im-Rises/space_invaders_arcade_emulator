mod binary_lib;
mod si_arcade;

fn main() {
    let space_invaders_arcade = si_arcade::SpaceInvadersArcade::new("romsPaths");
    space_invaders_arcade.start();
}
