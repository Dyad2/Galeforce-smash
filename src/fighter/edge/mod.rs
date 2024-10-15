use smash::{
    phx::Hash40,
    lib::lua_const::*,
    app::{
        lua_bind::*,
        sv_animcmd::*,
    },
    lua2cpp::{
        L2CFighterCommon,
        L2CAgentBase
    }
};
use smash_script::*;
use smashline::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("edge");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}