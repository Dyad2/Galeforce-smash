use std::arch::asm;
use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smashline::*;
use smash_script::*;


use crate::fighters::common::galeforce::*;
use galeforce_utils::vars::*;
use custom_var::*;

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn worst_fe_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            if [hash40("special_s_wall_attack_b"), hash40("special_s_wall_attack_f")].contains(&curr_motion_kind) {
                MotionModule::set_rate(fighter.module_accessor, 1.4);
            }
        }
    }
}

//weapon
#[acmd_script( agent = "kamui_ryusensya", script = "game_regular", category = ACMD_GAME, low_priority)]
unsafe fn ryusensya_min(weapon: &mut L2CAgentBase) {

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.0, 45, 90, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 9.0, 45, 90, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            smash_script::attack!(weapon, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            macros::ATK_LERP_RATIO(weapon, WorkModule::get_float(weapon.module_accessor, *WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLOAT_HOLD_RATE));
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
}

#[acmd_script( agent = "kamui_ryusensya", script = "game_shotmax", category = ACMD_GAME, low_priority)]
unsafe fn ryusensya_max(weapon: &mut L2CAgentBase) {

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0, 55, 90, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
}


#[acmd_script( agent = "kamui_waterdragon", scripts = ["game_speciallwhit, game_speciallwhitturn", "game_specialairlwhit, game_specialairlwhitturn"], category = ACMD_GAME, low_priority)]
unsafe fn dragoncounter(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

    frame(lua_state, 26.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 80, 20, 0, 90, 12.0, 0.0, 6.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0, 80, 20, 0, 90, 12.0, 0.0, 6.0, -11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
            AttackModule::set_force_reaction(weapon.module_accessor, 0, true, false);
            AttackModule::set_force_reaction(weapon.module_accessor, 1, true, false);
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
                if macros::is_excute(weapon)
                {
                    AttackModule::set_optional_hit_sound(weapon.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
                    AttackModule::set_optional_hit_sound(weapon.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
                }
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 8.0, 90, 20, 0, 90, 8.0, 0.0, 21.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
            macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 8.0, 90, 20, 0, 90, 8.0, 0.0, 21.0, -11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
            AttackModule::set_force_reaction(weapon.module_accessor, 2, true, false);
            AttackModule::set_force_reaction(weapon.module_accessor, 3, true, false);
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
                if macros::is_excute(weapon)
                {
                    AttackModule::set_optional_hit_sound(weapon.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
                    AttackModule::set_optional_hit_sound(weapon.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
                }
            }
        }
    frame(lua_state, 31.);
        if macros::is_excute(weapon)
        {
            AttackModule::clear_all(weapon.module_accessor);
            MotionModule::set_rate(weapon.module_accessor, 1.2);
        }
}

//global edits
#[acmd_script( agent = "kamui", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "kamui", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//Ground
#[acmd_script( agent = "kamui", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 2.5);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 10.5, 45, 100, 0, 30, 5.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 10.5, 45, 100, 0, 30, 4.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 45, 100, 0, 30, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "kamui", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 9.0, 88, 85, 0, 55, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 88, 85, 0, 55, 5.0, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.0, 88, 85, 0, 55, 5.0, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 88, 85, 0, 55, 5.0, 0.0, 12.0, 4.0, Some(0.0), Some(12.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 3.0, false);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 88, 85, 0, 55, 5.0, 0.0, 22.0, -2.5, Some(0.0), Some(22.0), Some(2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 3.0, false)
        }
    wait(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
}

#[acmd_script( agent = "kamui", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        frame(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 100, 80, 0, 50, 5.0, 0.0, 2.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.5, 100, 80, 0, 50, 4.0, 0.0, 2.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.5, 100, 80, 0, 50, 4.0, 0.0, 2.0, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 1.5);
            }
        wait(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
}

//air
#[acmd_script( agent = "kamui", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND as i32, false, -1);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, smash::phx::Hash40{hash: hash40("attack_air_n")}, false, 0.0);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 7.0, 70, 105, 0, 45, 5.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 7.0, 70, 105, 0, 45, 4.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("handl"), 7.0, 70, 105, 0, 45, 3.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 1, Hash40::new("haver"), 7.0, 70, 105, 0, 45, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 1, Hash40::new("haver"), 7.0, 70, 105, 0, 45, 4.0, 0.0, 4.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 1, Hash40::new("haver"), 7.0, 70, 105, 0, 45, 3.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    wait(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 5.5, 70, 105, 0, 45, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 5.5, 70, 105, 0, 45, 3.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("handl"), 5.5, 70, 105, 0, 45, 2.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 1, Hash40::new("haver"), 5.5, 70, 105, 0, 45, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 1, Hash40::new("haver"), 5.5, 70, 105, 0, 45, 3.5, 0.0, 4.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 1, Hash40::new("haver"), 5.5, 70, 105, 0, 45, 2.5, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    wait(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
    frame(lua_state, 46.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//specials
#[acmd_script( agent = "kamui", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, *SITUATION_KIND_AIR);
            sv_animcmd::SA_SET(fighter.lua_state_agent);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 4.5, 85, 100, 160, 0, 5.0, 0.0, 4.0, -3.0, Some(0.0), Some(4.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 0, 70, 32, 0, 7.0, 0.0, 3.5, -14.0, Some(0.0), Some(3.5), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true,0, 0.0, 0, false, false, true,true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_TILT_BODY_ON);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 1, Hash40::new("rot"), 1.2, 80, 100, 150, 0, 2.0, 0.0, 5.5, 0.0, None, None, None, 0.7,   0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 1, Hash40::new("rot"), 1.2, 105, 100, 150, 0, 5.5, 0.0, 5.5, 6.0, None, None, None, 0.7,  0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 1, Hash40::new("rot"), 1.2, 68, 100, 150, 0, 5.5, 0.0, 5.5, -6.0, None, None, None, 0.7,  0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 3, 1, Hash40::new("rot"), 1.2, 100, 100, 165, 0, 5.0, 0.0, -1.0, 5.5, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 4, 1, Hash40::new("rot"), 1.2, 73, 100, 165, 0, 5.0, 0.0, -1.0, -5.5, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 2, Hash40::new("rot"), 3.0, 65, 170, 0, 70, 8.5, 0.0, 5.5, -7.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 2, Hash40::new("rot"), 3.0, 65, 170, 0, 70, 8.5, 0.0, 5.5, 7.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 39.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_TILT_BODY_ON);
        }
    frame(lua_state, 45.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_AIR_CONTROL);
        }
    frame(lua_state, 49.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.8);
        }
}

