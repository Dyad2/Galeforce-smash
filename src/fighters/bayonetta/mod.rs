use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::sv_battle_object;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;
use crate::utils::*;
use crate::var::*;

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        
        //GA - Witches' Ascent
        // type: restriction lift
        //  after using afterburner kick once, hitting with dabk allows an additional use of upwards abk
        if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U 
          && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
            WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_ON);
        }
        //allows one more abk after hitting with a second dabk
        if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D 
          && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) 
          && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_ON) {
            WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_CONFIRM);
        }
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_ON) && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_CONFIRM) {
            WorkModule::set_int(boma, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
            if (status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U || status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D) && MotionModule::frame(boma) < 3.0 {
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
                WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_CONFIRM);
                WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_ON);
                galeforce_apply_effect(boma, 0.66);
            }
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_ON);
            WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_GALEFORCE_ATTACK_CONFIRM)
        }

        //remove gravity on bats startup
        if StatusModule::status_kind(boma) == *FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && MotionModule::frame(boma) <= 10. {
                KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }

        //dodge offset
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset_ftilt_num == 2 {
                    FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset_ftilt_num = 0;
                    FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset = false;
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                    MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40{hash: hash40("attack_s3_s2")}, 0.0, 0.0, 0.0);
                }
                if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset_ftilt_num == 3 {
                    FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset_ftilt_num = 0;
                    FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset = false;
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                    MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40{hash: hash40("attack_s3_s3")}, 0.0, 0.0, 0.0);
                }
            }
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset_ftilt_num != 0 {
                FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset = true;
                if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) >= 0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40{hash: hash40("escape_f")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ESCAPE_F, false);
                }
                if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) < 0.2 && ControlModule::get_stick_x(boma) * PostureModule::lr(boma) > -0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40{hash: hash40("escape")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ESCAPE, false);
                }
                if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) <= -0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40{hash: hash40("escape_b")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ESCAPE_B, false);
                }
            }
        }

        if ![hash40("attack_s3_s"), hash40("attack_s3_s2"), hash40("escape_b"), hash40("escape"), hash40("escape_f")].contains(&curr_motion_kind) {
            FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset_ftilt_num = 0;
            FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset = false;
            FIGHTER_GLOBALS[entry_id as usize].bayo_has_used_dodge_offset_2nd = false;
        }

        //removes wicked weave costume
        if is_status_damage(boma) {
            smash_script::notify_event_msc_cmd!(fighter, 0x25fd66ecef as u64, 30, 0, 1);
        }
    }
}

