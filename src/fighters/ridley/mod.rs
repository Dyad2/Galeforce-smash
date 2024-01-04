use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use galeforce_utils::{vars::*};
use custom_var::*;

mod acmd;
mod opff;
mod sounds;

pub fn install() {
    let agent = &mut smashline::Agent::new("ridley");
    acmd::install(agent);
    opff::install(agent);
    sounds::install(agent);
}