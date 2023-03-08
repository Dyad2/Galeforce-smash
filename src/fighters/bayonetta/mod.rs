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
    lib::{
        lua_const::*,
    },
    lua2cpp::{
        L2CFighterCommon, 
        L2CAgentBase
    }
};

use smashline::*;
use smash_script::*;

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
    acmd::install();
    bullet_arts::install();
    effect::install();
    opff::install();
    sound::install();
    special::install();
    //status::install();
    weapon::install();
}