use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*};
use custom_var::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("wolf");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}