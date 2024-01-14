use super::*;
use crate::fighter::common::opff::common_fighter_frame;

    //GA - Terry Go!
    //type: buff
    //grants access to super specials. yep that's it :)
    //more code in notify_log_event hook
pub unsafe extern "C" fn dolly_galeforce_attack(fighter : &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 && VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM) {
        VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
        galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM) {
        zelda_buff_effect(fighter);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
        }
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
    }
    //cleanup
    if DamageModule::damage(fighter.module_accessor, 0) < 100.0 || [hash40("super_special"), hash40("super_special2_blow")].contains(&curr_motion_kind) {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
    }
}


pub unsafe extern "C" fn dolly_specialn_charge(fighter : &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if VarModule::get_float(fighter.module_accessor, dolly::instance::float::SPECIAL_N_CHARGE) >= 1.0 {
        VarModule::set_float(fighter.module_accessor, dolly::instance::float::SPECIAL_N_CHARGE, 1.0);
    }
    if curr_motion_kind == hash40("special_n") || curr_motion_kind == hash40("special_air_n") {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
            if MotionModule::frame(fighter.module_accessor) >= 6.0 && MotionModule::frame(fighter.module_accessor) < 14.0 
            && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                MotionModule::set_rate(fighter.module_accessor, 0.2);
                VarModule::add_float(fighter.module_accessor, dolly::instance::float::SPECIAL_N_CHARGE, 0.015);
            }
            else {
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        }
    }
    else {
        VarModule::set_float(fighter.module_accessor, dolly::instance::float::SPECIAL_N_CHARGE, 0.0);
    }
}

unsafe extern "C" fn dolly_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    dolly_galeforce_attack(fighter);
    dolly_specialn_charge(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, dolly_frame);
}