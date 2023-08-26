use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use galeforce_utils::vars::*;
use custom_var::*;

#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
fn ohbanana_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //dash attack to neutral air when diddy falls offstage
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S &&
         StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND &&
         VarModule::is_flag(fighter.battle_object, donkey::instance::flag::GET_BARREL) {
            VarModule::off_flag(fighter.battle_object, donkey::instance::flag::GET_BARREL);
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), false.into());
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
        }
        // else {

        // }
    }
}
//global edits
#[acmd_script( agent = "donkey", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "donkey", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
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

#[acmd_script( agent = "donkey", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.25);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 100, 75, 0, 70, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 100, 85, 0, 70, 4.0, 3.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 10.0, 100, 85, 0, 70, 5.3, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            AttackModule::clear_all(fighter.module_accessor);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        }
}

#[acmd_script( agent = "donkey", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
        	macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        	macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 6.0, 361, 80, 0, 10, 5.0, 0.0, 0.0, 0.0, Some(7.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 361, 80, 0, 5, 5.5, 8.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL as i32), 0);
        }
}

//air
#[acmd_script( agent = "donkey", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn attackairhi(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 13.0, 90, 90, 0, 32, 7.5, 2.5, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        }
    wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 9.0, 85, 90, 0, 45, 7.0, 2.5, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        }
    wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent)
        {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//Specials
#[acmd_script( agent = "donkey", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 95, 90, 30, 0, 6.0, 6.5, -1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 5.0, 80, 90, 30, 0, 5.0, 0.0, -1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 5.0, 80, 90, 30, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 5.0, 366, 90, 30, 0, 6.0, 6.5, -1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 4, 0, Hash40::new("armr"), 5.0, 366, 90, 30, 0, 5.0, 0.0, -1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 5, 0, Hash40::new("shoulderr"), 5.0, 366, 90, 30, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 367, 55, 90, 0, 5.0, 0.0, 7.0, 26.0, Some(0.0),  Some(14.0),  Some(26.0), 0.8, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.4, 20, 100, 65, 0, 6.0, 0.0, 12.0, 13.0, None, None, None, 0.8, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.5, 40, 100, 0, 65, 6.0, 0.0, 13.5, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 61.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 40, 157, 0, 40, 10.0, 0.0, 11.0, 20.0,  Some(0.0), Some(11.0), Some(-4.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "donkey", script = "game_specialn", category = ACMD_GAME, low_priority)]
unsafe fn specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.5, 361, 70, 0, 45, 7.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.5, 361, 70, 0, 45, 5.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 12.5, 361, 70, 0, 45, 4.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("waist"), 12.5, 361, 70, 0, 45, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::add_power(fighter.module_accessor, 0, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            AttackModule::add_power(fighter.module_accessor, 1, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            AttackModule::add_power(fighter.module_accessor, 2, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            AttackModule::add_power(fighter.module_accessor, 3, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == FIGHTER_KIND_KIRBY {
                macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.5, 361, 70, 0, 45, 7.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.5, 361, 70, 0, 45, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 12.5, 361, 70, 0, 45, 4.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("waist"), 12.5, 361, 70, 0, 45, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                AttackModule::add_power(fighter.module_accessor, 0, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
                AttackModule::add_power(fighter.module_accessor, 1, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
                AttackModule::add_power(fighter.module_accessor, 2, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
                AttackModule::add_power(fighter.module_accessor, 3, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            }
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            macros::HIT_RESET_ALL(fighter);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "donkey", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.5, 361, 70, 0, 45, 7.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.5, 361, 70, 0, 45, 5.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.5, 361, 70, 0, 45, 4.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("waist"), 9.5, 361, 70, 0, 45, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::add_power(fighter.module_accessor, 0, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            AttackModule::add_power(fighter.module_accessor, 1, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            AttackModule::add_power(fighter.module_accessor, 2, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            AttackModule::add_power(fighter.module_accessor, 3, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == FIGHTER_KIND_KIRBY {
                macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.5, 361, 70, 0, 45, 7.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.5, 361, 70, 0, 45, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.5, 361, 70, 0, 45, 4.0, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("waist"), 9.5, 361, 70, 0, 45, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                AttackModule::add_power(fighter.module_accessor, 0, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
                AttackModule::add_power(fighter.module_accessor, 1, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
                AttackModule::add_power(fighter.module_accessor, 2, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
                AttackModule::add_power(fighter.module_accessor, 3, WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD) as f32, false);
            }
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            macros::HIT_RESET_ALL(fighter);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script(agent = "donkey", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn specialairs(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_S_FLAG_FALL_START);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 270, 40, 0, 30, 6.5, 0.0, 5.0, 9.5, Some(0.0), Some(5.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 70, 40, 0, 15, 10.0, 0.0, 11.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script(agent = "donkey", script = "game_specials", category = ACMD_GAME, low_priority)]
unsafe fn specials(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter)
        {
            VarModule::on_flag(fighter.battle_object, donkey::instance::flag::GET_BARREL);
        }
    //frame(fighter.lua_state_agent, 33.0);
    //    if macros::is_excute(fighter)
    //    {
    //        ItemModule::throw_item(fighter.module_accessor, 22.5, 4.0, 1.0, 0, true, 0.0);
    //    } 
    // frame(fighter.lua_state_agent, 35.0);
    //     if macros::is_excute(fighter)
    //     {
    //         IS_DK_START_ITEM_CHUCK[ENTRY_ID] = false;
    //     }
}

#[acmd_script(agent = "donkey", script = "effect_specials", category = ACMD_EFFECT, low_priority)]
unsafe fn fx_specials(_fighter: &mut L2CAgentBase) {

    //leave empty
}

#[acmd_script(agent = "donkey", script =  "sound_specials", category = ACMD_SOUND, low_priority)]
unsafe fn sound_specials(_fighter: &mut L2CAgentBase) {
    
    //leave empty
}
#[acmd_script(agent = "donkey", script =  "expression_specials", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_specials(_fighter: &mut L2CAgentBase) {
    
    //leave empty
}

//grabs
#[acmd_script( agent = "donkey", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.8, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(13.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 2.9, 0.0, 7.0, 1.1, Some(0.0), Some(7.0), Some(16.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.15); //1.2 was faf 37
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}
#[acmd_script( agent = "donkey", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 4.6, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(15.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 2.3, 0.0, 7.0, 1.7, Some(0.0), Some(7.0), Some(18.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.15); //1.2 was faf 44
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}

#[acmd_script( agent = "donkey", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.8, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-21.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 2.9, 0.0, 7.0, -1.1, Some(0.0), Some(7.0), Some(-24.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.15); //1.2 was faf 40
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}

//others
#[acmd_script( agent = "donkey", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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

#[acmd_script( agent = "donkey", script = "expression_landingheavy", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_landingheavy(fighter: &mut L2CAgentBase) {
    
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::WAVEDASH) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        ohbanana_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attackhi3,
        attacklw3,
        attackairhi,
        specialhi,
        specialn,
        specialairn,
        specialairs,
        specials,
        fx_specials,
        sound_specials,
        expression_specials,
        catch,
        catchdash,
        catchturn,
        escapeairslide,
        expression_landingheavy
    );
}