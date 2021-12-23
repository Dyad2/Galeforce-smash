use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash::app::{BattleObject, sv_animcmd::*, sv_system, sv_battle_object, lua_bind::*};
use smashline::*;
use smash_script::*;
use std::mem;

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;

#[fighter_frame( agent = FIGHTER_KIND_GEKKOUGA )]
fn greninja_not_amphinobi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let curr_motion_kind = MotionModule::motion_kind(boma);

        println!("shuriken charge rate: {}", WorkModule::get_float(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE));

        //GA - Protean
        // type: Cancel, then Buff
        // Greninja can decide to not use the substitute counter attack to get the protean buff instead. while the buff is active, the next use of a water or dark move is more powerful
        if curr_motion_kind == hash40("special_lw_hit") || curr_motion_kind == hash40("special_air_lw_hit") {
            if MotionModule::frame(boma) <= 3.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = true;
                FIGHTER_GLOBALS[entry_id as usize].ga_on_confirm = true;
                StopModule::cancel_hit_stop(boma);
                galeforce_apply_effect(boma, 0.5);
                //EffectModule::detach_kind(boma, Hash40::new("sys_counter_flash"), 0);
            }
        }
        //allows greninja to disappear, then cancel the attack
        else if curr_motion_kind == hash40("special_lw_attack") || curr_motion_kind == hash40("special_air_lw_attack") {
            if MotionModule::frame(boma) <= 3.0 && FIGHTER_GLOBALS[entry_id as usize].ga_on_confirm {
                FIGHTER_GLOBALS[entry_id as usize].greninja_ga_xlu = 10;
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
                }
            }
        }
        else {
            FIGHTER_GLOBALS[entry_id as usize].ga_on_confirm = false;
        }
        if FIGHTER_GLOBALS[entry_id as usize].ga_on {
            //cleanup
            if FIGHTER_GLOBALS[entry_id as usize].greninja_ga_xlu > 0 {
                FIGHTER_GLOBALS[entry_id as usize].greninja_ga_xlu -= 1;
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
            }
            else {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            }
            //water shuriken
            if WorkModule::get_float(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) > 0.1 {
                FIGHTER_GLOBALS[entry_id as usize].greninja_shuricharge = WorkModule::get_float(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE);
            }
            if FIGHTER_GLOBALS[entry_id as usize].greninja_shuricharge >= 0.2 
              && [*FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_SHOT, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT].contains(&status_kind) 
              && MotionModule::frame(boma) >= 16.0 {
                FIGHTER_GLOBALS[entry_id as usize].greninja_shuricharge = FIGHTER_GLOBALS[entry_id as usize].greninja_shuricharge / 2.0;
                StatusModule::change_status_force(boma, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_HOLD, false);
                ArticleModule::generate_article(boma, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_SHURIKEN, false, 0);
                WorkModule::set_float(boma, FIGHTER_GLOBALS[entry_id as usize].greninja_shuricharge,*FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE);
                // if situation_kind == *SITUATION_KIND_GROUND {
                //     MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new("special_n_shot"), 1.0, 1.0, 1.0);
                // }
                // else {
                //     MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new("special_air_n_shot"), 1.0, 1.0, 1.0);
                // }
            }
        }
    }
}

//weapons
#[acmd_script( agent = "gekkouga_water", scripts = ["game_specialhil", "game_specialhir"], category = ACMD_GAME, low_priority)]
unsafe fn water_specialhilr(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let weapon_struct =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(lua_state));
    let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon_struct) as u32;

    //weapons using owner_boma stuff
    if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
        let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
        let owner_entry_id = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        if FIGHTER_GLOBALS[owner_entry_id as usize].ga_on {
            frame(lua_state, 1.);
                if macros::is_excute(weapon)
                {
                    macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 58, 80, 0, 60, 4.7, 0.0, 0.0, 1.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
                }
        }
    }
    else {
        frame(lua_state, 1.);
            if macros::is_excute(weapon)
            {
                macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 58, 80, 0, 60, 4.7, 0.0, 0.0, 1.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
            }
        }
}

//global edits
#[acmd_script( agent = "gekkouga", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "gekkouga", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "gekkouga", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.5);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 30, 0, 5.0, 0.0, 27.0, 0.0, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 367, 100, 10, 0, 5.0, 0.0, 27.0, 0.0, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 14.0, 90, 104, 0, 30, 6.5, 0.0, 27.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 1, Hash40::new("haver"), 11.0, 44, 100, 0, 30, 5.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 1, Hash40::new("havel"), 11.0, 44, 100, 0, 30, 5.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 1, Hash40::new("haver"), 11.0, 44, 100, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 1, Hash40::new("havel"), 11.0, 44, 100, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear(boma, 0, false);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 10.0, 40, 98, 0, 30, 3.2, 0.0, 8.0, 12.0, Some(0.0), Some(8.0), Some(-12.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 2, false);
            AttackModule::clear(boma, 3, false);
            AttackModule::clear(boma, 4, false);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}

