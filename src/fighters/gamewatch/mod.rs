//use smash::phx::Hash40;
use smash::lib::{
    lua_const::*,
    L2CValue,
};
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*, table_const::*};
use custom_var::*;

mod acmd;
mod opff;
//mod status;


pub fn install() {
    let agent = &mut smashline::Agent::new("gamewatch");
    acmd::install(agent);
    opff::install(agent);
}