use smash::phx::Hash40;
use smash::hash40;
use smash::lib::{lua_const::*, L2CValue};
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod specials;
mod status;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

pub unsafe fn check_add_charge(fighter: &mut L2CAgentBase) {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::add_int(fighter.battle_object, luigi::instance::int::ELEC_CHARGE, 1);
        luigi_charge_effect(fighter, 1.25);
    }
}

// #[fighter_frame( agent = FIGHTER_KIND_LUIGI )]
// fn fingerguns_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         let status_kind = StatusModule::status_kind(fighter.module_accessor);
//         let elec_charge = VarModule::get_int(fighter.battle_object, luigi::instance::int::ELEC_CHARGE);

//         //rewrite in status, charge bonus is set frame 1
//         if status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE {
//             if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && fighter.global_table[MOTION_FRAME].get_i32() < 14 {
//                 if elec_charge <= 4 {
//                     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_BONUS);
//                 }
//             }
//             else { //prevents from setting the flag to true if the player smash inputs
//                 WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_BONUS);
//             }
//         }
//         if status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM {
//             if elec_charge != 0 && VarModule::is_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_CHARGE_USED) {
//                 galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
//             }
//         }
//     }
// }


//weapons
#[acmd_script( agent = "luigi_plunger", script = "game_aircatch", category = ACMD_GAME, low_priority )]
unsafe fn game_plunger_aircatch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent)
    {
        macros::ATTACK(agent, 0, 0, Hash40::new("plungerrope26"), 5.0, 361, 100, 0, 10, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
}
#[acmd_script( agent = "luigi_fireball", script = "game_regular", category = ACMD_GAME, low_priority)]
unsafe fn fireballregular(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 361, 22, 0, 12, 2.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            AttackModule::set_add_reaction_frame_revised(weapon.module_accessor, 0, 5.0, false);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
}

//global edits
#[acmd_script( agent = "luigi", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "luigi", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "luigi", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 2.0, 0.0, 6.6, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 2.5, 0.0, 6.6, 9.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 20, 0, 20, 2.5, 0.0, 6.6, 12.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 2.5, 0.0, 6.6, 12.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "luigi", script = "game_attacks3hi", category = ACMD_GAME, low_priority)]
unsafe fn attacks3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.0, 361, 75, 0, 50, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 9.0, 361, 75, 0, 50, 3.8, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "luigi", script = "game_attacks3s", category = ACMD_GAME, low_priority)]
unsafe fn attacks3s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.0, 361, 75, 0, 50, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 9.0, 361, 75, 0, 50, 3.8, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "luigi", script = "game_attacks3lw", category = ACMD_GAME, low_priority)]
unsafe fn attacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.0, 361, 75, 0, 50, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 9.0, 361, 75, 0, 50, 3.8, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "luigi", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 0, 5, 0, 50, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 0, 5, 0, 60, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 128, 0, 60, 5.0, 0.0, 8.0, 12.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 128, 0, 60, 4.0, 0.0, 8.0, 5.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUIGI_ATTACKDASH, *ATTACK_REGION_PUNCH);
}
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "luigi", script = "game_attacks4hi", category = ACMD_GAME, low_priority)]
unsafe fn attacks4hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 15.0, 52, 131, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 0.67, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 15.0, 52, 131, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 0.67, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            check_add_charge(fighter);
        }
}

#[acmd_script( agent = "luigi", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn attacks4s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 15.0, 42, 121, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 0.67, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 15.0, 42, 121, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 0.67, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            check_add_charge(fighter);
        }
}

#[acmd_script( agent = "luigi", script = "game_attacks4lw", category = ACMD_GAME, low_priority)]
unsafe fn attacks4lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 15.0, 37, 116, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 0.67, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 15.0, 37, 116, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 0.67, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            check_add_charge(fighter);
        }
}

#[acmd_script( agent = "luigi", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            JostleModule::set_status(fighter.module_accessor, false);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("snout"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 14.0, 110, 102, 0, 35, 4.8, 2.8, -1.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.0, 110, 102, 0, 35, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, true);
        }
}

