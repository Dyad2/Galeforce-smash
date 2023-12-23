use smash::phx::Hash40;
use smash::hash40;
use smash::lib::{lua_const::*, L2CValue};
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod acmd;
mod opff;
mod specials;
mod weapons;
mod status;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

pub fn install() {
    let agent = &mut smashline::Agent::new("luigi");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    status::install(agent);
    weapons::install(agent);
}