use super::*;

mod catch;
mod dash;
mod run;
mod escape_air;

pub fn install() {
    dash::install();
    run::install();
    catch::install();
    escape_air::install();
}