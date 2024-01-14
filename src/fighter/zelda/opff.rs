use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA - Hylia's helping hand
// type: cancel / restriction lift
//  allows zelda to stop charging phantom by shielding / airdodging. The phantom will continue charging
unsafe extern "C" fn zelda_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    
    if !is_operation_cpu(fighter.module_accessor) {
        if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            zelda_buff_effect(fighter);
            if [hash40("special_lw"), hash40("special_air_lw")].contains(&curr_motion_kind) && MotionModule::frame(fighter.module_accessor) > 10.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                    else if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                }
            }
        }
        if ![hash40("special_hi_start"), hash40("special_air_hi_start"), hash40("special_hi"), hash40("special_air_hi")].contains(&curr_motion_kind) {
            VarModule::off_flag(fighter.module_accessor, zelda::instance::flag::SPECIAL_HI_CANCEL);
        }
    }
}

unsafe extern "C" fn zelda_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    zelda_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, zelda_frame);
}