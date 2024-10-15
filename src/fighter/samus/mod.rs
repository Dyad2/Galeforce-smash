use smash::phx::{Hash40, Vector2f, Vector3f};
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::sv_animcmd::*;

use smash_script::*;
use smashline::*;

use galeforce_utils::vars::*;

mod acmd;
mod opff;
mod specials;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("samus");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    weapon::install();
    agent.install();
}