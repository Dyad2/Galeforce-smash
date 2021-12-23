use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;
use crate::utils::*;

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
fn noodle_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        //GA - ?
        // type: cancel
        //  allows minmin to cancel up tilt with grounded up special for vertical combos
        if !is_operation_cpu(boma) {
            if curr_motion_kind == hash40("attack_hi3") {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                        FIGHTER_GLOBALS[entry_id as usize].ga_on = true;
                    }
                }
                if !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                    if FIGHTER_GLOBALS[entry_id as usize].ga_on && MotionModule::frame(boma) >= 15.0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
                        galeforce_apply_effect(boma, 0.75);
                        FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                    }
                }
            }
            else {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
        }
    }
}

//global edits
#[acmd_script( agent = "tantan", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "tantan", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "tantan", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 4.5);
        }
        macros::FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 6.);
        macros::FT_MOTION_RATE(fighter, 1.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 180, 20, 0, 20, 3.0, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 25, 0, 25, 3.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(15.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 8., false);
            AttackModule::set_add_reaction_frame(boma, 1, 8., false);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        }
}

#[acmd_script( agent = "tantan", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

            if macros::is_excute(fighter)
            {
                FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.5, 4.5);
            }
        frame(lua_state, 8.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 6, 0, 50, 4.0, 0.0, 4.0, 4.0, Some(0.0), Some(4.0), Some(14.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 60, 20, 0, 30, 3.5, 0.0, 4.0, 4.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 80, 20, 0, 30, 4.0, 0.0, 4.0, 9.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 93, 20, 0, 30, 4.0, 0.0, 4.0, 14.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
                AttackModule::set_add_reaction_frame(boma, 0, 8., false);
                AttackModule::set_add_reaction_frame(boma, 1, 8., false);
                AttackModule::set_add_reaction_frame(boma, 2, 8., false);
                AttackModule::set_add_reaction_frame(boma, 3, 8., false);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
        frame(lua_state, 14.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            }
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
            }
        frame(lua_state, 17.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
            }
}

#[acmd_script( agent = "tantan", script = "game_attack13", category = ACMD_GAME, low_priority)]
unsafe fn attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 4.5);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 361, 90, 0, 60, 5.5, 0.0, 5.5, 4.0, Some(0.0), Some(12.0), Some(9.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}


#[acmd_script( agent = "tantan", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.5, 6.5, 9.0, 1.5);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 10.0, 40, 50, 0, 85, 4.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 40, 50, 0, 85, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 40, 50, 0, 85, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 8.0, 40, 50, 0, 85, 4.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.0, 40, 50, 0, 85, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 8.0, 40, 50, 0, 85, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.0, 2.9, 7.2, 7.2);
        }
}

//special
#[acmd_script( agent = "tantan", script = "game_specialhishort", category = ACMD_GAME, low_priority)]
unsafe fn specialhishort(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 1.);
            if macros::is_excute(fighter)
            {
               macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("armr4"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("arml5"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("arml4"), *HIT_STATUS_XLU);
            }
        frame(lua_state, 4.);
            if macros::is_excute(fighter)
            {
                macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 20, 0, 85, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
}

#[acmd_script( agent = "tantan", script = "game_specialhilong", category = ACMD_GAME, low_priority)]
unsafe fn specialhilong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 1.);
            if macros::is_excute(fighter)
            {
               macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("armr4"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("arml5"), *HIT_STATUS_XLU);
               macros::HIT_NODE(fighter, Hash40::new("arml4"), *HIT_STATUS_XLU);
            }
        frame(lua_state, 4.);
            if macros::is_excute(fighter)
            {
                macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 20, 0, 100, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
}

//other
#[acmd_script( agent = "tantan", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        noodle_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attack11,
        attack12,
        attack13,
        attackdash,
        specialhishort,
        specialhilong,
        escapeairslide
    );
}