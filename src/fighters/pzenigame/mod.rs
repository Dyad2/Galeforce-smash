use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;
use std::mem;

use galeforce_utils::{
        vars::*,
        table_const::*
};
use custom_var::*;

mod acmd;
mod opff;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("pzenigame");
    acmd::install(agent);
    opff::install(agent);
    weapon::install(agent);
}