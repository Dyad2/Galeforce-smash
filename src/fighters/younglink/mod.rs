use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::{
    lua_bind::*,
    sv_animcmd::*,
};
use smash::lua2cpp::{
    L2CFighterCommon,
    L2CAgentBase
};
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

mod acmd;
mod effects;
mod opff;
mod specials;

pub fn install() {
    let agent = &mut smashline::Agent::new("younglink");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    specials::install(agent);
}