//global edits
#[acmd_script( agent = "bayonetta", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//weapon
#[acmd_script( agent = "bayonetta_wickedweavearm", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn attackhi4wwa(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(weapon)
        {
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide") as i64);
        }
    frame(lua_state, 8.);
        if macros::is_excute(weapon)
        {
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_show") as i64);
        }
    frame(lua_state, 10.);
        if macros::is_excute(weapon)
        {
            LinkModule::unlink_all(boma);
        }
    frame(lua_state, 11.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 17.0, 86, 90, 0, 32, 10.5, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 17.0, 86, 90, 0, 32, 10.5, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
            WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        }
    frame(lua_state, 13.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 16.0, 86, 90, 0, 32, 10.5, 0.0, 20.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 16.0, 86, 90, 0, 32, 10.5, 0.0, 20.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    frame(lua_state, 15.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 15.0, 86, 90, 0, 32, 10.5, 0.0, 30.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 15.0, 86, 90, 0, 32, 10.5, 0.0, 30.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(weapon)
        {
            AttackModule::clear_all(boma);
        }
    wait(lua_state, 51.);
        if macros::is_excute(weapon)
        {
            WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        }
}

#[acmd_script( agent = "bayonetta_wickedweavearm", script = "game_attacks4hi", category = ACMD_GAME, low_priority)]
unsafe fn attacks4hiwwa(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(weapon)
        {
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_hide") as i64);
        }
    frame(lua_state, 8.);
        if macros::is_excute(weapon)
        {
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_show") as i64);
        }
    frame(lua_state, 9.);
        if macros::is_excute(weapon)
        {
            WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        }
    frame(lua_state, 12.);
        if macros::is_excute(weapon)
        {
            LinkModule::unlink_all(boma);
        }
    frame(lua_state, 13.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 35, 5.5, 0.0, 14.5, 7.0, Some(0.0), Some(14.5), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 14.0, 361, 100, 0, 35, 5.5, 0.0, 14.5, 7.0, Some(0.0), Some(14.5), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 16.0, 361, 100, 0, 35, 8.0, 0.0, 15.0, 29.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 16.0, 361, 100, 0, 35, 8.0, 0.0, 15.0, 29.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 4, 0, Hash40::new("top"), 14.0, 361, 100, 0, 35, 5.0, 0.0, 10.0, 10.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
        }
    wait(lua_state, 5.);
        if macros::is_excute(weapon)
        {
            AttackModule::clear_all(boma);
        }
    wait(lua_state, 40.);
        if macros::is_excute(weapon)
        {
            WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        }
}

#[acmd_script( agent = "bayonetta_wickedweavearm", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn attacks4wwa(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    //forward smash
    if MotionModule::motion_kind(owner_boma) == hash40("attack_s4_s") {
            if macros::is_excute(weapon)
            {
                VisibilityModule::set_int64(boma, hash40("body") as i64,  hash40("body_hide") as i64);
            }
        frame(lua_state, 8.);
            if macros::is_excute(weapon)
            {
                VisibilityModule::set_int64(boma,  hash40("body") as i64,  hash40("body_show") as i64);
            }
        frame(lua_state, 9.);
            if macros::is_excute(weapon)
            {
                WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
            }
        frame(lua_state, 12.);
            if macros::is_excute(weapon)
            {
                LinkModule::unlink_all(boma);
            }
        frame(lua_state, 13.);
            if macros::is_excute(weapon)
            {
                macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 14.0, 361, 97, 0, 35, 5.5, 0.0, 16.0, 7.0, Some(0.0), Some(12.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 14.0, 361, 97, 0, 35, 5.5, 0.0, 16.0, 7.0, Some(0.0), Some(12.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 16.0, 361, 97, 0, 35, 8.0, 0.0, 10.0, 29.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 16.0, 361, 97, 0, 35, 8.0, 0.0, 10.0, 29.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(weapon, 4, 0, Hash40::new("top"), 14.0, 361, 97, 0, 35, 5.0, 0.0, 10.0, 10.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
            }
        wait(lua_state, 3.);
            if macros::is_excute(weapon)
            {
                macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 14.0, 361, 85, 0, 40, 5.5, 0.0, 16.0, 7.0, Some(0.0), Some(12.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 14.0, 361, 85, 0, 40, 5.5, 0.0, 16.0, 7.0, Some(0.0), Some(12.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 14.0, 361, 85, 0, 40, 8.0, 0.0, 10.0, 29.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 14.0, 361, 85, 0, 40, 8.0, 0.0, 10.0, 29.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                macros::ATTACK(weapon, 4, 0, Hash40::new("top"), 14.0, 361, 85, 0, 40, 5.0, 0.0, 10.0, 10.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        wait(lua_state, 3.);
            if macros::is_excute(weapon)
            {
                AttackModule::clear_all(boma);
            }
        wait(lua_state, 40.);
            if macros::is_excute(weapon)
            {
                WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
            }
    }
    //jab finisher
    if MotionModule::motion_kind(owner_boma) == hash40("attack_100_end") {
        if macros::is_excute(weapon)
            {
                VisibilityModule::set_int64(boma, hash40("body") as i64,  hash40("body_hide") as i64);
            }
        frame(lua_state, 8.);
            if macros::is_excute(weapon)
            {
                VisibilityModule::set_int64(boma,  hash40("body") as i64,  hash40("body_show") as i64);
            }
        frame(lua_state, 9.);
            if macros::is_excute(weapon)
            {
                WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
            }
        frame(lua_state, 13.);
            if macros::is_excute(weapon)
            {
                macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 361, 100, 0, 80, 8.5, 0.0, 10.0, 7.0,  Some(0.0), Some(12.0), Some(29.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
            }
        wait(lua_state, 5.);
            if macros::is_excute(weapon)
            {
                AttackModule::clear_all(boma);
            }
        wait(lua_state, 40.);
            if macros::is_excute(weapon)
            {
                WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
            }
    }
    //tetsuzanko
    if MotionModule::motion_kind(owner_boma) == hash40("throw_f") {
        if macros::is_excute(weapon)
            {
                VisibilityModule::set_int64(boma, hash40("body") as i64,  hash40("body_hide") as i64);
            }
        frame(lua_state, 8.);
            if macros::is_excute(weapon)
            {
                VisibilityModule::set_int64(boma,  hash40("body") as i64,  hash40("body_show") as i64);
            }
        frame(lua_state, 9.);
            if macros::is_excute(weapon)
            {
                WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
            }
        frame(lua_state, 13.);
            if macros::is_excute(weapon)
            {
                macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 49, 150, 0, 60, 7.5, 0.0, 10.0, 7.0,  Some(0.0), Some(12.0), Some(29.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
                WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
            }
        wait(lua_state, 5.);
            if macros::is_excute(weapon)
            {
                AttackModule::clear_all(boma);
            }
        wait(lua_state, 40.);
            if macros::is_excute(weapon)
            {
                WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
            }
    }
}

#[acmd_script( agent = "bayonetta_wickedweavearm", script = "game_attacks4lw", category = ACMD_GAME, low_priority)]
unsafe fn attacks4lwwwa(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(weapon)
        {
            VisibilityModule::set_int64(boma,hash40("body") as i64,hash40("body_hide") as i64);
        }
    frame(lua_state, 8.);
        if macros::is_excute(weapon)
        {
            VisibilityModule::set_int64(boma,hash40("body") as i64,hash40("body_show") as i64);
        }
    frame(lua_state, 9.);
        if macros::is_excute(weapon)
        {
            WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        }
    frame(lua_state, 12.);
        if macros::is_excute(weapon)
        {
            LinkModule::unlink_all(boma);
        }
    frame(lua_state, 13.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 35, 5.5, 0.0, 14.0, 7.0, Some(0.0), Some(10.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 14.0, 361, 100, 0, 35, 5.5, 0.0, 14.0, 7.0, Some(0.0), Some(10.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 16.0, 361, 100, 0, 35, 8.0, 0.0, 7.0, 29.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 16.0, 361, 100, 0, 35, 8.0, 0.0, 7.0, 29.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(weapon, 4, 0, Hash40::new("top"), 14.0, 361, 100, 0, 35, 5.0, 0.0, 10.0, 10.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
        }
    wait(lua_state, 5.);
        if macros::is_excute(weapon)
        {
            AttackModule::clear_all(boma);
        }
    wait(lua_state, 40.);
        if macros::is_excute(weapon)
        {
            WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVEARM_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        }
}

#[acmd_script( agent = "bayonetta_wickedweaveleg", script = "game_attacklw4", category = ACMD_GAME, low_priority)]
unsafe fn attacklw4wwl(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(weapon)
        {
            VisibilityModule::set_int64(boma,hash40("body") as i64,hash40("body_hide") as i64);
        }
    frame(lua_state, 14.);
        if macros::is_excute(weapon)
        {
            VisibilityModule::set_int64(boma,hash40("body") as i64,hash40("body_show") as i64);
        }
    frame(lua_state, 15.);
        if macros::is_excute(weapon)
        {
            LinkModule::unlink_all(boma);
        }
    frame(lua_state, 16.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 16.0, 72, 90, 0, 10, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 16.0, 270, 80, 0, 10, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 15.0, 72, 90, 0, 10, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 15.0, 270, 80, 0, 10, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            WorkModule::on_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        }
    wait(lua_state, 2.);
        if macros::is_excute(weapon)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 41.);
        if macros::is_excute(weapon)
        {
            WorkModule::off_flag(boma, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_CANCEL_EFFECT);
        }
}

//ground
#[acmd_script( agent = "bayonetta", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, false, 10, 3, 10, 5, true);
            MotionModule::set_rate(boma, 1.6);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.1);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2, 361, 6, 0, 52, 3.0, 0.0, 10.0, 3.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 361, 6, 0, 40, 3.2, 0.0, 10.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 361, 6, 0, 52, 3.0, 0.0, 5.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 361, 6, 0, 30, 3.2, 0.0, 10.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 1.2, 361, 6, 0, 30, 3.2, 0.0, 10.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 8.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 8.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 8.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, 8.0, false);
            AttackModule::set_add_reaction_frame(boma, 4, 8.0, false);
            AttackModule::set_shield_stiff_mul(boma, 0.5); //makes it less likely to push fighters down platforms
        }
    frame(lua_state, 11.); //13
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 13.); //15
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, false, 10, 3, 10, 5, true);
            MotionModule::set_rate(boma, 1.6);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.1);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2, 361, 8, 0, 40, 3.0, 0.0, 10.8, 3.3, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 361, 8, 0, 40, 3.5, 0.0, 10.6, 7.2, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 361, 8, 0, 38, 4.0, 0.0, 10.3, 11.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 361, 8, 0, 38, 3.0, 0.0, 4.5, 6.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, 10.0, false);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 15.); //14
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attack13", category = ACMD_GAME, low_priority)]
unsafe fn jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
         if macros::is_excute(fighter)
         {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 10, 5, true);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, false, 10);
            MotionModule::set_rate(boma, 1.75); //0.6
         }
    frame(lua_state, 10.);
         if macros::is_excute(fighter)
         {
             MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2, 361, 6, 0, 50, 7.0, 0.0,7.0, 10.0, Some(0.0), Some(10.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 90, 6, 0, 40, 7.0, 0.0,7.0, 10.0, Some(0.0), Some(10.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
            AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
         }
    wait(lua_state, 2.);
         if macros::is_excute(fighter)
         {
             AttackModule::clear_all(boma);
         }
    frame(lua_state, 14.);
         if macros::is_excute(fighter)
         {
             MotionModule::set_rate(boma, 0.9);
             WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
             WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
             WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
         }
    frame(lua_state, 17.);
         if macros::is_excute(fighter)
         {
             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
         }
}

#[acmd_script( agent = "bayonetta", script = "game_attack100end", category = ACMD_GAME, low_priority)]
unsafe fn jab100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, true);
            smash_script::notify_event_msc_cmd!(fighter, 0x25fd66ecef as u64, 25, -1, 0);
            ArticleModule::generate_article(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, false, 0);
            ArticleModule::change_motion(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, Hash40::new("attack_s4_s"), false, 0.0);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 15, 5, true);
            MotionModule::set_rate(boma, 1.2);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
           macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 90, 100, 30, 0, 3.0, 0.0, 7.0, 28.0, Some(0.0), Some(7.0), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
           macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 90, 100, 20, 0, 4.5, 0.0, 11.0, 27.0, Some(0.0), Some(11.0), Some(11.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0,false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 75, 80, 0, 7.5, 0.0, 8.5, 18.0,  Some(0.0), Some(8.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            MotionModule::set_rate(boma, 1.0);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.375);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 57.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x25fd66ecef as u64, 30, 0, 1);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attacklw4", category = ACMD_GAME, low_priority)]
