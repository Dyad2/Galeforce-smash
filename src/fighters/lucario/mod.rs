use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::{sv_animcmd::*, lua_bind::*, sv_module_access::*};
use smashline::*;
use smash_script::*;
use std::mem;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*};
use custom_var::*;

static mut STICK_DIR : [i32; 9] = [0; 9];
static mut COMMAND_FRAME : [i32; 9] = [0; 9];

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
fn lucario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        //tail 3 intangibility
        if [hash40("attackhi3"), hash40("attacklw3"), hash40("attacks4"), 
          hash40("attackairhi"), hash40("attackairb"), hash40("attackairn"), hash40("attackairf"), hash40("attackairlw"),
          hash40("specialnstart"), hash40("specialnhold"), hash40("specialnmax"), hash40("specialnshoot"),
          hash40("specialairnstart"), hash40("specialairnhold"), hash40("specialairnmax"), hash40("specialairnshoot")].contains(&curr_motion_kind) 
          || curr_motion_kind == hash40("speciallw") && MotionModule::frame(fighter.module_accessor) < 4. && MotionModule::frame(fighter.module_accessor) > 10. {
            macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_NORMAL);
        }

        //charge input dair
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR 
        && (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind)) {
            STICK_DIR[entry_id as usize] = get_stick_dir(&mut *fighter.module_accessor);
            if [1,2,3].contains(&STICK_DIR[entry_id as usize]) {
                COMMAND_FRAME[entry_id as usize] = COMMAND_FRAME[entry_id as usize] +1;
            }
            else {
                COMMAND_FRAME[entry_id as usize] = 0;
                VarModule::off_flag(fighter.battle_object,  lucario::instance::flag::ATTACK_AIR_LW_CHARGED);
            }
            if COMMAND_FRAME[entry_id as usize] > 10 {
                VarModule::on_flag(fighter.battle_object, lucario::instance::flag::ATTACK_AIR_LW_CHARGED);
            }
        }
        else {
            COMMAND_FRAME[entry_id as usize] = 0;
            VarModule::off_flag(fighter.battle_object, lucario::instance::flag::ATTACK_AIR_LW_CHARGED);
        }

        //aura charge taunt
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") {
            if MotionModule::frame(fighter.module_accessor) >= 75. && MotionModule::frame(fighter.module_accessor) <= 76. {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") {
            if MotionModule::frame(fighter.module_accessor) >= 35. && MotionModule::frame(fighter.module_accessor) <= 40. {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    DamageModule::add_damage(fighter.module_accessor, 5.0, 0);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40{hash: hash40("appeal_hi_r")}, 17.0, 1.1, 0.0, true, false);
                }
                if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40{hash: hash40("appeal_hi_r")}, 81.0, 1.0, 0.0, true, false);
                }
            }
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") {
            if MotionModule::frame(fighter.module_accessor) >= 35. && MotionModule::frame(fighter.module_accessor) <= 40. {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    DamageModule::add_damage(fighter.module_accessor, 5.0, 0);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40{hash: hash40("appeal_hi_l")}, 17.0, 1.1, 0.0, true, false);
                }
                if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40{hash: hash40("appeal_hi_l")}, 81.0, 1.0, 0.0, true, false);
                }
            }
        }

        //rewriting aura. this is necessary because the params needed for the GA aren't read mid match :(
        // if WorkModule::get_int(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_MAX_AURA_TIMER) {
        //     if DamageModule::damage(fighter.module_accessor, 0) <= 65.0 {
        //         WorkModule::set_float(fighter.module_accessor, 1.0, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE); //If your damage is 65 or less, set your power multiplier to 1.0
        //     }
        //     if DamageModule::damage(fighter.module_accessor, 0) > 65.0 { //IF YOUR DAMAGE IS BIGGER THAN 65
        //         WorkModule::set_float(fighter.module_accessor, (100.0 + 0.38824 * (DamageModule::damage(fighter.module_accessor, 0) - 65.0)) / 100.0, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE) //Applies the 65%+ Lucario formula to attack power 
        //     }
        //     if WorkModule::get_float(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE) > 1.33 {
        //         WorkModule::set_float(fighter.module_accessor, 1.33, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE); //set max aura. reached at 150%
        //     }
        // }

        //GA - Aura Burst
        // type : buff
        //  hit someone with charged up taunt hitbox to gain maximum aura for a short time
        if !is_operation_cpu(fighter.module_accessor) {
            if VarModule::get_int(fighter.battle_object, lucario::instance::int::MAX_AURA_TIMER) > 0 {
                //WorkModule::set_float(fighter.module_accessor, 1.33, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE);
                if DamageModule::damage(fighter.module_accessor, 0) < 150.0 {
                    AttackModule::set_power_mul(fighter.module_accessor, (1.33 - (100.0 + 0.22 * DamageModule::damage(fighter.module_accessor, 0)) / 100.0) + 1.0);
                }
                else {
                    AttackModule::set_power_mul(fighter.module_accessor, 1.0);
                }
                VarModule::sub_int(fighter.battle_object, lucario::instance::int::MAX_AURA_TIMER, 1);
    
                if VarModule::get_int(fighter.battle_object, lucario::instance::int::MAX_AURA_TIMER) % 20 == 1 {
                    let pos = Vector3f  {x : 0., y : 3.5, z : 0.};
                    let rot = Vector3f  {x : 0., y : 0., z : 0.};
    
                    let handle = EffectModule::req_follow(fighter.module_accessor,
                        smash::phx::Hash40{hash: hash40("lucario_aura")},
                        smash::phx::Hash40{hash: hash40("top")}, 
                        &pos, &rot, 0.4, false, 0, 
                        0, 0, 0, 0, false, false) as u32;
    
                    EffectModule::set_rgb(fighter.module_accessor, handle, 5., 0., 2.);
                }
            }
            
            //visual effect
            if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                VarModule::set_int(fighter.battle_object, lucario::instance::int::MAX_AURA_TIMER, 480);
            }
        }

        //set aura
        // WorkModule::set_float(fighter.module_accessor,  WorkModule::get_float(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE), *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
        // WorkModule::set_float(fighter.module_accessor,  WorkModule::get_float(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE), *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_EFFECT_SCALE);
        // WorkModule::set_float(fighter.module_accessor,  WorkModule::get_float(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE), *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_DEBUG_AURAPOWER);
        // WorkModule::set_float(fighter.module_accessor,  WorkModule::get_float(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE), *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_PREV_AURAPOWER);
        // WorkModule::set_float(fighter.module_accessor,  WorkModule::get_float(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE), *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER);
        // WorkModule::set_float(fighter.module_accessor,  WorkModule::get_float(fighter.module_accessor, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE), *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER);
    }
}

