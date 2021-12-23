use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lib::LuaConst; //required for GA sounds
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;
use smash::app::FighterSpecializer_Reflet;
use smash::phx::Vector2f;
use std::mem;

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;
use crate::utils::*;

static mut ATTACK_SOUND_TYPE : LuaConst = COLLISION_SOUND_ATTR_CUTUP;

#[fighter_frame( agent = FIGHTER_KIND_REFLET )]
fn strategist_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let bo = smash::app::sv_system::battle_object(lua_state);
        let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
        let instance_boma = mem::transmute::<&mut smash::app::BattleObjectModuleAccessor, &mut smash::app::FighterModuleAccessor>(boma);

        if FIGHTER_GLOBALS[entry_id as usize].ga_on {
            ATTACK_SOUND_TYPE = COLLISION_SOUND_ATTR_MAGIC;
        }
        else if !WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK_SOUND_TYPE = COLLISION_SOUND_ATTR_CUTUP;
        }
        else {
            ATTACK_SOUND_TYPE = COLLISION_SOUND_ATTR_ELEC;
        }

        //Robin (reflet) Galeforce attack: Ignis
        // type: buff
        //  when thunder is charged up to arcthunder and thoron, Robin can sacrifice the charge to empower a normal, non-smash attack. 
        if !is_operation_cpu(boma) {
            println!("motion_frame: {}", MotionModule::frame(boma));
            println!("ga_motion_frame: {}", FIGHTER_GLOBALS[entry_id as usize].reflet_ga_motion_frame);
            robin_ignis_effect(boma);
            if AttackModule::is_attack(boma, 0, false) && FIGHTER_GLOBALS[entry_id as usize].ga_on {
                FIGHTER_GLOBALS[entry_id as usize].reflet_attack_occur = true;
            }
            //if robin tries to activate their GA and the frame # is lower than it was last frame, cleanup
            if MotionModule::frame(boma) - FIGHTER_GLOBALS[entry_id as usize].reflet_ga_motion_frame < 0.0 {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                FIGHTER_GLOBALS[entry_id as usize].reflet_attack_occur = false;
                FIGHTER_GLOBALS[entry_id as usize].once = false;
                FIGHTER_GLOBALS[entry_id as usize].reflet_ga_motion_frame = 0.;
                AttackModule::set_power_mul(boma, 1.0);
                WorkModule::set_int(boma, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
                FighterSpecializer_Reflet::exit_special_n_tron(instance_boma);
            }
            //activating the GA before the first active frame
            if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND) >= 2 {
                if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) 
                && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !FIGHTER_GLOBALS[entry_id as usize].reflet_attack_occur  {
                    FIGHTER_GLOBALS[entry_id as usize].once = true;
                    FIGHTER_GLOBALS[entry_id as usize].ga_on = true;
                    FIGHTER_GLOBALS[entry_id as usize].reflet_ga_motion_frame = MotionModule::frame(boma);
                    FighterSpecializer_Reflet::change_hud_kind(instance_bo, 3);
                    FighterSpecializer_Reflet::change_grimoire(instance_boma, 3);
                    FighterSpecializer_Reflet::get_active_thunder_num(instance_boma, 0);
                }
            }
            //applies power modifier throughout an entire move
            if FIGHTER_GLOBALS[entry_id as usize].ga_on {
                if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND) == 2 {
                    if !FIGHTER_GLOBALS[entry_id as usize].once {
                        WorkModule::set_int(boma, WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) - 5, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
                    }
                    if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                        AttackModule::set_power_mul(boma, 1.1);
                    }
                    else {
                        AttackModule::set_power_mul(boma, 1.25);
                    }
                }
                else { //necessarily at 3
                    if !FIGHTER_GLOBALS[entry_id as usize].once {
                        WorkModule::set_int(boma, WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) - 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
                    }
                    if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                        AttackModule::set_power_mul(boma, 1.2);
                    }
                    else {
                        AttackModule::set_power_mul(boma, 1.4);
                    }
                }
                if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                    galeforce_apply_effect(boma, 0.75);
                }
            }
        }

        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL && FIGHTER_GLOBALS[entry_id as usize].reflet_thunder_fx_handle == 0 {
            FIGHTER_GLOBALS[entry_id as usize].reflet_thunder_fx_handle = EffectModule::get_last_handle(boma) as u32;
        }

        if FIGHTER_GLOBALS[entry_id as usize].reflet_thunder_fx_handle != 0 && !FIGHTER_GLOBALS[entry_id as usize].ga_on {
            EffectModule::kill(boma, FIGHTER_GLOBALS[entry_id as usize].reflet_thunder_fx_handle, false, true);
            FIGHTER_GLOBALS[entry_id as usize].reflet_thunder_fx_handle = 0;
        }

        //remove thunder charge with taunt
        if curr_motion_kind == hash40("appeal_lw_r") || curr_motion_kind == hash40("appeal_lw_l") {
            WorkModule::set_int(boma, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
        }
    }
}

