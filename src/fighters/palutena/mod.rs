use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;
use crate::utils::*;

#[fighter_frame( agent = FIGHTER_KIND_PALUTENA )]
fn ARCHETYPE_EARTH_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);

        //GA - Divine lightstep
        // type: cancel
        //  cancel dash attack with a jump if it counter hits and opponent. additional code in opff
        if !is_operation_cpu(boma) {
            if curr_motion_kind == hash40("attack_dash") {
                if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) == false && FIGHTER_GLOBALS[entry_id as usize].ga_on {
                    if (ControlModule::is_enable_flick_jump(boma) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                        galeforce_apply_effect(boma, 0.75);
                        FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                    }
                }
            }
            else {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
        }
    }
}

//global edits
#[acmd_script( agent = "palutena", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "palutena", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "palutena", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 6.);
        if macros::is_excute(fighter){
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(fighter, Hash40::new("virtual"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 70, 76, 0, 71, 4.5, 0.0, 9.0, 11.0, Some(0.0), Some(11.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 70, 76, 0, 71, 3.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            AttackModule::clear(boma, 1, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 75, 60, 0, 80, 3.5, 0.0, 9.0, 11.0, Some(0.0), Some(12.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("virtual"), *HIT_STATUS_OFF);
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "palutena", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 88, 88, 0, 57, 4.5, 0.0, 21.0, 9.7, Some(0.0), Some(4.5), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 88, 88, 0, 57, 2.8, 0.0, 44.0, 9.7, Some(0.0), Some(4.5), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 88, 88, 0, 57, 2.8, 0.0, 65.0, 9.7, Some(0.0), Some(4.5), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 88, 88, 0, 57, 2.8, 0.0, 22.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 88, 88, 0, 57, 2.8, 0.0, 44.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 88, 88, 0, 57, 2.8, 0.0, 65.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}

//Air
#[acmd_script( agent = "palutena", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(fighter, Hash40::new("virtual"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 95, 0, 30, 6.5, 0.0, 10.2, -14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("virtual"), *HIT_STATUS_OFF);
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 35.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "palutena", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
        frame(lua_state, 4.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 1.4, 367, 100, 0, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 1.4, 367, 100, 0, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 2, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 3, 0, Hash40::new("stick"), 1.4, 100, 100, 80, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            }
        frame(lua_state, 24.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 1.4, 367, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 1.4, 367, 100, 65, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 2, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 3, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                AttackModule::set_add_reaction_frame(boma, 0, -13.0, false);
                AttackModule::set_add_reaction_frame(boma, 1, -13.0, false);
            }
        frame(lua_state, 28.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
        frame(lua_state, 29.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 5.0, 55, 155, 0, 35, 9.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 5.0, 55, 155, 0, 35, 11.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            }
        frame(lua_state, 31.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
        frame(lua_state, 40.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "palutena", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 4.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 9.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 92, 0, 31, 5.5, 0.0, 6.9, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 45, 95, 0, 35, 5.5, 0.0, 6.9, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
            }
        frame(lua_state, 13.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
        frame(lua_state, 24.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

//throws
#[acmd_script( agent = "palutena", script = "game_throwhi", category = ACMD_GAME, low_priority)]
unsafe fn throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 92, 57, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            macros::CHECK_FINISH_CAMERA(fighter, 7, 20);
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
            FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0., y: 8., z: 0.});
            FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "palutena", script = "game_throwb", category = ACMD_GAME, low_priority)]
unsafe fn throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 361, 65, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
            macros::CHECK_FINISH_CAMERA(fighter, 14, 19);
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
            FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
            FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 6., y: 8., z: 0.});
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            macros::REVERSE_LR(fighter);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
}

//other
#[acmd_script( agent = "palutena", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        ARCHETYPE_EARTH_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attackdash,
        attackhi4,
        attackairb,
        attackairn,
        attackairf,
        throwhi,
        throwb,
        escapeairslide
    );
}