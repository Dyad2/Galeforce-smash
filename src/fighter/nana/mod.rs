use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("nana");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}