use {
    smash::{
        lua2cpp::{
            L2CFighterCommon,
            L2CAgentBase
        },
        app:: {
            lua_bind::*,
            sv_animcmd::*
        },
        lib::lua_const::*,
        hash40,
    },
    smash_script::*,
    smashline::*,
    std::mem
};

use galeforce_utils::table_const::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("wiifit");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}