unsafe fn attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            if !WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK)
            {
                ArticleModule::generate_article(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, false, 0);
            }
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 3, 10, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, false, 10);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 100, 20, 0, 4.0, 0.0, 3.0, 7.0,  None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, 0);
            sv_animcmd::ATK_NO_REACTION_SEARCH_WITCH_TIME(fighter.lua_state_agent);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            AttackModule::set_power_mul_status(boma, 1.0);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 10, 5, true);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.75);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 368, 100, 145, 0, 3.0, 0.0, 6.0, 6.5, Some(0.0), Some(12.0), Some(6.5), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 368, 100, 100, 0, 3.0, 0.0, 6.0, 9.5, Some(0.0), Some(12.0), Some(9.5), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            let attack_pos = smash::phx::Vector2f {x: 0., y: 22.};
            AttackModule::set_vec_target_pos(boma, 0, Hash40{hash: hash40("top")}, &attack_pos, 6, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40{hash: hash40("top")}, &attack_pos, 6, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter){
            AttackModule::clear(boma, 1, false);
            MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 128, 27, 0, 65, 4.5, 0.0, 18.0, 7.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            let attack_pos = smash::phx::Vector2f {x: 0., y: 22.};
            AttackModule::set_vec_target_pos(boma, 0, Hash40{hash: hash40("top")}, &attack_pos, 6, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 6.0, 93, 60, 0, 55, 5.5, 0.0, 25.5, -2.0, Some(0.0), Some(25.5), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 6.0, 93, 60, 0, 55, 2.0, 0.0, 20.0, -4.0, Some(0.0), Some(20.0), Some(4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.1);
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
        frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10, 3, 10, 5, true);
            macros::FT_MOTION_RATE(fighter, 0.45);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 58, 15, 0, 55, 4.0, 0.0, 12.5, 8.5, Some(0.0), Some(12.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 45, 15, 0, 45, 4.0, 0.0, 12.5, 8.5, Some(0.0), Some(12.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 70, 15, 0, 50, 4.0, 0.0, 12.5, 16.5, Some(0.0), Some(12.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 70, 15, 0, 35, 4.0, 0.0, 12.5, 16.5, Some(0.0), Some(12.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 50, 15, 0, 60, 3.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            macros::FT_START_ADJUST_MOTION_FRAME_arg1(fighter, 0.85);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset_ftilt_num = 2;
            }
            else {
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
            }
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attacks32", category = ACMD_GAME, low_priority)]
unsafe fn attacks3s2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, true, false, false, 10, 3, 10, 5, true);
            macros::FT_MOTION_RATE(fighter, 0.6);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 48, 15, 0, 45, 6.5, 0.0, 13.5, 5.0, Some(0.0), Some(13.5), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 62, 15, 0, 50, 6.5, 0.0, 13.5, 13.0, Some(0.0), Some(13.5), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            macros::FT_START_ADJUST_MOTION_FRAME_arg1(fighter, 0.85);
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 5.0);
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset_ftilt_num = 3;
            }
            else {
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
            }
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_has_used_dodge_offset_2nd == false {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            }
            else {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            }
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 3, 15, 5, true);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, false, 10);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, false, 10);
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 6.5);
            macros::FT_MOTION_RATE(fighter, 0.5);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.8);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 55, 0, 70, 3.2, 0.0, 4.7, 1.0, Some(0.0), Some(4.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 78, 55, 0, 70, 3.2, 0.0, 4.7, 1.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter,0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 15, 5, true);
            //macros::FT_MOTION_RATE(fighter, 0.8);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            //macros::FT_MOTION_RATE(fighter, 1.4);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 71, 0, 78, 5.0, 0.0, 10.0, 10.5, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 50, 71, 0, 78, 1.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 50, 65, 0, 80, 4.5, 0.0, 10.0, 10.5, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 50, 65, 0, 80, 1.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.2);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 40, 30, 0, 70, 4.0, 0.0, 10.0, 10.5, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 40, 30, 0, 70, 1.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
}

