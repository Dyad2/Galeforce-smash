use smash::{
    hash40,
    phx::{
        Hash40, 
        Vector3f, 
    },
    app::{
        lua_bind::*,
        sv_animcmd::*,
    },
    lib::{
        lua_const::*,
        L2CValue
    },
    lua2cpp::{
        L2CFighterCommon, 
        L2CAgentBase
    }
};

use smashline::*;
use smash_script::*;

use galeforce_utils::{vars::*, table_const::*, utils::*};
use custom_var::*;
use crate::fighters::common::galeforce::*;

mod acmd;
mod opff;
mod specials;
mod status;
mod weapon;

pub fn install() {
    let agent = &mut smashline::Agent::new("zelda");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    status::install(agent);
    weapon::install(agent);
}