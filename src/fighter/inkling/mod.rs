use smash::phx::{Hash40, Vector3f};
//use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::{
    sv_animcmd::*,
    sv_animcmd,
};
use smashline::*;
use smash_script::*;

//use crate::custom_vars::*;
use galeforce_utils::vars::*;

mod acmd;
mod opff;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("inkling");
    acmd::install(agent);
    opff::install(agent);
    weapon::install(agent);
    agent.install();
}