//global edits
#[acmd_script( agent = "reflet", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "reflet", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "reflet", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 361, 70, 0, 62, 5.0, 0.0, 8.5, 14.0, Some(8.5), Some(8.0),Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "reflet", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.0, 96, 80, 0, 50, 3.5, 0.0, 5.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 6.0, 96, 80, 0, 60, 3.5, 0.0, 1.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 6.0, 96, 80, 0, 70, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "reflet", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.5, 4.0);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 0.0, 5.0);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 30, 40, 0, 40, 3.5, 0.0, 3.0, 11.0, Some(0.0), Some(5.0),Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 5.0);
            AttackModule::clear_all(boma);
        }
}

#[acmd_script( agent = "reflet", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);


    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    WorkModule::set_flag(boma, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_thunder") as i64);
                }
                else {
                    WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                }
            }
        }
    frame(lua_state, 8.);
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                if macros::is_excute(fighter)
                {
                    WorkModule::add_int(boma, -1, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 2.0, 15, 20, 0, 40, 4.0, 0.0, 0.0, -0.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 2.0, 15, 20, 0, 20, 4.0, 0.0, 7.4, -0.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            wait(lua_state, 8.);
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.0, 45, 95, 0, 80, 5.0, 0.0, 6.5, -0.5, Some(0.0), Some(1.0), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            wait(lua_state, 3.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                    if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                        let bo = smash::app::sv_system::battle_object(lua_state);
                        let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
                        let throwvec : Vector2f = Vector2f{x: -4.5, y: 9.0};
                        FighterSpecializer_Reflet::throwaway_sword(instance_bo, &throwvec, true);
                    }
                }
        }
        else {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 10.0, 45, 66, 0, 80, 4.0, 0.0, 7.4, -0.5, Some(0.0), Some(0.0), Some(-0.5), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            wait(lua_state, 3.);
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.0, 65, 60, 0, 80, 3.5, 0.0, 6.5, -0.5, Some(0.0), Some(1.0), Some(-0.5), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            wait(lua_state, 8.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
        }
}

