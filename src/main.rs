mod binary_lib;
pub mod si_arcade;

/*
To do list:
- Review DAA (prevent from reaching pc = 0x05BE) resume at 0x05BD
- Change file read
- Add memory mirror
 */

fn main() {
    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new("romsPaths");
    space_invaders_arcade.start();
}