//global edits
#[acmd_script( agent = "lucario", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "lucario", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "lucario", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(lua_state, 4.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(lua_state, 6.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 2.5, 361, 15, 0, 45, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 361, 15, 0, 25, 2.7, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 2.5, 180, 15, 0, 15, 2.7, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("arml"), 2.5, 361, 15, 0, 15, 2.7, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 6.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 6.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 6.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 6.0, false);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 0.75);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 11.0);
    if macros::is_excute(fighter)
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 29.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "lucario", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 15, 0, 45, 4.0, 0.0, 8.8, 8.0, Some(0.0), Some(8.8), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 6.0, false);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 0.75);
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 35.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "lucario", script = "game_attack13", category = ACMD_GAME, low_priority)]
unsafe fn attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));


    frame(lua_state, 9.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 65, 125, 0, 50, 5.5, 0.0, 8.0, 4.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 3.5, 65, 125, 0, 50, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 0.89);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 30.0);
    if macros::is_excute(fighter)
    {
        FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.0);
    }
    frame(lua_state, 35.0);
    if macros::is_excute(fighter)
    {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.0);
    if macros::is_excute(fighter)
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 19.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 106, 0, 40, 5.7, 0.0, 8.0, 11.7, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 361, 106, 0, 40, 3.3, 0.0, 8.0, 18.2, Some(0.0), Some(8.0), Some(6.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//air
#[acmd_script( agent = "lucario", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, false);
        }
    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 10.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 6.0, 361, 20, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 6.0, 65, 20, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 6.0, 361, 20, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 6.0, 65, 20, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
    frame(lua_state, 18.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 8.0, 361, 100, 0, 20, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 100, 0, 20, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            JostleModule::set_status(fighter.module_accessor, true);
        }
    frame(lua_state, 38.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "lucario", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.09);
        }
    frame(lua_state, 4.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 12.0);
        if macros::is_excute(fighter)
        {   
            macros::FT_MOTION_RATE(fighter, 1.);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 100, 0, 30, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 11.0, -9.0, Some(0.0), Some(11.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 40.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "lucario", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if !VarModule::is_flag(fighter.battle_object, lucario::instance::flag::ATTACK_AIR_LW_CHARGED) 
    {
            if macros::is_excute(fighter)
            {
                FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 3.0, 3.0, 8.0, 2.0);
            }
        frame(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                macros::SET_SPEED_EX(fighter, 0, WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW_SPEED), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_ATTACK_AIR_STATUS_WORK_ID_FLAG_DEC_SPEED);
                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 4.0);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 100, 30, 0, 20, 3.8, 0.0, -2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 50, 0, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 150, 100, 25, 0, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 11.0);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 100, 0, 60, 5.8, 0.0, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 100, 0, 60, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 25.0);
            if macros::is_excute(fighter)
            {
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
    }
    else {
            if macros::is_excute(fighter)
            {
                FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 3.0, 3.0, 8.0, 2.0);
            }
        frame(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                //macros::SET_SPEED_EX(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW_SPEED), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_ATTACK_AIR_STATUS_WORK_ID_FLAG_DEC_SPEED);
                //KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 4.0);
            if macros::is_excute(fighter)
            {
                //WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 367, 30, 0, 20, 3.8, 0.0, -2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 50, 0, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 367, 100, 25, 0, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
            }
        wait(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                MotionModule::set_rate(fighter.module_accessor, 0.4);
            }
        frame(lua_state, 11.0);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.0);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 270, 60, 0, 50, 5.8, 0.0, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 270, 60, 0, 50, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            }
        wait(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 25.0);
            if macros::is_excute(fighter)
            {
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
                VarModule::off_flag(fighter.battle_object, lucario::instance::flag::ATTACK_AIR_LW_CHARGED);
            }
    }
}

