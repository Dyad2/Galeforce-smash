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
mod shieldbreakdown;
mod shieldbreakfly;
mod turn;

pub fn install() {
    //attackair::install();
    attack_dash::install();
    catch::install();
    dash::install();
    escape_air::install();
    escape::install();
    guard::install();
    jumpsquat::install();
    landing::install();
    run::install();
    shieldbreakdown::install();
    shieldbreakfly::install();
    turn::install();
}