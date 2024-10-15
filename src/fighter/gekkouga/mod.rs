use smash::phx::Hash40;
//use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash::app::{BattleObject, sv_animcmd::*, sv_battle_object, lua_bind::*};
use std::mem;

use smash_script::*;
use smashline::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::vars::*;
use custom_var::*;

mod acmd;
mod opff;
mod specials;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("gekkouga");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    weapon::install();
    agent.install();
}