//Air
#[acmd_script( agent = "bayonetta", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn airhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20, 3, 15, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
            MotionModule::set_rate(boma, 0.8);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.333);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.5, 55, 75, 0, 40, 5.5, 6.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.5, 55, 75, 0, 40, 4.2, 2.5, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 7.5, 55, 75, 0, 40, 3.5, -2.2, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 3.);
    if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.5, 75, 90, 0, 40, 5.5, 6.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.5, 75, 90, 0, 40, 4.2, 2.5, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 7.5, 75, 90, 0, 40, 3.5, -2.2, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.5, 55, 75, 0, 40, 5.5, 6.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.5, 55, 75, 0, 40, 4.2, 2.5, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 7.5, 55, 75, 0, 40, 3.5, -2.2, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_CHECK_HOLD);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, false);
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 34.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackairhihold", category = ACMD_GAME, low_priority)]
unsafe fn airhihold(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 3.0, 48, 80, 0, 75, 3.8, 4.5, 1.0, 2.0, Some(0.0), Some(1.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn airf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, true, 10, 3, 10, 0, true);
        }   
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 50, 10, 0, 70, 2.8, 0.0, 9.2, 3.2, Some(0.0), Some(9.2), Some(7.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 28, 10, 0, 60, 2.8, 0.0, 14.8, 3.2, Some(0.0), Some(14.8), Some(7.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.2, 70, 10, 0, 60, 5.8, 0.0, 12.0, 5.0, Some(0.0), Some(12.0), Some(11.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.2, 45, 10, 0, 50, 5.8, 0.0, 12.0, 5.0, Some(0.0), Some(12.0), Some(11.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            //MotionModule::set_rate(boma, 1.4) //try to set FAF at 35
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackairf2", category = ACMD_GAME, low_priority)]
unsafe fn airf2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
           smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, true, 10, 3, 10, 0, true);
            MotionModule::set_rate(boma, 1.33);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 56, 10, 0, 60, 3.7, 0.0, 8.5, 6.0, Some(0.0), Some(8.5), Some(11.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 44, 10, 0, 55, 3.7, 0.0, 15.5, 6.0, Some(0.0), Some(15.5), Some(11.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.2, 82, 10, 0, 55, 7.4, 0.0, 12.0, 8.0, Some(0.0), Some(12.0), Some(15.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.2, 60, 10, 0, 45, 7.4, 0.0, 12.0, 8.0, Some(0.0), Some(12.0), Some(15.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 2.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, 2.0, false);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 31.);
        if macros::is_excute(fighter)
        {
            //MotionModule::set_rate(boma, 1.333) // set FAF to 37
            WorkModule::off_flag(boma,*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackairf3", category = ACMD_GAME, low_priority)]
unsafe fn airf3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 10, 3, 10, 5, true);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 10);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 10);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10);
            MotionModule::set_rate(boma, 1.2);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 54, 76, 0, 60, 4.2, -5.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 54, 76, 0, 60, 6.2, 1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 6.0, 54, 76, 0, 60, 6.2, 6.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 34.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn airn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20, 0, 15, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
            MotionModule::set_rate(boma, 2.9);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 8.0, 361, 70, 0, 60,6.0, 3.0, 0.0, 1.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS, false, 0, 0.0, 0,false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.0, 361, 70, 0, 60,4.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS, false, 0, 0.0, 0,false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 31.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 6.0, 361, 60, 0, 60,5.8, 3.0, 0.0, 1.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS, false, 0, 0.0, 0,false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 361, 60, 0, 60,4.2, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_POS, false, 0, 0.0, 0,false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 39.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.75);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_CHECK_HOLD);
        }
    frame(lua_state, 40.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, false);
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 47.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 69.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn airb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 10, 3, 10, 5, true);
            MotionModule::set_rate(boma, 2.0);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 120, 0, 16, 4.6, 0.0, 13.0, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.5, 361, 120, 0, 12, 4.2, 0.0, 13.0, -11.7, Some(0.0), Some(11.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 38.);
        if macros::is_excute(fighter) 
        {                
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn airlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.433); //motion_rate 0.7
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_KEEP_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, 0, -4.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 7.0, 270, 100, 75, 0, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 7.0, 270, 100, 75, 0, 4.0, 6.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 7.0, 70, 120, 0, 30, 4.0, -7.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), 7.0, 290, 100, 75, 0, 4.0, -7.0, 1.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 4, 0, Hash40::new("kneel"), 7.0, 270, 100, 75, 0, 4.0, 0.0, 1.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 5, 0, Hash40::new("kneel"), 7.0, 270, 100, 75, 0, 4.0, 6.0, 1.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 31.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 270, 100, 70, 0, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 6.0, 270, 100, 70, 0, 4.0, 5.5, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 42.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.2);
            AttackModule::clear_all(boma);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    frame(lua_state, 48.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_LANDING_DISABLE_ATTACK);
        }
    frame(lua_state, 56.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            CancelModule::enable_cancel(boma);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_landingairlw", category = ACMD_GAME, low_priority)]
unsafe fn landingairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_LANDING_DISABLE_ATTACK as i32)
        {
                if macros::is_excute(fighter)
                {
                    macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, false, 10, 3, 3, 0, true);
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 45, 145, 0, 80, 7.5, 0.0, 8.0, 5.0, Some(0.0), Some(8.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                }
            wait(lua_state, 3.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
            wait(lua_state, 1.);
                if macros::is_excute(fighter)
                {
                    WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
                    WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
                    WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
                }
        }
}

