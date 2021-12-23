use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;
use std::mem;

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;
use crate::utils::*;

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
fn lucario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        //tail 3 intangibility
        if [hash40("attackhi3"), hash40("attacklw3"), hash40("attacks4"), 
          hash40("attackairhi"), hash40("attackairb"), hash40("attackairn"), hash40("attackairf"), hash40("attackairlw"),
          hash40("specialnstart"), hash40("specialnhold"), hash40("specialnmax"), hash40("specialnshoot"),
          hash40("specialairnstart"), hash40("specialairnhold"), hash40("specialairnmax"), hash40("specialairnshoot")].contains(&curr_motion_kind) 
          || curr_motion_kind == hash40("speciallw") && MotionModule::frame(boma) < 4. && MotionModule::frame(boma) > 10. {
            macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_NORMAL);
        }

        if MotionModule::motion_kind(boma) == hash40("appeal_hi_r") || MotionModule::motion_kind(boma) == hash40("appeal_hi_l") {
            if MotionModule::frame(boma) >= 75. && MotionModule::frame(boma) <= 76. && !FIGHTER_GLOBALS[entry_id as usize].lucario_aura_charge {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }

        if MotionModule::motion_kind(boma) == hash40("appeal_hi_r") {
            //println!("frame: {}", MotionModule::frame(boma));
            if MotionModule::frame(boma) >= 35. && MotionModule::frame(boma) <= 40. {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    DamageModule::add_damage(boma, 5.0, 0);
                    FIGHTER_GLOBALS[entry_id as usize].lucario_aura_charge = true;
                    MotionModule::change_motion_inherit_frame(boma, Hash40{hash: hash40("appeal_hi_r")}, 17.0, 1.1, 0.0, true, false);
                }
                if FIGHTER_GLOBALS[entry_id as usize].lucario_aura_charge && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    FIGHTER_GLOBALS[entry_id as usize].lucario_aura_charge = false;
                    MotionModule::change_motion_inherit_frame(boma, Hash40{hash: hash40("appeal_hi_r")}, 81.0, 1.0, 0.0, true, false);
                }
            }
        }
        if MotionModule::motion_kind(boma) == hash40("appeal_hi_l") {
            //println!("frame: {}", MotionModule::frame(boma));
            if MotionModule::frame(boma) >= 35. && MotionModule::frame(boma) <= 40. {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    DamageModule::add_damage(boma, 5.0, 0);
                    FIGHTER_GLOBALS[entry_id as usize].lucario_aura_charge = true;
                    MotionModule::change_motion_inherit_frame(boma, Hash40{hash: hash40("appeal_hi_l")}, 17.0, 1.1, 0.0, true, false);
                }
                if FIGHTER_GLOBALS[entry_id as usize].lucario_aura_charge && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    FIGHTER_GLOBALS[entry_id as usize].lucario_aura_charge = false;
                    MotionModule::change_motion_inherit_frame(boma, Hash40{hash: hash40("appeal_hi_l")}, 81.0, 1.0, 0.0, true, false);
                }
            }
        }

        //rewriting aura. this is necessary because the params needed for the GA aren't read mid match :(
        if DamageModule::damage(boma, 0) <= 65.0 {
            FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale = 1.0;//If your damage is 65 or less, set your power multiplier to 1.0
        }
        if DamageModule::damage(boma, 0) > 65.0 { //IF YOUR DAMAGE IS BIGGER THAN 65
            FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale = (100.0 + 0.38824 * (DamageModule::damage(boma, 0) - 65.0)) / 100.0; //Applies the 65%+ Lucario formula to attack power 
        }
        if FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale > 1.33 {
            FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale = 1.33; //set max aura. reached at 150%
        }

        //GA - Aura Burst
        // type : buff
        //  hit someone with charged up taunt hitbox to gain maximum aura for a short time
        if !is_operation_cpu(boma) {
            if FIGHTER_GLOBALS[entry_id as usize].lucario_aura_max_timer > 0 {
                FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale = 1.33;
                FIGHTER_GLOBALS[entry_id as usize].lucario_aura_max_timer -= 1;
    
                if FIGHTER_GLOBALS[entry_id as usize].lucario_aura_max_timer % 20 == 1 {
                    FIGHTER_GLOBALS[entry_id as usize].once = true;
                    let pos = Vector3f  {x : 0., y : 3.5, z : 0.};
                    let rot = Vector3f  {x : 0., y : 0., z : 0.};
    
                    let handle = EffectModule::req_follow(boma,
                        smash::phx::Hash40{hash: hash40("lucario_aura")},
                        smash::phx::Hash40{hash: hash40("top")}, 
                        &pos, &rot, 0.4, false, 0, 
                        0, 0, 0, 0, false, false) as u32;
    
                    EffectModule::set_rgb(boma, handle, 5., 0., 2.);
                }
            }
            
            //visual effect
            if FIGHTER_GLOBALS[entry_id as usize].ga_on {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                galeforce_apply_effect(boma, 0.5);
                FIGHTER_GLOBALS[entry_id as usize].lucario_aura_max_timer = 480;
            }
        }

        //set aura
        //println!("lucario_aura: {}", FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale);
        WorkModule::set_float(boma, FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
        WorkModule::set_float(boma, FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_EFFECT_SCALE);
        //AttackModule::set_power_mul(boma, FIGHTER_GLOBALS[entry_id as usize].lucario_aura_scale); //set max aura. reached at 150%
    }
}

