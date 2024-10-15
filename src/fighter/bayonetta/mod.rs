use smash::{
    hash40,
    phx::{
        Hash40,
        Vector3f
    },
    app::{
        lua_bind::*,
        sv_battle_object,
        sv_animcmd::*,
        sv_animcmd,
    },
    lib::lua_const::*,
    lua2cpp::{
        L2CFighterCommon, 
        L2CAgentBase
    }
};

use smash_script::*;
use smashline::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*, utils::*};
use custom_var::*;

mod acmd;
mod opff;
//mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("bayonetta");
    acmd::install(agent);
    opff::install(agent);
    //status::install(agent);
}