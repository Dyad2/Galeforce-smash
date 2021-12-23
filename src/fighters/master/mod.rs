use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;
use crate::utils::*;

#[fighter_frame( agent = FIGHTER_KIND_MASTER )]
fn teacher_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let status_kind = StatusModule::status_kind(boma);

        if curr_motion_kind == hash40("special_air_n_start") 
           && MotionModule::frame(boma) <= 28.0 
           && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 
           && ControlModule::get_attack_air_kind(boma) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N {
            StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        }

        //GA - Atrocity
        // type: new move
        //  cancels forward smash (not angled down) with an altered side b that combos
        if !is_operation_cpu(boma) {
            if ![hash40("attack_s4_hi"), hash40("attack_s4_s"), hash40("special_s_front")].contains(&curr_motion_kind) && status_kind != *FIGHTER_STATUS_KIND_WAIT {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
            if [hash40("attack_s4_hi"), hash40("attack_s4_s")].contains(&curr_motion_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                if MotionModule::frame(boma) <= 30.0 {
                    StatusModule::change_status_force(boma, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT, false);
                    StopModule::end_stop(boma);
                    ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
                    FIGHTER_GLOBALS[entry_id as usize].ga_on = true;
                    MotionModule::set_frame(boma, 7.0, true);
                    galeforce_apply_effect(boma, 0.66);
                }
            }
            if FIGHTER_GLOBALS[entry_id as usize].ga_on && curr_motion_kind == hash40("special_s_front") &&
             !StopModule::is_stop(boma) && MotionModule::frame(boma) > 22.0 && ((ControlModule::is_enable_flick_jump(boma) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0) {
                //only jump is allowed, see lib.rs
                StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

//global edits
#[acmd_script( agent = "master", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "master", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "master", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0);
            ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40{hash: hash40("attack_hi4")}, false, 0.0);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
                ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40{hash: hash40("attack_hi4")}, true, WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME));
            }
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 368, 100, 0, 0, 7.0, 0.0, 9.0, 9.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false,0, -1.0, 7, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            let attack_pos = smash::phx::Vector2f {x: 3., y: 27.};
            AttackModule::set_vec_target_pos(boma, 0, Hash40{hash: hash40("top")}, &attack_pos, 8, false);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 368, 100, 0, 0, 7.0, 0.0, 23.5, 1.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false,0, -1.0, 7, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            let attack_pos = smash::phx::Vector2f {x: 3., y: 26.};
            AttackModule::set_vec_target_pos(boma, 0, Hash40{hash: hash40("top")}, &attack_pos, 8, false);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 5.5, 0.0, 26.0, 2.0, Some(0.0), Some(26.0), Some(2.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true,0, -1.0, 4, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 366, 100, 40, 0, 4.5, 0.0, 18.5, 2.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true,0, -1.0, 4, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 366, 100, 60, 0, 5.0, 0.0, 26.5, -6.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true,0, -1.0, 4, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 366, 100, 60, 0, 5.0, 0.0, 26.5, 10.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, true,0, -1.0, 4, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 10.0, 79, 37, 0, 63, 6.5, 0.0, 26.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false,0, -1.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 10.0, 79, 37, 0, 63, 5.5, 0.0, 18.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false,0, -1.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 10.0, 79, 37, 0, 63, 6.0, 0.0, 26.5, -6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false,0, -1.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 10.0, 79, 37, 0, 63, 6.0, 0.0, 26.5, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false,0, -1.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, 3.0, false);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                MotionModule::set_rate(boma, 1.75);
            }
        }
    frame(lua_state, 79.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::app::ArticleOperationTarget(0));
        }
}

