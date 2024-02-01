use {
    smash::{
        phx::Hash40,
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lib::{
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterCommon, 
            L2CAgentBase
        }
    },
    galeforce_utils::{
        vars::*, 
        table_const::*, 
    },
    std::mem,
    smash_script::*,
    custom_var::*,
};

mod acmd;
mod opff;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("pickel");
    acmd::install(agent);
    opff::install(agent);
    weapon::install();
    agent.install();
}