//Specials
#[acmd_script( agent = "bayonetta", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.65); //was 1.433
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 92, 100, 130, 0, 3.0, 0.0, 4.0, 1.0, Some(0.0), Some(7.0), Some(1.0), 0.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 100, 100, 130, 0, 5.0, 0.0, 6.0, 6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 92, 100, 120, 0, 3.0, 0.0, 11.0, 1.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 100, 100, 120, 0, 5.0, 0.0, 9.5, 6.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.33);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 367, 90, 45, 0, 4.0, 0.0, 26.0, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 90, 90, 65, 0, 5.0, 0.0, 20.0, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.2, 90, 90, 130, 0, 3.5, 0.0, 14.0, 0.0, Some(0.0), Some(9.0), Some(0.0), 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 10.0, false);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter) {
            if !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE as i32)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 32, 50, 0, 45, 7.5, 0.0, 26.5, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
                AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
            }
            else
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 3, 50, 0, 30, 7.5, 0.0, 26.5, 0.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
                AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
            }
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, true);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL as i32);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.65); //was 1.433
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 92, 100, 130, 0, 3.0, 0.0, 4.0, 1.0, Some(0.0), Some(7.0), Some(1.0), 0.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 100, 100, 130, 0, 5.0, 0.0, 6.0, 7.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 92, 100, 120, 0, 3.0, 0.0, 11.0, 1.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 100, 100, 120, 0, 5.0, 0.0, 9.5, 7.0, None, None, None, 0.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.33);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 367, 90, 40, 0, 4.0, 0.0, 26.0, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.2, 90, 90, 65, 0, 5.0, 0.0, 20.0, 0.0, None, None, None, 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.2, 90, 90, 130, 0, 3.5, 0.0, 14.0, 0.0, Some(0.0), Some(9.0), Some(0.0), 0.8, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 10.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 10.0, false);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter) {
            if !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE as i32)
            {
                 macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 32, 50, 0, 45, 7.5, 0.0, 26.5, 0.5, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
                 AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
            }
            else
            {
                 macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 3, 50, 0, 30, 7.5, 0.0, 26.5, 0.5, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
                 AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
            }
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, true);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsu", category = ACMD_GAME, low_priority)]
unsafe fn specialairsu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
            macros::FT_MOTION_RATE(fighter, 0.5);
        }
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 5, 0, 20, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 5);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 40, 50, 0, 60, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 38, 50, 0, 60, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 1.0, false);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 65, 35, 0, 60, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 60, 35, 0, 60, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 1.0, false);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 83, 33, 0, 60, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 78, 33, 0, 60, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 1.0, false);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 100, 112, 60, 0, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 95, 112, 50, 0, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 1.0, false);
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, true);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsd", category = ACMD_GAME, low_priority)]
unsafe fn specialairsd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 68, 20, 0, 100, 4.5, 0.0, 2.0, 4.5, Some(0.0), Some(4.0), Some(3.2), 1.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.5, 80, 38, 0, 90, 4.5, 0.0, 2.0, 4.5, Some(0.0), Some(4.0), Some(3.2), 1.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            AttackModule::clear_all(boma);
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, true);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}


