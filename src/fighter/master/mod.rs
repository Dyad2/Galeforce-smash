use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::vars::*;
use custom_var::*;

mod acmd;
mod effects;
mod opff;
mod specials;

pub fn install() {
    let agent = &mut smashline::Agent::new("master");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    specials::install(agent);
    agent.install();
}