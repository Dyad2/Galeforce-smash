use smash::phx::{Hash40, Vector3f};
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod acmd;
mod effects;
mod opff;
mod specials;

pub fn install() {
    let agent = &mut smashline::Agent::new("miiswordsman");
    acmd::install(agent);
    opff::install(agent);
    effects::install(agent);
    specials::install(agent);
}