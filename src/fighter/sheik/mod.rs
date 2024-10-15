use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;

use smash_script::*;
use smashline::*;

use crate::fighter::common::galeforce::sheik_ga_buff;
use galeforce_utils::{vars::*, table_const::*, utils::*};
use custom_var::*;

mod acmd;
mod effects;
mod opff;
mod specials;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("sheik");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    specials::install(agent);
    weapon::install();
    agent.install();
}