#[acmd_script( agent = "kamui", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, *SITUATION_KIND_AIR);
            sv_animcmd::SA_SET(fighter.lua_state_agent);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 4.5, 85, 100, 160, 0, 5.0, 0.0, 4.0, -3.0, Some(0.0), Some(4.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 0, 70, 32, 0, 7.0, 0.0, 3.5, -14.0, Some(0.0), Some(3.5), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true,0, 0.0, 0, false, false, true,true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_TILT_BODY_ON);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 1, Hash40::new("rot"), 1.2, 80, 100, 150, 0, 2.0, 0.0, 5.5, 0.0, None, None, None, 0.7,   0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 1, Hash40::new("rot"), 1.2, 105, 100, 150, 0, 5.5, 0.0, 5.5, 6.0, None, None, None, 0.7,  0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 1, Hash40::new("rot"), 1.2, 68, 100, 150, 0, 5.5, 0.0, 5.5, -6.0, None, None, None, 0.7,  0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 3, 1, Hash40::new("rot"), 1.2, 100, 100, 165, 0, 5.0, 0.0, -1.0, 5.5, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 4, 1, Hash40::new("rot"), 1.2, 73, 100, 165, 0, 5.0, 0.0, -1.0, -5.5, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 2, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 2, Hash40::new("rot"), 3.0, 65, 170, 0, 70, 8.5, 0.0, 5.5, -7.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 2, Hash40::new("rot"), 3.0, 65, 170, 0, 70, 8.5, 0.0, 5.5, 7.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 39.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_TILT_BODY_ON);
        }
    frame(lua_state, 45.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_HI_FLAG_AIR_CONTROL);
        }
    frame(lua_state, 49.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.8);
        }
}

#[acmd_script( agent = "kamui", script = "game_specialswallattackb", category = ACMD_GAME, low_priority)]
unsafe fn specialswallattackb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_wall_attack_b"), false, 0.0);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 100, 0, 70, 4.0, 0.0, 1.7, 12.5, Some(0.0), Some(1.7), Some(24.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_size(fighter.module_accessor, 0, 0.1);
            smash_script::attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, hash40("top"), 0, 2, 0);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_MOVE_KINETIC_PARAM);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_WALL_ATTACK_B_REVERSE_LR);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 55, 98, 0, 70, 8.3, 0.0, 2.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 55, 90, 0, 70, 4.5, 0.0, 3.4, -4.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 34.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_AIR_CONTROL);
        }
}

#[acmd_script( agent = "kamui", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority)]
unsafe fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.5); //TODO: doesn't work?
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_SHIELD);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_SHIELD);
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
}

//other
#[acmd_script( agent = "kamui", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        worst_fe_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        dragoncounter,
        ryusensya_min,
        ryusensya_max,
        attackhi3,
        attacks3,
        attacklw3,
        attackairn,
        specialhi,
        specialairhi,
        specialswallattackb,
        speciallw,
        escapeairslide
    );
}