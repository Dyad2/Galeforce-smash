use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use galeforce_utils::{table_const::*};

mod acmd;
mod effects;
mod opff;
mod specials;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("toonlink");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    specials::install(agent);
    weapon::install(agent);
    agent.install();
}
