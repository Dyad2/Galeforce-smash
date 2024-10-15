use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::{lua_bind::*, sv_animcmd::*};

use smash_script::*;
use smashline::*;

use galeforce_utils::{vars::*, table_const::*, utils::*};
use crate::fighter::common::galeforce::*;
use custom_var::*;

pub mod acmd;
pub mod effects;
pub mod opff;
pub mod specials;

pub fn install() {
    let agent = &mut smashline::Agent::new("marth");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    specials::install(agent);
    agent.install();
}