//air
#[acmd_script( agent = "reflet", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
                if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                        WorkModule::set_flag(boma, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_thunder") as i64);
                    }
                    else {
                        WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                    }
                }
            }
        frame(lua_state, 10.);
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 13.0, 95, 90, 0, 48, 5.0, 0.0, 6.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 13.0, 95, 90, 0, 48, 5.0, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 95, 90, 0, 48, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    }
                wait(lua_state, 4.);
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        AttackModule::clear(boma, 1, false);
                        AttackModule::clear(boma, 2, false);
                    }
                frame(lua_state, 24.);
                    if macros::is_excute(fighter)
                    {
                        AttackModule::clear_all(boma);
                        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                            WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                            VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                            let bo = smash::app::sv_system::battle_object(lua_state);
                            let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
                            let throwvec : Vector2f = Vector2f{x: -4.5, y: 9.0};
                            FighterSpecializer_Reflet::throwaway_sword(instance_bo, &throwvec, true);
                        }
                    }
            }
            else
            {
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 7.8, 95, 90, 0, 48, 3.0, 0.0, 6.0, -1.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 7.8, 95, 90, 0, 48, 3.0, 0.0, 2.0, -1.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 7.8, 95, 90, 0, 48, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    }
                wait(lua_state, 5.);
                    if macros::is_excute(fighter)
                    {
                        AttackModule::clear_all(boma);
                    }
            }
        frame(lua_state, 27.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "reflet", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                        WorkModule::set_flag(boma, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_thunder") as i64);
                    }
                    else {
                        WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                    }
                }
            }
        frame(lua_state, 6.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 12.);
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON)
                {
                        if macros::is_excute(fighter)
                        {
                            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 12.5, 55, 100, 0, 50, 5.0, 0.0, 7.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                            macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 12.5, 55, 100, 0, 50, 5.0, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.5,  55, 100, 0, 50, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        }
                    wait(lua_state, 4.);
                        if macros::is_excute(fighter)
                        {
                            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                            AttackModule::clear(boma, 1, false);
                            AttackModule::clear(boma, 2, false);
                        }
                    wait(lua_state, 4.);
                        if macros::is_excute(fighter)
                        {
                            AttackModule::clear_all(boma);
                        }
                    frame(lua_state, 22.);
                        if macros::is_excute(fighter)
                        {
                            if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0
                            {
                                WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                                VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                                let bo = smash::app::sv_system::battle_object(lua_state);
                                let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
                                let throwvec : Vector2f = Vector2f{x: -6.0, y: 15.0};
                                FighterSpecializer_Reflet::throwaway_sword(instance_bo, &throwvec, true);
                            }
                        }
                }
            else
            {
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 7.5, 361, 100, 0, 50, 2.5, 0.0, 6.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 7.5, 361, 100, 0, 50, 2.5, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"),  7.5, 361, 100, 0, 50, 2.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    }
                wait(lua_state, 5.);
                    if macros::is_excute(fighter)
                    {
                        AttackModule::clear_all(boma);
                    }
            }
        frame(lua_state, 27.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "reflet", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                macros::FT_MOTION_RATE(fighter, 0.75);
            }
        frame(lua_state, 6.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
                if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                        WorkModule::set_flag(boma, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_thunder") as i64);
                    }
                    else {
                        WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                    }
                }
            }
        frame(lua_state, 8.);
            if macros::is_excute(fighter)
            {
                macros::FT_MOTION_RATE(fighter, 1.0);
            }
        frame(lua_state, 9.);
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON)
            {
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 11.5, 30, 64, 0, 60, 5.0, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 11.5, 30, 64, 0, 60, 5.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 11.5, 30, 64, 0, 60, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    }
                frame(lua_state, 13.);
                    if macros::is_excute(fighter)
                    {
                        AttackModule::clear_all(boma);
                    }
                frame(lua_state, 22.);
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 11.5, 30, 64, 0, 60, 5.0, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 11.5, 30, 64, 0, 60, 5.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 11.5, 30, 64, 0, 60, 5.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    }
                frame(lua_state, 26.);
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 45, 80, 0, 60, 3.0, 0.0, 8.0, -1.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        AttackModule::clear(boma, 1, false);
                        AttackModule::clear(boma, 2, false);
                    }
                wait(lua_state, 5.);
                    if macros::is_excute(fighter)
                    {
                        AttackModule::clear_all(boma);
                    }
                frame(lua_state, 34.);
                    if macros::is_excute(fighter)
                    {
                        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0
                        {
                            WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                            VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                            let bo = smash::app::sv_system::battle_object(lua_state);
                            let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
                            let throwvec : Vector2f = Vector2f{x: -3.0, y: 17.0};
                            FighterSpecializer_Reflet::throwaway_sword(instance_bo, &throwvec, true);
                        }
                    }
                frame(lua_state, 42.);
                    if macros::is_excute(fighter)
                    {
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
                    }
            }
            else 
            {
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.9, 30, 55, 0, 60, 3.0, 0.0, 5.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 6.9, 30, 55, 0, 60, 3.0, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 6.9, 30, 55, 0, 60, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    }
                frame(lua_state, 13.);
                    if macros::is_excute(fighter)
                    {
                        AttackModule::clear_all(boma);
                    }
                frame(lua_state, 22.);
                    if macros::is_excute(fighter)
                    {
                        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.9, 30, 64, 0, 60, 3.0, 0.0, 5.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 6.9, 30, 64, 0, 60, 3.0, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 6.9, 30, 64, 0, 60, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    }
                frame(lua_state, 28.);
                    if macros::is_excute(fighter)
                    {
                        AttackModule::clear_all(boma);
                    }
                frame(lua_state, 34.);
                    if macros::is_excute(fighter)
                    {
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
                    }
            }
}