//specials
#[acmd_script( agent = "lucario", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority)]
unsafe fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_LUCARIO_SHIELD_KIND_SPLIT, *FIGHTER_LUCARIO_SHIELD_GROUP_KIND_SPLIT);
            shield(fighter.lua_state_agent);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
    frame(lua_state, 25.0);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_LUCARIO_SHIELD_KIND_SPLIT, *FIGHTER_LUCARIO_SHIELD_GROUP_KIND_SPLIT);
            shield(fighter.lua_state_agent);
            MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
}

//others
#[acmd_script( agent = "lucario", script = "game_appealhil", category = ACMD_GAME, low_priority)]
unsafe fn auraburst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.4);
        }
    frame(lua_state, 83.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 20, 0, 80, 12.5, 0.0, 12.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_AURA);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
}

#[acmd_script( agent = "lucario", script = "game_appealhir", category = ACMD_GAME, low_priority)]
unsafe fn auraburstr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.4);
        }
    frame(lua_state, 83.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 20, 0, 80, 12.5, 0.0, 12.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_AURA);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
}

//other
#[acmd_script( agent = "lucario", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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

//sound
#[acmd_script( agent = "lucario", script = "sound_appealhil", category = ACMD_SOUND, low_priority)]
unsafe fn appealhilsound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.0);
    if macros::is_excute(fighter)
    {
        macros::PLAY_SE(fighter, Hash40::new("se_lucario_appeal_h01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucario_appeal01"));
    }
}

#[acmd_script( agent = "lucario", script = "sound_appealhir", category = ACMD_SOUND, low_priority)]
unsafe fn appealhirsound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.0);
    if macros::is_excute(fighter)
    {
        macros::PLAY_SE(fighter, Hash40::new("se_lucario_appeal_h01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucario_appeal01"));
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        lucario_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attack11,
        attack12,
        attack13,
        attacks4,
        attackairn,
        attackairb,
        attackairlw,
        speciallw,
        auraburst,
        auraburstr,
        escapeairslide,
        appealhilsound,
        appealhirsound,
    );
}