use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

use crate::fighters::common::*;

pub unsafe fn edge_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {

    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    // general edge cancelling
    if FIGHTER_GLOBALS[entry_id as usize].wavedash && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        GroundModule::set_correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    if !(FIGHTER_GLOBALS[entry_id as usize].wavedash && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)) 
       && ([*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_WALK ].contains(&status_kind) 
      || (!WorkModule::is_flag(boma,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL) && status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)
      || (status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs() > 0.90)) {
        GroundModule::set_correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    //character specific cancels
    // if fighter_kind == *FIGHTER_KIND_BAYONETTA {
    //     if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING {
    //         if MotionModule::frame(boma) <= 5.0 {
    //             GroundModule::set_correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    //         }
    //         else {
    //             GroundModule::set_correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    //         }
    //     }
    // }
    if fighter_kind == *FIGHTER_KIND_DIDDY {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) <= 20.0 {
                GroundModule::set_correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            else {
                GroundModule::set_correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_MASTER && [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_LANDING, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH].contains(&status_kind) {
        GroundModule::set_correct(boma, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
}	