#[acmd_script( agent = "reflet", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    WorkModule::set_flag(boma, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_thunder") as i64);
                }
                else {
                    WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                }
            }
        }
    frame(lua_state, 9.);
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON)
        {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 15.0, 361, 93, 0, 40, 5.0, 0.0, 7.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 15.0, 361, 93, 0, 40, 5.0, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("armr"),  15.0, 361, 93, 0, 40, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            wait(lua_state, 3.);
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 361, 80, 0, 45, 3.0, -1.0, 8.0, -1.0, Some(-1.0), Some(2.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    AttackModule::clear(boma, 1, false);
                    AttackModule::clear(boma, 2, false);
                }
            wait(lua_state, 5.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
            frame(lua_state, 21.);
                if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0
                {
                    WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                    let bo = smash::app::sv_system::battle_object(lua_state);
                    let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
                    let throwvec : Vector2f = Vector2f{x: -8.0, y: 14.0};
                    FighterSpecializer_Reflet::throwaway_sword(instance_bo, &throwvec, true);
                }
            frame(lua_state, 34.);
                if macros::is_excute(fighter)
                {
                    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
                }
        }
        else
        {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 9.0, 361, 93, 0, 40, 2.5, 0.0, 6.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 9.0, 361, 93, 0, 40, 2.5, 0.0, 2.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 9.0, 361, 93, 0, 40, 2.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            wait(lua_state, 3.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
            frame(lua_state, 30.);
                if macros::is_excute(fighter)
                {
                    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
                }
        }
}

#[acmd_script( agent = "reflet", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);


    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    WorkModule::set_flag(boma, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_thunder") as i64);
                }
                else {
                    WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                }
            }
        }
    frame(lua_state, 13.);
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON)
        {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 270, 100, 0, 20, 4.5, 0.0, -8.0, -0.6, Some(0.0), Some(-4.0), Some(-0.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            frame(lua_state, 16.);
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 10.0, 65, 80, 0, 55, 4.2, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            frame(lua_state, 25.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
            frame(lua_state, 27.);
                if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0
                {
                    WorkModule::set_flag(boma, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_normal") as i64);
                    let bo = smash::app::sv_system::battle_object(lua_state);
                    let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
                    let throwvec : Vector2f = Vector2f{x: -1.0, y: 0.0};
                    FighterSpecializer_Reflet::throwaway_sword(instance_bo, &throwvec, true);
                }
        }
        else
        {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 7.2, 270, 80, 0, 55, 3.7, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            wait(lua_state, 3.);
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 7.2, 361, 80, 0, 45, 2.5, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *ATTACK_SOUND_TYPE, *ATTACK_REGION_SWORD);
                }
            frame(lua_state, 25.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(boma);
                }
        }
        frame(lua_state, 48.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

//other
#[acmd_script( agent = "reflet", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        strategist_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attacks3,
        attackhi3,
        attacklw3,
        attackdash,
        attackairhi,
        attackairf,
        attackairn,
        attackairb,
        attackairlw,
        escapeairslide
    );
}