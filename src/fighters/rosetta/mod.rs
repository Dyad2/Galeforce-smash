use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;
use std::mem;

use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::fighters::common::FIGHTER_GLOBALS;

#[fighter_frame( agent = FIGHTER_KIND_ROSETTA )]
fn rosetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);

        if (curr_motion_kind == hash40("special_n") && MotionModule::frame(boma) >= 25.) || curr_motion_kind == hash40("special_n_charge") {
            if (ControlModule::is_enable_flick_jump(boma) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                if situation_kind == SITUATION_KIND_GROUND {
                    FIGHTER_GLOBALS[entry_id as usize].rosa_tico_recall = true;
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
                else if situation_kind == SITUATION_KIND_AIR {
                    FIGHTER_GLOBALS[entry_id as usize].rosa_tico_recall = true;
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                }
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                FIGHTER_GLOBALS[entry_id as usize].rosa_tico_recall = true;
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
        }
    }
}

//global edits
#[acmd_script( agent = "rosetta", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "rosetta", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

//specials
#[acmd_script( agent = "rosetta", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
}

//grabs / throws
#[acmd_script( agent = "rosetta", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.85);
            GrabModule::set_rebound(boma, true);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.9, 0.0, 8.0, 2.1, Some(0.0), Some(8.0), Some(12.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            macros::game_CaptureCutCommon(fighter);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(boma, false);
        }
}

#[acmd_script( agent = "rosetta", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.9);
            GrabModule::set_rebound(boma, true);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.5, 0.0, 8.0, 2.5, Some(0.0), Some(8.0), Some(13.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            macros::game_CaptureCutCommon(fighter);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(boma, false);
        }
}

#[acmd_script( agent = "rosetta", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.8);
            GrabModule::set_rebound(boma, true);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-17.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.9, 0.0, 8.0, -2.1, Some(0.0), Some(8.0), Some(-19.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            macros::game_CaptureCutCommon(fighter);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(boma, false);
        }
}

#[acmd_script( agent = "rosetta", script = "game_throwhi", category = ACMD_GAME, low_priority)]
unsafe fn throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
           macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 88, 90, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
           macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::CHECK_FINISH_CAMERA(fighter,23, 14);
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
            FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
            FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 5., y: 3., z: 0.});
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 1.1);
        }
}

#[acmd_script( agent = "rosetta", script = "game_throwf", category = ACMD_GAME, low_priority)]
unsafe fn throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
           macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 85, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
           macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
           MotionModule::set_rate(boma, 1.2);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            macros::CHECK_FINISH_CAMERA(fighter,23, 14);
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
            FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
            FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 10., y: 4., z: 0.});
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "rosetta", script = "game_throwb", category = ACMD_GAME, low_priority)]
unsafe fn throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
           macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 40, 60, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
           macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            macros::CHECK_FINISH_CAMERA(fighter,-11, 10);
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
            FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
            FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: -3., y: 3., z: 0.});
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            AttackModule::clear_all(boma);
        }
}

//other
#[acmd_script( agent = "rosetta", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

pub fn install() {
    smashline::install_agent_frames!(
        rosetta_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        specialhi,
        catch,
        catchdash,
        catchturn,
        throwhi,
        throwf,
        throwb,
        escapeairslide,
    );
}