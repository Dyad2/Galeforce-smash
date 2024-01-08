#![allow(unused_must_use)]

use super::*;

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_Landing)]
unsafe fn bac_status_landing(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Landing()
}

#[skyline::hook(replace = L2CFighterCommon_status_Landing)]
unsafe fn status_landing(fighter: &mut L2CFighterCommon) -> L2CValue {
    //clears damage speed when using wavedash/land
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH) {
        fighter.clear_lua_stack();
        smash_script::lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, 0.0, 0.0);
        smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }

    fighter.status_LandingSub();
    fighter.status_LandingStiffness();
    fighter.sub_landing_start_check_damage_face();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Landing_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_Landing_Main)]
unsafe extern "C" fn bac_status_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Landing_Main()
}

#[skyline::hook(replace = L2CFighterCommon_status_Landing_Main)]
unsafe extern "C" fn status_LandingMain(fighter: &mut L2CFighterCommon) -> L2CValue {

    //checks if fighter is using wavedash, and enables some statuses accordingly
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH) {
        if fighter.global_table[MOTION_FRAME].get_i32() >= 12 {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    if !fighter.status_Landing_MainSub().get_bool() {
        fighter.sub_landing_cancel_check_damage_face();
    }
    return 1.into();
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Landing)]
unsafe extern "C" fn bac_status_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Landing()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Landing)]
unsafe extern "C" fn status_LandingEnd(fighter: &mut L2CFighterCommon) -> L2CValue {

    fighter.sub_landing_cancel_damage_face();
    VarModule::off_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH);
        
    return 0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_landing_fall_special)]
unsafe extern "C" fn bac_status_LandingEndFallSpecial(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Landing()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_landing_fall_special)]
unsafe extern "C" fn status_LandingEndFallSpecial(fighter: &mut L2CFighterCommon) -> L2CValue {

    VarModule::off_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH);
        
    return 0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            bac_status_landing_main,
            status_LandingMain,
            bac_status_landing,
            status_landing,
            bac_status_landing_end,
            status_LandingEnd,
            bac_status_LandingEndFallSpecial,
            status_LandingEndFallSpecial
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}