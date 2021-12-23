use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::utils::*;
use crate::var::*;

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let curr_motion_kind = MotionModule::motion_kind(boma);

        if (curr_motion_kind == hash40("special_air_hi") || curr_motion_kind == hash40("special_hi")) && MotionModule::frame(boma) >= 45.0 {
            StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, false);
            WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI)
        }
        if is_special_reset(boma) {
            WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI)
        }
    }
}

//global edits
#[acmd_script( agent = "richter", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "richter", script = "game_turndash", category = ACMD_GAME, low_priority)]
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
#[acmd_script( agent = "richter", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.33);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 1.5, 0.0, 8.5, 7.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 1.6, 0.0, 8.5, 9.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 20, 0, 15, 1.7, 0.0, 8.5, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 180, 20, 0, 20, 1.8, 0.0, 8.5, 14.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 1.8, 0.0, 8.5, 14.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
            AttackModule::set_add_reaction_frame(boma, 0, 2.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 2.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 2.0, false);
            AttackModule::set_add_reaction_frame(boma, 3, 2.0, false);
            AttackModule::set_add_reaction_frame(boma, 4, 2.0, false);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            CancelModule::enable_cancel(boma);
        }
}

#[acmd_script( agent = "richter", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 5.0, 361, 5, 0, 55, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 5.0, 361, 5, 0, 55, 2.8, -1.5, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "richter", script = "game_attacklw32", category = ACMD_GAME, low_priority)]
unsafe fn attacklw32(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.25);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(boma, false);
            WorkModule::on_flag(boma, *FIGHTER_SIMON_STATUS_ATTACK_LW32_WORK_ID_FLAG_LANDING_AIR);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 7.0, 361, 55, 0, 90, 4.0, -1.0, 1.5, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 7.0, 361, 55, 0, 90, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("hip"), 7.0, 361, 55, 0, 90, 5.0, -4.0, 0.0, -4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear(boma, 3, false);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 3.5, 50, 40, 0, 60, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 3.5, 50, 40, 0, 60, 3.0, -1.5, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::clear(boma, 2, false);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            JostleModule::set_status(boma, true);
        }
}

//other
#[acmd_script( agent = "richter", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.2);
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 6.0);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
            GrabModule::set_rebound(boma, true);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 6.6, 2.35, Some(0.0), Some(6.6), Some(10.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(boma, false);
        }
}

#[acmd_script( agent = "richter", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.2);
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.0, 4.0);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            GrabModule::set_rebound(boma, true);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.3, 0.0, 6.6, 2.7, Some(0.0), Some(6.6), Some(12.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
     macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(boma, false);
        }
}

#[acmd_script( agent = "richter", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.2);
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 6.5, 6.0);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.0);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            GrabModule::set_rebound(boma, true);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 6.6, -2.35, Some(0.0), Some(6.6), Some(-16.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(boma, false);
        }
}

//other
#[acmd_script( agent = "richter", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            //WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
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
        richter_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attack11,
        attacklw3,
        attacklw32,
        catch,
        catchdash,
        catchturn,
        escapeairslide,
    );
}