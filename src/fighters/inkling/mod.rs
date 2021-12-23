use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_battle_object;
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::fighters::common::FIGHTER_GLOBALS;

#[fighter_frame( agent = FIGHTER_KIND_INKLING )]
fn woomy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        if curr_motion_kind == hash40("attack_air_b") && (MotionModule::frame(boma) >= 12. && MotionModule::frame(boma) <= 13.)
        || curr_motion_kind == hash40("attack_air_lw") && (MotionModule::frame(boma) >= 18. && MotionModule::frame(boma) <= 19.) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                FIGHTER_GLOBALS[entry_id as usize].ink_arts = true;
            }
            else {
                FIGHTER_GLOBALS[entry_id as usize].ink_arts = false;
            }
        
            if FIGHTER_GLOBALS[entry_id as usize].ink_arts {
                FIGHTER_GLOBALS[entry_id as usize].ink_arts_delay += 1;
                if FIGHTER_GLOBALS[entry_id as usize].ink_arts_delay % 5 == 0 {
                    FIGHTER_GLOBALS[entry_id as usize].ink_arts_delay = 0;
                    ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_INKBULLET, false, 0);
                }
                MotionModule::set_rate(boma, 0.0);
                if curr_motion_kind == hash40("attack_air_b") {
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("inkling_splashooter_muzzle"), Hash40::new("muzzle"), -1, 0, 0, 0, 180, 0, 0.8, true);
                    macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 361, 10, 0, 12, 5.0, 0.0, 3.0, -16.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                    AttackModule::set_ink_value(boma, 0, 10.);
                }
                if curr_motion_kind == hash40("attack_air_lw") {
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("inkling_splashooter_muzzle"), Hash40::new("muzzle"), -1, 0, 0, 0, 180, 0, 0.8, true);
                    macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 361, 10, 0, 12, 5.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                    AttackModule::set_ink_value(boma, 0, 10.);
                }
            }
            else {
                MotionModule::set_rate(boma, 1.0);
                AttackModule::clear_all(boma);
                EffectModule::kill_kind(boma, Hash40::new("inkling_splashooter_muzzle"), false, false);
            }
        }
    }
}


#[acmd_script( agent = "inkling_inkbullet", script = "game_fly", category = ACMD_GAME, low_priority)]
unsafe fn inkbulletfly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    
        if macros::is_excute(weapon)
        {
            if MotionModule::motion_kind(owner_boma) == hash40("attack_air_b") {
                macros::SET_SPEED_EX(weapon, -5.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ID_MAIN);
            }
            if MotionModule::motion_kind(owner_boma) == hash40("attack_air_lw") {
                macros::SET_SPEED_EX(weapon, -5.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ID_MAIN);
            }
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 45, 100, 6, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 0.0);
        }
    wait(lua_state, 2.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 361, 100, 0, 10, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(boma, 0, 0.0);
        }
}

//global edits
#[acmd_script( agent = "inkling", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(boma, false);
            WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
            if !WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
                ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, 0);
                ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(MotionModule::motion_kind(boma)), false, 0.0);
                ArticleModule::set_frame(boma,*FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::frame(boma));
                ArticleModule::set_rate(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::rate(boma));
                ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
                if PostureModule::lr(boma) > 0.
                {
                    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("dash_l"), false, 0.0);
                }
                else
                {
                    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("dash_r"), false, 0.0);
                }
                ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
            }
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(boma, true);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
        }
}

#[acmd_script( agent = "inkling", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(boma, false);
            WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
            if !WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
                ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, 0);
                ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(MotionModule::motion_kind(boma)), false, 0.0);
                ArticleModule::set_frame(boma,*FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::frame(boma));
                ArticleModule::set_rate(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::rate(boma));
                ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
                if PostureModule::lr(boma) > 0.
                {
                    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("turn_dash_l"), false, 0.0);
                }
                else
                {
                    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("turn_dash_r"), false, 0.0);
                }
                ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
            }
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(boma, true);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
        }
}

//ground
#[acmd_script( agent = "inkling", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 3.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 30, 0, 25, 2.4, 0.0, 6.7, 6.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 2.4, 0.0, 6.7, 8.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 20, 0, 15, 2.4, 0.0, 6.7, 11.7, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 20, 0, 15, 2.4, 0.0, 6.7, 11.7, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        }
}

