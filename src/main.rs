mod binary_lib;
mod si_arcade;

fn main() {
    let space_invaders_arcade = si_arcade::SpaceInvadersArcade::new("romsPaths");
    space_invaders_arcade.start();

    let value: u8 = 0xF;
    let operand: u8 = 0x2;
    let result: u16 = (value & 0x0F) as u16 + (operand & 0x0F) as u16;
    println!("hey testing {}", result > 0xF)
}
