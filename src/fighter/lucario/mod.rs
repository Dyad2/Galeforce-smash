use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::{sv_animcmd::*, lua_bind::*, sv_module_access::*};
use std::mem;

use smash_script::*;
use smashline::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*, table_const::*};
use custom_var::*;

mod acmd;
mod opff;
mod specials;
mod sounds;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucario");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    sounds::install(agent);
    weapon::install();
    agent.install();
}