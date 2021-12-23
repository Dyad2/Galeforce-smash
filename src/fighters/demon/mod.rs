use smash::phx::Hash40;
use smash::hash40;
use smash::lib::{lua_const::*, L2CValue};
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

//directional inputs
//static mut DIR3 : [i32; 9] = [0; 9];
//static mut DIR2 : [i32; 9] = [0; 9];
static mut STICK_DIR : [i32; 9] = [0; 9];
static mut COMMAND_FRAME : [i32; 9] = [0; 9];
static mut INPUT_IS_COMMAND : [bool; 9] = [false; 9];
static mut IS_COMMAND_FAILED : [bool; 9] = [false; 9];

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::utils::*;

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
fn EWGF_simulator_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let cat4 = ControlModule::get_command_flag_cat(boma, 3);
        let status_kind = StatusModule::status_kind(boma);

        //println!("stick_dir: {}", get_stick_dir(boma));

        //flash tornado attack cancel
        if curr_motion_kind == hash40("attack_stand_5") && MotionModule::frame(boma) <= 3.0 && (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::is_enable_flick_jump(boma) && ControlModule::get_stick_y(boma) > 0.7 && ControlModule::get_flick_y(boma) < 3) {
            L2CFighterCommon::change_status_jump_mini_attack(fighter, L2CValue::I32(0));
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }

        //abolishing fist anim is stacked over oni front kick
        if curr_motion_kind == hash40("attack_s3_s") && MotionModule::frame(boma) >= 61.0 {
            StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        if curr_motion_kind == 68500721045 && MotionModule::frame(boma) <= 1.0 {
            println!("acmd_name_hash: {}", MotionModule::animcmd_name_hash(boma, Hash40::new_raw(68500721045)));
            if !INPUT_IS_COMMAND[entry_id as usize] {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("abolishing_fist"), 70.0, 1.0, 0.0, false, false);
            }
        }
        if curr_motion_kind == hash40("abolishing_fist") {
            let damage_info = DamageModule::start_damage_info_log(boma) as *mut smash::app::DamageInfo;
            if DamageModule::check_no_reaction(boma, damage_info) == 1 && StopModule::is_stop(boma) {
                FIGHTER_GLOBALS[entry_id as usize].blocked_attack = true;
            }
        }
        else {
            FIGHTER_GLOBALS[entry_id as usize].blocked_attack = false;
        }

        //Tsunami kick input - 632A
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND 
          && [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_WALK_BRAKE, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK].contains(&status_kind) {
            if get_stick_dir(boma) == 6 && FIGHTER_GLOBALS[entry_id as usize].once {
                FIGHTER_GLOBALS[entry_id as usize].once = false;
                STICK_DIR[entry_id as usize] = 6;
                COMMAND_FRAME[entry_id as usize] = 10;
            }
            if STICK_DIR[entry_id as usize] == 6 {
                if get_stick_dir(boma) == 3 {
                    STICK_DIR[entry_id as usize] = 3;
                    COMMAND_FRAME[entry_id as usize] = 10;
                }
                else if get_stick_dir(boma) != 6 {
                    IS_COMMAND_FAILED[entry_id as usize] = true;
                }
            }
            if STICK_DIR[entry_id as usize] == 3 {
                if get_stick_dir(boma) == 2 {
                    STICK_DIR[entry_id as usize] = 2;
                    COMMAND_FRAME[entry_id as usize] = 10;
                }
                else if get_stick_dir(boma) != 3 {
                    IS_COMMAND_FAILED[entry_id as usize] = true;
                }
            }
            if STICK_DIR[entry_id as usize] == 2 {
                if get_stick_dir(boma) != 2 && get_stick_dir(boma) != 5 {
                    IS_COMMAND_FAILED[entry_id as usize] = true;
                }
                else if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    INPUT_IS_COMMAND[entry_id as usize] = true;
                    StatusModule::change_status_force(boma, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_3, false);
                }
            }
            //cleanup
            if COMMAND_FRAME[entry_id as usize] > 0 {
                COMMAND_FRAME[entry_id as usize] -= 1;
            }
            else {
                IS_COMMAND_FAILED[entry_id as usize] = true;
            }
            if IS_COMMAND_FAILED[entry_id as usize] {
                STICK_DIR[entry_id as usize] = 0;
                INPUT_IS_COMMAND[entry_id as usize] = false;
                FIGHTER_GLOBALS[entry_id as usize].once = true;
                IS_COMMAND_FAILED[entry_id as usize] = false;
            }
        }
    }
}

