use super::*;

//mod attack_air;
mod attack_dash;
mod catch;
mod dash;
mod escape;
mod escape_air;
mod guard;
mod jumpsquat;
mod landing;
mod run;
//mod shieldbreakDown;
//mod shieldbreakFly;
mod turn;

pub fn install() {
    attack_dash::install();
    //attackair::install();
    catch::install();
    dash::install();
    escape::install();
    escape_air::install();
    guard::install();
    jumpsquat::install();
    landing::install();
    run::install();
    //shieldbreakDown::install();
    //shieldbreakFly::install();
    turn::install();
}