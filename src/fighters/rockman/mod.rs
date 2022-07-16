use std::arch::asm;
use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;


use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*};
use custom_var::*;

#[fighter_frame( agent = FIGHTER_KIND_ROCKMAN )]
fn RockNRoll_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        //GA - Sliding jump
            //type: cancel
            //hit an opponent with down tilt and press jump!
        if !is_operation_cpu(fighter.module_accessor) {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw3") {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !StopModule::is_stop(fighter.module_accessor) {
                    VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
                if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && VarModule::get_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER) < 15 {
                    VarModule::add_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 1);
                    if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                }
            }
            else {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::set_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 0);
            }
        }
    }
}

//global edits
#[acmd_script( agent = "rockman", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "rockman", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}
    
//others
#[acmd_script( agent = "rockman", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

pub fn install() {
    smashline::install_agent_frames!(
        RockNRoll_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        escapeairslide
    );
}