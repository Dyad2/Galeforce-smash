use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::{lua_bind::*, sv_animcmd::*};
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*};
use custom_var::*;

mod effect;
mod game;
mod sound;
mod opff;

//expressions
#[acmd_script( agent = "lucina", script = "expression_landingfallaether", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expressionspecialhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
            smash_script::slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
}

pub fn install() {
    effect::install();
    game::install();
    sound::install();
    opff::install();
    smashline::install_acmd_scripts!(
        expressionspecialhi4,
    );
}