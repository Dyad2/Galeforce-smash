use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smash_script::*;

use galeforce_utils::vars::*;
use custom_var::*;

mod acmd;
mod opff;
mod specials;

pub fn install() {
    let agent = &mut smashline::Agent::new("rosetta");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    agent.install();
}