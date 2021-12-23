use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;
use smash::phx::Vector3f;

//use crate::FIGHTER_CUTIN_MANAGER_ADDR;
//use crate::custom::FIGHTER_GLOBALS;
//use crate::custom::galeforce::*;
//use crate::custom::utils::*;

#[fighter_frame( agent = FIGHTER_KIND_TRAIL )]
fn funnymouse_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        if WorkModule::is_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N3_HOP)
          || WorkModule::is_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP)
          || WorkModule::is_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N1_HOP) {
              WorkModule::on_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N3_HOP);
              WorkModule::on_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP);
              WorkModule::on_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N1_HOP);
          }
        else {
              WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N3_HOP);
              WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP);
              WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N1_HOP);
        }
    }
}

//global edits
#[acmd_script( agent = "trail", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "trail", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//other
#[acmd_script( agent = "trail", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        funnymouse_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        escapeairslide
    );
}