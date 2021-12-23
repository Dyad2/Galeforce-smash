use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

//specials
//sing active frame: 27-36 / 61-70 / 100-115 FAF: 150
//sing mouth open: 23-45 / 57-83 / 97-125
//current active frames: 28-45 / 60-81 / 96-117 FAF: 149

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;

#[fighter_frame( agent = FIGHTER_KIND_PURIN )]
fn diva_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let status_kind = StatusModule::status_kind(boma);

        //GA - Sitrus and Chesto berries
        // type: cancel
        //  allows puff to wake early after landing rest, and heals 10%
        //   cpu is allowed to use it
        if FIGHTER_GLOBALS[entry_id as usize].ga_on {
            FIGHTER_GLOBALS[entry_id as usize].once = true;
            galeforce_apply_effect(boma, 0.75);
            FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            DamageModule::heal(boma, -10.0, 0);
        }
        if FIGHTER_GLOBALS[entry_id as usize].puff_rest_hit && !StopModule::is_stop(boma) {
            FIGHTER_GLOBALS[entry_id as usize].puff_rest_time += 1;
        }
        if FIGHTER_GLOBALS[entry_id as usize].puff_rest_time >= 61 {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                MotionModule::set_frame(boma, 170.0, false);
            }
            FIGHTER_GLOBALS[entry_id as usize].puff_rest_hit = false;
            FIGHTER_GLOBALS[entry_id as usize].puff_rest_time = 0;
        }
    }
}

//global edits
#[acmd_script( agent = "purin", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "purin", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

#[acmd_script( agent = "purin", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

     frame(lua_state,1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.85);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.7);
        }
    frame(lua_state, 57.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.8);
        }
    frame(lua_state, 97.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.75);
        }
}

#[acmd_script( agent = "purin", script = "game_specialhir", category = ACMD_GAME, low_priority)]
unsafe fn specialhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state,1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.85);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.7);
        }
    frame(lua_state, 57.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.8);
        }
    frame(lua_state, 97.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma,0.0);
        }
    wait(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.75);
        }
}

#[acmd_script( agent = "purin", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn specialairhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state,1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.85);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state,24.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.7);
        }
    frame(lua_state, 57.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state,25.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.8);
        }
    frame(lua_state, 97.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.75);
        }
}

#[acmd_script( agent = "purin", script = "game_specialairhir", category = ACMD_GAME, low_priority)]
unsafe fn specialairhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.85);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.7);
        }
    frame(lua_state, 57.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.8);
        }
    frame(lua_state, 97.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.45);
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.1, 361, 40, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.1, 361, 30, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.1, 361, 25, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_shield_stiff_mul(boma, 0.0);
        }
    wait(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.75);
        }
}

#[acmd_script( agent = "purin", script = "game_specialn", category = ACMD_GAME, low_priority)]
unsafe fn specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6);
            JostleModule::set_status(boma, false);
            AttackModule::set_attack_keep_rumble(boma, 0, true);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 30, 60, 0, 60, 2.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
}

#[acmd_script( agent = "purin", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6);
            JostleModule::set_status(boma, false);
            AttackModule::set_attack_keep_rumble(boma, 0, true);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 30, 60, 0, 60, 2.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
}

//grabs
#[acmd_script( agent = "purin", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 8.);
            if macros::is_excute(fighter)
            {
                GrabModule::set_rebound(boma, true);
            }
        frame(lua_state, 9.);
            if macros::is_excute(fighter)
            {
                macros::CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
                macros::CATCH(fighter, 1, Hash40::new("top"), 1.75, 0.0, 5.2, 2.25, Some(0.0), Some(5.2), Some(11.95), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }
        macros::game_CaptureCutCommon(fighter);
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
                WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
                GrabModule::set_rebound(boma, false);
            }
}

#[acmd_script( agent = "purin", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            GrabModule::set_rebound(boma, true);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 5.2, -4.0, Some(0.0), Some(5.2), Some(-17.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.75, 0.0, 5.2, -2.25, Some(0.0), Some(5.2), Some(-19.25), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(boma, false);
        }
}

//other
#[acmd_script( agent = "purin", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        diva_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        specialhil,
        specialhir,
        specialairhil,
        specialairhir,
        specialn,
        specialairn,
        catchdash,
        catchturn,
        escapeairslide,
    );
}