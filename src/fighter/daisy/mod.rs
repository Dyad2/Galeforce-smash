use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use galeforce_utils::vars::*;
use custom_var::*;
use crate::fighter::common::galeforce::*;

mod acmd;
mod specials;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("daisy");
    acmd::install(agent);
    specials::install(agent);
    opff::install(agent);
    agent.install();
}