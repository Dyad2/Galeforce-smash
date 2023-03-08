use smash::{
    hash40,
    phx::{
        Hash40,
    },
    lua2cpp:: {
        L2CFighterCommon,
        L2CAgentBase
    },
    app::{
        lua_bind::*,
        sv_animcmd::*
    },
    lib::lua_const::*
};
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{utils::*};

mod opff;
mod acmd;

pub fn install() {
    opff::install();
    acmd::install();
}