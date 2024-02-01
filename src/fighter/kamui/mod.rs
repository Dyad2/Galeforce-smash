use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smash_script::*;

use galeforce_utils::vars::*;
use custom_var::*;

mod acmd;
mod opff;
mod specials;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("kamui");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    weapon::install();
    agent.install();
}