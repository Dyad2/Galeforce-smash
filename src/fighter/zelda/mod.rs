use smash::{
    hash40,
    phx::{
        Hash40, 
        Vector2f, 
        Vector3f, 
    },
    app::{
        utility::get_kind,
        lua_bind::*,
        sv_animcmd::*,
        BattleObject,
        sv_battle_object,
    },
    lib::{
        lua_const::*,
        L2CValue
    },
    lua2cpp::{
        L2CFighterCommon, 
        L2CFighterBase, 
        L2CAgentBase
    }
};

use std::mem;
use smash_script::*;
use smashline::*;

use galeforce_utils::{vars::*, table_const::*, utils::*};
use custom_var::*;
use crate::fighter::common::galeforce::*;

mod acmd;
mod opff;
mod specials;
mod status;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("zelda");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    status::install(agent);
    weapon::install();
    agent.install();
}