//global edits
#[acmd_script( agent = "demon", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "demon", script = "game_dashb", category = ACMD_GAME, low_priority)]
unsafe fn dashb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "demon", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//ground
#[acmd_script( agent = "demon", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 5, 0, 30, 2.5, 0.0, 14.5, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 5, 0, 30, 3.5, 0.0, 11.0, 8.75, Some(0.0), Some(13.5), Some(8.75), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 5, 0, 30, 3.5, 0.0, 11.0, 3.25, Some(0.0), Some(13.5), Some(3.25), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 33 , 5, 0, 35, 3.5, 0.0, 13.2, 3.25, Some(0.0), Some(13.2), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 8., false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 8., false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 8., false);
            AttackModule::set_add_reaction_frame_revised(boma, 3, 8., false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.3);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.3);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.3);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        }
}

#[acmd_script( agent = "demon", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_push_speed_x_overlap_rate_status(boma, 20.0);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 5, 0, 30, 2.5, 0.0, 14.25, 9.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 5, 0, 30, 3.5, 0.0, 10.25, 8.75, Some(0.0), Some(13.0), Some(8.75), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 5, 0, 30, 3.5, 0.0, 10.25, 3.25, Some(0.0), Some(13.0), Some(8.75), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 33, 5, 0, 30, 3.5, 0.0, 12.9, 3.25, Some(0.0), Some(13.2), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 11.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 11.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 11.0, false);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_push_speed_x_overlap_rate_status(boma, 0.0);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
        }
}

#[acmd_script( agent = "demon", script = "game_attack13", category = ACMD_GAME, low_priority)]
unsafe fn attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 5, 0, 35, 2.5, 0.0, 14.5, 10.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 5, 0, 35, 3.5, 0.0, 13.5, 9.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 5, 0, 35, 3.5, 0.0, 13.5, 3.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 5, 0, 35, 3.5, 0.0, 8.75, 9.0, Some(0.0), Some(13.5), Some(9.0), 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 361, 5, 0, 35, 3.5, 0.0, 9.0, 3.0, Some(0.0), Some(13.5), Some(3.0), 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 2.0, 33, 5, 0, 35, 3.6, 0.0, 15.0, 3.0, Some(0.0), Some(15.0), Some(9.0), 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
        }
}

#[acmd_script( agent = "demon", script = "game_attack14", category = ACMD_GAME, low_priority)]
unsafe fn attack14(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 5, 0, 35, 2.5, 0.0, 14.0, 10.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 5, 0, 35, 3.6, 0.0, 9.5, 9.0, Some(0.0), Some(13.0), Some(9.0), 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 5, 0, 35, 3.6, 0.0, 9.5, 4.0, Some(0.0), Some(13.0), Some(4.0), 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 36, 5, 0, 35, 3.6, 0.0, 14.0, 4.0, Some(0.0), Some(14.0), Some(9.0), 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
        }
}

#[acmd_script( agent = "demon", script = "game_attack15", category = ACMD_GAME, low_priority)]
unsafe fn attack15(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 42, 20, 0, 40, 4.0, 0.0, 13.0, 4.0, Some(0.0), Some(15.0), Some(11.0), 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 3.5, 0.0, 10.0, 5.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 3.5, 0.0, 6.0, 5.0, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 3.5, 0.0, 9.75, 7.5, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 3.5, 0.0, 13.5, 10.75, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 3.0, 0, 5, 0, 40, 2.5, 0.0, 14.5, 10.5, None, None, None, 0.5, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x193bdcb0cc), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 9.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 9.0, false);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
        }
}