#[acmd_script( agent = "master", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            MotionModule::set_rate(boma, 1.25);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 90, 10, 0, 4.5, 0.0, 5.5, 6.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 2.0, 361, 90, 10, 0, 1.6, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 2.0, 180, 90, 25, 0, 2.6, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 180, 90, 35, 0, 2.0, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 2.0);
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            wait(lua_state, 1.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
            wait(lua_state, 16.);
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 85, 0, 55, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 361, 85, 0, 55, 2.5, 0.0, 3.0, 0.0, Some(0.0), Some(13.5), Some(0.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 18.0, 361, 75, 0, 55, 3.5, -0.5, 16.5, 0.0, Some(-0.5), Some(23.5), Some(0.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 18.0, 361, 75, 0, 55, 4.0, -0.5, 24.0, -0.3, Some(-0.5), Some(29.0), Some(-0.3), 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                }
            wait(lua_state, 12.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                    MotionModule::set_rate(boma, 1.0);
                }
        }
        else {
            wait(lua_state, 27.);
                if macros::is_excute(fighter)
                {
                    MotionModule::set_rate(boma, 1.0);
                }
        }
    frame(lua_state, 85.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, smash::app::ArticleOperationTarget(0));
        }
}

#[acmd_script( agent = "master", script = "game_attacks4hi", category = ACMD_GAME, low_priority)]
unsafe fn attacks4hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            MotionModule::set_rate(boma, 1.25);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 90, 10, 0, 4.5, 0.0, 5.5, 6.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 2.0, 361, 90, 10, 0, 1.6, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 2.0, 195, 90, 25, 0, 2.6, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 2.0, 195, 90, 35, 0, 2.0, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 3.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 2.0);
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            wait(lua_state, 1.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
            wait(lua_state, 16.);
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 85, 0, 55, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 12.0, 361, 85, 0, 55, 2.5, 0.0, 3.0, 0.0, Some(0.0), Some(13.5), Some(0.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 18.0, 361, 75, 0, 55, 3.5, -0.5, 16.5, 0.0, Some(-0.5), Some(23.5), Some(0.0), 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 18.0, 361, 75, 0, 55, 4.0, -0.5, 24.0, -0.3, Some(-0.5), Some(28.0), Some(-0.3), 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 18.0, 361, 75, 0, 55, 4.5, 0.0, 13.0, 26.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                }
            wait(lua_state, 10.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                    MotionModule::set_rate(boma, 1.0);
                }
        }
        else {
            wait(lua_state, 27.);
                if macros::is_excute(fighter)
                {
                    MotionModule::set_rate(boma, 1.0);
                }
        }
    frame(lua_state, 85.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, smash::app::ArticleOperationTarget(0));
        }
}

#[acmd_script( agent = "master", script = "game_attacks4lw", category = ACMD_GAME, low_priority)]
unsafe fn attacks4lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            MotionModule::set_rate(boma, 1.25);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 361, 85, 0, 55, 4.5, 0.0, 5.5, 6.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 11.5, 361, 85, 0, 55, 1.6, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 17.25, 36, 85, 0, 60, 2.6, -0.5, 16.5, 0.0, Some(-0.5), Some(23.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 17.25, 36, 85, 0, 60, 2.0, -0.5, 24.0, -0.3, Some(-0.5), Some(26.5), Some(-0.3), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 2.0);
        }
    wait(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 85.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR,smash::app::ArticleOperationTarget(0));
        }
}

//air
#[acmd_script( agent = "master", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, false, 0);
            ArticleModule::change_motion(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, smash::phx::Hash40{hash:hash40("attack_air_n")}, false, 0.0);
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 3.0);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 2.5, 367, 100, 40, 0, 3.5, -4.0, -2.8, 0.0, None, None, None, 0.7, 1.33, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 2.5, 367, 100, 40, 0, 2.7, -4.0, 1.3, 0.0, None, None, None, 0.7, 1.33, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 2.5, 367, 100, 35, 0, 3.0, 0.0, 9.0, 0.0, None, None, None, 0.7, 1.33, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 51, 150, 0, 44, 10.0, 0.0, 9.0, 2.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 39.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 53.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, smash::app::ArticleOperationTarget(0));
        }
}

