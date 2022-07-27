mod binary_lib;
pub mod si_arcade;

/*
To do list:
- Change file read
- Add memory mirror
- check all shifting
 */

fn main() {
    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new("romsPaths");
    // space_invaders_arcade.start();
}
