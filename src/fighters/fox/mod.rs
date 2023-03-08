use smash::phx::Hash40;
//use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*};
use custom_var::*;

#[fighter_frame( agent = FIGHTER_KIND_FOX )]
fn space_furry_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

        //GA: The Melee Special tm
        // type: cancel
        //  on hit, cancel shine with jump!
        if !is_operation_cpu(fighter.module_accessor) {
            if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
                //check if the shine hits
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
            }
            //cleanup (status)
            else {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::set_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 0);
            }
            //actual cancel happens here. Fox has jumps as long as shine hits opponents. Cannot change motions while in hitstun (hopefully)
            if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && !is_hitlag(fighter.module_accessor) {
                VarModule::add_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 1);
                if check_jump_input(fighter.module_accessor) && VarModule::get_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER) <= 10 {
                    if situation_kind == SITUATION_KIND_GROUND {
                        //StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
                        
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                        fighter.sub_transition_group_check_ground_jump_mini_attack();
                        fighter.sub_transition_group_check_ground_jump();

                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                    else if situation_kind == SITUATION_KIND_AIR {
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
                        fighter.sub_transition_group_check_air_jump_aerial();
                        
                        //StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                }
                //cleanup (timer)
                if VarModule::get_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER) > 20 {
                    VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    VarModule::set_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 0);
                }
            }
        }
    }
}

//weapon
#[acmd_script( agent = "fox_blasterbullet", script = "game_fly", category = ACMD_GAME, low_priority)]
unsafe fn laserfly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 1.44, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_blaster_throw_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_BLASTER, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 4.);
        if macros::is_excute(weapon)
        {
            macros::ATK_POWER(weapon, 0, 2);
        }
}

//global edits
#[acmd_script( agent = "fox", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "fox", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//ground
#[acmd_script( agent = "fox", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.5);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 361, 35, 0, 20, 2.5, 0.0, 9.3, 6.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.8, 361, 33, 0, 20, 2.7, 0.0, 9.3, 8.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.8, 180, 20, 0, 15, 2.7, 0.0, 9.3, 12.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.8, 361, 20, 0, 15, 2.7, 0.0, 9.3, 12.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        }
}

#[acmd_script( agent = "fox", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 361, 80, 0, 40, 3.4, 4.6, -0.7, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "fox", script = "game_attacks3lw", category = ACMD_GAME, low_priority)]
unsafe fn attacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 361, 20, 0, 50, 3.4, 4.6, -0.7, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

//air
#[acmd_script( agent = "fox", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.8, 367, 100, 120, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.8, 367, 100, 120, 0, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 1.8, 75, 110, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 1.8, 75, 110, 0, 50, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            let vertspeed = smash::phx::Vector3f {x: 0.0, y: 0.5, z: 0.0}; // y 0.3
            KineticModule::add_speed(fighter.module_accessor, &vertspeed);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 1.3, 367, 100, 120, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 1.3, 367, 100, 120, 0, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 1.3, 75, 130, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), 1.3, 75, 130, 0, 50, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            let vertspeed = smash::phx::Vector3f {x: 0.0, y: 1.4, z: 0.0}; // y 1.3
            KineticModule::add_speed(fighter.module_accessor, &vertspeed);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.8, 367, 100, 120, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.8, 367, 100, 120, 0, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 1.8, 75, 130, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 1.8, 75, 130, 0, 50, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            let vertspeed = smash::phx::Vector3f {x: 0.0, y: 1.15, z: 0.0}; // y 0.9
            KineticModule::add_speed(fighter.module_accessor, &vertspeed);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 2.8, 367, 100, 120, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 2.8, 367, 100, 120, 0, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 2.8, 75, 130, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), 2.8, 75, 130, 0, 50, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 26.);
        if macros::is_excute(fighter)
        {
            let vertspeed = smash::phx::Vector3f {x: 0.0, y: 0.75, z: 0.0}; // y 0.5
            KineticModule::add_speed(fighter.module_accessor, &vertspeed);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.8, 70, 103, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 2.0,  1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.8, 70, 103, 0, 60, 6.5, 5.1, -0.8, 1.2, None, None, None, 2.0,  1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 4.8, 70, 103, 0, 60, 5.5, 0.0, 0.0, 0.0, None, None, None, 2.0,  1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_STATUS_ATTACK_AIR_FLAG_LANDING_DISABLE_ATTACK);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 45.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "fox", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 361, 100, 0, 10, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 361, 100, 0, 10, 4.0, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 361, 100, 0, 10, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 361, 100, 0, 10, 3.5, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 2.5)
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    wait(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "fox", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 30, 109, 0, 20, 4.6, 5.9, -0.9, 1.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 30, 109, 0, 20, 4.6, 0.9, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 361, 88, 0, 10, 4.6, 5.9, -0.9, 1.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 361, 88, 0, 10, 4.6, 0.9, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 14.);
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

#[acmd_script( agent = "fox", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    wait(lua_state, 1.);
        for _ in 0..5 {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 325, 100, 20, 0, 4.5, 0.0, 8.2, -0.5, Some(0.0), Some(9.0), Some(2.0), 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("toel"), 1.4, 110, 30, 0, 30, 4.5, -0.5, -0.5, 0.0, Some(-0.5), Some(-0.5), Some(0.0), 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("toel"), 1.4, 367, 100, 60, 0, 6.0, 0.5, -4.0, 0.0, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                }
            wait(lua_state, 2.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(fighter.module_accessor);
                }
            wait(lua_state, 1.);
        }
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 140, 0, 50, 6.0, 0.0, 9.0, -1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 140, 0, 50, 7.0, 0.0, 2.8, 3.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//specials
#[acmd_script( agent = "fox", scripts = ["game_specialairlwstart", "game_speciallwstart"], category = ACMD_GAME, low_priority)]
unsafe fn speciallwstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.75);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 100, 60, 0, 7.5, 0.0, 6.5, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 20, 100, 60, 0, 7.5, 0.0, 6.5, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
}

#[acmd_script( agent = "fox", scripts = ["game_speciallwend", "game_speciallwairend"], category = ACMD_GAME, low_priority)]
unsafe fn speciallwend(fighter: &mut L2CAgentBase) {

        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
}

#[acmd_script( agent = "fox", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        space_furry_frame
    );
    smashline::install_acmd_scripts!(
        laserfly,
        dash,
        turndash,
        attack11,
        attacks3lw,
        attacks3,
        attackairf,
        attackairn,
        attackairb,
        attackairlw,
        speciallwstart,
        speciallwend,
        escapeairslide
    );
}