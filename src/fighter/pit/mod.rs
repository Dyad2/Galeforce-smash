use smash::phx::Hash40;
use smash::lib::{lua_const::*, L2CValue};
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;

use smash_script::*;
use smashline::*;

mod acmd;
mod opff;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("pit");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();
}