//tilts
#[acmd_script( agent = "demon", script = "game_attackstand31", category = ACMD_GAME, low_priority)]
unsafe fn tsunamikick(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_push_speed_x_overlap_rate_status(boma, 0.1);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 90, 120, 61, 0, 3.6, 0.0, 17.0, 11.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 90, 120, 61, 0, 4.2, 0.0, 17.0, 11.0, Some(0.0), Some(4.2), Some(3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 90, 120, 61, 0, 4.0, 0.0, 12.0, 4.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 70, 120, 21, 0, 4.0, 0.0, 12.0, 4.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 7.0, 90, 120, 21, 0, 3.6, 0.0, 17.0, 11.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 7.0, 90, 120, 21, 0, 4.2, 0.0, 17.0, 11.0, Some(0.0), Some(4.2), Some(3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            //AttackModule::set_add_reaction_frame_revised(boma, 0, 4.0, false);
            //AttackModule::set_add_reaction_frame_revised(boma, 1, 4.0, false);
            //AttackModule::set_add_reaction_frame_revised(boma, 2, 4.0, false);
            //AttackModule::set_add_reaction_frame_revised(boma, 3, 8.0, false);
            //AttackModule::set_add_reaction_frame_revised(boma, 4, 8.0, false);
            //AttackModule::set_add_reaction_frame_revised(boma, 5, 8.0, false);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_push_speed_x_overlap_rate_status(boma, 0.0);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_STAND_3_FLAG_CHECK_STEP);
            HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "demon", script = "game_attackstand32", category = ACMD_GAME, low_priority)]
unsafe fn tsunamikick2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 4.2, 0.0, 20.5, 5.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 3.4, 0.0, 17.0, 3.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 3.4, 0.0, 12.0, 1.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 4.2, 0.0, 20.5, 5.0, Some(0.0), Some(21.5), Some(3.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.5);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear(boma, 3, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 4.2, 0.0, 17.0, 10.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 3.4, 0.0, 15.0, 6.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 3.4, 0.0, 13.0, 3.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.5);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 4.2, 0.0, 10.5, 11.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 3.4, 0.0, 10.5, 7.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 50, 67, 0, 78, 3.4, 0.0, 10.5, 4.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 13, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.5);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "demon", script = "game_attackstand5", category = ACMD_GAME, low_priority)]
unsafe fn flashtornado(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 38, 69, 0, 55, 3.0, 0.0, 15.0, 10.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 46, 69, 0, 55, 5.2, 0.0, 10.5, 5.5, Some(0.0), Some(13.72), Some(10.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(boma);
        }
}

//input moves

//Abolishing Fist
#[acmd_script( agent = "demon", script = "game_abolishingfist", category = ACMD_GAME, low_priority)]
unsafe fn abolishingfist(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 71.);
        if macros::is_excute(fighter)
        {
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            MotionModule::set_rate(boma, 0.85);
        }
    frame(lua_state, 78.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            if FIGHTER_GLOBALS[entry_id as usize].blocked_attack {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 361, 10, 0, 50, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(-4.0), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new_raw(0x1985267897), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            }
            else {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 361, 10, 0, 50, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(-4.0), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            }
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.8);
        }
    frame(lua_state, 82.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 84.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.75);
        }
    frame(lua_state, 88.);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                CancelModule::enable_cancel(boma);
            }
        }
}

#[acmd_script( agent = "demon", script = "effect_abolishingfist", category = ACMD_EFFECT, low_priority)]
unsafe fn effectabolishingfist(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 77.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new_raw(0x16bcc4547e), Hash40::new("top"), 1, 14.7, 4, 0, -7, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_ALPHA(fighter,  0.3);
        }
}

#[acmd_script( agent = "demon", script = "expression_abolishingfist", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expressionabolishingfist(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
        if macros::is_excute(fighter)
        {
            smash_script::slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_80_attacks"), 0);
        }
    frame(lua_state, 76.);
        if macros::is_excute(fighter)
        {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_80_nohits"), 0, false, 0);
        }
}

