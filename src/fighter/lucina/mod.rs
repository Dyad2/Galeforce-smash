use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::{lua_bind::*, sv_animcmd::*};
use smash_script::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*, table_const::*};
use custom_var::*;

mod acmd;
mod effect;
mod opff;
mod sound;
mod special;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucina");
    acmd::install(agent);
    effect::install(agent);
    opff::install(agent);
    sound::install(agent);
    special::install(agent);
    agent.install();
}