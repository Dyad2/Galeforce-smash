use smash::phx::Hash40;
use smash::hash40;
use smash::lib::{lua_const::*, L2CValue};
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash_script::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*, table_const::*};
use custom_var::*;

mod acmd;
mod effects;
mod opff;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("rockman");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();
}