#[acmd_script( agent = "demon", script = "sound_abolishingfist", category = ACMD_SOUND, low_priority)]
unsafe fn soundabolishingfist(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 77.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_demon_swing_short01"));
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_demon_rnd_attack_10_01"));
        }
}

//wgf
#[acmd_script( agent = "demon", script = "game_attackstep2", category = ACMD_GAME, low_priority)]
unsafe fn attackstep2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.7);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.5);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.5, 88, 5, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 88, 5, 0, 90, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 88, 5, 0, 90, 3.0, 0.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handr"), 10.0, 81, 5, 0, 85, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 10.0, 81, 5, 0, 85, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 10.0, 81, 5, 0, 85, 3.0, 0.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 3, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 4, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 5, 14.0, false);
            AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_L, false);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.5, 88, 5, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 88, 5, 0, 90, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 88, 5, 0, 90, 3.0, 0.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handr"), 10.0, 81, 5, 0, 85, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 10.0, 81, 5, 0, 85, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 10.0, 81, 5, 0, 85, 3.0, 0.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 3, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 4, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 5, 14.0, false);
            AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_L, false);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 10.5, 88, 5, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 88, 5, 0, 90, 5.0, 0.0, 19.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 88, 5, 0, 90, 3.0, 0.0, 15.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handr"), 10.0, 81, 5, 0, 85, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 10.0, 81, 5, 0, 85, 5.0, 0.0, 19.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 10.0, 81, 5, 0, 85, 3.0, 0.0, 15.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 3, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 4, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 5, 14.0, false);
            AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_L, false);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            macros::FT_MOTION_RATE(fighter, 0.8);
        }
}

//ewgf
#[acmd_script( agent = "demon", script = "game_attackstep2f", category = ACMD_GAME, low_priority)]
unsafe fn attackstep2f(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.7);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.5);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.5, 88, 5, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.5, 88, 5, 0, 90, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.5, 88, 5, 0, 90, 3.0, 0.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handr"), 12.0, 81, 5, 0, 85, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 81, 5, 0, 85, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 12.0, 81, 5, 0, 85, 3.0, 0.0, 9.0, 3.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            //macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter,0, 1.2, 5.0); //push out of shield?
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 3, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 4, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 5, 14.0, false);
            AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_L, false);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.5, 88, 5, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.5, 88, 5, 0, 90, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.5, 88, 5, 0, 90, 3.0, 0.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handr"), 12.0, 81, 5, 0, 85, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 81, 5, 0, 85, 5.0, 0.0, 18.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 12.0, 81, 5, 0, 85, 3.0, 0.0, 13.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 3, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 4, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 5, 14.0, false);
            AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_L, false);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 12.5, 88, 5, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.5, 88, 5, 0, 90, 5.0, 0.0, 19.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.5, 88, 5, 0, 90, 3.0, 0.0, 15.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handr"), 12.0, 81, 5, 0, 85, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 81, 5, 0, 85, 5.0, 0.0, 19.0, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 12.0, 81, 5, 0, 85, 3.0, 0.0, 15.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
            AttackModule::set_add_reaction_frame_revised(boma, 0, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 1, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 2, 19.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 3, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 4, 14.0, false);
            AttackModule::set_add_reaction_frame_revised(boma, 5, 14.0, false);
            AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_L, false);
            AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_L, false);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            macros::FT_MOTION_RATE(fighter, 0.8);
        }
}

#[acmd_script( agent = "demon", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
            HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
}

pub fn install() {
    smashline::install_agent_frames!(
        EWGF_simulator_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        dashb,
        turndash,
        attack11,
        attack12,
        attack13,
        attack14,
        attack15,
        flashtornado,
        tsunamikick,
        tsunamikick2,
        abolishingfist,
            soundabolishingfist,
            effectabolishingfist,
            expressionabolishingfist,
        attackstep2,
        attackstep2f,
        escapeairslide
    );
}