//air
#[acmd_script( agent = "inkling", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 6.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.7);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 361, 85, 0, 35, 3.5, 0.0, 9.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 7.5, 361, 85, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("legr"), 7.5, 361, 85, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.5, 45, 85, 0, 35, 3.5, 0.0, 9.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 4, 0, Hash40::new("legl"), 7.5, 45, 85, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 5, 0, Hash40::new("legr"), 7.5, 45, 85, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1);
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 12.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 85, 0, 35, 3.5, 0.0, 9.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 5.0, 361, 85, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("legr"), 5.0, 361, 85, 0, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 28.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 56.0);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
}

//grabs / throws
#[acmd_script( agent = "inkling", script = "game_throwhi", category = ACMD_GAME, low_priority)]
unsafe fn throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
            if !WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
                ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, 0);
                ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(MotionModule::motion_kind(boma)), false, 0.0);
                ArticleModule::set_frame(boma,*FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::frame(boma));
                ArticleModule::set_rate(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::rate(boma));
                ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
                if PostureModule::lr(boma) > 0.
                {
                    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("throw_hi_l"), false, 0.0);
                }
                else
                {
                    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("throw_hi_r"), false, 0.0);
                }
            }
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 87, 70, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
        }
    frame(lua_state, 12.0);
        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(boma, false);
        }
    frame(lua_state, 21.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 30, 0, 80, 5.5, 0.0, 18.0, 0.0, None, None, None, 1.4, 0., *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
            AttackModule::set_catch_only_all(boma, true, false);
            macros::CHECK_FINISH_CAMERA(fighter, 1, 25);
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
            FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.7);
            FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0., y:10., z:0.});
        }
    frame(lua_state, 22.0);
        if macros::is_excute(fighter)
        {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 30.0);
        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(boma, true);
        }
    frame(lua_state, 35.0);
        if macros::is_excute(fighter)
        {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
        }
}

#[acmd_script( agent = "inkling", script = "game_throwf", category = ACMD_GAME, low_priority)]
unsafe fn throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 40, 60, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 22.0);
        if macros::is_excute(fighter)
        {
            if WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 100, 30, 0, 5.5, 0.0, 13.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
                AttackModule::set_ink_value(boma, 0, 80.0);
                AttackModule::set_catch_only_all(boma, true, false);
            }
            macros::CHECK_FINISH_CAMERA(fighter, 10, 1);
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
            FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
            FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 3., y: 3., z:0.});
        }
    frame(lua_state, 23.0);
        if macros::is_excute(fighter)
        {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            AttackModule::clear_all(boma);
        }

}

#[acmd_script( agent = "inkling", script = "game_throwlw", category = ACMD_GAME, low_priority)]
unsafe fn throwlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 70, 70, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 13.0);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, 7, 1);
            sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
            macros::CHECK_FINISH_CAMERA(fighter, 7, 0);
            let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
            FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
            FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{ x: 3., y: 0., z: 0.});
        }
    frame(lua_state, 14.0);
        if macros::is_excute(fighter)
        {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
}

#[acmd_script( agent = "inkling", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
            if !WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
                ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, 0);
                ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(MotionModule::motion_kind(boma)), false, 0.0);
                ArticleModule::set_frame(boma,*FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::frame(boma));
                ArticleModule::set_rate(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::rate(boma));
                ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
            }
        }
    frame(lua_state, 6.0);
        if macros::is_excute(fighter)
        {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
        }
    frame(lua_state, 12.0);
        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(boma, false);
        }
    frame(lua_state, 14.0);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
    frame(lua_state, 30.0);
        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(boma, true);
        }
    frame(lua_state, 43.0);
        if macros::is_excute(fighter)
        {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
        }
}

pub fn install() {
    //smashline::install_agent_frames!(
    //    woomy_frame
    //);
    smashline::install_acmd_scripts!(
        inkbulletfly,
        dash,
        turndash,
        attack11,
        attackairn,
        throwhi,
        throwf,
        throwlw,
        escapeairslide
    );
}