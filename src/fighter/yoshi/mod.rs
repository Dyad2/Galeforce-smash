use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;

use smash_script::*;
use smashline::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("yoshi");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}