use smash::phx::Hash40;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::sv_battle_object;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash_script::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::vars::*; 
use custom_var::*;

mod opff;
mod acmd;
mod specials;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("dolly");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    weapon::install();
    agent.install();
}