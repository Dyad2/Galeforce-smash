use smash::phx::Hash40;
//use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash::app::{BattleObject, sv_animcmd::*, sv_battle_object, lua_bind::*};
use smashline::*;
use smash_script::*;
use std::mem;

use crate::fighters::common::galeforce::*;
use galeforce_utils::vars::*;
use galeforce_utils::utils::get_battle_object_from_id;
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
}