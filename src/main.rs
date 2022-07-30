mod binary_lib;
mod si_arcade;

// Check every "cpu.regs.set_reset_flag(Flag::C" flag changes, the one in dcr is correct

fn main() {
    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new();
    space_invaders_arcade.start();
}
