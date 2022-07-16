use std::arch::asm;
use smash::phx::Hash40;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::sv_battle_object;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smashline::*;
use smash_script::*;

use crate::func_hooks::get_battle_object_from_id;
use crate::fighters::common::galeforce::*;
use galeforce_utils::vars::*;
//use galeforce_utils::utils::*;
use custom_var::*;

#[fighter_frame( agent = FIGHTER_KIND_DOLLY)]
fn dolly_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if VarModule::get_float(fighter.battle_object, dolly::instance::float::SPECIAL_N_CHARGE) >= 1.0 {
            VarModule::set_float(fighter.battle_object, dolly::instance::float::SPECIAL_N_CHARGE, 1.0);
        }
        if curr_motion_kind == hash40("special_n") || curr_motion_kind == hash40("special_air_n") {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
                if MotionModule::frame(fighter.module_accessor) >= 6.0 && MotionModule::frame(fighter.module_accessor) < 14.0 
                && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                    MotionModule::set_rate(fighter.module_accessor, 0.2);
                    VarModule::add_float(fighter.battle_object, dolly::instance::float::SPECIAL_N_CHARGE, 0.015);
                }
                else {
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                }
            }
        }
        else {
            VarModule::set_float(fighter.battle_object, dolly::instance::float::SPECIAL_N_CHARGE, 0.0);
        }

        //GA - Terry Go!
        //type: buff
        //grants access to super specials. yep that's it :)
        //more code in opff, opponent-side
        if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 && VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM) {
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM) {
            zelda_buff_effect(fighter);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
            }
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
        }
        //cleanup
        if DamageModule::damage(fighter.module_accessor, 0) < 100.0 {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
        }
    }
}

//global edits
#[acmd_script( agent = "dolly", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "dolly", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//weapon
#[acmd_script( agent = "dolly_wave", script = "game_normalw", category = ACMD_GAME, low_priority)]
unsafe fn normalw(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_object_id = owner_boma.battle_object_id;
    let owner_object = get_battle_object_from_id(owner_object_id);
    //let fighter = get_fighter_common_from_accessor(owner_boma);

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 2.4, 0.0, 12.0, -5.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 1.8, 0.0, 2.0, -10.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    frame(lua_state, 2.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 2.4, 0.0, 9.0, -4.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 1.8, 0.0, 2.0, -8.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    frame(lua_state, 6.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 2.4, 0.0, 6.5, -2.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 1.8, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
}

#[acmd_script( agent = "dolly_wave", script = "game_normal", category = ACMD_GAME, low_priority)]
unsafe fn normal(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_object_id = owner_boma.battle_object_id;
    let owner_object = get_battle_object_from_id(owner_object_id);

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 2.4, 0.0, 12.0, -5.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 9.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 1.8, 0.0, 2.0, -10.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    frame(lua_state, 2.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 2.4, 0.0, 9.0, -4.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 9.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 1.8, 0.0, 2.0, -8.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    frame(lua_state, 6.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 2.4, 0.0, 6.5, -2.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 9.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 55, 20, 0, 50, 1.8, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
}

#[acmd_script( agent = "dolly_wave", script = "game_normalairw", category = ACMD_GAME, low_priority)]
unsafe fn normalairw(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_object_id = owner_boma.battle_object_id;
    let owner_object = get_battle_object_from_id(owner_object_id);

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 10.0, -2.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 9.4, -0.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -4.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 8.8, 0.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -6.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 8.2, 1.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -7.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 7.6, 2.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -9.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -11.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 6.4, 5.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -12.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 5.8, 6.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -14.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 5.2, 7.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -15.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 4.6, 8.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -17.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
}

#[acmd_script( agent = "dolly_wave", script = "game_normalair", category = ACMD_GAME, low_priority)]
unsafe fn normalair(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_object_id = owner_boma.battle_object_id;
    let owner_object = get_battle_object_from_id(owner_object_id);

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 10.0, -2.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 9.2, -0.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -5.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 8.4, 1.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -7.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 7.6, 3.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -10.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 6.5, 5.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -6.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 6.0, 7.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -15.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 5.2, 8.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 11.0 * (1. + VarModule::get_float(owner_object,dolly::instance::float::SPECIAL_N_CHARGE)), 45, 30, 0, 85, 3.0, 0.0, 0.0, -17.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        }
}

//ground
#[acmd_script( agent = "dolly", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.35);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
           macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
           macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 10.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 37, 36, 0, 53, 4.0, 0.0, 10.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 37, 36, 0, 53, 4.0, 0.0, 11.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
           macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
           macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
    wait(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.65);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
}

#[acmd_script( agent = "dolly", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 30, 0, 25, 3.5, 0.0, 5.0, 2.0, None, None, None, 1.4, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 80, 30, 0, 25, 3.5, 0.0, 4.0, 7.0, None, None, None, 1.4, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 86, 30, 0, 25, 3.5, 0.0, 3.0, 12.0, None, None, None, 1.4, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter) 
        {
            MotionModule::set_rate(fighter.module_accessor, 1.2); //was 1.428
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
}

//specials
#[acmd_script( agent = "dolly", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
}

//other
#[acmd_script( agent = "dolly", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        dolly_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        normalw,
        normal,
        normalairw,
        normalair,
        attacks3,
        attacklw3,
        specialairn,
        escapeairslide,
    );
}