//global edits
#[acmd_script( agent = "lucario", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "lucario", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "lucario", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 4.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(boma, 1.0);
    }
    frame(lua_state, 6.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 2.5, 361, 20, 0, 45, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 361, 20, 0, 25, 2.7, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 2.5, 180, 20, 0, 15, 2.7, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("arml"), 2.5, 361, 20, 0, 15, 2.7, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 6.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 6.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 6.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 6.0, false);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 0.75);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 11.0);
    if macros::is_excute(fighter)
    {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
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
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 5.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 20, 0, 45, 4.0, 0.0, 8.8, 8.0, Some(0.0), Some(8.8), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 6.0, false);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 0.75);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
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
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));


    frame(lua_state, 9.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 65, 120, 0, 50, 5.5, 0.0, 8.0, 4.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 3.0, 65, 120, 0, 50, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 0.89);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if macros::is_excute(fighter)
    {
        FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.0);
    }
    frame(lua_state, 35.0);
    if macros::is_excute(fighter)
    {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(boma);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));


    frame(lua_state, 4.0);
    if macros::is_excute(fighter)
    {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
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
        AttackModule::clear_all(boma);
    }
}

//air
#[acmd_script( agent = "lucario", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
        }
    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
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
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 1.5);
        }
    frame(lua_state, 18.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 8.0, 361, 100, 0, 20, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 100, 0, 20, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            JostleModule::set_status(boma, true);
        }
    frame(lua_state, 38.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "lucario", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.09);
        }
    frame(lua_state, 4.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
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
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 40.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//others
#[acmd_script( agent = "lucario", script = "game_appealhil", category = ACMD_GAME, low_priority)]
unsafe fn auraburst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.4);
        }
    frame(lua_state, 83.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 20, 0, 80, 12.5, 0.0, 12.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_AURA);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = true;
                CancelModule::enable_cancel(boma);
            }
        }
}

#[acmd_script( agent = "lucario", script = "game_appealhir", category = ACMD_GAME, low_priority)]
unsafe fn auraburstr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.4);
        }
    frame(lua_state, 83.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 20, 0, 80, 12.5, 0.0, 12.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_AURA);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = true;
                CancelModule::enable_cancel(boma);
            }
        }
}

//other
#[acmd_script( agent = "lucario", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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

//sound
#[acmd_script( agent = "lucario", script = "sound_appealhil", category = ACMD_SOUND, low_priority)]
unsafe fn appealhilsound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

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
    let boma = sv_system::battle_object_module_accessor(lua_state);

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
        auraburst,
        auraburstr,
        escapeairslide,
        appealhilsound,
        appealhirsound,
    );
}