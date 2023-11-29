use smash::{
    hash40,
    phx::{
        Hash40, 
        Vector2f,
        Vector3f
    },
    app::{
        lua_bind::*,
        sv_battle_object,
        sv_animcmd::*,
        sv_animcmd,
    },
    lib::{
        lua_const::*,
    },
    lua2cpp::{
        L2CFighterCommon, 
        L2CAgentBase
    }
};

use smash_script::*;
use smashline::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*, utils::*};
use custom_var::*;

mod acmd;
mod bullet_arts;
mod effect;
mod opff;
mod sound;
mod special;
//mod status;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("bayonetta");
    acmd::install(agent);
    bullet_arts::install(agent);
    effect::install(agent);
    opff::install(agent);
    sound::install(agent);
    special::install(agent);
    //status::install();
    weapon::install(agent);
}