//air
#[acmd_script( agent = "gekkouga", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 7.);
        for _ in 0..4 {
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 367, 100, 0, 55, 6.2, 0.0, 24.0, 0.0, Some(0.0), Some(20.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
            wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
            wait(lua_state, 1.);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 75, 168, 0, 40, 6.2, 0.0, 18.0, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 75, 168, 0, 40, 7.0, 0.0, 24.0, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 35.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "gekkouga", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 0.66);
            if !FIGHTER_GLOBALS[entry_id as usize].ga_on {
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 14.0, 45, 95, 0, 30, 4.7, 0.0, -8.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 14.0, 45, 95, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 45, 95, 0, 30, 4.0, 0.0, 7.5, 7.0, Some(0.0), Some(8.5), Some(14.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
            else {
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 14.0, 36, 95, 0, 30, 4.7, 0.0, -8.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 14.0, 36, 95, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 36, 95, 0, 30, 4.0, 0.0, 7.5, 7.0, Some(0.0), Some(8.5), Some(14.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].ga_on && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
            MotionModule::set_rate(boma, 1.0);
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 40.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

#[acmd_script( agent = "gekkouga", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 3.0, 367,100, 0, 0, 5.5, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new( "collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.0, 35, 100, 100, 0, 4.3, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("legr"), 3.0, 35, 100, 130, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new( "collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("footr"), 3.0, 45, 10, 0, 90, 5.5, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new( "collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 4, 0, Hash40::new("kneer"), 3.0, 45, 10, 0, 90, 4.3, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 5, 0, Hash40::new("legr"), 3.0, 45, 10, 0, 90, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new( "collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 2.5, 365, 100, 0, 0, 5.5, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new( "collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 2.5, 45, 100, 105, 0, 4.3, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("legr"), 2.5, 35, 100, 140, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new( "collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 6.0, 361, 95, 0, 60, 5.8, 0.0, -1.5, 0.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new( "collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 361, 95, 0, 60, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("legr"), 6.0, 361, 95, 0, 60, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new( "collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 35.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//specials
#[acmd_script( agent = "gekkouga", scripts = ["game_specialhi", "game_specialhi2", "game_specialairhi"], category = ACMD_GAME, low_priority)]
unsafe fn specialhiall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    if FIGHTER_GLOBALS[entry_id as usize].ga_on {
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
            HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_WATER, false, 0);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_WATER, false, 0);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_WATER, false, 0);
        }
    }
    else {
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_WATER, false, 0);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(boma, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_WATER, false, 0);
        }
    }
}

#[acmd_script( agent = "gekkouga", scripts = ["game_specialhifall", "game_specialhi2fall", "game_specialhilandingfall"], category = ACMD_GAME, low_priority)]
unsafe fn specialhifall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    if FIGHTER_GLOBALS[entry_id as usize].ga_on {
        if macros::is_excute(fighter)
        {
            HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
            FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
        }
    }
}

#[acmd_script( agent = "gekkouga", script = "game_specialsattackf", category = ACMD_GAME, low_priority)]
unsafe fn specialsattackf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.9);
            if !FIGHTER_GLOBALS[entry_id as usize].ga_on {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 36, 104, 0, 60, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
            else {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 36, 104, 0, 60, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].ga_on {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                }
            }
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "gekkouga", script = "game_specialsattackb", category = ACMD_GAME, low_priority)]
unsafe fn specialsattackb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.9);
            if !FIGHTER_GLOBALS[entry_id as usize].ga_on {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 48, 103, 0, 60, 8.0, 0.0, 8.0, -15.0, Some(0.0), Some(8.5), Some(-12.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
            else {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 48, 103, 0, 60, 8.0, 0.0, 8.0, -15.0, Some(0.0), Some(8.5), Some(-12.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].ga_on {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                }
            }
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "gekkouga", script = "game_specialairsattackb", category = ACMD_GAME, low_priority)]
unsafe fn specialairsattackb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::SET_SPEED_EX(fighter, 0, 2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.9);
            if !FIGHTER_GLOBALS[entry_id as usize].ga_on {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 48, 103, 0, 60, 8.0, 0.0, 8.0, -15.0, Some(0.0), Some(8.5), Some(-12.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
            else {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 48, 103, 0, 60, 8.0, 0.0, 8.0, -15.0, Some(0.0), Some(8.5), Some(-12.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].ga_on {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                }
            }
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "gekkouga", script = "game_specialairsattackf", category = ACMD_GAME, low_priority)]
unsafe fn specialairsattackf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::SET_SPEED_EX(fighter, 0, 2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.9);
            if !FIGHTER_GLOBALS[entry_id as usize].ga_on {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 36, 104, 0, 60, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
            else {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 36, 104, 0, 60, 8.5, 0.0, 16.0, 14.5, Some(0.0), Some(9.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            if FIGHTER_GLOBALS[entry_id as usize].ga_on {
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                }
            }
            AttackModule::clear_all(boma);
        }
}

//grabs and throws
#[acmd_script( agent = "gekkouga", script = "game_catchattack", category = ACMD_GAME, low_priority)]
unsafe fn catchattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            if !FIGHTER_GLOBALS[entry_id as usize].ga_on {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 30, 0, 6.0, 0.0, 6.0, 10.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
            }
            else {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 100, 30, 0, 6.0, 0.0, 6.0, 10.0, None, None, None, 1.45, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
            }
            AttackModule::set_catch_only_all(boma, true, false);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}

//others
#[acmd_script( agent = "gekkouga", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        greninja_not_amphinobi_frame
    );
    smashline::install_acmd_scripts!(
        water_specialhilr,
        dash,
        turndash,
        attackhi4,
        attackairhi,
        attackairf,
        attackairb,
        specialhiall,
        specialhifall,
        specialsattackf,
        specialairsattackf,
        specialsattackb,
        specialairsattackb,
        catchattack,
        escapeairslide
    );
}