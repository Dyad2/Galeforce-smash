use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;
use smash::phx::Vector3f;

//use crate::custom_vars::*;
use galeforce_utils::vars::*;

mod acmd;
mod effects;
mod opff;
mod specials;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    specials::install(agent);
}