#[acmd_script( agent = "bayonetta", script = "game_specialsholdend", category = ACMD_GAME, low_priority)]
unsafe fn specialsholdend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 2.0);
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 2, 0, 0, 0, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 4.0, 75, 8, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.3, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 4.0, 80, 8, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.3, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
        }
    frame(lua_state, 34.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.33);
        }
    frame(lua_state, 37.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.0, 98, 65, 0, 55, 5.3, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.0, 98, 65, 0, 55, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("waist"), 4.0, 90, 65, 0, 55, 4.5, 0.0, -0.8, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 4.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 4.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 4.0, false);
        }
    frame(lua_state, 46.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsdlanding", category = ACMD_GAME, low_priority)]
unsafe fn specialairsdlanding(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 45, 50, 0, 80, 4.0, 0.0, 4.0, 7.0, Some(0.0), Some(4.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_speciallw", category = ACMD_GAME, low_priority)]
unsafe fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
            macros::SEARCH(fighter, 0, 0,Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_FIEB,*COLLISION_PART_MASK_BODY_HEAD, false);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
            smash_script::search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        }
    frame(lua_state, 44.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairlw", category = ACMD_GAME, low_priority)]
unsafe fn specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
            macros::SEARCH(fighter, 0, 0,Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_FIEB,*COLLISION_PART_MASK_BODY_HEAD, false);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
            smash_script::search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        }
    frame(lua_state, 36.);
        if !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME_SUCCESS)
        {
            if macros::is_excute(fighter)
            {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    frame(lua_state, 44.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        }
}

//grabs & throws
#[acmd_script( agent = "bayonetta", script = "game_throwf", category = ACMD_GAME, low_priority)]
unsafe fn throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x25fd66ecef as u64, 25, -1, 0);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 55, 100, 80, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F,  0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, false, 0);
            ArticleModule::change_motion(boma, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM, Hash40::new("attack_s4_s"), false, 0.0);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            //macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 55, 160, 0, 30, 6.0, 0.0, 10.0, 5.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
            AttackModule::set_catch_only_all(boma,true, false);
            macros::CHECK_FINISH_CAMERA(fighter, 14, 7);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 40.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x25fd66ecef as u64, 30, 0, 1);
        }
}

