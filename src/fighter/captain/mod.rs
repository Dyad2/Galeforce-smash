use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use std::mem;

use smash_script::*;
use smashline::*;

use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

mod effects;
mod acmd;
mod specials;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("captain");
    acmd::install(agent);
    effects::install(agent);
    specials::install(agent);
    opff::install(agent);
    agent.install();
}
