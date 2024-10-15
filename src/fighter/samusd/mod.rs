use smash::phx::{Hash40, Vector2f, Vector3f};
use smash::hash40;
use smash::lib::{lua_const::*, L2CValue};
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;

use smash_script::*;
use smashline::*;

use galeforce_utils::vars::*;

mod acmd;
mod effects;
mod opff;
mod status;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("samusd");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    status::install(agent);
    weapon::install();
    agent.install();
}