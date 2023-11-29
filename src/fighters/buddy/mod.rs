use smash::phx::Hash40;
//use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::LuaConst; //required for uair
use smash::lua2cpp::L2CAgentBase;
//use smash::lua2cpp::L2CFighterCommon;
use smash::app::lua_bind::*;
use smash::phx::Vector3f;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use galeforce_utils::vars::*;

mod acmd;
mod specials;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("buddy");
    acmd::install(agent);
    specials::install(agent);
    opff::install(agent);
}