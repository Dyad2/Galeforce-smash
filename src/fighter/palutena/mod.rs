use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::{sv_animcmd::*, lua_bind::*};

use smash_script::*;
use smashline::*;

use crate::fighter::common::galeforce::*;
use galeforce_utils::{vars::*,table_const::*};
use custom_var::*;

mod acmd;
mod opff;
mod specials;

pub fn install() {
    let agent = &mut smashline::Agent::new("palutena");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    agent.install();
}