#[acmd_script( agent = "master", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
            MotionModule::set_rate(boma, 1.25);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.66);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 48, 100, 0, 52, 3.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(13.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.75, 361, 84, 0, 52, 3.5, 0.0, 5.0, 16.5, Some(0.0), Some(5.0), Some(25.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 1.);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 53.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, smash::app::ArticleOperationTarget(0));
        }
}

//specials
#[acmd_script( agent = "master", script = "game_specialsf", category = ACMD_GAME, low_priority)]
unsafe fn specialsf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST)
        }
    frame(lua_state, 8.);
        if FIGHTER_GLOBALS[entry_id as usize].ga_on {
                if macros::is_excute(fighter)
                {
                    MotionModule::set_rate(boma, 1.25);
                    macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 12.0, 85, 25, 0, 80, 3.5, 0.0, 25.0, -1.0, Some(0.0), Some(19.5), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 12.0, 78, 25, 0, 80, 3.5, 0.0, 13.5, -1.0, Some(0.0), Some(8.5), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 78, 25, 0, 80, 5.0, 0.0, 5.0, 11.0, Some(0.0), Some(10.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 85, 25, 0, 80, 5.5, 0.0, 6.0, 22.0, Some(0.0), Some(11.0), Some(22.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                }
            wait(lua_state, 4.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
        }
        else {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 17.25, 59, 40, 0, 115, 3.5, 0.0, 25.0, -1.0, Some(0.0), Some(19.5), Some(-1.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 11.5, 45, 35, 0, 85, 3.0, 0.0, 13.5, -1.0, Some(0.0), Some(8.5), Some(-1.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.5, 45, 35, 0, 85, 5.0, 0.0, 5.0, 11.0, Some(0.0), Some(10.0), Some(11.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 17.25, 59, 40, 0, 115, 5.5, 0.0, 6.0, 22.0, Some(0.0), Some(11.0), Some(22.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 17.25, 59, 40, 0, 115, 2.0, 0.0, 8.5, 28.0, Some(0.0), Some(14.0), Some(28.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                }
            frame(lua_state, 10.);
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.5, 45, 35, 0, 85, 5.0, 0.0, 5.0, 11.0, Some(0.0), Some(14.0), Some(11.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 17.25, 59, 40, 0, 115, 5.5, 0.0, 6.0, 22.0, Some(0.0), Some(21.0), Some(22.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                    macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 17.25, 59, 40, 0, 115, 2.0, 0.0, 8.5, 28.0, Some(0.0), Some(18.5), Some(28.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                }
            frame(lua_state, 13.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
            frame(lua_state, 14.);
                if macros::is_excute(fighter)
                {
                    WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
                }
            frame(lua_state, 29.);
                if macros::is_excute(fighter)
                {
                    WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
                }
        }
    frame(lua_state, 53.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, smash::app::ArticleOperationTarget(0));
        }
}

#[acmd_script( agent = "master", script = "game_specialairsf", category = ACMD_GAME, low_priority)]
unsafe fn specialairsf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 10., 5., 5., 9.);
            MotionModule::set_rate(boma, 1.33);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5., 5., 5., 5.);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.7);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_NONE);
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 14.25, 55, 59, 0, 86, 4.5, 0.0, 24.5, -2.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 14.25, 55, 59, 0, 86, 5.0, 0.0, 19.5, -2.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.5, 45, 35, 0, 85, 4.5, 0.0, 11.5, -2.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.5, 45, 35, 0, 85, 4.5, 0.0, 7.0, -2.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
        }
    frame(lua_state, 26.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
        }
    frame(lua_state, 48.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, smash::app::ArticleOperationTarget(0));
            WorkModule::off_flag(boma, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
        }
}

//other
#[acmd_script( agent = "master", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        teacher_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attackhi4,
        attacks4,
        attacks4hi,
        attacks4lw,
        attackairn,
        attackairf,
        specialsf,
        specialairsf,
        escapeairslide
    );
}