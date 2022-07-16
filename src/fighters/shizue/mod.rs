use std::arch::asm;
use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;



use crate::fighters::common::galeforce::*;
use galeforce_utils::vars::*;
use custom_var::*;

// #[fighter_frame( agent = FIGHTER_KIND_SHIZUE )]
// fn capitalism_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         let lua_state = fighter.lua_state_agent;
//         let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

//         //GA - Isabellistic Trap
//         //type: restriction lift
//         //launch mine while in forward or back throw
//         if !is_operation_cpu(fighter.module_accessor) {
//             // if [hash40("throw_f"), hash40("throw_b")].contains(&curr_motion_kind) {
//             //     if MotionModule::frame(fighter.module_accessor) >= 10.0 && MotionModule::frame(fighter.module_accessor) < 21.0 && WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DO_ONCE) {
//             //         if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
//             //             ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(0), false);
//             //             VarModule::on_flag(fighter.battle_object, _INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_ON);
//             //             WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DO_ONCE);
//             //         }
//             //     }
//             // }
//             // else {
//             //     VarModule::on_flag(fighter.battle_object, _INSTANCE_WORK_ID_FLAG_DO_ONCE);
//             // }
//             // if WorkModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
//             //     galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
//             //     WorkModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
//             // }
//             if [hash40("throw_f"), hash40("throw_b")].contains(&curr_motion_kind) {
//                 if MotionModule::frame(fighter.module_accessor) >= 10.0 && MotionModule::frame(fighter.module_accessor) < 21.0 {
//                     if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
//                         ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(0), false);
//                         WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DO_ONCE);
//                         if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DO_ONCE) {
//                         }
//                     }
//                 }
//             }
//             else {
//                 VarModule::on_flag(fighter.battle_object, _INSTANCE_WORK_ID_FLAG_DO_ONCE);
//             }
//             if WorkModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
//                 WorkModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
//             }
//         }
//     }
// }

//weapons
#[acmd_script( agent = "shizue_pot", script = "game_throwed", category = ACMD_GAME, low_priority)]
unsafe fn potthrowed(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("have"), 10.0, 70, 60, 0, 65, 4.2, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
    wait(lua_state, 6.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("have"), 6.0, 70, 60, 0, 65, 3.7, 0.0, 2.5, 0.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
}

#[acmd_script( agent = "shizue_bullet", script = "game_shootf", category = ACMD_GAME, low_priority)]
unsafe fn bulletshootf(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.0, 361, 100, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
    frame(lua_state, 4.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 10, 1.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
}

#[acmd_script( agent = "shizue_bullet", script = "game_shootb", category = ACMD_GAME, low_priority)]
unsafe fn bulletshootb(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.0, 361, 100, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
    frame(lua_state, 4.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 10, 1.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
}

#[acmd_script( agent = "shizue_clayrocket", script = "game_fly", category = ACMD_GAME, low_priority)]
unsafe fn clayrocketfly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.6, 96, 100, 70, 0, 6.0, 0.0, 0.5, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    wait(lua_state, 11.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.9, 86, 10, 0, 70, 6.0, 0.0, 0.5, 0.0, None, None, None, 0.3, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
}

//global edits
#[acmd_script( agent = "shizue", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "shizue", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//air
#[acmd_script( agent = "shizue", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 100, 0, 40, 7.0, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 85, 100, 0, 40, 5.0, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        }
    frame(lua_state, 32.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 39.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//specials
#[acmd_script( agent = "shizue", script = "game_speciallwset", category = ACMD_GAME, low_priority)]
unsafe fn speciallwset(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_LW_FLAG_SET);
        }
}

//throws
#[acmd_script( agent = "shizue", script = "game_throwf", category = ACMD_GAME, low_priority)]
unsafe fn throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 10, 0, 102, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            //GA - Isabellistic Trap
            //type: restriction lift
            //launch mine while in forward or back throw
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(0), false);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
            }
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
            macros::CHECK_FINISH_CAMERA(fighter, 14, 8);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
}

#[acmd_script( agent = "shizue", script = "game_throwb", category = ACMD_GAME, low_priority)]
unsafe fn throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
           macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 135, 30, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
           macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
         }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::CHECK_FINISH_CAMERA(fighter, -7, 2);
            //GA - Isabellistic Trap
            //type: restriction lift
            //launch mine while in forward or back throw
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
            }
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
            {
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::REVERSE_LR(fighter);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
}

//other
#[acmd_script( agent = "shizue", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
    //smashline::install_agent_frames!(
    //    capitalism_frame
    //);
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        potthrowed,
        bulletshootf,
        bulletshootb,
        clayrocketfly,
        attackairhi,
        speciallwset,
        throwf,
        throwb,
        escapeairslide
    );
}