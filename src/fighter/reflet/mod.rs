use smash::hash40;
use smash::phx::{Hash40, Vector2f};
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::{BattleObjectModuleAccessor, FighterSpecializer_Reflet, lua_bind::*, sv_animcmd::*, sv_system};
use smash_script::*;
use std::mem;

use crate::fighter::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*};
use custom_var::*;

static mut THUNDER_FX_HANDLE : [u32; 9] = [1; 9];

unsafe fn get_hitsound(boma: &mut BattleObjectModuleAccessor) -> i32 {
    let ATTACK_SOUND_TYPE: i32;

    if WorkModule::is_flag(boma, commons::instance::flag::GALEFORCE_ATTACK_ON) {
        ATTACK_SOUND_TYPE = *COLLISION_SOUND_ATTR_MAGIC;
    }
    else if !WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        ATTACK_SOUND_TYPE = *COLLISION_SOUND_ATTR_CUTUP;
    }
    else {
        ATTACK_SOUND_TYPE = *COLLISION_SOUND_ATTR_ELEC;
    }
    return ATTACK_SOUND_TYPE;
}

mod acmd;
mod agent_init;
mod opff;
mod specials;

pub fn install() {
    let agent = &mut smashline::Agent::new("reflet");
    acmd::install(agent);
    agent_init::install(agent);
    opff::install(agent);
    specials::install(agent);
    agent.install();
}