//air
#[acmd_script( agent = "luigi", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn attackairhi(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 11.0, 65, 95, 0, 20, 4.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 11.0, 65, 95, 0, 20, 4.0, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent)
        {
            macros::ATK_POWER(agent, 0, 7);
            macros::ATK_POWER(agent, 1, 7);
        }
    frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent)
        {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "luigi", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn attackairf(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 8.0, 30, 80, 0, 30, 5.6, 3.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 8.0, 30, 80, 0, 30, 3.84, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent)
        {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "luigi", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn attackairn(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent)
        {
            //if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            //    macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 12.0, 90, 90, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            //    macros::ATTACK(agent, 1, 0, Hash40::new("head"), 12.0, 90, 90, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            //    macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 12.0, 90, 90, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            //}
            //else {
                macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 12.0, 361, 90, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(agent, 1, 0, Hash40::new("head"), 12.0, 361, 90, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 12.0, 361, 90, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            //}
        }
    wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent)
        {
            //if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 6.0, 80, 100, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(agent, 1, 0, Hash40::new("head"), 6.0, 80, 100, 0, 20, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 6.0, 80, 100, 0, 20, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            //}
            //else {
            //    macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 6.0, 361, 100, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            //    macros::ATTACK(agent, 1, 0, Hash40::new("head"), 6.0, 361, 100, 0, 20, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            //    macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 6.0, 361, 100, 0, 20, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            //}
        }
    frame(agent.lua_state_agent, 32.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 36.0);
        if macros::is_excute(agent)
        {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "luigi", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn attackairlw(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 270, 80, 0, 30, 3.0, 1.0, 1.0, 2.8, Some(-1.0), Some(1.0), Some(2.8), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 80, 0, 30, 3.0, 1.0, 1.0, 2.8, Some(-1.0), Some(1.0), Some(2.8), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 8.0, 361, 100, 0, 20, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 8.0, 361, 100, 0, 20, 5.2, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent)
        {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//throws
#[acmd_script( agent = "luigi", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn throwhi(agent: &mut L2CAgentBase) {

        if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU)
        {
            if macros::is_excute(agent)
            {
                ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_hi"), false, -1.0);
            }
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 72, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("top"), 6.0, 88, 55, 0, 95, 8.5, 0.0, 8.0, 10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
            AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        }
    frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent)
        {
            macros::CHECK_FINISH_CAMERA(agent, -2, 24);
            FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
            FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), smash::phx::Vector3f{x: 0.0, y: 7.0, z: 0.0});
        }
    frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent)
        {
            let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        }
    frame(agent.lua_state_agent, 19.);
        if macros::is_excute(agent)
        {
            check_add_charge(agent);
        }
}

#[acmd_script( agent = "luigi", script = "game_throwf", category = ACMD_GAME, low_priority)]
unsafe fn throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

            if macros::is_excute(fighter)
            {
                if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40{hash: hash40("throw_f")}, false, 0.0);
                }
                macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 65, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
                macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        frame(lua_state, 12.);
            if macros::is_excute(fighter) 
            {
                macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 30, 0, 90, 8.0, 0.0, 7.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
            }
        frame(lua_state, 14.);
            if macros::is_excute(fighter)
            {
                FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
                FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), smash::phx::Vector3f{x: 12., y: -1., z: 0.});
                macros::CHECK_FINISH_CAMERA(fighter,18, 4);
                //macros::FT_CATCH_STOP(fighter,5, 1);
            }
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
            }
}

// #[acmd_script( agent = "luigi", script = "game_throwfl", category = ACMD_GAME, low_priority)]
// unsafe fn throwfl(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//             if macros::is_excute(fighter)
//             {
//                 if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
//                     ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40{hash: hash40("throw_f")}, false, 0.0);
//                 }
//                 macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 65, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
//                 macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
//             }
//         frame(lua_state, 12.);
//             if macros::is_excute(fighter) 
//             {
//                 macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 30, 0, 90, 8.0, 0.0, 7.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
//             }
//         frame(lua_state, 14.);
//             if macros::is_excute(fighter)
//             {
//                 FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
//                 FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), smash::phx::Vector3f{x: 12., y: -1., z: 0.});
//                 macros::CHECK_FINISH_CAMERA(fighter,18, 4);
//                 //macros::FT_CATCH_STOP(fighter,5, 1);
//             }
//         frame(lua_state, 15.);
//             if macros::is_excute(fighter)
//             {
//                 AttackModule::clear_all(fighter.module_accessor);
//                 macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
//             }
// }

#[acmd_script( agent = "luigi", script = "game_throwb", category = ACMD_GAME, low_priority)]
unsafe fn throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40{hash: hash40("throw_b")}, false, 0.0);
            }
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter) 
        {
            macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 30, 0, 90, 8.0, 0.0, 7.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
            FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
            FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), smash::phx::Vector3f{x: 10., y: -1., z: 0.});
            macros::CHECK_FINISH_CAMERA(fighter, 21, 3);
            //FT_CATCH_STOP(5, 1);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            macros::REVERSE_LR(fighter);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
}

// #[acmd_script( agent = "luigi", script = "game_throwbl", category = ACMD_GAME, low_priority)]
// unsafe fn throwbl(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//         if macros::is_excute(fighter)
//         {
//             if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
//                 ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40{hash: hash40("throw_b")}, false, 0.0);
//             }
//             macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
//             macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
//         }
//     frame(lua_state, 12.);
//         if macros::is_excute(fighter) 
//         {
//             macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 30, 0, 90, 8.0, 0.0, 7.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
//         }
//     frame(lua_state, 14.);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
//             FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
//             FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), smash::phx::Vector3f{x: 10., y: -1., z: 0.});
//             macros::CHECK_FINISH_CAMERA(fighter, 21, 3);
//             //FT_CATCH_STOP(5, 1);
//         }
//     frame(lua_state, 15.);
//         if macros::is_excute(fighter)
//         {
//             AttackModule::clear_all(fighter.module_accessor);
//             macros::REVERSE_LR(fighter);
//             macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
//         }
// }

//other
#[acmd_script( agent = "luigi", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
    specials::install();
    status::install();
    smashline::install_acmd_scripts!(
        game_plunger_aircatch,
        fireballregular,
        dash,
        turndash,
        attack11,
        attacks3hi,
        attacks3s,
        attacks3lw,
        attackdash,
        attacks4hi,
        attacks4s,
        attacks4lw,
        attackhi4,
        attackairhi,
        attackairf,
        attackairn,
        attackairlw,
        throwhi,
        throwf,
        //throwfl,
        throwb,
        //throwbl,
        escapeairslide
    );
}