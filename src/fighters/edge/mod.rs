use smash::{
    phx::Hash40,
    lib::lua_const::*,
    app::{
        lua_bind::*,
        utility::get_kind,
        sv_animcmd::*,
    },
    lua2cpp::{
        L2CFighterCommon,
        L2CAgentBase
    }
};
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*, utils::get_battle_object_from_id};
use custom_var::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("edge");
    acmd::install(agent);
    opff::install(agent);
}