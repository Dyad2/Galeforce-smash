use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smash_script::*;

mod acmd;
mod opff;
mod specials;
mod weapons;

pub fn install() {
    let agent = &mut smashline::Agent::new("miigunner");
    acmd::install(agent);
    opff::install(agent);
    weapons::install(agent);
    specials::install(agent);
    agent.install();
}