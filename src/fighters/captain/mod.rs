use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;
use std::mem;

use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

mod effects;

#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if [hash40("special_air_lw"), hash40("special_lw")].contains(&curr_motion_kind) && fighter.global_table[MOTION_FRAME].get_i32() <= 4 {
            if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.0 && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::DO_ONCE) {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
                let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
                FighterKineticEnergyMotion::reverse_chara_dir(fighter_kinetic_energy_motion);
            }
        }
        else {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
        }
    }
}

//global edits
#[acmd_script( agent = "captain", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "captain", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

#[acmd_script( agent = "captain", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 25, 0, 20, 2.5, 0.0, 10.0, 6.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 180, 20, 0, 20, 3.0, 0.0, 10.0, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 361, 25, 0, 20, 2.8, 0.0, 10.0, 8.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 361, 20, 0, 20, 3.0, 0.0, 10.0, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        }
}

#[acmd_script( agent = "captain", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 25, 0, 20, 2.5, 0.0, 10.0, 6.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 25, 0, 20, 3.0, 0.0, 10.0, 10.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 361, 20, 0, 20, 3.6, 0.0, 10.0, 14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 10.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
}

#[acmd_script( agent = "captain", scripts = ["game_attacks4lw", "game_attacks4s", "game_attacks4hi"], category = ACMD_GAME, low_priority)]
unsafe fn attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 9.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    frame(lua_state, 12.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
    frame(lua_state, 15.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
    frame(lua_state, 20.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 19.0, 43, 94, 0, 36, 3.0, 0.0, 0.0, 0.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 19.0, 43, 94, 0, 36, 4.0, 0.0, 0.0, 0.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 19.0, 43, 94, 0, 36, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 12.0, 43, 94, 0, 36, 3.0, 0.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 43, 94, 0, 36, 4.0, 0.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 12.0, 43, 94, 0, 36, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
}

#[acmd_script( agent = "captain", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 9.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
    frame(lua_state, 22.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.2);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 10, 0, 30, 4.8, 0.0, 21.0, 1.5, Some(0.0), Some(0.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 80, 10, 0, 30, 4.8, 0.0, 28.0, 5.0, Some(0.0), Some(0.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,   *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 90, 30, 0, 80, 5.7, 0.0, 7.5, 7.5, Some(0.0), Some(16.0), Some(1.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            
            let vecwhatev = smash::phx::Vector2f {x: 0.0, y: 29.0}; // y 0.3
            AttackModule::set_vec_target_pos(fighter.module_accessor, 3, Hash40::new("top"), &vecwhatev, 7, false);
        }
    frame(lua_state, 24.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    frame(lua_state, 28.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.75);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 70, 72, 0, 70, 6.0, 0.0, 29.0, 5.5, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 70, 74, 0, 70, 5.5, 0.0, 21.0, 2.5, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 70, 76, 0, 70, 5.0, 0.0, 16.0, 0.5, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 30.0);
        if macros::is_excute(fighter)
        {
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

//aerials
#[acmd_script( agent = "captain", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 14.0);
        if macros::is_excute(fighter)
        {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CAPTAIN_STATUS_ATTACK_AIR_WORK_INT_CRITICAL_ATTACK_ID);
            macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 22.0, 32, 81, 0, 30, 3.75, 4.4, -0.2, -1.0, Some(4.4), Some(0.3), Some(1.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 80, 0, 35, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_ATTACK_AIR_WORK_FLAG_CRITICAL);
            macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 6.0, 361, 80, 0, 35, 4.7, 4.4, -0.2, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 80, 0, 35, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
        }
    frame(lua_state, 31.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 42.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//specials
#[acmd_script( agent = "captain", script = "game_specialsend", category = ACMD_GAME, low_priority)]
unsafe fn specialsend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.5);
            JostleModule::set_status(fighter.module_accessor, false);
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
        }
    frame(lua_state, 6.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.8);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 10.0, 85, 30, 0, 82, 6.0, 0.0, -2.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 85, 30, 0, 82, 5.5, 0.0, 9.0, 5.5, Some(0.0), Some(9.0), Some(12.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear(fighter.module_accessor, 1, false);
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 10.0, 85, 30, 0, 82, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.75);
            AttackModule::clear_all(fighter.module_accessor);
            JostleModule::set_status(fighter.module_accessor, true);
        }
}

#[acmd_script( agent = "captain", script = "game_specialn", category = ACMD_GAME, low_priority)]
unsafe fn specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.75);
        }
    frame(lua_state, 6.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
        }
    frame(lua_state, 53.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.66);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 367, 100, 80, 0, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 5.0, 367, 100, 80, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.0, 367, 100, 80, 0, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == FIGHTER_KIND_KIRBY {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 4.5, -3.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 4.0, 367, 80, 80,  0, 4.2, 5.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            }
        }
    wait(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            //VarModule::on_flag(fighter.battle_object, captain::status::flag::ALLOW_SPECIAL_N_CRITICAL);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 1, Hash40::new("armr"), 8.0, 361, 125, 0, 50, 4.5, -1.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 1, Hash40::new("shoulderr"), 8.0, 361, 125, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 1, Hash40::new("armr"), 8.0, 361, 125, 0, 50, 4.0, 4.2, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 3.0);
        if macros::is_excute(fighter)
        {
            //MotionModule::set_rate(fighter.module_accessor, 1.1);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "captain", script = "game_specialairn", category = ACMD_GAME, low_priority)]
unsafe fn specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::SET_SPEED_EX(fighter, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor), 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            MotionModule::set_rate(fighter.module_accessor, 1.75);
        }
    frame(lua_state, 6.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
            KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    frame(lua_state, 53.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.66);
            macros::SET_SPEED_EX(fighter, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor), 1.66, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 367, 100, 80, 0, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 5.0, 367, 100, 80, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.0, 367, 100, 80, 0, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == FIGHTER_KIND_KIRBY {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 2.5, -3.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            }
        }
    wait(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 1, Hash40::new("armr"), 8.0, 361, 125, 0, 50, 4.5, -1.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 1, Hash40::new("shoulderr"), 8.0, 361, 125, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 1, Hash40::new("armr"), 8.0, 361, 125, 0, 50, 4.0, 4.2, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 3.0);
        if macros::is_excute(fighter)
        {
            //MotionModule::set_rate(fighter.module_accessor, 1.1);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "captain", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
    effects::install();
    smashline::install_agent_frames!(
        captain_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attack11,
        attack12,
        attacks4,
        attackhi4,
        attackairf,
        specialsend,
        specialn,
        specialairn,
        escapeairslide,
    );
}
