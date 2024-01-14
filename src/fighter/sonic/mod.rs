use smash::{
    hash40,
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

use crate::fighter::common::galeforce::*;
use galeforce_utils::{utils::*, vars::*};

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("sonic");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}