//Other
#[acmd_script( agent = "bayonetta", script = "game_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn escapeb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 2, 0, 0, 0, false);
                smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
            }
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            }
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, false);
            }
        }
}

#[acmd_script( agent = "bayonetta", script = "game_escapen", category = ACMD_GAME, low_priority)]
unsafe fn escapen(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 2, 0, 0, 0, false);
                //smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
            }
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            }
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, false);
            }
        }
}

#[acmd_script( agent = "bayonetta", script = "game_escapef", category = ACMD_GAME, low_priority)]
unsafe fn escapef(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 2, 0, 0, 0, false);
                smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
            }
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            }
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            macros::REVERSE_LR(fighter);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].bayo_dodge_offset {
                smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, false);
            }
        }
}

#[acmd_script( agent = "bayonetta", script = "game_escapeair", category = ACMD_GAME, low_priority)]
unsafe fn escapeair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
        smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
        smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        }
}

//other
#[acmd_script( agent = "bayonetta", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
        }
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

#[acmd_script( agent = "bayonetta", script = "game_win2", category = ACMD_GAME, low_priority)]
unsafe fn win2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 2, 0, 0, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        }
    frame(lua_state, 35.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, false);
        }
}

//effects
#[acmd_script( agent = "bayonetta", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            let effect_kind = WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind, Hash40::new("top"), 5, 14.5, 3, -49, 27, 86, 1.1, true);
            sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.0);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 11.0);
        if macros::is_excute(fighter)
        {
	        macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 27, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            if WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1, true, 0.059, 0.38, 1);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
            else 
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1, true, 1, 0.118, 0.118);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_beretta_rotation"), true, true);
        }
}

