// mod si_arcade;


fn main() {
    // let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new("romsPaths");
    // space_invaders_arcade.start();
    let mut val1 = 5;
    let val2 = 10;

    attribute_val(&mut val1, val2);

    println!("From main {} {}", val1, val2)
}

fn attribute_val(val1: &mut u32, val2: u32) {
    *val1 = 50;
    println!("From func {} {}", val1, val2);
}
