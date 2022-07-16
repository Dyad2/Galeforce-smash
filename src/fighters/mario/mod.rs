use std::arch::asm;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;
use smash::phx::Vector3f;

//use crate::custom_vars::*;
use galeforce_utils::vars::*;

//global edits
#[acmd_script( agent = "mario", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "mario", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//ground
#[acmd_script( agent = "mario", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 361, 30, 0, 20, 2.0, 0.0, 6.3, 6.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 361, 30, 0, 20, 2.3, 0.0, 6.3, 8.7, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.2, 180, 20, 0, 15, 2.6, 0.0, 6.3, 11.6, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.2, 361, 20, 0, 15, 2.6, 0.0, 6.3, 11.6, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        }
}

//air
#[acmd_script( agent = "mario", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.3333);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.0, 75, 135, 0, 20, 4.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 75, 135, 0, 20, 5.5, 3.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "mario", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        frame(lua_state, 4.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 5.);
            for _ in 0..4
            {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 94, 15, 0, 50, 4.0, 0.0, -0.5, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.4, 94, 15, 0, 25, 7.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
                }
            wait(lua_state, 1.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(fighter.module_accessor);
                }
            wait(lua_state, 1.);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.75);
            }
        frame(lua_state, 22.);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        frame(lua_state, 23.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 75, 100, 0, 80, 11.0, 0.0, 6.8, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 33.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "mario", script = "game_landingairlw", category = ACMD_GAME, low_priority)]
unsafe fn landingairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 100, 60, 0, 4.3, 0.0, 3.2, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 100, 60, 0, 4.3, 0.0, 3.2, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 100, 60, 0, 4.3, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

//specials
#[acmd_script( agent = "mario", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 0.4);
            }
        frame(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.0);
                macros::SA_SET(fighter, *SITUATION_KIND_AIR);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        wait(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 92, 100, 170, 0, 3.8, 0.0, 6.5, 8.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 60, 100, 110, 0, 3.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 92, 100, 110, 0, 3.8, 0.0, 11.5, 8.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 2, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 3, true, false);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        frame(lua_state, 13.);
            if macros::is_excute(fighter)
            {
                smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            }
        frame(lua_state, 17.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
            }
    }
    else {
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 0.4);
            }
        frame(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.0);
                macros::SA_SET(fighter, *SITUATION_KIND_AIR);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        wait(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 92, 100, 170, 0, 3.8, 0.0, 6.5, 8.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 60, 100, 110, 0, 3.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 92, 100, 110, 0, 3.8, 0.0, 11.5, 8.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 2, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 3, true, false);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        frame(lua_state, 13.);
            if macros::is_excute(fighter)
            {
                smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            }
        frame(lua_state, 17.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
    }
}

#[acmd_script( agent = "mario", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 0.4);
            }
        frame(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.0);
                macros::SA_SET(fighter, *SITUATION_KIND_AIR);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        wait(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 8.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 11.5, 8.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 2, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 3, true, false);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        frame(lua_state, 13.);
            if macros::is_excute(fighter)
            {
                smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            }
        frame(lua_state, 17.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
            }
    }
    else
     {
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 0.4);
            }
        frame(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.0);
                macros::SA_SET(fighter, *SITUATION_KIND_AIR);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        wait(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 8.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 11.5, 8.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 2, true, false);
                AttackModule::set_no_finish_camera(fighter.module_accessor, 3, true, false);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        frame(lua_state, 13.);
            if macros::is_excute(fighter)
            {
                smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            }
        frame(lua_state, 17.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 145, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
    }
}

//throws
#[acmd_script( agent = "mario", script = "game_throwb", category = ACMD_GAME, low_priority)]
unsafe fn throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

            if macros::is_excute(fighter)
            {
               macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 45, 60, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
               macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        frame(lua_state, 13.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("hip"), 8.0, 50, 70, 0, 100, 4.0, -1.0, 6.0, 0.0, Some(-3.2), Some(13.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
            }
        frame(lua_state, 40.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::REVERSE_LR(fighter);
            }
        frame(lua_state, 43.);
            if macros::is_excute(fighter)
            {
                macros::CHECK_FINISH_CAMERA(fighter,14, 7);
                FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
                FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 8.0, y: 0.0, z: 0.0});
            }
        frame(lua_state, 44.);
            if macros::is_excute(fighter)
            {
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            }
}

//other
#[acmd_script( agent = "mario", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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


//will hit bystanders, should probably be a code edit instead of acmd?
#[acmd_script( agent = "mario", script = "game_stepjump", category = ACMD_GAME, low_priority)]
unsafe fn stepjump(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 270, 0, 0, 0, 5.0, 0.0, -5.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {        
            AttackModule::clear_all(fighter.module_accessor);
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attack11,
        attackairhi,
        attackairlw,
        landingairlw,
        specialhi,
        specialairhi,
        stepjump,
        throwb,
        escapeairslide
    );
}