#[acmd_script( agent = "bayonetta", script = "effect_attackdash", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            if WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.059, 0.38, 1.0);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
            else 
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 1, 0.118, 0.118);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), true, true);
            //macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 7, 0.5, 0, 0, 0, 0, 1.4, true);
            let effect_kind = WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_STILETTO);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind, Hash40::new("armr"), 7, 0.5, 0, 0, 90, 0, 1.0, true);
            sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    // frame(lua_state, 22.);
    //     for _ in 0..2 {
    //             if macros::is_excute(fighter)
    //             {
	//                 macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
	//             }
    //         wait(lua_state, 1.);
    //             if macros::is_excute(fighter)
    //             {
	//                 macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
	//             }
    //         wait(lua_state, 3.);
    //     }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            if WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.059, 0.38, 1.0);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
            else 
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 1, 0.118, 0.118);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
        }
    frame(lua_state, 42.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), true, true);
        }
}

//looks ugly
#[acmd_script( agent = "bayonetta", script = "effect_attackairn", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            if PostureModule::lr(boma) == 1.0
            {
                let scale = Vector3f {x : 0.8, y : 1.0, z : 1.0};
                let rot = Vector3f  {x : 0., y : 0.5, z : 0.0};
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1), Hash40::new("top"), 0, 10, 0, 0, 30, 175, 1.2, true); //x, y, z, ?, ?, ?, ?, ?
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                let handle  = EffectModule::get_last_handle(boma) as u32;
                EffectModule::set_rot(boma, handle, &rot);
                //macros::LAST_EFFECT_SET_RATE(fighter, 1.25);
                EffectModule::set_scale_last(boma, &scale);
            }
            else
            {
                let scale = Vector3f {x : 0.8, y : 1.0, z : 1.0};
                let rot = Vector3f  {x : 0., y : -0.5, z : 0.0};
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1), Hash40::new("top"), 0, 10, 0, 0, 30, 175, 1.2, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                let handle  = EffectModule::get_last_handle(boma) as u32;
                EffectModule::set_rot(boma, handle, &rot);
                //macros::LAST_EFFECT_SET_RATE(fighter, 1.25);
                EffectModule::set_scale_last(boma, &scale);
            }
        }
    // frame(lua_state, 30.);
    //     if macros::is_excute(fighter)
    //     {
    //         //let scale = Vector3f {x : 0.66, y : 1.1, z : 0.9};
    //         fighter.clear_lua_stack();
    //         smash_script::lua_args!(fighter, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1), Hash40::new("top"), 0, 10, 0, 0, -140, 175, 1.2, true);
    //         sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
    //         macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    //         //EffectModule::set_scale_last(boma, &scale);
    //     }
}


pub fn install() {
    smashline::install_agent_frames!(
        bayo_frame
    );
    smashline::install_acmd_scripts!(
        attackhi4wwa,
        attacks4wwa,
        attacks4hiwwa, 
        attacks4lwwwa,
        attacklw4wwl,
        dash,
        turndash,
        jab1,
        jab2,
        jab3,
        jab100end,
        attackhi3,
        attacks3,
        attacks3s2,
        attacklw3,
        //attacklw4, currently bugged, bayo either can't charge or the ww never comes out
        attackdash,
        airhi,
        airhihold,
        airf,
        airf2,
        airf3,
        airn,
        airb,
        airlw,
        landingairlw,
        specialhi,
        specialairhi,
        specialairsu,
        specialairsd,
        specialsholdend,
        specialairsdlanding,
        speciallw,
        specialairlw,
        throwf,
        escapeb,
        escapen,
        escapef,
        escapeair,
        escapeairslide,
        effectattackhi3,
        effectattackdash,
        win2,
        //effectattackairn,
    );
}